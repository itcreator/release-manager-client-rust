/* 
 * Release Manager
 *
 * This application generate version for your software.
 *
 * OpenAPI spec version: 0.2.0
 * Contact: vitalleshchyk@gmail.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use hyper;

pub struct Configuration<C: hyper::client::Connect> {
  pub base_path: String,
  pub client: hyper::client::Client<C>,
}

impl<C: hyper::client::Connect> Configuration<C> {
  pub fn new(client: hyper::client::Client<C>) -> Configuration<C> {
    Configuration {
      base_path: "http://localhost".to_owned(),
      client: client,
    }
  }
}
