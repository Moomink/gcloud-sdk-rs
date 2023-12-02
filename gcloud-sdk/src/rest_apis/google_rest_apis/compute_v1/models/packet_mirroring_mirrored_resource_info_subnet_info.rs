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
pub struct PacketMirroringMirroredResourceInfoSubnetInfo {
    /// [Output Only] Unique identifier for the subnetwork; defined by the server.
    #[serde(rename = "canonicalUrl", skip_serializing_if = "Option::is_none")]
    pub canonical_url: Option<String>,
    /// Resource URL to the subnetwork for which traffic from/to all VM instances will be mirrored.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl PacketMirroringMirroredResourceInfoSubnetInfo {
    pub fn new() -> PacketMirroringMirroredResourceInfoSubnetInfo {
        PacketMirroringMirroredResourceInfoSubnetInfo {
            canonical_url: None,
            url: None,
        }
    }
}