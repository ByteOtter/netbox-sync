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
pub struct RackUnit {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<f32>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "face", skip_serializing_if = "Option::is_none")]
    pub face: Option<Box<crate::models::Face1>>,
    #[serde(rename = "device", skip_serializing_if = "Option::is_none")]
    pub device: Option<Box<crate::models::NestedDevice>>,
    #[serde(rename = "occupied", skip_serializing_if = "Option::is_none")]
    pub occupied: Option<bool>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
}

impl RackUnit {
    pub fn new() -> RackUnit {
        RackUnit {
            id: None,
            name: None,
            face: None,
            device: None,
            occupied: None,
            display: None,
        }
    }
}


