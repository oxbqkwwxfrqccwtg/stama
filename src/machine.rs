use crate::Execution;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct Machine {
    start_at: String,
    states: BTreeMap<String, serde_json::Value>
}


impl Machine {
    pub fn execute(&self, input: Option<serde_json::Value>) -> Execution {
        Execution::new(&self.start_at, self.states.clone(), input)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Result};

    #[test]
    fn test_json() {

        let input = r#"{
            "StartAt": "",
            "States": {
                "test": {}
            }
        }"#;

        let result: Machine = serde_json::from_str(input).unwrap();
    }
}
