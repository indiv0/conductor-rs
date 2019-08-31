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

use conductor::{MetadataClient, TaskDef};
use tokio::runtime::current_thread::Runtime;

fn init() -> (MetadataClient, Runtime) {
    let _ = pretty_env_logger::try_init();
    let c = MetadataClient::new();
    let rt = Runtime::new().expect("runtime");
    (c, rt)
}

#[cfg(feature = "integration_tests")]
#[test]
fn given_one_task_def_when_create_new_task_definitions_then_return_ok() {
    let (c, mut rt) = init();

    let task_def = TaskDef::new("eat_spam".to_string());
    let res = rt.block_on(c.create_new_task_definitions(&[task_def]));
    assert!(res.is_ok());
}

#[cfg(feature = "integration_tests")]
#[test]
fn given_multiple_task_defs_when_create_new_task_definitions_then_return_ok() {
    let (c, mut rt) = init();

    let task_defs = vec![
        TaskDef::new("get_spam".to_owned()),
        TaskDef::new("eat_spam".to_owned()),
    ];
    let res = rt.block_on(c.create_new_task_definitions(&task_defs));
    assert!(res.is_ok());
}
