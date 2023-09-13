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
pub struct WritableProvider {
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
    /// 32-bit autonomous system number
    #[serde(rename = "asn", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub asn: Option<Option<i32>>,
    #[serde(rename = "account", skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(rename = "portal_url", skip_serializing_if = "Option::is_none")]
    pub portal_url: Option<String>,
    #[serde(rename = "noc_contact", skip_serializing_if = "Option::is_none")]
    pub noc_contact: Option<String>,
    #[serde(rename = "admin_contact", skip_serializing_if = "Option::is_none")]
    pub admin_contact: Option<String>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "asns", skip_serializing_if = "Option::is_none")]
    pub asns: Option<Vec<i32>>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTag>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
    #[serde(rename = "created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub created: Option<Option<String>>,
    #[serde(rename = "last_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<Option<String>>,
    #[serde(rename = "circuit_count", skip_serializing_if = "Option::is_none")]
    pub circuit_count: Option<i32>,
}

impl WritableProvider {
    pub fn new(name: String, slug: String) -> WritableProvider {
        WritableProvider {
            id: None,
            url: None,
            display: None,
            name,
            slug,
            asn: None,
            account: None,
            portal_url: None,
            noc_contact: None,
            admin_contact: None,
            comments: None,
            asns: None,
            tags: None,
            custom_fields: None,
            created: None,
            last_updated: None,
            circuit_count: None,
        }
    }
}


