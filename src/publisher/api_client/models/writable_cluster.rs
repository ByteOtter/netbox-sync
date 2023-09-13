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
pub struct WritableCluster {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: i32,
    #[serde(rename = "group", deserialize_with = "Option::deserialize")]
    pub group: Option<i32>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "tenant", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Option<i32>>,
    #[serde(rename = "site", deserialize_with = "Option::deserialize")]
    pub site: Option<i32>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTag>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
    #[serde(rename = "created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub created: Option<Option<String>>,
    #[serde(rename = "last_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<Option<String>>,
    #[serde(rename = "device_count", skip_serializing_if = "Option::is_none")]
    pub device_count: Option<i32>,
    #[serde(rename = "virtualmachine_count", skip_serializing_if = "Option::is_none")]
    pub virtualmachine_count: Option<i32>,
}

impl WritableCluster {
    pub fn new(name: String, r#type: i32, group: Option<i32>, site: Option<i32>) -> WritableCluster {
        WritableCluster {
            id: None,
            url: None,
            display: None,
            name,
            r#type,
            group,
            status: None,
            tenant: None,
            site,
            comments: None,
            tags: None,
            custom_fields: None,
            created: None,
            last_updated: None,
            device_count: None,
            virtualmachine_count: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "planned")]
    Planned,
    #[serde(rename = "staging")]
    Staging,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "decommissioning")]
    Decommissioning,
    #[serde(rename = "offline")]
    Offline,
}

impl Default for Status {
    fn default() -> Status {
        Self::Planned
    }
}
