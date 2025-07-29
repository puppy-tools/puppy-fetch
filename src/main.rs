mod enums;
mod context;
mod git_utils;
mod clap_impl;
mod logging;

use context::Context;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use git2::{ObjectType, Repository, TreeWalkMode, TreeWalkResult};

use crate::enums::context_flow::ContextFlow;
use crate::enums::reference_category::ReferenceCategory;
use crate::git_utils::reference::{valid_references, is_symbolic_reference};

fn main() -> Result<(), Box<dyn Error>> {
    let ctx = match Context::new().init() {
        ContextFlow::Initialized(ctx) => ctx,
        ContextFlow::HaltOk => return Ok(()),
        ContextFlow::HaltErr(err) => return Err(err)
    };

    traceln!("Context initialized.");
    debugln!("{}", ctx.args_as_str());

    let repo = Repository::try_from(&ctx)?;
    
    for reference in valid_references(&repo)? {
        let ref_name = if let Some(ref_name) = reference.name() { ref_name } else { continue };
        let ref_name_short = if let Some(s) = ref_name.rsplit_once("/") { s.1 } else { continue };

        if is_symbolic_reference(&reference) { 
            traceln!("Skipping symbolic reference, '{ref_name}' ...");
            continue; 
        } 

        if let Some(regex) = ctx.ref_regex() {
            if !regex.is_match(ref_name_short) {
                traceln!(
                    "Skipping reference, /{}/ didn't match against, '{}' ...", 
                    regex.as_str(),
                    ref_name_short
                );            
            }
        }

        let category = ReferenceCategory::from(&reference);

        if !ctx.allowed_category(category.clone()) { 
            traceln!("Skipping reference, category '{category:?}' was filtered out ...");
            continue; 
        }
        
        let tree = match reference.peel_to_tree() {
            Ok(tree) => {
                traceln!("Successfully peeled reference, '{ref_name_short}', to tree.");
                tree
            },
            Err(e) => {
                debugln!("Couldn't peel reference, '{ref_name_short}', to tree.\n{e}");
                continue;
            }
        };

        let _ = tree.walk(TreeWalkMode::PreOrder, |root, entry| {
            if !ctx.allowed_search_dir(root) { return TreeWalkResult::Ok }
            
            let node_name = match entry.name() {
                Some(node_name) => node_name,
                None => {
                    debugln!("Entry name was not valid UTF-8, skipping node...");
                    return TreeWalkResult::Ok
                }
            };

            match entry.kind() {
                Some(kind) => {
                    if kind != ObjectType::Blob { 
                        debugln!("Object type was not 'Blob'. Skipping node, '{node_name}'...");
                        return TreeWalkResult::Ok;
                    } else {
                        kind
                    }
                },
                None => {
                    debugln!("Couldn't get the object type for, '{node_name}'. Skipping node...");
                    return TreeWalkResult::Ok;
                },
            };

            if let Some(regex) = ctx.blob_name_regex() {
                if !regex.is_match(&node_name) {
                    traceln!("Skipping node, /{}/ didn't match against, '{node_name}' ...", regex.as_str());
                    return TreeWalkResult::Ok;
                }
            }

            let obj = match entry.to_object(&repo) {
                Ok(obj) => {
                    traceln!("Successfully converted entry, '{node_name}', to object.");
                    obj
                },
                Err(e) => {
                    debugln!("Couldn't convert entry: '{node_name}', to object. Skipping node...\n{e}");
                    return TreeWalkResult::Ok;
                },
            };

            let blob = match obj.into_blob() {
                Ok(blob) => {
                    traceln!("Successfully converted object, '{node_name}', to blob.");
                    blob
                },
                Err(_) => {
                    debugln!("Couldn't convert object, '{node_name}', to blob...");
                    return TreeWalkResult::Ok;
                },
            };

            let write_location = format!("{}/{}/{}", ctx.destination(), ref_name_short, root);

            if let Ok(dir_exists) = std::fs::exists(Path::new(&write_location)) {
                if !dir_exists {
                    traceln!("Directory, '{write_location}', doesn't exist, attempting to create valid path.");
                    if let Err(_) = std::fs::create_dir_all(write_location.clone()) { 
                        debugln!("Couldn't create one or more directories to: '{write_location}'.");
                        return TreeWalkResult::Ok; 
                    }
                }
            }

            let qualified_file_path = format!("{write_location}/{node_name}");

            traceln!("Attempting to open/create file at, '{qualified_file_path}'");
            let file_open_result = OpenOptions::new()
                .create(true)
                .write(true)
                .open(&format!("{write_location}/{node_name}"))
            ;

            match file_open_result {
                Ok(mut file) => {
                    traceln!("Successfully opened file, '{node_name}'.");
                    traceln!("Attempting to write blob content to file, '{node_name}'.");
                    match file.write_all(blob.content()) {
                        Ok(_) => traceln!("Successfully wrote {} bytes to, '{node_name}'", blob.content().len()),
                        Err(e) => debugln!("Couldn't write to file, '{node_name}'...\n{e}")
                    };
                }
                Err(e) => {
                    debugln!("File couldn't be opened/created.\n{e}");
                    return TreeWalkResult::Ok;
                }
            }

            TreeWalkResult::Ok
        });
    }

    /*
        The way things are currently structured means that this statement might be a misnomer.
        During the tree walk, there are a number of fail points that could be considered recoverable...
        however there are also a number that may be considered unrecoverable.

        For example, the fetch operation would ideally continue if a reference had a bad utf-encoding.
        Failure to write to a file however, may be considered unrecoverable.
        These are opinionated errors, and I am considering all of them recoverable for the time being.
        This will likely be handled differently in later maintained versions...
        potentially involving clap arguments or a more opinionated way of error handling.
    */
    traceln!("Fetch operation succeeded!");

    Ok(())
}
