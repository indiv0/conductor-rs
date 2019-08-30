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

use serde::{Deserialize, Serialize};

/// Conductor task definition.
///
/// Task definitions define task-level parameters like keys for input and output
/// data.
///
/// # Examples
///
/// Create a task definition:
///
/// ```rust
/// use conductor::TaskDef;
/// #
/// # fn main() {
/// let task_def = TaskDef::new("eat_spam".to_string());
/// # }
/// ```
///
/// Print a task's name:
///
/// ```rust
/// # use conductor::TaskDef;
/// #
/// # fn main() {
/// # let task_def = TaskDef::new("eat_spam".to_string());
/// println!("Task name: {}", task_def.name());
/// # }
/// ```
///
/// # Invariants
///
/// * Tasks **MUST** have a unique name.
#[derive(Debug, Deserialize, Serialize)]
pub struct TaskDef {
    name: String,
}

impl TaskDef {
    /// Creates a task definition.
    ///
    /// This function takes an argument of type `String` and creates a task
    /// definition with that name.
    ///
    /// # Examples
    ///
    /// Create a task definition, using a name from a program argument:
    ///
    /// ```rust
    /// use conductor::TaskDef;
    /// #
    /// # fn main() {
    /// if let Some(name) = std::env::args().skip(1).next() {
    ///     let task_def = TaskDef::new(name);
    /// }
    /// # }
    /// ```
    pub fn new(name: String) -> Self {
        Self { name }
    }

    /// Returns the task's name.
    ///
    /// # Examples
    ///
    /// Get the task's name and print it:
    ///
    /// ```rust
    /// # use conductor::TaskDef;
    /// #
    /// # fn main() {
    /// # let task_def = TaskDef::new("eat_spam".to_string());
    /// println!("Task name: {}", task_def.name());
    /// # }
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the task's name.
    ///
    /// This function takes a `String` as an argument and sets the task's name
    /// to that value.
    ///
    /// # Examples
    ///
    /// Set a task's name and print it:
    ///
    /// ```rust
    /// # use conductor::TaskDef;
    /// #
    /// # fn main() {
    /// # let mut task_def = TaskDef::new("get_spam".to_string());
    /// task_def.set_name("eat_spam".to_string());
    /// println!("Task name: {}", task_def.name());
    /// # }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

#[cfg(test)]
mod tests {
    use crate::task::TaskDef;
    use serde_json::json;

    #[test]
    fn task_def_name_gets_serialized() {
        let task_def = TaskDef::new("eat_spam".to_string());
        let json = serde_json::to_value(&task_def).expect("serialized");
        assert_eq!(json["name"], "eat_spam");
    }

    #[test]
    fn task_def_name_gets_deserialized() {
        let json = json!({ "name": "eat_spam" });
        let task_def: TaskDef = serde_json::from_value(json).expect("deserialized");
        assert_eq!(task_def.name(), "eat_spam");
    }

    #[test]
    fn get_name_returns_the_name() {
        let task_def = TaskDef::new("get_spam".to_string());
        assert_eq!(task_def.name(), "get_spam");
    }

    #[test]
    fn set_name_sets_the_name() {
        let mut task_def = TaskDef::new("get_spam".to_string());
        task_def.set_name("eat_spam".to_string());
        assert_eq!(task_def.name(), "eat_spam");
    }
}
