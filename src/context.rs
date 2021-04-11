use crate::store::{load_from_json, persist_json};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const CONTEXT_FILE: &str = "context.json";

#[derive(Serialize, Deserialize)]
pub struct Context {
    variables: HashMap<String, f64>,
}

impl Context {
    pub fn build() -> Self {
        match load_from_json::<Self>(CONTEXT_FILE) {
            Ok(ctx) => ctx,
            Err(_) => Self {
                variables: HashMap::new(),
            },
        }
    }

    pub fn get(&self) -> &HashMap<String, f64> {
        &self.variables
    }
}
