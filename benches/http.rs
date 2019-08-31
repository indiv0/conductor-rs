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
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mockito::{mock, Matcher};
use serde_json::json;
use tokio::runtime::current_thread::Runtime;

fn bench_simple(c: &mut Criterion) {
    let _ = pretty_env_logger::try_init().expect("env_logger");

    let mut group = c.benchmark_group("HTTP group");

    let mut c = MetadataClient::new();
    let mut rt = Runtime::new().expect("runtime");
    let task_defs = vec![
        TaskDef::new("get_spam".to_string()),
        TaskDef::new("eat_spam".to_string()),
    ];
    group.bench_function("MetadataClient::create_new_task_definitions", |b| {
        b.iter(|| {
            rt.block_on(c.create_new_task_definitions(black_box(&task_defs)))
                .expect("create task defs")
        })
    });

    c.set_base_url(mockito::server_url());
    let _m = mock("POST", "/metadata/taskdefs")
        .match_query(Matcher::Missing)
        .match_header("accept", "application/json")
        .match_header("content-type", "application/json")
        .match_body(Matcher::Json(
            json!([{"name": "get_spam"}, {"name": "eat_spam"}]),
        ))
        .with_status(204)
        .with_header("content-type", "application/json")
        .create();
    group.bench_function("MetadataClient::create_new_task_definitions mock", |b| {
        b.iter(|| {
            rt.block_on(c.create_new_task_definitions(black_box(&task_defs)))
                .expect("create task defs")
        })
    });

    group.finish()
}

criterion_group!(benches, bench_simple);
criterion_main!(benches);
