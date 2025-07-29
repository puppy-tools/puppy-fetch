use regex::Regex;
use clap::{ArgGroup, Parser};
use clap::{crate_name, crate_version, crate_authors};

use crate::clap_impl::sensitive_string::SensitiveString;
use crate::enums::reference_category::ReferenceCategory;
use crate::enums::level_filter_arg::LevelFilterArg;

#[derive(Parser, Debug)]
#[clap(group(ArgGroup::new("source_type").args(["local", "remote"])))]
#[clap(group(ArgGroup::new("auth_method").args(["ssh_agent", "ssh_key", "auth_token"])))]
#[command(
    disable_help_flag = true,
    disable_help_subcommand = true,
    disable_version_flag = true, 
    arg_required_else_help = true,

    name = crate_name!(),
    version = crate_version!(),
    author = crate_authors!(),
    styles = crate::clap_impl::styling::styles()
)]
pub struct Args {
    pub source: Option<String>,
    pub destination: Option<String>,

    #[arg(long)] 
    pub local: bool,
    #[arg(long, requires="source")] 
    pub remote: bool, 
    #[arg(long, ignore_case=true)] 
    pub log_level: Option<LevelFilterArg>,

    #[arg(short='T', long, value_delimiter=',')] 
    #[arg(help_heading=HELP_HEADING_FILTERING)]
    pub ref_types: Option<Vec<ReferenceCategory>>,
    #[arg(short='s', long, value_delimiter=',')]
    #[arg(help_heading=HELP_HEADING_FILTERING)]
    pub search_dirs: Option<Vec<String>>,
    #[arg(short='X', long)] 
    #[arg(help_heading=HELP_HEADING_FILTERING)]
    pub ref_regex: Option<Regex>,
    #[arg(short='x', long)] 
    #[arg(help_heading=HELP_HEADING_FILTERING)]
    pub obj_regex: Option<Regex>,

    #[arg(long)] 
    #[arg(help_heading=HELP_HEADING_AUTH)]
    pub ssh_agent: bool,
    #[arg(long)] 
    #[arg(help_heading=HELP_HEADING_AUTH)]
    pub ssh_key: Option<String>,
    #[arg(long)] 
    #[arg(help_heading=HELP_HEADING_AUTH)]
    pub auth_token: Option<SensitiveString>,

    #[arg(short, long)] 
    #[arg(help_heading=HELP_HEADING_ABOUT)]
    pub help: bool,
    #[arg(short='V', long, visible_short_alias='v')] 
    #[arg(help_heading=HELP_HEADING_ABOUT)]
    pub version: bool
}

const HELP_HEADING_FILTERING : &str = "Filtering";
const HELP_HEADING_AUTH : &str = "Authorization";
const HELP_HEADING_ABOUT : &str = "About";