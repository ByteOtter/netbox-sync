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
pub struct NestedPowerPort {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "device", skip_serializing_if = "Option::is_none")]
    pub device: Option<Box<crate::models::NestedDevice>>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "cable", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub cable: Option<Option<i32>>,
    #[serde(rename = "_occupied", skip_serializing_if = "Option::is_none")]
    pub _occupied: Option<bool>,
}

impl NestedPowerPort {
    pub fn new(name: String) -> NestedPowerPort {
        NestedPowerPort {
            id: None,
            url: None,
            display: None,
            device: None,
            name,
            cable: None,
            _occupied: None,
        }
    }
}

