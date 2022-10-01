use serde::{Deserialize, Serialize}; /*
                                      * Cloud Storage JSON API
                                      *
                                      * Stores and retrieves potentially large, immutable data objects.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// RewriteResponse : A rewrite response.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RewriteResponse {
    /// true if the copy is finished; otherwise, false if the copy is in progress. This property is always present in the response.
    #[serde(rename = "done", skip_serializing_if = "Option::is_none")]
    pub done: Option<bool>,
    /// The kind of item this is.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The total size of the object being copied in bytes. This property is always present in the response.
    #[serde(rename = "objectSize", skip_serializing_if = "Option::is_none")]
    pub object_size: Option<String>,
    #[serde(rename = "resource", skip_serializing_if = "Option::is_none")]
    pub resource: Option<Box<crate::google_rest_apis::storage_v1::models::Object>>,
    /// A token to use in subsequent requests to continue copying data. This token is present in the response only when there is more data to copy.
    #[serde(rename = "rewriteToken", skip_serializing_if = "Option::is_none")]
    pub rewrite_token: Option<String>,
    /// The total bytes written so far, which can be used to provide a waiting user with a progress indicator. This property is always present in the response.
    #[serde(
        rename = "totalBytesRewritten",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_bytes_rewritten: Option<String>,
}

impl RewriteResponse {
    /// A rewrite response.
    pub fn new() -> RewriteResponse {
        RewriteResponse {
            done: None,
            kind: None,
            object_size: None,
            resource: None,
            rewrite_token: None,
            total_bytes_rewritten: None,
        }
    }
}