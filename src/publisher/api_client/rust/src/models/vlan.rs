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
pub struct Vlan {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "site", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub site: Option<Option<Box<crate::models::NestedSite>>>,
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<Box<crate::models::NestedVlanGroup>>,
    #[serde(rename = "vid")]
    pub vid: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "tenant", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Option<Box<crate::models::NestedTenant>>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<crate::models::Status8>>,
    #[serde(rename = "role", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub role: Option<Option<Box<crate::models::NestedRole>>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "l2vpn_termination", skip_serializing_if = "Option::is_none")]
    pub l2vpn_termination: Option<Box<crate::models::NestedL2VpnTermination>>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTag>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
    #[serde(rename = "created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub created: Option<Option<String>>,
    #[serde(rename = "last_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<Option<String>>,
    #[serde(rename = "prefix_count", skip_serializing_if = "Option::is_none")]
    pub prefix_count: Option<i32>,
}

impl Vlan {
    pub fn new(vid: i32, name: String) -> Vlan {
        Vlan {
            id: None,
            url: None,
            display: None,
            site: None,
            group: None,
            vid,
            name,
            tenant: None,
            status: None,
            role: None,
            description: None,
            l2vpn_termination: None,
            tags: None,
            custom_fields: None,
            created: None,
            last_updated: None,
            prefix_count: None,
        }
    }
}

