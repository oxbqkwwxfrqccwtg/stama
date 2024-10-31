mod pass;
mod meta;


pub use crate::journal::{Journal, EventObject};
pub use meta::Meta;
pub use pass::PassState;


pub type StateComposite = (Meta, State);


pub type StateInput<'a> = Option<&'a serde_json::Value>;


pub type StateOutput = Option<serde_json::Value>;


#[derive(Debug)]
pub enum State {
    Pass(PassState)
}


impl State {

    pub fn new(r#type: &StateType, json: serde_json::Value) -> Self {

        match r#type {
            StateType::Pass => State::Pass(
                serde_json::from_value(json.clone()).unwrap()
            ),
            _ => todo!(),
        }
    }

    pub fn execute(
        &self,
        input: StateInput,
        mut journal: Journal
    ) -> StateOutput {

        match self {
            State::Pass(state) => state.execute(input, &journal)
        }
    }
}

pub type StateType = EventObject;
