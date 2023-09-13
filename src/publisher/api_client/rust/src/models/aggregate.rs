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
pub struct Aggregate {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "family", skip_serializing_if = "Option::is_none")]
    pub family: Option<Box<crate::models::Family>>,
    #[serde(rename = "prefix")]
    pub prefix: String,
    #[serde(rename = "rir")]
    pub rir: Box<crate::models::NestedRir>,
    #[serde(rename = "tenant", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Option<Box<crate::models::NestedTenant>>>,
    #[serde(rename = "date_added", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_added: Option<Option<String>>,
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

impl Aggregate {
    pub fn new(prefix: String, rir: crate::models::NestedRir) -> Aggregate {
        Aggregate {
            id: None,
            url: None,
            display: None,
            family: None,
            prefix,
            rir: Box::new(rir),
            tenant: None,
            date_added: None,
            description: None,
            tags: None,
            custom_fields: None,
            created: None,
            last_updated: None,
        }
    }
}


