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

use conductor::TaskDef;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_simple(c: &mut Criterion) {
    let mut group = c.benchmark_group("TaskDef group");

    group.bench_function("TaskDef::new", |b| {
        b.iter(|| TaskDef::new(black_box("eat_spam".to_string())))
    });

    let task_def = TaskDef::new("eat_spam".to_string());
    group.bench_function("TaskDef Serialize", |b| {
        b.iter(|| serde_json::to_string(black_box(&task_def)).expect("serialized"))
    });

    let json = serde_json::to_string(&task_def).expect("serialized");
    group.bench_function("TaskDef Deserialize", |b| {
        b.iter(|| serde_json::from_str::<TaskDef>(black_box(&json)))
    });

    group.finish()
}

criterion_group!(benches, bench_simple);
criterion_main!(benches);
