use serde::{Serialize, Deserialize};
use crate::Journal;


/// Pass state
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct PassState {
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<serde_json::Value>
}


impl PassState {

    pub fn execute(
        &self,
        _input: Option<&serde_json::Value>,
        _journal: &Journal,
    ) -> Option<serde_json::Value> {
        match &self.result {
            Some(result) => Some(result.clone()),
            _ => None
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde_json_de_with_options() {

        let data = r#"{
            "Result": {
                "x-datum": 0.381018,
                "y-datum": 622.2269926397355
            }
        }"#;

        let out: PassState = serde_json::from_str(data).unwrap();

        let result = out.result.unwrap();

        assert_eq!(result["x-datum"], 0.381018);
        assert_eq!(result["y-datum"], 622.2269926397355);
    }

    #[test]
    fn test_serde_json_de_without_options() {

        let data = r#"{
        }"#;

        let out: PassState = serde_json::from_str(data).unwrap();

        assert_eq!(out.result, None);
    }
}
