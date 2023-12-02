use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServiceAttachmentConsumerProjectLimit {
    /// The value of the limit to set.
    #[serde(rename = "connectionLimit", skip_serializing_if = "Option::is_none")]
    pub connection_limit: Option<i32>,
    /// The network URL for the network to set the limit for.
    #[serde(rename = "networkUrl", skip_serializing_if = "Option::is_none")]
    pub network_url: Option<String>,
    /// The project id or number for the project to set the limit for.
    #[serde(rename = "projectIdOrNum", skip_serializing_if = "Option::is_none")]
    pub project_id_or_num: Option<String>,
}

impl ServiceAttachmentConsumerProjectLimit {
    pub fn new() -> ServiceAttachmentConsumerProjectLimit {
        ServiceAttachmentConsumerProjectLimit {
            connection_limit: None,
            network_url: None,
            project_id_or_num: None,
        }
    }
}