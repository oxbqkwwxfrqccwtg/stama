use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use super::StateType;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct Meta {
    pub r#type: StateType,
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<PayloadTemplate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result_selector: Option<PayloadTemplate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry: Option<Vec<Retrier>>,
    ////catch
}


type PayloadTemplate = BTreeMap<String, String>;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct Retrier {
    error_equals: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval_seconds: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_attempts: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backoff_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jitter_strategy: Option<String>
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde_json_de_with_option() {

        let input = r#"{
            "Type": "Pass",
            "Comment": "Comment"
        }"#;

        let result: Meta = serde_json::from_str(input).unwrap();

        assert_eq!(result.r#type, StateType::Pass);
        assert_eq!(result.comment, Some(String::from("Comment")));
    }

    #[test]
    fn test_serde_json_de_without_option() {

        let input = r#"{
            "Type": "Pass"
        }"#;

        let result: Meta = serde_json::from_str(input).unwrap();

        assert_eq!(result.r#type, StateType::Pass);
    }

    /// This test verifies that a `Meta` struct can be deserialized from a
    /// `serde_json::Value`.
    ///
    /// The purpose of this test is to ensure that cascading deserialization
    /// works correctly. First, the JSON string is deserialized into a
    /// `serde_json::Value`, and then it is deserialized into a `Meta` struct.
    /// This ensures that intermediate representations can be correctly handled
    /// by our deserialization logic.
    #[test]
    fn test_serde_json_de_cascaded() {

        let input = r#"{
            "Type": "Pass"
        }"#;

        let result: serde_json::Value = serde_json::from_str(input).unwrap();

        let _cresult: Meta = serde_json::from_value(result.clone()).unwrap();
    }
}
