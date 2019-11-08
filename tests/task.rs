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

/*
use conductor::TaskDef;
use tokio::runtime::current_thread::Runtime;

#[test]
fn create_new_task_definition() {
    let mut rt = Runtime::new().expect("runtime new");

    let task_def = TaskDef::new("foo".to_string());
    let c = MetadataClient;
    let res = c.create_new_task_definitions(vec![task_def]);

    rt.block_on(res).expect("create new task definitions");
}
*/
