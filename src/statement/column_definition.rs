use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::statement::validate::validate_alphanumunderscore;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct ColumnDefinition {
    #[validate(custom(function = "validate_alphanumunderscore"))]
    #[serde(rename = "name")]
    pub name: String,

    #[validate(length(min = 1))]
    #[serde(rename = "type")]
    pub col_type: String,

    #[serde(rename = "length")]
    pub length: i32,

    #[serde(rename = "primary_key")]
    pub primary_key: bool,

    #[serde(rename = "index")]
    pub index: bool,

    #[serde(rename = "default_value")]
    pub default_value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ColumnsDefinition(pub Vec<ColumnDefinition>);

impl ColumnsDefinition {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn to_string(&self) -> String {
        format!("{:?}", self.0)
    }
}
