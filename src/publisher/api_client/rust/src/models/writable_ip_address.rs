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
pub struct WritableIpAddress {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "family", skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    /// IPv4 or IPv6 address (with mask)
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "vrf", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub vrf: Option<Option<i32>>,
    #[serde(rename = "tenant", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Option<i32>>,
    /// The operational status of this IP
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// The functional role of this IP
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
    #[serde(rename = "assigned_object_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub assigned_object_type: Option<Option<String>>,
    #[serde(rename = "assigned_object_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub assigned_object_id: Option<Option<i32>>,
    #[serde(rename = "assigned_object", skip_serializing_if = "Option::is_none")]
    pub assigned_object: Option<serde_json::Value>,
    /// The IP for which this address is the \"outside\" IP
    #[serde(rename = "nat_inside", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub nat_inside: Option<Option<i32>>,
    #[serde(rename = "nat_outside", skip_serializing_if = "Option::is_none")]
    pub nat_outside: Option<Vec<crate::models::NestedIpAddress>>,
    /// Hostname or FQDN (not case-sensitive)
    #[serde(rename = "dns_name", skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
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

impl WritableIpAddress {
    pub fn new(address: String) -> WritableIpAddress {
        WritableIpAddress {
            id: None,
            url: None,
            display: None,
            family: None,
            address,
            vrf: None,
            tenant: None,
            status: None,
            role: None,
            assigned_object_type: None,
            assigned_object_id: None,
            assigned_object: None,
            nat_inside: None,
            nat_outside: None,
            dns_name: None,
            description: None,
            tags: None,
            custom_fields: None,
            created: None,
            last_updated: None,
        }
    }
}

/// The operational status of this IP
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "reserved")]
    Reserved,
    #[serde(rename = "deprecated")]
    Deprecated,
    #[serde(rename = "dhcp")]
    Dhcp,
    #[serde(rename = "slaac")]
    Slaac,
}

impl Default for Status {
    fn default() -> Status {
        Self::Active
    }
}
/// The functional role of this IP
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "loopback")]
    Loopback,
    #[serde(rename = "secondary")]
    Secondary,
    #[serde(rename = "anycast")]
    Anycast,
    #[serde(rename = "vip")]
    Vip,
    #[serde(rename = "vrrp")]
    Vrrp,
    #[serde(rename = "hsrp")]
    Hsrp,
    #[serde(rename = "glbp")]
    Glbp,
    #[serde(rename = "carp")]
    Carp,
}

impl Default for Role {
    fn default() -> Role {
        Self::Loopback
    }
}
