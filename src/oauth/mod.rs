pub mod common;
pub mod providers;
pub mod router;

use crate::util::has_env;
use common::{OauthProvider, ProviderRequirements};
use std::collections::HashMap;

pub type OauthRequirements = HashMap<String, ProviderRequirements>;

pub fn get_requirements() -> OauthRequirements {
    vec![(
        "mastodon".to_owned(),
        providers::MastodonProvider::get_requirements(),
    )]
    .into_iter()
    .collect()
}

pub fn check_requirements() -> bool {
    has_env("OAUTH_URL_BASE")
}
