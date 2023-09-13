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
pub struct LengthUnit {
    #[serde(rename = "label")]
    pub label: Label,
    #[serde(rename = "value")]
    pub value: Value,
}

impl LengthUnit {
    pub fn new(label: Label, value: Value) -> LengthUnit {
        LengthUnit {
            label,
            value,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Label {
    #[serde(rename = "Kilometers")]
    Kilometers,
    #[serde(rename = "Meters")]
    Meters,
    #[serde(rename = "Centimeters")]
    Centimeters,
    #[serde(rename = "Miles")]
    Miles,
    #[serde(rename = "Feet")]
    Feet,
    #[serde(rename = "Inches")]
    Inches,
}

impl Default for Label {
    fn default() -> Label {
        Self::Kilometers
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Value {
    #[serde(rename = "km")]
    Km,
    #[serde(rename = "m")]
    M,
    #[serde(rename = "cm")]
    Cm,
    #[serde(rename = "mi")]
    Mi,
    #[serde(rename = "ft")]
    Ft,
    #[serde(rename = "in")]
    In,
}

impl Default for Value {
    fn default() -> Value {
        Self::Km
    }
}

