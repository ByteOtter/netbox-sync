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
pub struct WritableService {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "device", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub device: Option<Option<i32>>,
    #[serde(rename = "virtual_machine", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub virtual_machine: Option<Option<i32>>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "ports")]
    pub ports: Vec<i32>,
    #[serde(rename = "protocol")]
    pub protocol: Protocol,
    #[serde(rename = "ipaddresses", skip_serializing_if = "Option::is_none")]
    pub ipaddresses: Option<Vec<i32>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTag>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
    #[serde(rename = "created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub created: Option<Option<String>>,
    #[serde(rename = "last_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<Option<String>>,
}

impl WritableService {
    pub fn new(name: String, ports: Vec<i32>, protocol: Protocol) -> WritableService {
        WritableService {
            id: None,
            url: None,
            display: None,
            device: None,
            virtual_machine: None,
            name,
            ports,
            protocol,
            ipaddresses: None,
            description: None,
            tags: None,
            custom_fields: None,
            created: None,
            last_updated: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Protocol {
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "udp")]
    Udp,
    #[serde(rename = "sctp")]
    Sctp,
}

impl Default for Protocol {
    fn default() -> Protocol {
        Self::Tcp
    }
}
