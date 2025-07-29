use clap::{crate_version, Command, CommandFactory, Parser};
use log::LevelFilter;
use regex::Regex;

use crate::clap_impl::args::Args;

use crate::enums::{
    auth_method::AuthMethod,
    context_flow::ContextFlow,
    source_type::SourceType,
    level_filter_arg::LevelFilterArg,
    reference_category::ReferenceCategory
};


pub struct Context {
    cmd: Command,
    args: Args,
}

impl Context {
    pub fn new() -> Self { 
        Self { 
            cmd: Args::command(), 
            args: Args::parse() 
        } 
    }

    pub fn init(mut self) -> ContextFlow {
        if self.args.help { 
            self.print_help();
            return ContextFlow::HaltOk 
        }

        if self.args.version { 
            self.print_version();
            return ContextFlow::HaltOk 
        }

        if let Err(e) = crate::logging::plain_logger(self.log_level()) {
            return ContextFlow::HaltErr(Box::new(e))
        }

        ContextFlow::Initialized(self)
    }

    pub fn print_help(&mut self) { let _ = self.cmd.print_help(); }
    pub fn print_version(&self) { print!("{}", crate_version!()) }

    pub fn log_level(&self) -> LevelFilter {
        self.args.log_level.unwrap_or(LevelFilterArg::Info).into() 
    }

    pub fn source_type(&self) -> SourceType {
        if self.args.local { SourceType::Local }
        else if self.args.remote { SourceType::Remote }
        else { SourceType::Local }
    }

    pub fn source(&self) -> String {
        match self.args.source.clone() {
            Some(source) => source,
            None => match self.source_type() {
                SourceType::Local => ".".to_string(),
                SourceType::Remote => panic!("Source must be supplied for a remote"),
            }
        }
    }

    pub fn destination(&self) -> String {
        match self.args.destination.clone() {
            Some(destination) => destination,
            None => match self.source_type() {
                SourceType::Local => "out".to_string(),
                SourceType::Remote => "out".to_string()
            }
        }
    }

    pub fn args_as_str(&self) -> String { format!("{:#?}", self.args) } 

    pub fn auth_method(&self) -> AuthMethod { AuthMethod::from(&self.args) }

    pub fn ref_regex(&self) -> Option<Regex> { self.args.ref_regex.clone() }

    pub fn source_base(&self) -> Option<String> {
        match self.source().rsplit_once("/") {
            Some(split) => Some(split.1.to_string()),
            None => None,
        }
    }

    pub fn allowed_category(&self, category: ReferenceCategory) -> bool {
        match self.args.ref_types.clone() {
            Some(list) => list.contains(&category),
            None => true,
        }
    }

    pub fn allowed_search_dir(&self, path: &str) -> bool {
        match self.args.search_dirs.clone() {
            Some(list) => list.contains(&path.to_string()),
            None => true,
        }
    }

    pub fn blob_name_regex(&self) -> Option<Regex> { self.args.obj_regex.clone() }
}