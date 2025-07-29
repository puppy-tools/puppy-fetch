use git2::CredentialType;

use crate::clap_impl::args::Args;

/*
    An enum that describes a more restricted version of allowed git2::CredentialType(s).
    
    Because this program implements its authorization methods ...
    as individual switches in clap this enum provides a nice interface to convert ...
    from &crate::clap_impl::args::Args to AuthMethod and use in exhaustive pattern matching.
*/

// allow lint for 'SCREAMING_SNAKE_CASE' to match bitflag consts of CredentialType
#[allow(non_camel_case_types)] 
#[derive(PartialEq, Clone)]
pub enum AuthMethod {
    SSH_AGENT,
    SSH_KEY(String),
    AUTH_TOKEN(String),
    NONE,
}

impl From<AuthMethod> for CredentialType {
    fn from(value: AuthMethod) -> Self {
        match value {
            AuthMethod::SSH_AGENT     => Self::SSH_MEMORY,
            AuthMethod::SSH_KEY(_)    => Self::SSH_KEY,
            AuthMethod::AUTH_TOKEN(_) => Self::USER_PASS_PLAINTEXT,
            AuthMethod::NONE          => Self::USERNAME,
        }
    }
} 

impl From<&Args> for AuthMethod {
    fn from(value: &Args) -> Self {
        if value.ssh_agent {
            Self::SSH_AGENT
        } else if let Some(key) = value.ssh_key.clone() {
            Self::SSH_KEY(key)
        } else if let Some(token) = value.auth_token.clone() {
            Self::AUTH_TOKEN(token.into())
        } else {
            Self::NONE
        }
    }
}
