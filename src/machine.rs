use crate::Execution;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

use crate::journal::Journal;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct Machine {
    start_at: String,
    states: BTreeMap<String, serde_json::Value>
}


impl<'a> Machine {
    pub fn execute(&self, input: Option<serde_json::Value>, journal: &'a Journal) -> Execution {
        Execution::new(&self.start_at, self.states.clone(), input, journal)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json() {

        let input = r#"{
            "StartAt": "",
            "States": {
                "test": {}
            }
        }"#;

        let _result: Machine = serde_json::from_str(input).unwrap();
    }
}
