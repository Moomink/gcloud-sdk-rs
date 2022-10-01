use serde::{Deserialize, Serialize}; /*
                                      * BigQuery API
                                      *
                                      * A data platform for customers to create, manage, share and query data.
                                      *
                                      * The version of the OpenAPI document: v2
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ParquetOptions {
    /// [Optional] Indicates whether to use schema inference specifically for Parquet LIST logical type.
    #[serde(
        rename = "enableListInference",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_list_inference: Option<bool>,
    /// [Optional] Indicates whether to infer Parquet ENUM logical type as STRING instead of BYTES by default.
    #[serde(rename = "enumAsString", skip_serializing_if = "Option::is_none")]
    pub enum_as_string: Option<bool>,
}

impl ParquetOptions {
    pub fn new() -> ParquetOptions {
        ParquetOptions {
            enable_list_inference: None,
            enum_as_string: None,
        }
    }
}