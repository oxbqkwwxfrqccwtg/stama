use serde::{Serialize, Deserialize, Serializer, Deserializer};

use std::collections::BTreeMap;



#[derive(Debug,Serialize, Deserialize, PartialEq)]
enum StateType {
    Pass
}

type PayloadTemplate<'a> = BTreeMap<&'a str, &'a str>;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct Retrier<'a> {
    #[serde(borrow)]
    error_equals: Vec<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval_seconds: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_attempts: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backoff_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jitter_strategy: Option<&'a str>
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct Meta<'a> {
    r#type: StateType,
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_path: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_path: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result_path: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<PayloadTemplate<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result_selector: Option<PayloadTemplate<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry: Option<Vec<Retrier<'a>>>,
    ////catch
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Result};

    #[test]
    fn test_json() {

        let input = r#"{
            "Type": "Pass",
            "Comment": "Comment"
        }"#;

        let result: Meta = serde_json::from_str(input).unwrap();

        assert_eq!(result.r#type, StateType::Pass);
        assert_eq!(result.comment, Some("Comment"));
    }
}
