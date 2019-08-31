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

//! conductor-rs is a Netflix [Conductor](https://netflix.github.io/conductor/)
//! client library implementation.
//!
//! # Quick Start
//!
//! To execute a workflow, you need a [`TaskDef`] for each task in your
//! workflow:
//! ```rust
//! use conductor::TaskDef;
//! #
//! # fn main() {
//! let get_spam = TaskDef::new("get_spam".to_string());
//! let eat_spam = TaskDef::new("eat_spam".to_string());
//! # }
//! ```
//!
//! Each [`TaskDef`] should be uploaded to the Conductor server using
//! the [`MetadataClient`].
//!
//! [`TaskDef`]: ./struct.TaskDef.html
//! [`MetadataClient`]: ./struct.MetadataClient.html

#![warn(
    anonymous_parameters,
    deprecated_in_future,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    future_incompatible,
    indirect_structural_match,
    keyword_idents,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_debug_implementations,
    missing_docs,
    non_ascii_idents,
    nonstandard_style,
    private_in_public,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unstable_features,
    unused,
    variant_size_differences
)]
#![deny(rust_2018_idioms, unsafe_code)]

mod error;
mod http;
mod task;

pub use crate::http::MetadataClient;
pub use error::Result;
pub use task::TaskDef;
