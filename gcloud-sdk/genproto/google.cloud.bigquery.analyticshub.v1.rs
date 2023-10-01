/// A data exchange is a container that lets you share data. Along with the
/// descriptive information about the data exchange, it contains listings that
/// reference shared datasets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataExchange {
    /// Output only. The resource name of the data exchange.
    /// e.g. `projects/myproject/locations/US/dataExchanges/123`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Human-readable display name of the data exchange. The display
    /// name must contain only Unicode letters, numbers (0-9), underscores (_),
    /// dashes (-), spaces ( ), ampersands (&) and must not start or end with
    /// spaces. Default value is an empty string. Max length: 63 bytes.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. Description of the data exchange. The description must not
    /// contain Unicode non-characters as well as C0 and C1 control codes except
    /// tabs (HT), new lines (LF), carriage returns (CR), and page breaks (FF).
    /// Default value is an empty string.
    /// Max length: 2000 bytes.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Optional. Email or URL of the primary point of contact of the data
    /// exchange. Max Length: 1000 bytes.
    #[prost(string, tag = "4")]
    pub primary_contact: ::prost::alloc::string::String,
    /// Optional. Documentation describing the data exchange.
    #[prost(string, tag = "5")]
    pub documentation: ::prost::alloc::string::String,
    /// Output only. Number of listings contained in the data exchange.
    #[prost(int32, tag = "6")]
    pub listing_count: i32,
    /// Optional. Base64 encoded image representing the data exchange. Max
    /// Size: 3.0MiB Expected image dimensions are 512x512 pixels, however the API
    /// only performs validation on size of the encoded data. Note: For byte
    /// fields, the content of the fields are base64-encoded (which increases the
    /// size of the data by 33-36%) when using JSON on the wire.
    #[prost(bytes = "vec", tag = "7")]
    pub icon: ::prost::alloc::vec::Vec<u8>,
    /// Optional. Configurable data sharing environment option for a data exchange.
    #[prost(message, optional, tag = "8")]
    pub sharing_environment_config: ::core::option::Option<SharingEnvironmentConfig>,
}
/// Sharing environment is a behavior model for sharing data within a
/// data exchange. This option is configurable for a data exchange.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SharingEnvironmentConfig {
    #[prost(oneof = "sharing_environment_config::Environment", tags = "1, 2")]
    pub environment: ::core::option::Option<sharing_environment_config::Environment>,
}
/// Nested message and enum types in `SharingEnvironmentConfig`.
pub mod sharing_environment_config {
    /// Default Analytics Hub data exchange, used for secured data sharing.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DefaultExchangeConfig {}
    /// Data Clean Room (DCR), used for privacy-safe and secured data sharing.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DcrExchangeConfig {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Environment {
        /// Default Analytics Hub data exchange, used for secured data sharing.
        #[prost(message, tag = "1")]
        DefaultExchangeConfig(DefaultExchangeConfig),
        /// Data Clean Room (DCR), used for privacy-safe and secured data sharing.
        #[prost(message, tag = "2")]
        DcrExchangeConfig(DcrExchangeConfig),
    }
}
/// Contains details of the data provider.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataProvider {
    /// Optional. Name of the data provider.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Email or URL of the data provider.
    /// Max Length: 1000 bytes.
    #[prost(string, tag = "2")]
    pub primary_contact: ::prost::alloc::string::String,
}
/// Contains details of the listing publisher.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Publisher {
    /// Optional. Name of the listing publisher.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Email or URL of the listing publisher.
    /// Max Length: 1000 bytes.
    #[prost(string, tag = "2")]
    pub primary_contact: ::prost::alloc::string::String,
}
/// Contains the reference that identifies a destination bigquery dataset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestinationDatasetReference {
    /// Required. A unique ID for this dataset, without the project name. The ID
    /// must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_).
    /// The maximum length is 1,024 characters.
    #[prost(string, tag = "1")]
    pub dataset_id: ::prost::alloc::string::String,
    /// Required. The ID of the project containing this dataset.
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
}
/// Defines the destination bigquery dataset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestinationDataset {
    /// Required. A reference that identifies the destination dataset.
    #[prost(message, optional, tag = "1")]
    pub dataset_reference: ::core::option::Option<DestinationDatasetReference>,
    /// Optional. A descriptive name for the dataset.
    #[prost(message, optional, tag = "2")]
    pub friendly_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional. A user-friendly description of the dataset.
    #[prost(message, optional, tag = "3")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional. The labels associated with this dataset. You can use these
    /// to organize and group your datasets.
    /// You can set this property when inserting or updating a dataset.
    /// See <https://cloud.google.com/resource-manager/docs/creating-managing-labels>
    /// for more information.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Required. The geographic location where the dataset should reside. See
    /// <https://cloud.google.com/bigquery/docs/locations> for supported
    /// locations.
    #[prost(string, tag = "5")]
    pub location: ::prost::alloc::string::String,
}
/// A listing is what gets published into a data exchange that a subscriber can
/// subscribe to. It contains a reference to the data source along with
/// descriptive information that will help subscribers find and subscribe the
/// data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Listing {
    /// Output only. The resource name of the listing.
    /// e.g. `projects/myproject/locations/US/dataExchanges/123/listings/456`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Human-readable display name of the listing. The display name must
    /// contain only Unicode letters, numbers (0-9), underscores (_), dashes (-),
    /// spaces ( ), ampersands (&) and can't start or end with spaces. Default
    /// value is an empty string. Max length: 63 bytes.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. Short description of the listing. The description must not
    /// contain Unicode non-characters and C0 and C1 control codes except tabs
    /// (HT), new lines (LF), carriage returns (CR), and page breaks (FF). Default
    /// value is an empty string. Max length: 2000 bytes.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Optional. Email or URL of the primary point of contact of the listing.
    /// Max Length: 1000 bytes.
    #[prost(string, tag = "4")]
    pub primary_contact: ::prost::alloc::string::String,
    /// Optional. Documentation describing the listing.
    #[prost(string, tag = "5")]
    pub documentation: ::prost::alloc::string::String,
    /// Output only. Current state of the listing.
    #[prost(enumeration = "listing::State", tag = "7")]
    pub state: i32,
    /// Optional. Base64 encoded image representing the listing. Max Size: 3.0MiB
    /// Expected image dimensions are 512x512 pixels, however the API only
    /// performs validation on size of the encoded data.
    /// Note: For byte fields, the contents of the field are base64-encoded (which
    /// increases the size of the data by 33-36%) when using JSON on the wire.
    #[prost(bytes = "vec", tag = "8")]
    pub icon: ::prost::alloc::vec::Vec<u8>,
    /// Optional. Details of the data provider who owns the source data.
    #[prost(message, optional, tag = "9")]
    pub data_provider: ::core::option::Option<DataProvider>,
    /// Optional. Categories of the listing. Up to two categories are allowed.
    #[prost(enumeration = "listing::Category", repeated, packed = "false", tag = "10")]
    pub categories: ::prost::alloc::vec::Vec<i32>,
    /// Optional. Details of the publisher who owns the listing and who can share
    /// the source data.
    #[prost(message, optional, tag = "11")]
    pub publisher: ::core::option::Option<Publisher>,
    /// Optional. Email or URL of the request access of the listing.
    /// Subscribers can use this reference to request access.
    /// Max Length: 1000 bytes.
    #[prost(string, tag = "12")]
    pub request_access: ::prost::alloc::string::String,
    /// Optional. If set, restricted export configuration will be propagated and
    /// enforced on the linked dataset.
    #[prost(message, optional, tag = "13")]
    pub restricted_export_config: ::core::option::Option<
        listing::RestrictedExportConfig,
    >,
    /// Listing source.
    #[prost(oneof = "listing::Source", tags = "6")]
    pub source: ::core::option::Option<listing::Source>,
}
/// Nested message and enum types in `Listing`.
pub mod listing {
    /// A reference to a shared dataset. It is an existing BigQuery dataset with a
    /// collection of objects such as tables and views that you want to share
    /// with subscribers.
    /// When subscriber's subscribe to a listing, Analytics Hub creates a linked
    /// dataset in
    /// the subscriber's project. A Linked dataset is an opaque, read-only BigQuery
    /// dataset that serves as a _symbolic link_ to a shared dataset.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BigQueryDatasetSource {
        /// Resource name of the dataset source for this listing.
        /// e.g. `projects/myproject/datasets/123`
        #[prost(string, tag = "1")]
        pub dataset: ::prost::alloc::string::String,
    }
    /// Restricted export config, used to configure restricted export on linked
    /// dataset.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RestrictedExportConfig {
        /// Optional. If true, enable restricted export.
        #[prost(bool, tag = "3")]
        pub enabled: bool,
        /// Output only. If true, restrict direct table access(read
        /// api/tabledata.list) on linked table.
        #[prost(bool, tag = "1")]
        pub restrict_direct_table_access: bool,
        /// Optional. If true, restrict export of query result derived from
        /// restricted linked dataset table.
        #[prost(bool, tag = "2")]
        pub restrict_query_result: bool,
    }
    /// State of the listing.
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
    pub enum State {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// Subscribable state. Users with dataexchange.listings.subscribe permission
        /// can subscribe to this listing.
        Active = 1,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Active => "ACTIVE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ACTIVE" => Some(Self::Active),
                _ => None,
            }
        }
    }
    /// Listing categories.
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
    pub enum Category {
        Unspecified = 0,
        Others = 1,
        AdvertisingAndMarketing = 2,
        Commerce = 3,
        ClimateAndEnvironment = 4,
        Demographics = 5,
        Economics = 6,
        Education = 7,
        Energy = 8,
        Financial = 9,
        Gaming = 10,
        Geospatial = 11,
        HealthcareAndLifeScience = 12,
        Media = 13,
        PublicSector = 14,
        Retail = 15,
        Sports = 16,
        ScienceAndResearch = 17,
        TransportationAndLogistics = 18,
        TravelAndTourism = 19,
    }
    impl Category {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Category::Unspecified => "CATEGORY_UNSPECIFIED",
                Category::Others => "CATEGORY_OTHERS",
                Category::AdvertisingAndMarketing => "CATEGORY_ADVERTISING_AND_MARKETING",
                Category::Commerce => "CATEGORY_COMMERCE",
                Category::ClimateAndEnvironment => "CATEGORY_CLIMATE_AND_ENVIRONMENT",
                Category::Demographics => "CATEGORY_DEMOGRAPHICS",
                Category::Economics => "CATEGORY_ECONOMICS",
                Category::Education => "CATEGORY_EDUCATION",
                Category::Energy => "CATEGORY_ENERGY",
                Category::Financial => "CATEGORY_FINANCIAL",
                Category::Gaming => "CATEGORY_GAMING",
                Category::Geospatial => "CATEGORY_GEOSPATIAL",
                Category::HealthcareAndLifeScience => {
                    "CATEGORY_HEALTHCARE_AND_LIFE_SCIENCE"
                }
                Category::Media => "CATEGORY_MEDIA",
                Category::PublicSector => "CATEGORY_PUBLIC_SECTOR",
                Category::Retail => "CATEGORY_RETAIL",
                Category::Sports => "CATEGORY_SPORTS",
                Category::ScienceAndResearch => "CATEGORY_SCIENCE_AND_RESEARCH",
                Category::TransportationAndLogistics => {
                    "CATEGORY_TRANSPORTATION_AND_LOGISTICS"
                }
                Category::TravelAndTourism => "CATEGORY_TRAVEL_AND_TOURISM",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CATEGORY_UNSPECIFIED" => Some(Self::Unspecified),
                "CATEGORY_OTHERS" => Some(Self::Others),
                "CATEGORY_ADVERTISING_AND_MARKETING" => {
                    Some(Self::AdvertisingAndMarketing)
                }
                "CATEGORY_COMMERCE" => Some(Self::Commerce),
                "CATEGORY_CLIMATE_AND_ENVIRONMENT" => Some(Self::ClimateAndEnvironment),
                "CATEGORY_DEMOGRAPHICS" => Some(Self::Demographics),
                "CATEGORY_ECONOMICS" => Some(Self::Economics),
                "CATEGORY_EDUCATION" => Some(Self::Education),
                "CATEGORY_ENERGY" => Some(Self::Energy),
                "CATEGORY_FINANCIAL" => Some(Self::Financial),
                "CATEGORY_GAMING" => Some(Self::Gaming),
                "CATEGORY_GEOSPATIAL" => Some(Self::Geospatial),
                "CATEGORY_HEALTHCARE_AND_LIFE_SCIENCE" => {
                    Some(Self::HealthcareAndLifeScience)
                }
                "CATEGORY_MEDIA" => Some(Self::Media),
                "CATEGORY_PUBLIC_SECTOR" => Some(Self::PublicSector),
                "CATEGORY_RETAIL" => Some(Self::Retail),
                "CATEGORY_SPORTS" => Some(Self::Sports),
                "CATEGORY_SCIENCE_AND_RESEARCH" => Some(Self::ScienceAndResearch),
                "CATEGORY_TRANSPORTATION_AND_LOGISTICS" => {
                    Some(Self::TransportationAndLogistics)
                }
                "CATEGORY_TRAVEL_AND_TOURISM" => Some(Self::TravelAndTourism),
                _ => None,
            }
        }
    }
    /// Listing source.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// Required. Shared dataset i.e. BigQuery dataset source.
        #[prost(message, tag = "6")]
        BigqueryDataset(BigQueryDatasetSource),
    }
}
/// A subscription represents a subscribers' access to a particular set of
/// published data. It contains references to associated listings,
/// data exchanges, and linked datasets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Subscription {
    /// Output only. The resource name of the subscription.
    /// e.g. `projects/myproject/locations/US/subscriptions/123`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Timestamp when the subscription was created.
    #[prost(message, optional, tag = "2")]
    pub creation_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Timestamp when the subscription was last modified.
    #[prost(message, optional, tag = "3")]
    pub last_modify_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Organization of the project this subscription belongs to.
    #[prost(string, tag = "4")]
    pub organization_id: ::prost::alloc::string::String,
    /// Output only. Display name of the project of this subscription.
    #[prost(string, tag = "10")]
    pub organization_display_name: ::prost::alloc::string::String,
    /// Output only. Current state of the subscription.
    #[prost(enumeration = "subscription::State", tag = "7")]
    pub state: i32,
    /// Output only. Map of listing resource names to associated linked resource,
    /// e.g. projects/123/locations/US/dataExchanges/456/listings/789
    /// ->
    /// projects/123/datasets/my_dataset
    ///
    /// For listing-level subscriptions, this is a map of size 1.
    /// Only contains values if state == STATE_ACTIVE.
    #[prost(map = "string, message", tag = "8")]
    pub linked_dataset_map: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        subscription::LinkedResource,
    >,
    /// Output only. Email of the subscriber.
    #[prost(string, tag = "9")]
    pub subscriber_contact: ::prost::alloc::string::String,
    #[prost(oneof = "subscription::ResourceName", tags = "5, 6")]
    pub resource_name: ::core::option::Option<subscription::ResourceName>,
}
/// Nested message and enum types in `Subscription`.
pub mod subscription {
    /// Reference to a linked resource tracked by this Subscription.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LinkedResource {
        #[prost(oneof = "linked_resource::Reference", tags = "1")]
        pub reference: ::core::option::Option<linked_resource::Reference>,
    }
    /// Nested message and enum types in `LinkedResource`.
    pub mod linked_resource {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Reference {
            /// Output only. Name of the linked dataset, e.g.
            /// projects/subscriberproject/datasets/linked_dataset
            #[prost(string, tag = "1")]
            LinkedDataset(::prost::alloc::string::String),
        }
    }
    /// State of the subscription.
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
    pub enum State {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// This subscription is active and the data is accessible.
        Active = 1,
        /// The data referenced by this subscription is out of date and should be
        /// refreshed. This can happen when a data provider adds or removes datasets.
        Stale = 2,
        /// This subscription has been cancelled or revoked and the data is no longer
        /// accessible.
        Inactive = 3,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Active => "STATE_ACTIVE",
                State::Stale => "STATE_STALE",
                State::Inactive => "STATE_INACTIVE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "STATE_ACTIVE" => Some(Self::Active),
                "STATE_STALE" => Some(Self::Stale),
                "STATE_INACTIVE" => Some(Self::Inactive),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ResourceName {
        /// Output only. Resource name of the source Listing.
        /// e.g. projects/123/locations/US/dataExchanges/456/listings/789
        #[prost(string, tag = "5")]
        Listing(::prost::alloc::string::String),
        /// Output only. Resource name of the source Data Exchange.
        /// e.g. projects/123/locations/US/dataExchanges/456
        #[prost(string, tag = "6")]
        DataExchange(::prost::alloc::string::String),
    }
}
/// Message for requesting the list of data exchanges.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDataExchangesRequest {
    /// Required. The parent resource path of the data exchanges.
    /// e.g. `projects/myproject/locations/US`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of results to return in a single response page. Leverage
    /// the page tokens to iterate through the entire collection.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Page token, returned by a previous call, to request the next page of
    /// results.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Message for response to the list of data exchanges.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDataExchangesResponse {
    /// The list of data exchanges.
    #[prost(message, repeated, tag = "1")]
    pub data_exchanges: ::prost::alloc::vec::Vec<DataExchange>,
    /// A token to request the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Message for requesting the list of data exchanges from projects in an
/// organization and location.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrgDataExchangesRequest {
    /// Required. The organization resource path of the projects containing
    /// DataExchanges. e.g. `organizations/myorg/locations/US`.
    #[prost(string, tag = "1")]
    pub organization: ::prost::alloc::string::String,
    /// The maximum number of results to return in a single response page. Leverage
    /// the page tokens to iterate through the entire collection.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Page token, returned by a previous call, to request the next page of
    /// results.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Message for response to listing data exchanges in an organization and
/// location.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrgDataExchangesResponse {
    /// The list of data exchanges.
    #[prost(message, repeated, tag = "1")]
    pub data_exchanges: ::prost::alloc::vec::Vec<DataExchange>,
    /// A token to request the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Message for getting a data exchange.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataExchangeRequest {
    /// Required. The resource name of the data exchange.
    /// e.g. `projects/myproject/locations/US/dataExchanges/123`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for creating a data exchange.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDataExchangeRequest {
    /// Required. The parent resource path of the data exchange.
    /// e.g. `projects/myproject/locations/US`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID of the data exchange.
    /// Must contain only Unicode letters, numbers (0-9), underscores (_).
    /// Should not use characters that require URL-escaping, or characters
    /// outside of ASCII, spaces.
    /// Max length: 100 bytes.
    #[prost(string, tag = "2")]
    pub data_exchange_id: ::prost::alloc::string::String,
    /// Required. The data exchange to create.
    #[prost(message, optional, tag = "3")]
    pub data_exchange: ::core::option::Option<DataExchange>,
}
/// Message for updating a data exchange.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDataExchangeRequest {
    /// Required. Field mask specifies the fields to update in the data exchange
    /// resource. The fields specified in the
    /// `updateMask` are relative to the resource and are not a full request.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The data exchange to update.
    #[prost(message, optional, tag = "2")]
    pub data_exchange: ::core::option::Option<DataExchange>,
}
/// Message for deleting a data exchange.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDataExchangeRequest {
    /// Required. The full name of the data exchange resource that you want to
    /// delete. For example, `projects/myproject/locations/US/dataExchanges/123`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for requesting the list of listings.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListListingsRequest {
    /// Required. The parent resource path of the listing.
    /// e.g. `projects/myproject/locations/US/dataExchanges/123`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of results to return in a single response page. Leverage
    /// the page tokens to iterate through the entire collection.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Page token, returned by a previous call, to request the next page of
    /// results.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Message for response to the list of Listings.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListListingsResponse {
    /// The list of Listing.
    #[prost(message, repeated, tag = "1")]
    pub listings: ::prost::alloc::vec::Vec<Listing>,
    /// A token to request the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Message for getting a listing.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetListingRequest {
    /// Required. The resource name of the listing.
    /// e.g. `projects/myproject/locations/US/dataExchanges/123/listings/456`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for creating a listing.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateListingRequest {
    /// Required. The parent resource path of the listing.
    /// e.g. `projects/myproject/locations/US/dataExchanges/123`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID of the listing to create.
    /// Must contain only Unicode letters, numbers (0-9), underscores (_).
    /// Should not use characters that require URL-escaping, or characters
    /// outside of ASCII, spaces.
    /// Max length: 100 bytes.
    #[prost(string, tag = "2")]
    pub listing_id: ::prost::alloc::string::String,
    /// Required. The listing to create.
    #[prost(message, optional, tag = "3")]
    pub listing: ::core::option::Option<Listing>,
}
/// Message for updating a Listing.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateListingRequest {
    /// Required. Field mask specifies the fields to update in the listing
    /// resource. The fields specified in the `updateMask` are relative to the
    /// resource and are not a full request.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The listing to update.
    #[prost(message, optional, tag = "2")]
    pub listing: ::core::option::Option<Listing>,
}
/// Message for deleting a listing.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteListingRequest {
    /// Required. Resource name of the listing to delete.
    /// e.g. `projects/myproject/locations/US/dataExchanges/123/listings/456`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for subscribing to a listing.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeListingRequest {
    /// Required. Resource name of the listing that you want to subscribe to.
    /// e.g. `projects/myproject/locations/US/dataExchanges/123/listings/456`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Resulting destination of the listing that you subscribed to.
    #[prost(oneof = "subscribe_listing_request::Destination", tags = "3")]
    pub destination: ::core::option::Option<subscribe_listing_request::Destination>,
}
/// Nested message and enum types in `SubscribeListingRequest`.
pub mod subscribe_listing_request {
    /// Resulting destination of the listing that you subscribed to.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// BigQuery destination dataset to create for the subscriber.
        #[prost(message, tag = "3")]
        DestinationDataset(super::DestinationDataset),
    }
}
/// Message for response when you subscribe to a listing.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeListingResponse {
    /// Subscription object created from this subscribe action.
    #[prost(message, optional, tag = "1")]
    pub subscription: ::core::option::Option<Subscription>,
}
/// Message for subscribing to a Data Exchange.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeDataExchangeRequest {
    /// Required. Resource name of the Data Exchange.
    /// e.g. `projects/publisherproject/locations/US/dataExchanges/123`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The parent resource path of the Subscription.
    /// e.g. `projects/subscriberproject/locations/US`
    #[prost(string, tag = "2")]
    pub destination: ::prost::alloc::string::String,
    /// Required. Name of the subscription to create.
    /// e.g. `subscription1`
    #[prost(string, tag = "4")]
    pub subscription: ::prost::alloc::string::String,
    /// Email of the subscriber.
    #[prost(string, tag = "3")]
    pub subscriber_contact: ::prost::alloc::string::String,
}
/// Message for response when you subscribe to a Data Exchange.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeDataExchangeResponse {
    /// Subscription object created from this subscribe action.
    #[prost(message, optional, tag = "1")]
    pub subscription: ::core::option::Option<Subscription>,
}
/// Message for refreshing a subscription.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshSubscriptionRequest {
    /// Required. Resource name of the Subscription to refresh.
    /// e.g. `projects/subscriberproject/locations/US/subscriptions/123`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for response when you refresh a subscription.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshSubscriptionResponse {
    /// The refreshed subscription resource.
    #[prost(message, optional, tag = "1")]
    pub subscription: ::core::option::Option<Subscription>,
}
/// Message for getting a subscription.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSubscriptionRequest {
    /// Required. Resource name of the subscription.
    /// e.g. projects/123/locations/US/subscriptions/456
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for listing subscriptions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSubscriptionsRequest {
    /// Required. The parent resource path of the subscription.
    /// e.g. projects/myproject/locations/US
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The filter expression may be used to filter by Data Exchange or Listing.
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// The maximum number of results to return in a single response page.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Page token, returned by a previous call.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// Message for response to the listing of subscriptions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSubscriptionsResponse {
    /// The list of subscriptions.
    #[prost(message, repeated, tag = "1")]
    pub subscriptions: ::prost::alloc::vec::Vec<Subscription>,
    /// Next page token.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Message for listing subscriptions of a shared resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSharedResourceSubscriptionsRequest {
    /// Required. Resource name of the requested target. This resource may be
    /// either a Listing or a DataExchange. e.g.
    /// projects/123/locations/US/dataExchanges/456 OR e.g.
    /// projects/123/locations/US/dataExchanges/456/listings/789
    #[prost(string, tag = "1")]
    pub resource: ::prost::alloc::string::String,
    /// If selected, includes deleted subscriptions in the response
    /// (up to 63 days after deletion).
    #[prost(bool, tag = "2")]
    pub include_deleted_subscriptions: bool,
    /// The maximum number of results to return in a single response page.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Page token, returned by a previous call.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// Message for response to the listing of shared resource subscriptions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSharedResourceSubscriptionsResponse {
    /// The list of subscriptions.
    #[prost(message, repeated, tag = "1")]
    pub shared_resource_subscriptions: ::prost::alloc::vec::Vec<Subscription>,
    /// Next page token.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Message for revoking a subscription.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevokeSubscriptionRequest {
    /// Required. Resource name of the subscription to revoke.
    /// e.g. projects/123/locations/US/subscriptions/456
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for response when you revoke a subscription.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevokeSubscriptionResponse {}
/// Message for deleting a subscription.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSubscriptionRequest {
    /// Required. Resource name of the subscription to delete.
    /// e.g. projects/123/locations/US/subscriptions/456
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Represents the metadata of a long-running operation in Analytics Hub.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// Output only. The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Server-defined resource path for the target of the operation.
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// Output only. Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: ::prost::alloc::string::String,
    /// Output only. Human-readable status of the operation, if any.
    #[prost(string, tag = "5")]
    pub status_message: ::prost::alloc::string::String,
    /// Output only. Identifies whether the user has requested cancellation
    /// of the operation. Operations that have successfully been cancelled
    /// have [Operation.error][] value with a
    /// [google.rpc.Status.code][google.rpc.Status.code] of 1, corresponding to
    /// `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod analytics_hub_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// The `AnalyticsHubService` API facilitates data sharing within and across
    /// organizations. It allows data providers to publish listings that reference
    /// shared datasets. With Analytics Hub, users can discover and search for
    /// listings that they have access to. Subscribers can view and subscribe to
    /// listings. When you subscribe to a listing, Analytics Hub creates a linked
    /// dataset in your project.
    #[derive(Debug, Clone)]
    pub struct AnalyticsHubServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AnalyticsHubServiceClient<tonic::transport::Channel> {
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
    impl<T> AnalyticsHubServiceClient<T>
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
        ) -> AnalyticsHubServiceClient<InterceptedService<T, F>>
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
            AnalyticsHubServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists all data exchanges in a given project and location.
        pub async fn list_data_exchanges(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDataExchangesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDataExchangesResponse>,
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
                "/google.cloud.bigquery.analyticshub.v1.AnalyticsHubService/ListDataExchanges",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.analyticshub.v1.AnalyticsHubService",
                        "ListDataExchanges",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all data exchanges from projects in a given organization and
        /// location.
        pub async fn list_org_data_exchanges(
            &mut self,
            request: impl tonic::IntoRequest<super::ListOrgDataExchangesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListOrgDataExchangesResponse>,
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
                "/google.cloud.bigquery.analyticshub.v1.AnalyticsHubService/ListOrgDataExchanges",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.analyticshub.v1.AnalyticsHubService",
                        "ListOrgDataExchanges",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the details of a data exchange.
        pub async fn get_data_exchange(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDataExchangeRequest>,
        ) -> std::result::Result<tonic::Response<super::DataExchange>, tonic::Status> {
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
                "/google.cloud.bigquery.analyticshub.v1.AnalyticsHubService/GetDataExchange",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.analyticshub.v1.AnalyticsHubService",
                        "GetDataExchange",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new data exchange.
        pub async fn create_data_exchange(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDataExchangeRequest>,
        ) -> std::result::Result<tonic::Response<super::DataExchange>, tonic::Status> {
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
                "/google.cloud.bigquery.analyticshub.v1.AnalyticsHubService/CreateDataExchange",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.analyticshub.v1.AnalyticsHubService",
                        "CreateDataExchange",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates an existing data exchange.
        pub async fn update_data_exchange(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDataExchangeRequest>,
        ) -> std::result::Result<tonic::Response<super::DataExchange>, tonic::Status> {
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
                "/google.cloud.bigquery.analyticshub.v1.AnalyticsHubService/UpdateDataExchange",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.analyticshub.v1.AnalyticsHubService",
                        "UpdateDataExchange",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes an existing data exchange.
        pub async fn delete_data_exchange(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDataExchangeRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.bigquery.analyticshub.v1.AnalyticsHubService/DeleteDataExchange",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.analyticshub.v1.AnalyticsHubService",
                        "DeleteDataExchange",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all listings in a given project and location.
        pub async fn list_listings(
            &mut self,
            request: impl tonic::IntoRequest<super::ListListingsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListListingsResponse>,
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
                "/google.cloud.bigquery.analyticshub.v1.AnalyticsHubService/ListListings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.analyticshub.v1.AnalyticsHubService",
                        "ListListings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the details of a listing.
        pub async fn get_listing(
            &mut self,
            request: impl tonic::IntoRequest<super::GetListingRequest>,
        ) -> std::result::Result<tonic::Response<super::Listing>, tonic::Status> {
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
                "/google.cloud.bigquery.analyticshub.v1.AnalyticsHubService/GetListing",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.analyticshub.v1.AnalyticsHubService",
                        "GetListing",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new listing.
        pub async fn create_listing(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateListingRequest>,
        ) -> std::result::Result<tonic::Response<super::Listing>, tonic::Status> {
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
                "/google.cloud.bigquery.analyticshub.v1.AnalyticsHubService/CreateListing",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.analyticshub.v1.AnalyticsHubService",
                        "CreateListing",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates an existing listing.
        pub async fn update_listing(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateListingRequest>,
        ) -> std::result::Result<tonic::Response<super::Listing>, tonic::Status> {
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
                "/google.cloud.bigquery.analyticshub.v1.AnalyticsHubService/UpdateListing",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.analyticshub.v1.AnalyticsHubService",
                        "UpdateListing",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a listing.
        pub async fn delete_listing(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteListingRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.bigquery.analyticshub.v1.AnalyticsHubService/DeleteListing",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.analyticshub.v1.AnalyticsHubService",
                        "DeleteListing",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Subscribes to a listing.
        ///
        /// Currently, with Analytics Hub, you can create listings that
        /// reference only BigQuery datasets.
        /// Upon subscription to a listing for a BigQuery dataset, Analytics Hub
        /// creates a linked dataset in the subscriber's project.
        pub async fn subscribe_listing(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeListingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SubscribeListingResponse>,
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
                "/google.cloud.bigquery.analyticshub.v1.AnalyticsHubService/SubscribeListing",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.analyticshub.v1.AnalyticsHubService",
                        "SubscribeListing",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a Subscription to a Data Exchange. This is a long-running operation
        /// as it will create one or more linked datasets.
        pub async fn subscribe_data_exchange(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeDataExchangeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.bigquery.analyticshub.v1.AnalyticsHubService/SubscribeDataExchange",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.analyticshub.v1.AnalyticsHubService",
                        "SubscribeDataExchange",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Refreshes a Subscription to a Data Exchange. A Data Exchange can become
        /// stale when a publisher adds or removes data. This is a long-running
        /// operation as it may create many linked datasets.
        pub async fn refresh_subscription(
            &mut self,
            request: impl tonic::IntoRequest<super::RefreshSubscriptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.bigquery.analyticshub.v1.AnalyticsHubService/RefreshSubscription",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.analyticshub.v1.AnalyticsHubService",
                        "RefreshSubscription",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the details of a Subscription.
        pub async fn get_subscription(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSubscriptionRequest>,
        ) -> std::result::Result<tonic::Response<super::Subscription>, tonic::Status> {
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
                "/google.cloud.bigquery.analyticshub.v1.AnalyticsHubService/GetSubscription",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.analyticshub.v1.AnalyticsHubService",
                        "GetSubscription",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all subscriptions in a given project and location.
        pub async fn list_subscriptions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSubscriptionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListSubscriptionsResponse>,
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
                "/google.cloud.bigquery.analyticshub.v1.AnalyticsHubService/ListSubscriptions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.analyticshub.v1.AnalyticsHubService",
                        "ListSubscriptions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all subscriptions on a given Data Exchange or Listing.
        pub async fn list_shared_resource_subscriptions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ListSharedResourceSubscriptionsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ListSharedResourceSubscriptionsResponse>,
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
                "/google.cloud.bigquery.analyticshub.v1.AnalyticsHubService/ListSharedResourceSubscriptions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.analyticshub.v1.AnalyticsHubService",
                        "ListSharedResourceSubscriptions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Revokes a given subscription.
        pub async fn revoke_subscription(
            &mut self,
            request: impl tonic::IntoRequest<super::RevokeSubscriptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RevokeSubscriptionResponse>,
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
                "/google.cloud.bigquery.analyticshub.v1.AnalyticsHubService/RevokeSubscription",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.analyticshub.v1.AnalyticsHubService",
                        "RevokeSubscription",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a subscription.
        pub async fn delete_subscription(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSubscriptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.bigquery.analyticshub.v1.AnalyticsHubService/DeleteSubscription",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.analyticshub.v1.AnalyticsHubService",
                        "DeleteSubscription",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets the IAM policy.
        pub async fn get_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::iam::v1::GetIamPolicyRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::iam::v1::Policy>,
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
                "/google.cloud.bigquery.analyticshub.v1.AnalyticsHubService/GetIamPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.analyticshub.v1.AnalyticsHubService",
                        "GetIamPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Sets the IAM policy.
        pub async fn set_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::iam::v1::SetIamPolicyRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::super::iam::v1::Policy>,
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
                "/google.cloud.bigquery.analyticshub.v1.AnalyticsHubService/SetIamPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.analyticshub.v1.AnalyticsHubService",
                        "SetIamPolicy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the permissions that a caller has.
        pub async fn test_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::super::super::iam::v1::TestIamPermissionsResponse,
            >,
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
                "/google.cloud.bigquery.analyticshub.v1.AnalyticsHubService/TestIamPermissions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.bigquery.analyticshub.v1.AnalyticsHubService",
                        "TestIamPermissions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
