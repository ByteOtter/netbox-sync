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
pub struct WritablePrefix {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "family", skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    /// IPv4 or IPv6 network with mask
    #[serde(rename = "prefix")]
    pub prefix: String,
    #[serde(rename = "site", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub site: Option<Option<i32>>,
    #[serde(rename = "vrf", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub vrf: Option<Option<i32>>,
    #[serde(rename = "tenant", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Option<i32>>,
    #[serde(rename = "vlan", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub vlan: Option<Option<i32>>,
    /// Operational status of this prefix
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// The primary function of this prefix
    #[serde(rename = "role", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub role: Option<Option<i32>>,
    /// All IP addresses within this prefix are considered usable
    #[serde(rename = "is_pool", skip_serializing_if = "Option::is_none")]
    pub is_pool: Option<bool>,
    /// Treat as 100% utilized
    #[serde(rename = "mark_utilized", skip_serializing_if = "Option::is_none")]
    pub mark_utilized: Option<bool>,
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
    #[serde(rename = "children", skip_serializing_if = "Option::is_none")]
    pub children: Option<i32>,
    #[serde(rename = "_depth", skip_serializing_if = "Option::is_none")]
    pub _depth: Option<i32>,
}

impl WritablePrefix {
    pub fn new(prefix: String) -> WritablePrefix {
        WritablePrefix {
            id: None,
            url: None,
            display: None,
            family: None,
            prefix,
            site: None,
            vrf: None,
            tenant: None,
            vlan: None,
            status: None,
            role: None,
            is_pool: None,
            mark_utilized: None,
            description: None,
            tags: None,
            custom_fields: None,
            created: None,
            last_updated: None,
            children: None,
            _depth: None,
        }
    }
}

/// Operational status of this prefix
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "container")]
    Container,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "reserved")]
    Reserved,
    #[serde(rename = "deprecated")]
    Deprecated,
}

impl Default for Status {
    fn default() -> Status {
        Self::Container
    }
}

