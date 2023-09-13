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
pub struct JobResult {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "completed", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub completed: Option<Option<String>>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "obj_type", skip_serializing_if = "Option::is_none")]
    pub obj_type: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<crate::models::Status6>>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::models::NestedUser>>,
    #[serde(rename = "data", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub data: Option<Option<serde_json::Value>>,
    #[serde(rename = "job_id")]
    pub job_id: uuid::Uuid,
}

impl JobResult {
    pub fn new(name: String, job_id: uuid::Uuid) -> JobResult {
        JobResult {
            id: None,
            url: None,
            display: None,
            created: None,
            completed: None,
            name,
            obj_type: None,
            status: None,
            user: None,
            data: None,
            job_id,
        }
    }
}


