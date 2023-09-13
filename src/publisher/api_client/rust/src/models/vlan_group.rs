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
pub struct VlanGroup {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "scope_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<Option<String>>,
    #[serde(rename = "scope_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<Option<i32>>,
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<serde_json::Value>,
    /// Lowest permissible ID of a child VLAN
    #[serde(rename = "min_vid", skip_serializing_if = "Option::is_none")]
    pub min_vid: Option<i32>,
    /// Highest permissible ID of a child VLAN
    #[serde(rename = "max_vid", skip_serializing_if = "Option::is_none")]
    pub max_vid: Option<i32>,
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
    #[serde(rename = "vlan_count", skip_serializing_if = "Option::is_none")]
    pub vlan_count: Option<i32>,
}

impl VlanGroup {
    pub fn new(name: String, slug: String) -> VlanGroup {
        VlanGroup {
            id: None,
            url: None,
            display: None,
            name,
            slug,
            scope_type: None,
            scope_id: None,
            scope: None,
            min_vid: None,
            max_vid: None,
            description: None,
            tags: None,
            custom_fields: None,
            created: None,
            last_updated: None,
            vlan_count: None,
        }
    }
}


