use std::ops::{Deref, DerefMut};

use serde::{Serialize, Deserialize, Serializer, Deserializer};
use std::collections::BTreeMap;
use crate::state::Meta;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct Machine<'a> {

    start_at: &'a str,
    states: BTreeMap<String, serde_json::Value>
}


impl<'a> Machine<'a> {

    fn execute(&self) {

        Execution::new(self.start_at, self.states.clone());
    }
}


pub struct Execution<'a> {

    next: &'a str,
    states: BTreeMap<String, (Meta<'a>, serde_json::Value)>
}

impl<'a> Execution<'a> {

    fn new (next: &'a str, states: BTreeMap<String, serde_json::Value>) -> Self {

        let mut nstates: BTreeMap<String, (Meta<'a>, serde_json::Value)> = BTreeMap::new();

        for (name, state) in states {

            let meta: Meta<'a> = serde_json::from_value(state.clone()).unwrap();

            nstates.insert(String::from(name), (meta, state));
        }

        Execution {
            next: next,
            states: nstates
        }
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
