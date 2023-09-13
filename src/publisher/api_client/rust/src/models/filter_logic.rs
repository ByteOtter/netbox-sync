/*
 * NetBox API
 *
 * API to access NetBox
 *
 * The version of the OpenAPI document: 3.3
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FilterLogic {
    #[serde(rename = "label")]
    pub label: Label,
    #[serde(rename = "value")]
    pub value: Value,
}

impl FilterLogic {
    pub fn new(label: Label, value: Value) -> FilterLogic {
        FilterLogic {
            label,
            value,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Label {
    #[serde(rename = "Disabled")]
    Disabled,
    #[serde(rename = "Loose")]
    Loose,
    #[serde(rename = "Exact")]
    Exact,
}

impl Default for Label {
    fn default() -> Label {
        Self::Disabled
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Value {
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "loose")]
    Loose,
    #[serde(rename = "exact")]
    Exact,
}

impl Default for Value {
    fn default() -> Value {
        Self::Disabled
    }
}
