use clap::ValueEnum;
use git2::Reference;

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum ReferenceCategory { 
    #[clap(aliases=["local", "head"])]
    Branch, // also allows 'local' or 'head'
    Remote, 
    Tag, 
    Note,
    #[clap(skip)] // hides 'unknown' in clap parsing
    Unknown 
}

impl From<&Reference<'_>> for ReferenceCategory {
    fn from(value: &Reference) -> Self {
        if value.is_branch() { 
            ReferenceCategory::Branch
        } else if value.is_note() {
            ReferenceCategory::Note
        } else if value.is_remote() {
            ReferenceCategory::Remote 
        } else if value.is_tag() {
            ReferenceCategory::Tag
        } else {
            ReferenceCategory::Unknown
        }
    }
}