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
pub struct L2Vpn {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "identifier", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Option<i32>>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Box<crate::models::Type8>>,
    #[serde(rename = "import_targets", skip_serializing_if = "Option::is_none")]
    pub import_targets: Option<Vec<crate::models::NestedRouteTarget>>,
    #[serde(rename = "export_targets", skip_serializing_if = "Option::is_none")]
    pub export_targets: Option<Vec<crate::models::NestedRouteTarget>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "tenant", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Option<Box<crate::models::NestedTenant>>>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTag>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
    #[serde(rename = "created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub created: Option<Option<String>>,
    #[serde(rename = "last_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<Option<String>>,
}

impl L2Vpn {
    pub fn new(name: String, slug: String) -> L2Vpn {
        L2Vpn {
            id: None,
            url: None,
            display: None,
            identifier: None,
            name,
            slug,
            r#type: None,
            import_targets: None,
            export_targets: None,
            description: None,
            tenant: None,
            tags: None,
            custom_fields: None,
            created: None,
            last_updated: None,
        }
    }
}


