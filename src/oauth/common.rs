use axum::{async_trait, Router};

use crate::util::{has_env, Error};

#[async_trait]
pub trait OauthProvider<T> {
    fn get_url<F: FnOnce(String) -> String>(f: Option<F>) -> Option<String>;
    fn get_requirements() -> ProviderRequirements;
    fn get_router() -> Router;
    async fn publish(config: T, message: String) -> Result<String, Error>;
}

#[derive(Debug)]
pub struct ProviderRequirements {
    pub fulfilled: Vec<(String, String)>,
    pub missing: Vec<(String, String)>,

    pub has_missing: bool,
}

impl From<Vec<(String, String)>> for ProviderRequirements {
    fn from(requirements: Vec<(String, String)>) -> Self {
        Self::new(requirements)
    }
}

impl From<Vec<(&str, &str)>> for ProviderRequirements {
    fn from(requirements: Vec<(&str, &str)>) -> Self {
        Self::new(
            requirements
                .into_iter()
                .map(|(r, n)| (r.to_owned(), n.to_owned()))
                .collect(),
        )
    }
}

impl ProviderRequirements {
    pub fn new(requirements: Vec<(String, String)>) -> Self {
        let mut fulfilled = vec![];
        let mut missing = vec![];

        requirements.iter().for_each(|(r, n)| {
            if has_env(r) {
                fulfilled.push((r.to_owned(), n.to_owned()))
            } else {
                missing.push((r.to_owned(), n.to_owned()))
            }
        });

        Self {
            fulfilled,
            missing: missing.clone(),
            has_missing: !missing.is_empty(),
        }
    }

    pub fn get_missing_list_str(&self) -> String {
        self.missing
            .clone()
            .into_iter()
            .map(|(name, _)| name)
            .collect::<Vec<String>>()
            .join(", ")
    }
}
