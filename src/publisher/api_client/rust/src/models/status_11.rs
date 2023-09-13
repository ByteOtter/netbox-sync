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
pub struct Status11 {
    #[serde(rename = "label")]
    pub label: Label,
    #[serde(rename = "value")]
    pub value: Value,
}

impl Status11 {
    pub fn new(label: Label, value: Value) -> Status11 {
        Status11 {
            label,
            value,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Label {
    #[serde(rename = "Offline")]
    Offline,
    #[serde(rename = "Active")]
    Active,
    #[serde(rename = "Planned")]
    Planned,
    #[serde(rename = "Staged")]
    Staged,
    #[serde(rename = "Failed")]
    Failed,
    #[serde(rename = "Decommissioning")]
    Decommissioning,
}

impl Default for Label {
    fn default() -> Label {
        Self::Offline
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Value {
    #[serde(rename = "offline")]
    Offline,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "planned")]
    Planned,
    #[serde(rename = "staged")]
    Staged,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "decommissioning")]
    Decommissioning,
}

impl Default for Value {
    fn default() -> Value {
        Self::Offline
    }
}
