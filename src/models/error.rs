/* 
 * Release Manager
 *
 * This application generate version for your software.
 *
 * OpenAPI spec version: 0.2.0
 * Contact: vitalleshchyk@gmail.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// Error : Contains all the properties any error response from the API will contain. Some properties are optional so might be empty most of the time 

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
  /// the error code, this is not necessarily the http status code
  #[serde(rename = "code")]
  code: i32,
  /// a human readable version of the error
  #[serde(rename = "message")]
  message: String,
  /// an optional url for getting more help about this error
  #[serde(rename = "helpUrl")]
  help_url: Option<String>
}

impl Error {
  /// Contains all the properties any error response from the API will contain. Some properties are optional so might be empty most of the time 
  pub fn new(code: i32, message: String) -> Error {
    Error {
      code: code,
      message: message,
      help_url: None
    }
  }

  pub fn set_code(&mut self, code: i32) {
    self.code = code;
  }

  pub fn with_code(mut self, code: i32) -> Error {
    self.code = code;
    self
  }

  pub fn code(&self) -> &i32 {
    &self.code
  }


  pub fn set_message(&mut self, message: String) {
    self.message = message;
  }

  pub fn with_message(mut self, message: String) -> Error {
    self.message = message;
    self
  }

  pub fn message(&self) -> &String {
    &self.message
  }


  pub fn set_help_url(&mut self, help_url: String) {
    self.help_url = Some(help_url);
  }

  pub fn with_help_url(mut self, help_url: String) -> Error {
    self.help_url = Some(help_url);
    self
  }

  pub fn help_url(&self) -> Option<&String> {
    self.help_url.as_ref()
  }

  pub fn reset_help_url(&mut self) {
    self.help_url = None;
  }

}



