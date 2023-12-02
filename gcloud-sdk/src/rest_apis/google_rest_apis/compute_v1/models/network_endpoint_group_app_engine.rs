use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// NetworkEndpointGroupAppEngine : Configuration for an App Engine network endpoint group (NEG). The service is optional, may be provided explicitly or in the URL mask. The version is optional and can only be provided explicitly or in the URL mask when service is present. Note: App Engine service must be in the same project and located in the same region as the Serverless NEG.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NetworkEndpointGroupAppEngine {
    /// Optional serving service. The service name is case-sensitive and must be 1-63 characters long. Example value: \"default\", \"my-service\".
    #[serde(rename = "service", skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    /// A template to parse service and version fields from a request URL. URL mask allows for routing to multiple App Engine services without having to create multiple Network Endpoint Groups and backend services. For example, the request URLs \"foo1-dot-appname.appspot.com/v1\" and \"foo1-dot-appname.appspot.com/v2\" can be backed by the same Serverless NEG with URL mask \"<service>-dot-appname.appspot.com/<version>\". The URL mask will parse them to { service = \"foo1\", version = \"v1\" } and { service = \"foo1\", version = \"v2\" } respectively.
    #[serde(rename = "urlMask", skip_serializing_if = "Option::is_none")]
    pub url_mask: Option<String>,
    /// Optional serving version. The version name is case-sensitive and must be 1-100 characters long. Example value: \"v1\", \"v2\".
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl NetworkEndpointGroupAppEngine {
    /// Configuration for an App Engine network endpoint group (NEG). The service is optional, may be provided explicitly or in the URL mask. The version is optional and can only be provided explicitly or in the URL mask when service is present. Note: App Engine service must be in the same project and located in the same region as the Serverless NEG.
    pub fn new() -> NetworkEndpointGroupAppEngine {
        NetworkEndpointGroupAppEngine {
            service: None,
            url_mask: None,
            version: None,
        }
    }
}