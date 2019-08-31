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
use hyper::{header, Body, Client, Method, Request, StatusCode, Uri};
use log::debug;
use serde::Serialize;

/// A Client to make requests to the Conductor metadata API.
#[derive(Debug)]
pub struct MetadataClient {
    base_url: String,
}

impl MetadataClient {
    /// TODO: docs
    pub fn new() -> Self {
        Self::default()
    }

    /// TODO: docs
    pub fn set_base_url(&mut self, base_url: String) {
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

    /// Creates new task definition(s).
    ///
    /// TODO
    pub async fn create_new_task_definitions(&self, task_defs: &[TaskDef]) -> Result<()> {
        if task_defs.is_empty() {
            return Err(Error::new_empty_task_def_list());
        }

        let client = Client::new();
        let req = self.post("metadata/taskdefs", task_defs)?;
        let resp = client.request(req).await?;

        let status = resp.status();
        debug!("Response: {:#?}", resp);
        let body = resp.into_body().try_concat().await?;
        debug!("Body: {:#?}", body);

        if status != StatusCode::NO_CONTENT {
            return Err(Error::new_unexpected_status());
        }

        assert!(body.is_empty());
        Ok(())
    }
}

impl Default for MetadataClient {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:8080/api".to_string(),
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

    #[test]
    fn given_empty_task_def_vec_when_create_new_task_definitions_then_return_err() {
        use crate::error::{InvalidArgument, Kind};

        let (c, mut rt) = init();

        let res = rt.block_on(c.create_new_task_definitions(&Vec::new()));
        assert_matches!(res, Err(ref err) if *err.kind() == Kind::InvalidArgument(InvalidArgument::EmptyTaskDefList));
    }

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
