/// QuotaInfo represents information about a particular quota for a given
/// project, folder or organization.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuotaInfo {
    /// Resource name of this QuotaInfo.
    /// The ID component following "locations/" must be "global".
    /// Example:
    /// `projects/123/locations/global/services/compute.googleapis.com/quotaInfos/CpusPerProjectPerRegion`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The id of the quota, which is unquie within the service.
    /// Example: `CpusPerProjectPerRegion`
    #[prost(string, tag = "2")]
    pub quota_id: ::prost::alloc::string::String,
    /// The metric of the quota. It specifies the resources consumption the quota
    /// is defined for.
    /// Example: `compute.googleapis.com/cpus`
    #[prost(string, tag = "3")]
    pub metric: ::prost::alloc::string::String,
    /// The name of the service in which the quota is defined.
    /// Example: `compute.googleapis.com`
    #[prost(string, tag = "4")]
    pub service: ::prost::alloc::string::String,
    /// Whether this is a precise quota. A precise quota is tracked with absolute
    /// precision. In contrast, an imprecise quota is not tracked with precision.
    #[prost(bool, tag = "5")]
    pub is_precise: bool,
    /// The reset time interval for the quota. Refresh interval applies to rate
    /// quota only.
    /// Example: "minute" for per minute, "day" for per day, or "10 seconds" for
    /// every 10 seconds.
    #[prost(string, tag = "6")]
    pub refresh_interval: ::prost::alloc::string::String,
    /// The container type of the QuotaInfo.
    #[prost(enumeration = "quota_info::ContainerType", tag = "7")]
    pub container_type: i32,
    /// The dimensions the quota is defined on.
    #[prost(string, repeated, tag = "8")]
    pub dimensions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The display name of the quota metric
    #[prost(string, tag = "9")]
    pub metric_display_name: ::prost::alloc::string::String,
    /// The display name of the quota.
    #[prost(string, tag = "10")]
    pub quota_display_name: ::prost::alloc::string::String,
    /// The unit in which the metric value is reported, e.g., "MByte".
    #[prost(string, tag = "11")]
    pub metric_unit: ::prost::alloc::string::String,
    /// Whether it is eligible to request a higher quota value for this quota.
    #[prost(message, optional, tag = "12")]
    pub quota_increase_eligibility: ::core::option::Option<QuotaIncreaseEligibility>,
    /// Whether the quota value is fixed or adjustable
    #[prost(bool, tag = "13")]
    pub is_fixed: bool,
    /// The collection of dimensions info ordered by their dimensions from more
    /// specific ones to less specific ones.
    #[prost(message, repeated, tag = "14")]
    pub dimensions_infos: ::prost::alloc::vec::Vec<DimensionsInfo>,
    /// Whether the quota is a concurrent quota. Concurrent quotas are enforced
    /// on the total number of concurrent operations in flight at any given time.
    #[prost(bool, tag = "15")]
    pub is_concurrent: bool,
    /// URI to the page where the user can request more quotas for the cloud
    /// service, such as
    /// <https://docs.google.com/spreadsheet/viewform?formkey=abc123&entry_0={email}&entry_1={id}.>
    /// Google Developers Console UI replace {email} with the current
    /// user's e-mail, {id} with the current project number, or organization ID
    /// with "organizations/" prefix. For example,
    /// <https://docs.google.com/spreadsheet/viewform?formkey=abc123&entry_0=johndoe@gmail.com&entry_1=25463754,>
    /// or
    /// <https://docs.google.com/spreadsheet/viewform?formkey=abc123&entry_0=johndoe@gmail.com&entry_1=organizations/26474422.>
    #[prost(string, tag = "17")]
    pub service_request_quota_uri: ::prost::alloc::string::String,
}
/// Nested message and enum types in `QuotaInfo`.
pub mod quota_info {
    /// The enumeration of the types of a cloud resource container.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ContainerType {
        /// Unspecified container type.
        Unspecified = 0,
        /// consumer project
        Project = 1,
        /// folder
        Folder = 2,
        /// organization
        Organization = 3,
    }
    impl ContainerType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ContainerType::Unspecified => "CONTAINER_TYPE_UNSPECIFIED",
                ContainerType::Project => "PROJECT",
                ContainerType::Folder => "FOLDER",
                ContainerType::Organization => "ORGANIZATION",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CONTAINER_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "PROJECT" => Some(Self::Project),
                "FOLDER" => Some(Self::Folder),
                "ORGANIZATION" => Some(Self::Organization),
                _ => None,
            }
        }
    }
}
/// Eligibility information regarding requesting increase adjustment of a quota.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuotaIncreaseEligibility {
    /// Whether a higher quota value can be requested for the quota.
    #[prost(bool, tag = "1")]
    pub is_eligible: bool,
    /// The reason of why it is ineligible to request increased value of the quota.
    /// If the is_eligible field is true, it defaults to
    /// INELIGIBILITY_REASON_UNSPECIFIED.
    #[prost(enumeration = "quota_increase_eligibility::IneligibilityReason", tag = "2")]
    pub ineligibility_reason: i32,
}
/// Nested message and enum types in `QuotaIncreaseEligibility`.
pub mod quota_increase_eligibility {
    /// The enumeration of reasons when it is ineligible to request increase
    /// adjustment.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum IneligibilityReason {
        /// Default value when is_eligible is true.
        Unspecified = 0,
        /// The container is not linked with a valid billing account.
        NoValidBillingAccount = 1,
        /// Other reasons.
        Other = 2,
    }
    impl IneligibilityReason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                IneligibilityReason::Unspecified => "INELIGIBILITY_REASON_UNSPECIFIED",
                IneligibilityReason::NoValidBillingAccount => "NO_VALID_BILLING_ACCOUNT",
                IneligibilityReason::Other => "OTHER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INELIGIBILITY_REASON_UNSPECIFIED" => Some(Self::Unspecified),
                "NO_VALID_BILLING_ACCOUNT" => Some(Self::NoValidBillingAccount),
                "OTHER" => Some(Self::Other),
                _ => None,
            }
        }
    }
}
/// QuotaPreference represents the preferred quota configuration specified for
/// a project, folder or organization. There is only one QuotaPreference
/// resource for a quota value targeting a unique set of dimensions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuotaPreference {
    /// Required except in the CREATE requests.
    /// The resource name of the quota preference.
    /// The ID component following "locations/" must be "global".
    /// Example:
    /// `projects/123/locations/global/quotaPreferences/my-config-for-us-east1`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The dimensions that this quota preference applies to. The key of the map
    /// entry is the name of a dimension, such as "region", "zone", "network_id",
    /// and the value of the map entry is the dimension value.
    ///
    /// If a dimension is missing from the map of dimensions, the quota preference
    /// applies to all the dimension values except for those that have other quota
    /// preferences configured for the specific value.
    ///
    /// NOTE: QuotaPreferences can only be applied across all values of "user" and
    /// "resource" dimension. Do not set values for "user" or "resource" in the
    /// dimension map.
    ///
    /// Example: {"provider", "Foo Inc"} where "provider" is a service specific
    /// dimension.
    #[prost(map = "string, string", tag = "2")]
    pub dimensions: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Required. Preferred quota configuration.
    #[prost(message, optional, tag = "3")]
    pub quota_config: ::core::option::Option<QuotaConfig>,
    /// Optional. The current etag of the quota preference. If an etag is provided
    /// on update and does not match the current server's etag of the quota
    /// preference, the request will be blocked and an ABORTED error will be
    /// returned. See <https://google.aip.dev/134#etags> for more details on etags.
    #[prost(string, tag = "4")]
    pub etag: ::prost::alloc::string::String,
    /// Output only. Create time stamp
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Update time stamp
    #[prost(message, optional, tag = "6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. The name of the service to which the quota preference is applied.
    #[prost(string, tag = "7")]
    pub service: ::prost::alloc::string::String,
    /// Required. The id of the quota to which the quota preference is applied. A
    /// quota name is unique in the service. Example: `CpusPerProjectPerRegion`
    #[prost(string, tag = "8")]
    pub quota_id: ::prost::alloc::string::String,
    /// Output only. Is the quota preference pending Google Cloud approval and
    /// fulfillment.
    #[prost(bool, tag = "10")]
    pub reconciling: bool,
    /// The reason / justification for this quota preference.
    #[prost(string, tag = "11")]
    pub justification: ::prost::alloc::string::String,
    /// Required. Input only. An email address that can be used for quota related
    /// communication between the Google Cloud and the user in case the Google
    /// Cloud needs further information to make a decision on whether the user
    /// preferred quota can be granted.
    ///
    /// The Google account for the email address must have quota update permission
    /// for the project, folder or organization this quota preference is for.
    #[prost(string, tag = "12")]
    pub contact_email: ::prost::alloc::string::String,
}
/// The preferred quota configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuotaConfig {
    /// Required. The preferred value. Must be greater than or equal to -1. If set
    /// to -1, it means the value is "unlimited".
    #[prost(int64, tag = "1")]
    pub preferred_value: i64,
    /// Output only. Optional details about the state of this quota preference.
    #[prost(string, tag = "2")]
    pub state_detail: ::prost::alloc::string::String,
    /// Output only. Granted quota value.
    #[prost(message, optional, tag = "3")]
    pub granted_value: ::core::option::Option<i64>,
    /// Output only. The trace id that the Google Cloud uses to provision the
    /// requested quota. This trace id may be used by the client to contact Cloud
    /// support to track the state of a quota preference request. The trace id is
    /// only produced for increase requests and is unique for each request. The
    /// quota decrease requests do not have a trace id.
    #[prost(string, tag = "4")]
    pub trace_id: ::prost::alloc::string::String,
    /// The annotations map for clients to store small amounts of arbitrary data.
    /// Do not put PII or other sensitive information here.
    /// See <https://google.aip.dev/128#annotations>
    #[prost(map = "string, string", tag = "5")]
    pub annotations: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. The origin of the quota preference request.
    #[prost(enumeration = "quota_config::Origin", tag = "6")]
    pub request_origin: i32,
}
/// Nested message and enum types in `QuotaConfig`.
pub mod quota_config {
    /// The enumeration of the origins of quota preference requests.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Origin {
        /// The unspecified value.
        Unspecified = 0,
        /// Created through Cloud Console.
        CloudConsole = 1,
        /// Generated by automatic quota adjustment.
        AutoAdjuster = 2,
    }
    impl Origin {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Origin::Unspecified => "ORIGIN_UNSPECIFIED",
                Origin::CloudConsole => "CLOUD_CONSOLE",
                Origin::AutoAdjuster => "AUTO_ADJUSTER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ORIGIN_UNSPECIFIED" => Some(Self::Unspecified),
                "CLOUD_CONSOLE" => Some(Self::CloudConsole),
                "AUTO_ADJUSTER" => Some(Self::AutoAdjuster),
                _ => None,
            }
        }
    }
}
/// The detailed quota information such as effective quota value for a
/// combination of dimensions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DimensionsInfo {
    /// The map of dimensions for this dimensions info. The key of a map entry
    /// is "region", "zone" or the name of a service specific dimension, and the
    /// value of a map entry is the value of the dimension.  If a dimension does
    /// not appear in the map of dimensions, the dimensions info applies to all
    /// the dimension values except for those that have another DimenisonInfo
    /// instance configured for the specific value.
    /// Example: {"provider" : "Foo Inc"} where "provider" is a service specific
    /// dimension of a quota.
    #[prost(map = "string, string", tag = "1")]
    pub dimensions: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Quota details for the specified dimensions.
    #[prost(message, optional, tag = "2")]
    pub details: ::core::option::Option<QuotaDetails>,
    /// The applicable regions or zones of this dimensions info. The field will be
    /// set to \['global'\] for quotas that are not per region or per zone.
    /// Otherwise, it will be set to the list of locations this dimension info is
    /// applicable to.
    #[prost(string, repeated, tag = "3")]
    pub applicable_locations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The quota details for a map of dimensions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuotaDetails {
    /// The value currently in effect and being enforced.
    #[prost(int64, tag = "1")]
    pub value: i64,
}
/// Enumerations of quota safety checks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum QuotaSafetyCheck {
    /// Unspecified quota safety check.
    Unspecified = 0,
    /// Validates that a quota mutation would not cause the consumer's effective
    /// limit to be lower than the consumer's quota usage.
    QuotaDecreaseBelowUsage = 1,
    /// Validates that a quota mutation would not cause the consumer's effective
    /// limit to decrease by more than 10 percent.
    QuotaDecreasePercentageTooHigh = 2,
}
impl QuotaSafetyCheck {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            QuotaSafetyCheck::Unspecified => "QUOTA_SAFETY_CHECK_UNSPECIFIED",
            QuotaSafetyCheck::QuotaDecreaseBelowUsage => "QUOTA_DECREASE_BELOW_USAGE",
            QuotaSafetyCheck::QuotaDecreasePercentageTooHigh => {
                "QUOTA_DECREASE_PERCENTAGE_TOO_HIGH"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "QUOTA_SAFETY_CHECK_UNSPECIFIED" => Some(Self::Unspecified),
            "QUOTA_DECREASE_BELOW_USAGE" => Some(Self::QuotaDecreaseBelowUsage),
            "QUOTA_DECREASE_PERCENTAGE_TOO_HIGH" => {
                Some(Self::QuotaDecreasePercentageTooHigh)
            }
            _ => None,
        }
    }
}
/// Message for requesting list of QuotaInfos
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListQuotaInfosRequest {
    /// Required. Parent value of QuotaInfo resources.
    /// Listing across different resource containers (such as 'projects/-') is not
    /// allowed.
    ///
    /// Example names:
    /// `projects/123/locations/global/services/compute.googleapis.com`
    /// `folders/234/locations/global/services/compute.googleapis.com`
    /// `organizations/345/locations/global/services/compute.googleapis.com`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server may return fewer items than
    /// requested. If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Message for response to listing QuotaInfos
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListQuotaInfosResponse {
    /// The list of QuotaInfo
    #[prost(message, repeated, tag = "1")]
    pub quota_infos: ::prost::alloc::vec::Vec<QuotaInfo>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Message for getting a QuotaInfo
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetQuotaInfoRequest {
    /// Required. The resource name of the quota info.
    ///
    /// An example name:
    /// `projects/123/locations/global/services/compute.googleapis.com/quotaInfos/CpusPerProjectPerRegion`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for requesting list of QuotaPreferences
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListQuotaPreferencesRequest {
    /// Required. Parent value of QuotaPreference resources.
    /// Listing across different resource containers (such as 'projects/-') is not
    /// allowed.
    ///
    /// When the value starts with 'folders' or 'organizations', it lists the
    /// QuotaPreferences for org quotas in the container. It does not list the
    /// QuotaPreferences in the descendant projects of the container.
    ///
    /// Example parents:
    /// `projects/123/locations/global`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server may return fewer items than
    /// requested. If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filter result QuotaPreferences by their state, type,
    /// create/update time range.
    ///
    /// Example filters:
    /// `state=PENDING OR state=PENDING_PARTIALLY_GRANTED`
    /// `state=PENDING OR state=PENDING_PARTIALLY_GRANTED AND
    ///   creation_time>2022-12-03T10:30:00`
    ///
    /// If no filter is provided, returns all pending quota preferences.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. How to order of the results. By default, the results are ordered
    /// by create time.
    ///
    /// Example orders:
    /// `type`
    /// `state, create_time`
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Message for response to listing QuotaPreferences
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListQuotaPreferencesResponse {
    /// The list of QuotaPreference
    #[prost(message, repeated, tag = "1")]
    pub quota_preferences: ::prost::alloc::vec::Vec<QuotaPreference>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for getting a QuotaPreference
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetQuotaPreferenceRequest {
    /// Required. Name of the resource
    ///
    /// Example name:
    /// `projects/123/locations/global/quota_preferences/my-config-for-us-east1`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for creating a QuotaPreference
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateQuotaPreferenceRequest {
    /// Required. Value for parent.
    ///
    /// Example:
    /// `projects/123/locations/global`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Id of the requesting object, must be unique under its parent.
    /// If client does not set this field, the service will generate one.
    #[prost(string, tag = "2")]
    pub quota_preference_id: ::prost::alloc::string::String,
    /// Required. The resource being created
    #[prost(message, optional, tag = "3")]
    pub quota_preference: ::core::option::Option<QuotaPreference>,
    /// The list of quota safety checks to be ignored.
    #[prost(enumeration = "QuotaSafetyCheck", repeated, tag = "4")]
    pub ignore_safety_checks: ::prost::alloc::vec::Vec<i32>,
}
/// Message for updating a QuotaPreference
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateQuotaPreferenceRequest {
    /// Optional. Field mask is used to specify the fields to be overwritten in the
    /// QuotaPreference resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The resource being updated
    #[prost(message, optional, tag = "2")]
    pub quota_preference: ::core::option::Option<QuotaPreference>,
    /// Optional. If set to true, and the quota preference is not found, a new one
    /// will be created. In this situation, `update_mask` is ignored.
    #[prost(bool, tag = "3")]
    pub allow_missing: bool,
    /// Optional. If set to true, validate the request, but do not actually update.
    /// Note that a request being valid does not mean that the request is
    /// guaranteed to be fulfilled.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
    /// The list of quota safety checks to be ignored.
    #[prost(enumeration = "QuotaSafetyCheck", repeated, tag = "5")]
    pub ignore_safety_checks: ::prost::alloc::vec::Vec<i32>,
}
/// Generated client implementations.
pub mod cloud_quotas_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// The Cloud Quotas API is an infrastructure service for Google Cloud that lets
    /// service consumers list and manage their resource usage limits.
    ///
    /// - List/Get the metadata and current status of the quotas for a service.
    /// - Create/Update quota preferencess that declare the preferred quota values.
    /// - Check the status of a quota preference request.
    /// - List/Get pending and historical quota preference.
    #[derive(Debug, Clone)]
    pub struct CloudQuotasClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CloudQuotasClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> CloudQuotasClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> CloudQuotasClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            CloudQuotasClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Lists QuotaInfos of all quotas for a given project, folder or organization.
        pub async fn list_quota_infos(
            &mut self,
            request: impl tonic::IntoRequest<super::ListQuotaInfosRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListQuotaInfosResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.api.cloudquotas.v1.CloudQuotas/ListQuotaInfos",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.api.cloudquotas.v1.CloudQuotas",
                        "ListQuotaInfos",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieve the QuotaInfo of a quota for a project, folder or organization.
        pub async fn get_quota_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GetQuotaInfoRequest>,
        ) -> std::result::Result<tonic::Response<super::QuotaInfo>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.api.cloudquotas.v1.CloudQuotas/GetQuotaInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.api.cloudquotas.v1.CloudQuotas",
                        "GetQuotaInfo",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists QuotaPreferences in a given project, folder or organization.
        pub async fn list_quota_preferences(
            &mut self,
            request: impl tonic::IntoRequest<super::ListQuotaPreferencesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListQuotaPreferencesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.api.cloudquotas.v1.CloudQuotas/ListQuotaPreferences",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.api.cloudquotas.v1.CloudQuotas",
                        "ListQuotaPreferences",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single QuotaPreference.
        pub async fn get_quota_preference(
            &mut self,
            request: impl tonic::IntoRequest<super::GetQuotaPreferenceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QuotaPreference>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.api.cloudquotas.v1.CloudQuotas/GetQuotaPreference",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.api.cloudquotas.v1.CloudQuotas",
                        "GetQuotaPreference",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new QuotaPreference that declares the desired value for a quota.
        pub async fn create_quota_preference(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateQuotaPreferenceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QuotaPreference>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.api.cloudquotas.v1.CloudQuotas/CreateQuotaPreference",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.api.cloudquotas.v1.CloudQuotas",
                        "CreateQuotaPreference",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the parameters of a single QuotaPreference. It can updates the
        /// config in any states, not just the ones pending approval.
        pub async fn update_quota_preference(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateQuotaPreferenceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QuotaPreference>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.api.cloudquotas.v1.CloudQuotas/UpdateQuotaPreference",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.api.cloudquotas.v1.CloudQuotas",
                        "UpdateQuotaPreference",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
