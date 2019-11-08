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

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    // Create the task definitions.
    let task_defs = vec![
        TaskDef::new("get_spam".to_string()),
        TaskDef::new("eat_spam".to_string()),
    ];
    println!("Task definitions: {:#?}", task_defs);

    // Define the base URL of the Conductor server.
    let url = "http://localhost:8080/api";

    // Create the metadata client.
    let mut metadata_client = MetadataClient::new();
    metadata_client.set_base_url(url.to_string());

    // Register the task definitions on the Conductor server.
    metadata_client.create_new_task_definitions(&task_defs).await?;
    println!("Registered task definitions");

    Ok(())
}
