use std::path::Path;

use git2::build::RepoBuilder;
use git2::{FetchOptions, RemoteCallbacks, Repository};
use crate::context::Context; 
use crate::enums::source_type::SourceType;
use crate::git_utils::callbacks::{handle_credentials, handle_sideband_progress, handle_transfer_progress};


impl TryFrom<&Context> for Repository {
    type Error = git2::Error;

    fn try_from(ctx: &Context) -> Result<Self, Self::Error> {
        match ctx.source_type() {
            SourceType::Local => Repository::open(ctx.source()),
            SourceType::Remote => {
                let base = ctx.source_base().expect("Couldn't get base name from source.");
                let path = &format!("remotes/{base}");
                let into = Path::new(path);
        
                let repo = RepoBuilder::new()
                    .bare(true)
                    .fetch_options(fetch_options(&ctx))
                    .clone(&ctx.source(), &into)
                ;
                
                repo
            }
        }
    }
}

fn fetch_options(ctx: &Context) -> FetchOptions<'_> {
    let mut rc = RemoteCallbacks::new();
    handle_credentials(ctx.auth_method(), &mut rc);
    handle_sideband_progress(&mut rc);
    handle_transfer_progress(&mut rc);
    let mut fo = FetchOptions::new();
    fo.remote_callbacks(rc);
    fo
} 