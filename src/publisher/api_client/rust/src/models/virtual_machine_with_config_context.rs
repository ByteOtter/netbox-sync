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
pub struct VirtualMachineWithConfigContext {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<crate::models::Status11>>,
    #[serde(rename = "site", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub site: Option<Option<Box<crate::models::NestedSite>>>,
    #[serde(rename = "cluster", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Option<Box<crate::models::NestedCluster>>>,
    #[serde(rename = "device", skip_serializing_if = "Option::is_none")]
    pub device: Option<Box<crate::models::NestedDevice>>,
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Box<crate::models::NestedDeviceRole>>,
    #[serde(rename = "tenant", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Option<Box<crate::models::NestedTenant>>>,
    #[serde(rename = "platform", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub platform: Option<Option<Box<crate::models::NestedPlatform>>>,
    #[serde(rename = "primary_ip", skip_serializing_if = "Option::is_none")]
    pub primary_ip: Option<Box<crate::models::NestedIpAddress>>,
    #[serde(rename = "primary_ip4", skip_serializing_if = "Option::is_none")]
    pub primary_ip4: Option<Box<crate::models::NestedIpAddress>>,
    #[serde(rename = "primary_ip6", skip_serializing_if = "Option::is_none")]
    pub primary_ip6: Option<Box<crate::models::NestedIpAddress>>,
    #[serde(rename = "vcpus", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub vcpus: Option<Option<f32>>,
    #[serde(rename = "memory", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub memory: Option<Option<i32>>,
    #[serde(rename = "disk", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub disk: Option<Option<i32>>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "local_context_data", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub local_context_data: Option<Option<serde_json::Value>>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTag>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
    #[serde(rename = "config_context", skip_serializing_if = "Option::is_none")]
    pub config_context: Option<serde_json::Value>,
    #[serde(rename = "created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub created: Option<Option<String>>,
    #[serde(rename = "last_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<Option<String>>,
}

impl VirtualMachineWithConfigContext {
    pub fn new(name: String) -> VirtualMachineWithConfigContext {
        VirtualMachineWithConfigContext {
            id: None,
            url: None,
            display: None,
            name,
            status: None,
            site: None,
            cluster: None,
            device: None,
            role: None,
            tenant: None,
            platform: None,
            primary_ip: None,
            primary_ip4: None,
            primary_ip6: None,
            vcpus: None,
            memory: None,
            disk: None,
            comments: None,
            local_context_data: None,
            tags: None,
            custom_fields: None,
            config_context: None,
            created: None,
            last_updated: None,
        }
    }
}

