use crate::clap_impl::args::Args;

#[derive(Debug)]
pub enum SourceType { Local, Remote }

impl From<&Args> for SourceType {
    fn from(value: &Args) -> Self {
        if value.local { return Self::Local }
        if value.remote { return Self::Remote }

        // if nothing is set, fallback to local
        Self::Local 
    }
}