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
pub struct Token {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "user")]
    pub user: Box<crate::models::NestedUser>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "expires", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub expires: Option<Option<String>>,
    #[serde(rename = "last_used", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_used: Option<Option<String>>,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Permit create/update/delete operations using this key
    #[serde(rename = "write_enabled", skip_serializing_if = "Option::is_none")]
    pub write_enabled: Option<bool>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "allowed_ips", skip_serializing_if = "Option::is_none")]
    pub allowed_ips: Option<Vec<serde_json::Value>>,
}

impl Token {
    pub fn new(user: crate::models::NestedUser) -> Token {
        Token {
            id: None,
            url: None,
            display: None,
            user: Box::new(user),
            created: None,
            expires: None,
            last_used: None,
            key: None,
            write_enabled: None,
            description: None,
            allowed_ips: None,
        }
    }
}


