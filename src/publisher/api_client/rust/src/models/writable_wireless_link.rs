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
pub struct WritableWirelessLink {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "interface_a")]
    pub interface_a: i32,
    #[serde(rename = "interface_b")]
    pub interface_b: i32,
    #[serde(rename = "ssid", skip_serializing_if = "Option::is_none")]
    pub ssid: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "tenant", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Option<i32>>,
    #[serde(rename = "auth_type", skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<AuthType>,
    #[serde(rename = "auth_cipher", skip_serializing_if = "Option::is_none")]
    pub auth_cipher: Option<AuthCipher>,
    #[serde(rename = "auth_psk", skip_serializing_if = "Option::is_none")]
    pub auth_psk: Option<String>,
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

impl WritableWirelessLink {
    pub fn new(interface_a: i32, interface_b: i32) -> WritableWirelessLink {
        WritableWirelessLink {
            id: None,
            url: None,
            display: None,
            interface_a,
            interface_b,
            ssid: None,
            status: None,
            tenant: None,
            auth_type: None,
            auth_cipher: None,
            auth_psk: None,
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
pub enum Status {
    #[serde(rename = "connected")]
    Connected,
    #[serde(rename = "planned")]
    Planned,
    #[serde(rename = "decommissioning")]
    Decommissioning,
}

impl Default for Status {
    fn default() -> Status {
        Self::Connected
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AuthType {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "wep")]
    Wep,
    #[serde(rename = "wpa-personal")]
    WpaPersonal,
    #[serde(rename = "wpa-enterprise")]
    WpaEnterprise,
}

impl Default for AuthType {
    fn default() -> AuthType {
        Self::Open
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AuthCipher {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "tkip")]
    Tkip,
    #[serde(rename = "aes")]
    Aes,
}

impl Default for AuthCipher {
    fn default() -> AuthCipher {
        Self::Auto
    }
}

