use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// BackendServiceCdnPolicyNegativeCachingPolicy : Specify CDN TTLs for response error codes.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BackendServiceCdnPolicyNegativeCachingPolicy {
    /// The HTTP status code to define a TTL against. Only HTTP status codes 300, 301, 302, 307, 308, 404, 405, 410, 421, 451 and 501 are can be specified as values, and you cannot specify a status code more than once.
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    /// The TTL (in seconds) for which to cache responses with the corresponding status code. The maximum allowed value is 1800s (30 minutes), noting that infrequently accessed objects may be evicted from the cache before the defined TTL.
    #[serde(rename = "ttl", skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i32>,
}

impl BackendServiceCdnPolicyNegativeCachingPolicy {
    /// Specify CDN TTLs for response error codes.
    pub fn new() -> BackendServiceCdnPolicyNegativeCachingPolicy {
        BackendServiceCdnPolicyNegativeCachingPolicy {
            code: None,
            ttl: None,
        }
    }
}