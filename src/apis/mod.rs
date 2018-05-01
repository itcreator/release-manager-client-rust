use hyper;
use serde_json;

#[derive(Debug)]
pub enum Error {
    Hyper(hyper::Error),
    Serde(serde_json::Error),
}

impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

use super::models::*;

mod project_api;
pub use self::project_api::{ ProjectApi, ProjectApiClient };
mod version_incremental_api;
pub use self::version_incremental_api::{ VersionIncrementalApi, VersionIncrementalApiClient };
mod version_semantic_api;
pub use self::version_semantic_api::{ VersionSemanticApi, VersionSemanticApiClient };

pub mod configuration;
pub mod client;
