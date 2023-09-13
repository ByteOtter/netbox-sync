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
pub struct NestedModule {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "device", skip_serializing_if = "Option::is_none")]
    pub device: Option<Box<crate::models::NestedDevice>>,
    #[serde(rename = "module_bay", skip_serializing_if = "Option::is_none")]
    pub module_bay: Option<Box<crate::models::ModuleNestedModuleBay>>,
    #[serde(rename = "module_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub module_type: Option<Option<Box<crate::models::NestedModuleType>>>,
}

impl NestedModule {
    pub fn new() -> NestedModule {
        NestedModule {
            id: None,
            url: None,
            display: None,
            device: None,
            module_bay: None,
            module_type: None,
        }
    }
}


