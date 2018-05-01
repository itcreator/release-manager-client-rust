use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  project_api: Box<::apis::ProjectApi>,
  version_incremental_api: Box<::apis::VersionIncrementalApi>,
  version_semantic_api: Box<::apis::VersionSemanticApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      project_api: Box::new(::apis::ProjectApiClient::new(rc.clone())),
      version_incremental_api: Box::new(::apis::VersionIncrementalApiClient::new(rc.clone())),
      version_semantic_api: Box::new(::apis::VersionSemanticApiClient::new(rc.clone())),
    }
  }

  pub fn project_api(&self) -> &::apis::ProjectApi{
    self.project_api.as_ref()
  }

  pub fn version_incremental_api(&self) -> &::apis::VersionIncrementalApi{
    self.version_incremental_api.as_ref()
  }

  pub fn version_semantic_api(&self) -> &::apis::VersionSemanticApi{
    self.version_semantic_api.as_ref()
  }


}
