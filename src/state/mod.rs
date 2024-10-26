mod pass;
mod meta;


pub use crate::journal::{Journal, Event, EventObject, EventStatus};
pub use meta::Meta;
pub use pass::PassState;
use jsonpath_rust::{JsonPath};


pub type StateComposite = (Meta, State);


pub type StateInput = serde_json::Value;


pub type StateOutput = serde_json::Value;


#[derive(Debug)]
pub enum State {
    Pass(PassState)
}


pub type StateType = EventObject;


pub fn execute(
    state: &StateComposite,
    mut input: Option<StateInput>,
    journal: &Journal
) -> Option<StateOutput> {

    journal.put(Event::lookup(&state.0.r#type, &EventStatus::StateEntered)?, input.as_ref());

    input = match &input {
        Some(input) => match &state.0.input_path {
            Some(path) => {
                let path2: JsonPath<serde_json::Value> = JsonPath::try_from(path.as_str()).ok()?;

                Some(path2.find(&input))
            },
            _ => Some(input.clone())
        },
        _ => input
    };

    //a journal which is scoped to the EventObject of the state type
    let pjournal = journal.partition(Some(&state.0.r#type));

    let mut result = match &state.1 {
        State::Pass(state) => state.execute(input.as_ref(), &pjournal)
    };

    result = match &result {
        Some(result) => match &state.0.result_path {
            Some(path) => {
                let path2: JsonPath<serde_json::Value> = JsonPath::try_from(path.as_str()).ok()?;

                Some(path2.find(&result))
            },
            _ => Some(result.clone())
        },
        _ => result.clone()
    };

    let output = match &result {
        Some(result) => match &state.0.output_path {
            Some(path) => {
                let path2: JsonPath<serde_json::Value> = JsonPath::try_from(path.as_str()).ok()?;

                Some(path2.find(&result))
            },
            _ => Some(result.clone())
        },
        _ => result.clone()
    };

    journal.put(Event::lookup(&state.0.r#type, &EventStatus::StateExited)?, output.as_ref());

    output.clone()
}
