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
pub struct ComponentNestedModule {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "device")]
    pub device: i32,
    #[serde(rename = "module_bay", skip_serializing_if = "Option::is_none")]
    pub module_bay: Option<Box<crate::models::ModuleNestedModuleBay>>,
}

impl ComponentNestedModule {
    pub fn new(device: i32) -> ComponentNestedModule {
        ComponentNestedModule {
            id: None,
            url: None,
            display: None,
            device,
            module_bay: None,
        }
    }
}


