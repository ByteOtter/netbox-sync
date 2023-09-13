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
pub struct Status9 {
    #[serde(rename = "label")]
    pub label: Label,
    #[serde(rename = "value")]
    pub value: Value,
}

impl Status9 {
    pub fn new(label: Label, value: Value) -> Status9 {
        Status9 {
            label,
            value,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Label {
    #[serde(rename = "Container")]
    Container,
    #[serde(rename = "Active")]
    Active,
    #[serde(rename = "Reserved")]
    Reserved,
    #[serde(rename = "Deprecated")]
    Deprecated,
}

impl Default for Label {
    fn default() -> Label {
        Self::Container
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Value {
    #[serde(rename = "container")]
    Container,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "reserved")]
    Reserved,
    #[serde(rename = "deprecated")]
    Deprecated,
}

impl Default for Value {
    fn default() -> Value {
        Self::Container
    }
}

