use crate::journal::{Journal, Event};
use crate::state::{
    Meta,
    StateType,
    State,
    StateComposite,
    execute as execute_state
};
use std::collections::BTreeMap;


pub struct Execution<'a> {

    next: Option<String>,
    states: BTreeMap<String, StateComposite>,
    result: Option<serde_json::Value>,
    journal: Journal<'a>
}


impl<'a> Execution<'a> {
    pub fn new(
        start_at: & str,
        states: BTreeMap<String, serde_json::Value>,
        input: Option<serde_json::Value>
    ) -> Self {

        let journal = Journal::new();

        let mut nstates: BTreeMap<String, StateComposite> = BTreeMap::new();

        for (k, v) in states {

            let next: String = k.clone();

            let meta: Meta = serde_json::from_value(v.clone()).unwrap();

            let state = match meta.r#type {
                StateType::Pass => State::Pass(
                    serde_json::from_value(v.clone()).unwrap()
                ),
                _ => todo!(),
            };

            nstates.insert(next.to_string(), (meta, state));
        }

        journal.put(Event::ExecutionStarted, input.as_ref());

        Execution {
            next: Some(String::from(start_at)),
            states: nstates,
            result: input,
            journal: journal
        }
    }
}


impl Iterator for Execution<'_> {

    type Item = String;

    fn next(&mut self) -> Option<String> {

        let composite: &StateComposite = &self.states[&self.next.clone()
                                                      .unwrap()];

        self.result = match &self.next {
            Some(_) => {
                let result = execute_state(
                    &composite,
                    self.result.clone(),
                    &self.journal
                );

                result
            }
            None => None
        };

        self.next = match &composite.0.end {
            Some(true) => {

                self.journal.put(
                    Event::ExecutionSucceeded,
                    self.result.as_ref()
                );

                None
            },
            Some(false) | None => match &composite.0.next {
                Some(next) => {
                    Some(next.clone())
                },
                _ => panic!("state leads nowhere: {:?}", composite),
            }
        };

        self.next.clone()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Result};

    #[test]
    fn test_json() {

        let start_at = "test";

        let states: BTreeMap<String, serde_json::Value> = serde_json::from_str(r#"{
            "test": {
                "Type": "Pass",
                "Result": [1,2,3],
                "Next": "test2"
            },
            "test2": {
                "Type": "Pass",
                "Result": [1,2,3],
                "End": true
            }
        }"#).unwrap();

        let result: Execution = Execution::new(start_at, states, None);
    }

}
