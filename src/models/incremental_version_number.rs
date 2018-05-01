/* 
 * Release Manager
 *
 * This application generate version for your software.
 *
 * OpenAPI spec version: 0.2.0
 * Contact: vitalleshchyk@gmail.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IncrementalVersionNumber {
  #[serde(rename = "version")]
  version: i32
}

impl IncrementalVersionNumber {
  pub fn new(version: i32) -> IncrementalVersionNumber {
    IncrementalVersionNumber {
      version: version
    }
  }

  pub fn set_version(&mut self, version: i32) {
    self.version = version;
  }

  pub fn with_version(mut self, version: i32) -> IncrementalVersionNumber {
    self.version = version;
    self
  }

  pub fn version(&self) -> &i32 {
    &self.version
  }


}


