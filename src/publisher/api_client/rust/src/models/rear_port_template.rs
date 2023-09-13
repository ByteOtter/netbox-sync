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
pub struct RearPortTemplate {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "device_type", skip_serializing_if = "Option::is_none")]
    pub device_type: Option<Box<crate::models::NestedDeviceType>>,
    #[serde(rename = "module_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub module_type: Option<Option<Box<crate::models::NestedModuleType>>>,
    ///  {module} is accepted as a substitution for the module bay position when attached to a module type. 
    #[serde(rename = "name")]
    pub name: String,
    /// Physical label
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Box<crate::models::Type1>,
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "positions", skip_serializing_if = "Option::is_none")]
    pub positions: Option<i32>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub created: Option<Option<String>>,
    #[serde(rename = "last_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<Option<String>>,
}

impl RearPortTemplate {
    pub fn new(name: String, r#type: crate::models::Type1) -> RearPortTemplate {
        RearPortTemplate {
            id: None,
            url: None,
            display: None,
            device_type: None,
            module_type: None,
            name,
            label: None,
            r#type: Box::new(r#type),
            color: None,
            positions: None,
            description: None,
            created: None,
            last_updated: None,
        }
    }
}


