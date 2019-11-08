// Copyright 2018 Netflix, Inc.
// Copyright 2019 Nikita Pekin
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::error::{Error, Result};
use crate::task::TaskDef;
use futures_util::try_stream::TryStreamExt;
use hyper::{header, Body, Client, Method, Request, Uri};
use hyper::client::HttpConnector;
use log::debug;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct ErrorResponse {
    status: u16, // TODO: make this match int size
    code: Option<String>,
    message: String,
    instance: Option<String>,
    retryable: bool,
    #[serde(rename="validationErrors")]
    validation_errors: ValidationErrorVec,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: rewrite this to be a proper formatter.
        write!(f, "[code=\"{:?}\", message=\"{}\", instance=\"{:?}\", retryable=\"{}\", validation errors=\"{}\"]", self.code, self.message, self.instance, self.retryable, self.validation_errors)
    }
}

#[derive(Debug, Deserialize, PartialEq)]
struct ValidationErrorVec(Vec<ValidationError>);

impl fmt::Display for ValidationErrorVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: rewrite this to be a proper formatter.
        write!(f, "[")?;
        for i in &self.0 {
            write!(f, "{}", i)?;
        }
        write!(f, "]")
    }
}

#[derive(Debug, Deserialize, PartialEq)]
struct ValidationError {
    path: String,
    message: String,
    invalid_value: Option<String>,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: rewrite this to be a proper formatter.
        write!(f, "[path=\"{}\", message=\"{}\", invalid value=\"{:?}\"]", self.path, self.message, self.invalid_value)
    }
}

#[derive(Debug)]
struct BaseClient {
    base_url: String,
    client: Client<HttpConnector>,
}

impl BaseClient {
    fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: Client::new(),
        }
    }

    /// TODO: docs
    fn set_base_url(&mut self, base_url: String) {
        self.base_url = base_url;
    }

    fn post<B>(&self, path: &str, body: B) -> Result<Request<Body>>
    where
        B: Serialize,
    {
        let uri = format!("{}/{}", self.base_url, path);
        let uri = uri.parse::<Uri>().expect("parsed URL");
        let json_body = serde_json::to_string(&body)?;
        let req = Request::builder()
            .method(Method::POST)
            .uri(uri)
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::ACCEPT, "application/json")
            .body(Body::from(json_body))
            .map_err(From::from);
        debug!("Request: {:#?}", req);
        req
    }

    async fn execute(&self, request: Request<Body>) -> Result<()> {
        let resp = self.client.request(request).await?;

        let status = resp.status();
        debug!("Response: {:#?}", resp);
        let body = resp.into_body().try_concat().await?;
        debug!("Body: {:#?}", body);

        if status.is_redirection() || status.is_client_error() || status.is_server_error() {
            // TODO: don't throw away the URI, status code, and response body here.
            match serde_json::from_slice(&body) {
                Ok(error_response) => {
                    panic!("{}", error_response);
                    return Err(Error::new_error_response(error_response))
                },
                Err(err) => {
                    // TODO: remove this
                    panic!("{}, {:?}", err, &body);
                    return Err(Error::new_unexpected_status(status))
                }
            }
        }

        assert!(body.is_empty());
        Ok(())
    }
}

/// A client to make requests to the Conductor metadata API.
///
/// # Examples
///
/// Create a new `MetadataClient`:
///
/// ```rust
/// use conductor::MetadataClient;
///
/// # fn main() {
/// let metadata_client = MetadataClient::new();
/// # }
/// ```
///
/// Create new task definitions and register them on the Conductor server:
/// ```rust,no_run
/// use conductor::{MetadataClient, TaskDef};
/// # use conductor::Result;
///
/// # #[tokio::main]
/// # async fn main() -> Result<()> {
/// let task_defs = vec![
///     TaskDef::new("get_spam".to_string()),
///     TaskDef::new("eat_spam".to_string()),
/// ];
/// let metadata_client = MetadataClient::new();
/// metadata_client.create_new_task_definitions(&task_defs).await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct MetadataClient {
    client: BaseClient,
}

impl MetadataClient {
    /// Creates a new `MetadataClient`.
    pub fn new() -> Self {
        Self::default()
    }

    /// TODO: docs
    pub fn set_base_url(&mut self, base_url: String) {
        self.client.set_base_url(format!("{}/metadata", base_url));
    }

    /// Creates new task definition(s).
    ///
    /// TODO
    pub async fn create_new_task_definitions(&self, task_defs: &[TaskDef]) -> Result<()> {
        /*
        if task_defs.is_empty() {
            return Err(Error::new_empty_task_def_list());
        }
        */

        let req = self.client.post("taskdefs", task_defs)?;
        self.client.execute(req).await
    }
}

impl Default for MetadataClient {
    fn default() -> Self {
        Self {
            client: BaseClient::new("http://localhost:8080/api/metadata".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{MetadataClient, TaskDef};
    use lazy_static::lazy_static;
    use matches::assert_matches;
    use mockito::{mock, Matcher};
    use serde_json::json;
    use tokio::runtime::current_thread::Runtime;

    fn init() -> (&'static MetadataClient, Runtime) {
        lazy_static! {
            static ref METADATA_CLIENT: MetadataClient = {
                let mut c = MetadataClient::new();
                c.set_base_url(mockito::server_url());
                c
            };
        }

        let _ = pretty_env_logger::try_init();
        let rt = Runtime::new().expect("runtime");
        (&*METADATA_CLIENT, rt)
    }

    /*
    #[test]
    fn given_empty_task_def_vec_when_create_new_task_definitions_then_return_err() {
        use crate::error::{InvalidArgument, Kind};

        let (c, mut rt) = init();

        let res = rt.block_on(c.create_new_task_definitions(&Vec::new()));
        assert_matches!(res, Err(ref err) if *err.kind() == Kind::InvalidArgument(InvalidArgument::EmptyTaskDefList));
    }
    */

    #[test]
    fn given_single_task_def_then_return_no_content() {
        let (c, mut rt) = init();

        let m = mock("POST", "/metadata/taskdefs")
            .match_query(Matcher::Missing)
            .match_header("accept", "application/json")
            .match_header("content-type", "application/json")
            .match_body(Matcher::Json(json!([{"name": "get_spam"}])))
            .with_status(204)
            .with_header("content-type", "application/json")
            .create();

        let task_defs = vec![TaskDef::new("get_spam".to_string())];
        let res = rt.block_on(c.create_new_task_definitions(&task_defs));
        m.assert();
        assert_matches!(res, Ok(()));
    }

    #[test]
    fn given_multiple_task_def_then_return_no_content() {
        let (c, mut rt) = init();

        let m = mock("POST", "/metadata/taskdefs")
            .match_query(Matcher::Missing)
            .match_header("accept", "application/json")
            .match_header("content-type", "application/json")
            .match_body(Matcher::Json(
                json!([{"name": "get_spam"}, {"name": "eat_spam"}]),
            ))
            .with_status(204)
            .with_header("content-type", "application/json")
            .create();

        let task_defs = vec![
            TaskDef::new("get_spam".to_string()),
            TaskDef::new("eat_spam".to_string()),
        ];
        let res = rt.block_on(c.create_new_task_definitions(&task_defs));
        m.assert();
        assert_matches!(res, Ok(()));
    }
}
