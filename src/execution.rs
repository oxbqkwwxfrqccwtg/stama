use crate::journal::{Journal, Event, EventStatus, Record, EventObject};
use crate::state::{
    Meta,
    State,
    StateComposite,
};
use std::collections::BTreeMap;


pub struct Execution<'a> {

    next: Option<String>,
    states: BTreeMap<String, StateComposite>,
    result: Option<&'a serde_json::Value>,
    input: Option<&'a serde_json::Value>,
    pub journal: Journal<'a>
}


impl<'a> Execution<'a> {
    pub fn new(
        start_at: & str,
        states: BTreeMap<String, serde_json::Value>,
        input: Option<serde_json::Value>,
        mut journal: &'a Journal,
    ) -> Self {

        let mut nstates: BTreeMap<String, StateComposite> = BTreeMap::new();

        for (k, v) in states {

            let next: String = k.clone();

            let meta: Meta = serde_json::from_value(v.clone()).unwrap();

            let state: State = State::new(&meta.r#type, v);

            nstates.insert(next.to_string(), (meta, state));
        }

        let rinput = journal.add(Record::Orig {
            r#type: Event::ExecutionStarted,
            payload: input
        });

        Execution {
            next: Some(String::from(start_at)),
            states: nstates,
            input: rinput,
            result: None,
            journal: journal.partition(Some(&EventObject::Execution)),
        }
    }
}


impl Iterator for Execution<'_> {

    type Item = String;

    fn next(&mut self) -> Option<String> {

        let input: Option<&serde_json::Value> = match self.result {
            None => self.input,
            Some(result) => Some(&result),
        };

        let composite: &StateComposite = &self.states[&self.next.clone()
                                                      .unwrap()];

        self.result = match &self.next {
            Some(_) => {

                let r#type = Event::lookup(&composite.0.r#type, &EventStatus::StateEntered)?;

                let result = State::execute(
                    &composite.1,
                    self.journal.add(Record::Ref {
                        r#type: r#type,
                        payload: input
                    }),
                    self.journal.partition(None),
                );

                self.journal.add(Record::Orig {
                    r#type: Event::lookup(&composite.0.r#type, &EventStatus::StateExited)?,
                    payload: result
                })
            },
            None => None,
        };

        self.next = match &composite.0.end {
            Some(true) => {

                self.journal.add(Record::Orig {
                    r#type: Event::ExecutionSucceeded,
                    payload: None
                });

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

        let binding = Journal::new_root();

        let _result: Execution = Execution::new(start_at, states, None, &binding);
    }

}
