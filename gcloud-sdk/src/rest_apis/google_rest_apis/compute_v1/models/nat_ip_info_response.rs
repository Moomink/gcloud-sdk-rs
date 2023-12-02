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
pub struct NatIpInfoResponse {
    /// [Output Only] A list of NAT IP information.
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Vec<crate::google_rest_apis::compute_v1::models::NatIpInfo>>,
}

impl NatIpInfoResponse {
    pub fn new() -> NatIpInfoResponse {
        NatIpInfoResponse { result: None }
    }
}