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
pub struct PowerPort {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "device")]
    pub device: Box<crate::models::NestedDevice>,
    #[serde(rename = "module", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub module: Option<Option<Box<crate::models::ComponentNestedModule>>>,
    #[serde(rename = "name")]
    pub name: String,
    /// Physical label
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Box<crate::models::Type5>>,
    /// Maximum power draw (watts)
    #[serde(rename = "maximum_draw", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub maximum_draw: Option<Option<i32>>,
    /// Allocated power draw (watts)
    #[serde(rename = "allocated_draw", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub allocated_draw: Option<Option<i32>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Treat as if a cable is connected
    #[serde(rename = "mark_connected", skip_serializing_if = "Option::is_none")]
    pub mark_connected: Option<bool>,
    #[serde(rename = "cable", skip_serializing_if = "Option::is_none")]
    pub cable: Option<Box<crate::models::NestedCable>>,
    #[serde(rename = "cable_end", skip_serializing_if = "Option::is_none")]
    pub cable_end: Option<String>,
    ///  Return the appropriate serializer for the link termination model. 
    #[serde(rename = "link_peers", skip_serializing_if = "Option::is_none")]
    pub link_peers: Option<Vec<String>>,
    #[serde(rename = "link_peers_type", skip_serializing_if = "Option::is_none")]
    pub link_peers_type: Option<String>,
    ///  Return the appropriate serializer for the type of connected object. 
    #[serde(rename = "connected_endpoints", skip_serializing_if = "Option::is_none")]
    pub connected_endpoints: Option<Vec<String>>,
    #[serde(rename = "connected_endpoints_type", skip_serializing_if = "Option::is_none")]
    pub connected_endpoints_type: Option<String>,
    #[serde(rename = "connected_endpoints_reachable", skip_serializing_if = "Option::is_none")]
    pub connected_endpoints_reachable: Option<bool>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTag>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
    #[serde(rename = "created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub created: Option<Option<String>>,
    #[serde(rename = "last_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<Option<String>>,
    #[serde(rename = "_occupied", skip_serializing_if = "Option::is_none")]
    pub _occupied: Option<bool>,
}

impl PowerPort {
    pub fn new(device: crate::models::NestedDevice, name: String) -> PowerPort {
        PowerPort {
            id: None,
            url: None,
            display: None,
            device: Box::new(device),
            module: None,
            name,
            label: None,
            r#type: None,
            maximum_draw: None,
            allocated_draw: None,
            description: None,
            mark_connected: None,
            cable: None,
            cable_end: None,
            link_peers: None,
            link_peers_type: None,
            connected_endpoints: None,
            connected_endpoints_type: None,
            connected_endpoints_reachable: None,
            tags: None,
            custom_fields: None,
            created: None,
            last_updated: None,
            _occupied: None,
        }
    }
}


