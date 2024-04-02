/// Represents a set of declarations about what (if any) ad partners
/// are associated with a given creative. This can be set at the network level,
/// as a default for all creatives, or overridden for a particular creative.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdPartnerDeclaration {
    /// They type of declaration.
    #[prost(enumeration = "declaration_type_enum::DeclarationType", tag = "1")]
    pub r#type: i32,
    /// The resource names of AdPartners being declared.
    /// Format: "networks/{network_code}/adPartners/{ad_partner_id}"
    #[prost(string, repeated, tag = "2")]
    pub ad_partners: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Wrapper message for
/// [DeclarationTypeEnum][google.ads.admanager.v1.DeclarationTypeEnum].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeclarationTypeEnum {}
/// Nested message and enum types in `DeclarationTypeEnum`.
pub mod declaration_type_enum {
    /// The declaration about third party data usage on the associated entity.
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
    pub enum DeclarationType {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// No ad technology providers to declare.
        None = 1,
        /// There are are ad technology providers to declare on this entity.
        Declared = 2,
    }
    impl DeclarationType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DeclarationType::Unspecified => "DECLARATION_TYPE_UNSPECIFIED",
                DeclarationType::None => "NONE",
                DeclarationType::Declared => "DECLARED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DECLARATION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "NONE" => Some(Self::None),
                "DECLARED" => Some(Self::Declared),
                _ => None,
            }
        }
    }
}
/// The AdPartner resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdPartner {
    /// Identifier. The resource name of the AdPartner.
    /// Format: `networks/{network_code}/adPartners/{ad_partner_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request object for GetAdPartner method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAdPartnerRequest {
    /// Required. The resource name of the AdPartner.
    /// Format: `networks/{network_code}/adPartners/{ad_partner_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request object for ListAdPartners method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAdPartnersRequest {
    /// Required. The parent, which owns this collection of AdPartners.
    /// Format: `networks/{network_code}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of AdPartners to return. The service may
    /// return fewer than this value. If unspecified, at most 50 AdPartners will be
    /// returned. The maximum value is 1000; values above 1000 will be coerced to
    /// 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListAdPartners` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListAdPartners` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Expression to filter the response.
    /// See syntax details at
    /// <https://developers.google.com/ad-manager/api/beta/filters>
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Expression to specify sorting order.
    /// See syntax details at
    /// <https://developers.google.com/ad-manager/api/beta/filters#order>
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
    /// Optional. Number of individual resources to skip while paginating.
    #[prost(int32, tag = "6")]
    pub skip: i32,
}
/// Response object for ListAdPartnersRequest containing matching AdPartner
/// resources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAdPartnersResponse {
    /// The AdPartner from the specified network.
    #[prost(message, repeated, tag = "1")]
    pub ad_partners: ::prost::alloc::vec::Vec<AdPartner>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Total number of AdPartners.
    /// If a filter was included in the request, this reflects the total number
    /// after the filtering is applied.
    ///
    /// `total_size` will not be calculated in the response unless it has been
    /// included in a response field mask. The response field mask can be provided
    /// to the method by using the URL parameter `$fields` or `fields`, or by using
    /// the HTTP/gRPC header `X-Goog-FieldMask`.
    ///
    /// For more information, see
    /// <https://developers.google.com/ad-manager/api/beta/field-masks>
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Generated client implementations.
pub mod ad_partner_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Provides methods for handling AdPartner objects.
    #[derive(Debug, Clone)]
    pub struct AdPartnerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AdPartnerServiceClient<tonic::transport::Channel> {
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
    impl<T> AdPartnerServiceClient<T>
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
        ) -> AdPartnerServiceClient<InterceptedService<T, F>>
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
            AdPartnerServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// API to retrieve a AdPartner object.
        pub async fn get_ad_partner(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAdPartnerRequest>,
        ) -> std::result::Result<tonic::Response<super::AdPartner>, tonic::Status> {
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
                "/google.ads.admanager.v1.AdPartnerService/GetAdPartner",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ads.admanager.v1.AdPartnerService",
                        "GetAdPartner",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// API to retrieve a list of AdPartner objects.
        pub async fn list_ad_partners(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAdPartnersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAdPartnersResponse>,
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
                "/google.ads.admanager.v1.AdPartnerService/ListAdPartners",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ads.admanager.v1.AdPartnerService",
                        "ListAdPartners",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Wrapper message for
/// [AppliedAdsenseEnabled][google.ads.admanager.v1.AppliedAdsenseEnabledEnum.AppliedAdsenseEnabled]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppliedAdsenseEnabledEnum {}
/// Nested message and enum types in `AppliedAdsenseEnabledEnum`.
pub mod applied_adsense_enabled_enum {
    /// Specifies if serving ads from the AdSense content network is enabled.
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
    pub enum AppliedAdsenseEnabled {
        /// No adsense enabled setting applied directly; value will be inherited from
        /// parent or system default.
        Unspecified = 0,
        /// Serving ads from AdSense content network is enabled.
        True = 1,
        /// Serving ads from AdSense content network is disabled.
        False = 2,
    }
    impl AppliedAdsenseEnabled {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AppliedAdsenseEnabled::Unspecified => {
                    "APPLIED_ADSENSE_ENABLED_UNSPECIFIED"
                }
                AppliedAdsenseEnabled::True => "TRUE",
                AppliedAdsenseEnabled::False => "FALSE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "APPLIED_ADSENSE_ENABLED_UNSPECIFIED" => Some(Self::Unspecified),
                "TRUE" => Some(Self::True),
                "FALSE" => Some(Self::False),
                _ => None,
            }
        }
    }
}
/// Wrapper message for
/// [EnvironmentType][google.ads.admanager.v1.EnvironmentTypeEnum.EnvironmentType].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnvironmentTypeEnum {}
/// Nested message and enum types in `EnvironmentTypeEnum`.
pub mod environment_type_enum {
    /// The different environments in which an ad can be shown.
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
    pub enum EnvironmentType {
        /// No value specified
        Unspecified = 0,
        /// A regular web browser.
        Browser = 1,
        /// Video players.
        VideoPlayer = 2,
    }
    impl EnvironmentType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EnvironmentType::Unspecified => "ENVIRONMENT_TYPE_UNSPECIFIED",
                EnvironmentType::Browser => "BROWSER",
                EnvironmentType::VideoPlayer => "VIDEO_PLAYER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ENVIRONMENT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "BROWSER" => Some(Self::Browser),
                "VIDEO_PLAYER" => Some(Self::VideoPlayer),
                _ => None,
            }
        }
    }
}
/// Represents the dimensions of an AdUnit, LineItem, or Creative.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Size {
    /// Required. The width of the [Creative](google.ads.admanager.v1.Creative),
    /// [AdUnit](google.ads.admanager.v1.AdUnit), or
    /// [LineItem](google.ads.admanager.v1.LineItem).
    #[prost(int32, tag = "1")]
    pub width: i32,
    /// Required. The height of the [Creative](google.ads.admanager.v1.Creative),
    /// [AdUnit](google.ads.admanager.v1.AdUnit), or
    /// [LineItem](google.ads.admanager.v1.LineItem).
    #[prost(int32, tag = "2")]
    pub height: i32,
    /// Required. The SizeType of the
    /// [Creative](google.ads.admanager.v1.Creative),
    /// [AdUnit](google.ads.admanager.v1.AdUnit), or
    /// [LineItem](google.ads.admanager.v1.LineItem).
    #[prost(enumeration = "size_type_enum::SizeType", tag = "3")]
    pub size_type: i32,
}
/// Wrapper message for
/// [SizeType][google.ads.admanager.v1.SizeTypeEnum.SizeType].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SizeTypeEnum {}
/// Nested message and enum types in `SizeTypeEnum`.
pub mod size_type_enum {
    /// The different Size types for an ad.
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
    pub enum SizeType {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// Dimension based size, an actual height and width in pixels.
        Pixel = 1,
        /// Size is expressed as a ratio. For example, 4:1 could be
        /// met by a 100 x 25 sized image.
        AspectRatio = 2,
        /// Out-of-page (Interstitial) size that is not related to the slot it is
        /// served. This must be used with 1x1 size.
        Interstitial = 3,
        /// Size is ignored. This must be used with 1x1
        /// size.
        Ignored = 4,
        /// Native size, which is a function of the how the client renders the
        /// creative. This must be used with 1x1 size.
        Native = 5,
        /// Fluid size. Automatically sizes the ad by filling the width of the
        /// enclosing column and adjusting the height as appropriate. This must be
        /// used with 1x1 size.
        Fluid = 6,
        /// Audio size. Used with audio ads. This must be used with 1x1 size.
        Audio = 7,
    }
    impl SizeType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SizeType::Unspecified => "SIZE_TYPE_UNSPECIFIED",
                SizeType::Pixel => "PIXEL",
                SizeType::AspectRatio => "ASPECT_RATIO",
                SizeType::Interstitial => "INTERSTITIAL",
                SizeType::Ignored => "IGNORED",
                SizeType::Native => "NATIVE",
                SizeType::Fluid => "FLUID",
                SizeType::Audio => "AUDIO",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SIZE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "PIXEL" => Some(Self::Pixel),
                "ASPECT_RATIO" => Some(Self::AspectRatio),
                "INTERSTITIAL" => Some(Self::Interstitial),
                "IGNORED" => Some(Self::Ignored),
                "NATIVE" => Some(Self::Native),
                "FLUID" => Some(Self::Fluid),
                "AUDIO" => Some(Self::Audio),
                _ => None,
            }
        }
    }
}
/// Represents the size, environment, and companions of an ad in an ad unit.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdUnitSize {
    /// Required. The Size of the AdUnit.
    #[prost(message, optional, tag = "1")]
    pub size: ::core::option::Option<Size>,
    /// Required. The EnvironmentType of the AdUnit
    #[prost(enumeration = "environment_type_enum::EnvironmentType", tag = "2")]
    pub environment_type: i32,
    /// The companions for this ad unit size. Companions are only valid if the
    /// environment is
    /// [VIDEO_PLAYER][google.ads.admanager.v1.EnvironmentTypeEnum.EnvironmentType].
    #[prost(message, repeated, tag = "3")]
    pub companions: ::prost::alloc::vec::Vec<Size>,
}
/// Represents a Label that can be applied to an entity.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppliedLabel {
    /// Required. The label to be applied.
    /// Format: "networks/{network_code}/labels/{label_id}"
    #[prost(string, tag = "1")]
    pub label: ::prost::alloc::string::String,
    /// Specifies whether or not to negate the effects of the label.
    #[prost(bool, tag = "2")]
    pub negated: bool,
}
/// Represents a Frequency Cap that can be applied to an entity.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FrequencyCap {
    /// The maximum number of impressions for this frequency cap.
    #[prost(int64, optional, tag = "1")]
    pub max_impressions: ::core::option::Option<i64>,
    /// The number of time units over which the frequency cap is effective.
    #[prost(int64, optional, tag = "2")]
    pub time_amount: ::core::option::Option<i64>,
    /// The units of time of this frequency cap.
    #[prost(enumeration = "time_unit_enum::TimeUnit", optional, tag = "3")]
    pub time_unit: ::core::option::Option<i32>,
}
/// Wrapper message for TimeUnit.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeUnitEnum {}
/// Nested message and enum types in `TimeUnitEnum`.
pub mod time_unit_enum {
    /// Unit of time for the frequency cap.
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
    pub enum TimeUnit {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// Minute
        Minute = 1,
        /// Hour
        Hour = 2,
        /// Day
        Day = 3,
        /// Week
        Week = 4,
        /// Month
        Month = 5,
        /// Lifetime
        Lifetime = 6,
        /// Per pod of ads in a video stream. Only valid for entities in a
        /// VIDEO_PLAYER environment.
        Pod = 7,
        /// Per video stream. Only valid for entities in a VIDEO_PLAYER environment.
        Stream = 8,
    }
    impl TimeUnit {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TimeUnit::Unspecified => "TIME_UNIT_UNSPECIFIED",
                TimeUnit::Minute => "MINUTE",
                TimeUnit::Hour => "HOUR",
                TimeUnit::Day => "DAY",
                TimeUnit::Week => "WEEK",
                TimeUnit::Month => "MONTH",
                TimeUnit::Lifetime => "LIFETIME",
                TimeUnit::Pod => "POD",
                TimeUnit::Stream => "STREAM",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TIME_UNIT_UNSPECIFIED" => Some(Self::Unspecified),
                "MINUTE" => Some(Self::Minute),
                "HOUR" => Some(Self::Hour),
                "DAY" => Some(Self::Day),
                "WEEK" => Some(Self::Week),
                "MONTH" => Some(Self::Month),
                "LIFETIME" => Some(Self::Lifetime),
                "POD" => Some(Self::Pod),
                "STREAM" => Some(Self::Stream),
                _ => None,
            }
        }
    }
}
/// The AdUnit resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdUnit {
    /// Identifier. The resource name of the AdUnit.
    /// Format: `networks/{network_code}/adUnits/{ad_unit_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. AdUnit ID.
    #[prost(int64, tag = "15")]
    pub ad_unit_id: i64,
    /// Required. Immutable. The AdUnit's parent. Every ad unit has a parent except
    /// for the root ad unit, which is created by Google. Format:
    /// "networks/{network_code}/adUnits/{ad_unit_id}"
    #[prost(string, tag = "10")]
    pub parent_ad_unit: ::prost::alloc::string::String,
    /// Output only. The path to this AdUnit in the ad unit hierarchy represented
    /// as a list from the root to this ad unit's parent. For root ad units, this
    /// list is empty.
    #[prost(message, repeated, tag = "11")]
    pub parent_path: ::prost::alloc::vec::Vec<AdUnitParent>,
    /// Required. The display name of the ad unit. Its maximum length is 255
    /// characters.
    #[prost(string, tag = "9")]
    pub display_name: ::prost::alloc::string::String,
    /// Immutable. A string used to uniquely identify the ad unit for the purposes
    /// of serving the ad. This attribute is optional and can be set during ad unit
    /// creation. If it is not provided, it will be assigned by Google based off of
    /// the ad unit ID.
    #[prost(string, tag = "2")]
    pub ad_unit_code: ::prost::alloc::string::String,
    /// Output only. The status of this ad unit.  It defaults to ACTIVE.
    #[prost(enumeration = "ad_unit::Status", tag = "13")]
    pub status: i32,
    /// Non-empty default. The value to use for the HTML link's target attribute.
    /// This value will be interpreted as TOP if left blank.
    #[prost(enumeration = "target_window_enum::TargetWindow", tag = "12")]
    pub target_window: i32,
    /// Optional. The resource names of Teams directly applied to this AdUnit.
    /// Format: "networks/{network_code}/teams/{team_id}"
    #[prost(string, repeated, tag = "3")]
    pub applied_teams: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. The resource names of all Teams that this AdUnit is on as well
    /// as those inherited from parent AdUnits. Format:
    /// "networks/{network_code}/teams/{team_id}"
    #[prost(string, repeated, tag = "4")]
    pub teams: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. A description of the ad unit. The maximum length is 65,535
    /// characters.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    /// Optional. If this field is set to true, then the AdUnit will not be
    /// implicitly targeted when its parent is. Traffickers must explicitly
    /// target such an AdUnit or else no line items will serve to it. This
    /// feature is only available for Ad Manager 360 accounts.
    #[prost(bool, tag = "6")]
    pub explicitly_targeted: bool,
    /// Output only. This field is set to true if the ad unit has any children.
    #[prost(bool, tag = "7")]
    pub has_children: bool,
    /// Output only. The instant this AdUnit was last modified.
    #[prost(message, optional, tag = "8")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. The sizes that can be served inside this ad unit.
    #[prost(message, repeated, tag = "14")]
    pub ad_unit_sizes: ::prost::alloc::vec::Vec<AdUnitSize>,
    /// Optional. Determines what set top box video on demand channel this ad unit
    /// corresponds to in an external set top box ad campaign system.
    #[prost(string, tag = "17")]
    pub external_set_top_box_channel_id: ::prost::alloc::string::String,
    /// Optional. The duration after which an Ad Unit will automatically refresh.
    /// This is only valid for ad units in mobile apps. If not set, the ad unit
    /// will not refresh.
    #[prost(message, optional, tag = "19")]
    pub refresh_delay: ::core::option::Option<::prost_types::Duration>,
    /// Optional. The ID of the CTV application that this ad unit is within.
    #[prost(int64, tag = "20")]
    pub ctv_application_id: i64,
    /// Optional. The set of labels applied directly to this ad unit.
    #[prost(message, repeated, tag = "21")]
    pub applied_labels: ::prost::alloc::vec::Vec<AppliedLabel>,
    /// Output only. Contains the set of labels applied directly to the ad unit as
    /// well as those inherited from the parent ad units. If a label has been
    /// negated, only the negated label is returned. This field is readonly and is
    /// assigned by Google.
    #[prost(message, repeated, tag = "22")]
    pub effective_applied_labels: ::prost::alloc::vec::Vec<AppliedLabel>,
    /// Optional. The set of label frequency caps applied directly to this ad unit.
    /// There is a limit of 10 label frequency caps per ad unit.
    #[prost(message, repeated, tag = "23")]
    pub applied_label_frequency_caps: ::prost::alloc::vec::Vec<LabelFrequencyCap>,
    /// Output only. The label frequency caps applied directly to the ad unit as
    /// well as those inherited from parent ad units.
    #[prost(message, repeated, tag = "24")]
    pub effective_label_frequency_caps: ::prost::alloc::vec::Vec<LabelFrequencyCap>,
    /// Optional. The smart size mode for this ad unit. This attribute is optional
    /// and defaults to SmartSizeMode.NONE for fixed sizes.
    #[prost(enumeration = "smart_size_mode_enum::SmartSizeMode", tag = "25")]
    pub smart_size_mode: i32,
    /// Optional. The value of AdSense enabled directly applied to this ad unit.
    /// This attribute is optional and if not specified this ad unit will inherit
    /// the value of effectiveAdsenseEnabled from its ancestors.
    #[prost(
        enumeration = "applied_adsense_enabled_enum::AppliedAdsenseEnabled",
        tag = "26"
    )]
    pub applied_adsense_enabled: i32,
    /// Output only. Specifies whether or not the AdUnit is enabled for serving ads
    /// from the AdSense content network. This attribute defaults to the ad unit's
    /// parent or ancestor's setting if one has been set. If no ancestor of the ad
    /// unit has set appliedAdsenseEnabled, the attribute is defaulted to true.
    #[prost(bool, tag = "27")]
    pub effective_adsense_enabled: bool,
}
/// Nested message and enum types in `AdUnit`.
pub mod ad_unit {
    /// The status of an AdUnit.
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
    pub enum Status {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// The ad unit is active, available for targeting, and serving.
        Active = 1,
        /// The ad unit will be visible in the UI, but ignored by serving.
        Inactive = 2,
        /// The ad unit will be hidden in the UI and ignored by serving.
        Archived = 3,
    }
    impl Status {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Status::Unspecified => "STATUS_UNSPECIFIED",
                Status::Active => "ACTIVE",
                Status::Inactive => "INACTIVE",
                Status::Archived => "ARCHIVED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATUS_UNSPECIFIED" => Some(Self::Unspecified),
                "ACTIVE" => Some(Self::Active),
                "INACTIVE" => Some(Self::Inactive),
                "ARCHIVED" => Some(Self::Archived),
                _ => None,
            }
        }
    }
}
/// The summary of a parent AdUnit.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdUnitParent {
    /// Output only. The parent of the current AdUnit
    /// Format: `networks/{network_code}/adUnits/{ad_unit_id}`
    #[prost(string, tag = "1")]
    pub parent_ad_unit: ::prost::alloc::string::String,
    /// Output only. The display name of the parent AdUnit.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. A string used to uniquely identify the ad unit for the
    /// purposes of serving the ad.
    #[prost(string, tag = "3")]
    pub ad_unit_code: ::prost::alloc::string::String,
}
/// Wrapper message for
/// [TargetWindow][google.ads.admanager.v1.TargetWindowEnum.TargetWindow].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetWindowEnum {}
/// Nested message and enum types in `TargetWindowEnum`.
pub mod target_window_enum {
    /// Corresponds to an HTML link's target attribute.
    /// See <http://www.w3.org/TR/html401/present/frames.html#adef-target>
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
    pub enum TargetWindow {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// Specifies that the link should open in the full body of the page.
        Top = 1,
        /// Specifies that the link should open in a new window.
        Blank = 2,
    }
    impl TargetWindow {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TargetWindow::Unspecified => "TARGET_WINDOW_UNSPECIFIED",
                TargetWindow::Top => "TOP",
                TargetWindow::Blank => "BLANK",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TARGET_WINDOW_UNSPECIFIED" => Some(Self::Unspecified),
                "TOP" => Some(Self::Top),
                "BLANK" => Some(Self::Blank),
                _ => None,
            }
        }
    }
}
/// Frequency cap using a label.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelFrequencyCap {
    /// The label to used for frequency capping.
    /// Format: "networks/{network_code}/labels/{label_id}"
    #[prost(string, tag = "1")]
    pub label: ::prost::alloc::string::String,
    /// The frequency cap.
    #[prost(message, optional, tag = "2")]
    pub frequency_cap: ::core::option::Option<FrequencyCap>,
}
/// Wrapper message for
/// [SmartSizeMode][google.ads.admanager.v1.SmartSizeModeEnum.SmartSizeMode].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SmartSizeModeEnum {}
/// Nested message and enum types in `SmartSizeModeEnum`.
pub mod smart_size_mode_enum {
    /// The smart size mode for this ad unit. This attribute is optional and
    /// defaults to SmartSizeMode.NONE for fixed sizes.
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
    pub enum SmartSizeMode {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// Fixed size mode (default).
        None = 1,
        /// The height is fixed for the request, the width is a range.
        SmartBanner = 2,
        /// Height and width are ranges.
        DynamicSize = 3,
    }
    impl SmartSizeMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SmartSizeMode::Unspecified => "SMART_SIZE_MODE_UNSPECIFIED",
                SmartSizeMode::None => "NONE",
                SmartSizeMode::SmartBanner => "SMART_BANNER",
                SmartSizeMode::DynamicSize => "DYNAMIC_SIZE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SMART_SIZE_MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "NONE" => Some(Self::None),
                "SMART_BANNER" => Some(Self::SmartBanner),
                "DYNAMIC_SIZE" => Some(Self::DynamicSize),
                _ => None,
            }
        }
    }
}
/// Request object for GetAdUnit method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAdUnitRequest {
    /// Required. The resource name of the AdUnit.
    /// Format: `networks/{network_code}/adUnits/{ad_unit_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request object for ListAdUnits method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAdUnitsRequest {
    /// Required. The parent, which owns this collection of AdUnits.
    /// Format: `networks/{network_code}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of AdUnits to return. The service may return
    /// fewer than this value. If unspecified, at most 50 ad units will be
    /// returned. The maximum value is 1000; values above 1000 will be coerced to
    /// 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListAdUnits` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListAdUnits` must match
    /// the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Expression to filter the response.
    /// See syntax details at
    /// <https://developers.google.com/ad-manager/api/beta/filters>
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Expression to specify sorting order.
    /// See syntax details at
    /// <https://developers.google.com/ad-manager/api/beta/filters#order>
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
    /// Optional. Number of individual resources to skip while paginating.
    #[prost(int32, tag = "6")]
    pub skip: i32,
}
/// Response object for ListAdUnitsRequest containing matching AdUnit resources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAdUnitsResponse {
    /// The AdUnit from the specified network.
    #[prost(message, repeated, tag = "1")]
    pub ad_units: ::prost::alloc::vec::Vec<AdUnit>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Total number of AdUnits.
    /// If a filter was included in the request, this reflects the total number
    /// after the filtering is applied.
    ///
    /// `total_size` will not be calculated in the response unless it has been
    /// included in a response field mask. The response field mask can be provided
    /// to the method by using the URL parameter `$fields` or `fields`, or by using
    /// the HTTP/gRPC header `X-Goog-FieldMask`.
    ///
    /// For more information, see
    /// <https://developers.google.com/ad-manager/api/beta/field-masks>
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Generated client implementations.
pub mod ad_unit_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Provides methods for handling AdUnit objects.
    #[derive(Debug, Clone)]
    pub struct AdUnitServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AdUnitServiceClient<tonic::transport::Channel> {
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
    impl<T> AdUnitServiceClient<T>
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
        ) -> AdUnitServiceClient<InterceptedService<T, F>>
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
            AdUnitServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// API to retrieve an AdUnit object.
        pub async fn get_ad_unit(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAdUnitRequest>,
        ) -> std::result::Result<tonic::Response<super::AdUnit>, tonic::Status> {
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
                "/google.ads.admanager.v1.AdUnitService/GetAdUnit",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.ads.admanager.v1.AdUnitService", "GetAdUnit"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// API to retrieve a list of AdUnit objects.
        pub async fn list_ad_units(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAdUnitsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAdUnitsResponse>,
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
                "/google.ads.admanager.v1.AdUnitService/ListAdUnits",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ads.admanager.v1.AdUnitService",
                        "ListAdUnits",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// / AdManagerError contains all the information required for processing a
/// / particular error thrown by the AdManager API.
/// /
/// / At least one AdManagerError should be included in all error messages sent
/// to / the client.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdManagerError {
    /// The unique identifying string for this error.
    #[prost(string, tag = "1")]
    pub error_code: ::prost::alloc::string::String,
    /// A publisher appropriate explanation of this error.
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    /// The field path that triggered this error.
    #[prost(string, tag = "3")]
    pub field_path: ::prost::alloc::string::String,
    /// The value that triggered this error.
    #[prost(string, tag = "4")]
    pub trigger: ::prost::alloc::string::String,
    /// The stack trace that accompanies this error.
    #[prost(string, tag = "5")]
    pub stack_trace: ::prost::alloc::string::String,
    /// A list of messages that carry any additional error details.
    #[prost(message, repeated, tag = "6")]
    pub details: ::prost::alloc::vec::Vec<::prost_types::Any>,
}
/// Wrapper message for
/// [CompanyCreditStatus][google.ads.admanager.v1.CompanyCreditStatusEnum.CompanyCreditStatus]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompanyCreditStatusEnum {}
/// Nested message and enum types in `CompanyCreditStatusEnum`.
pub mod company_credit_status_enum {
    /// The credit status of a company.
    ///
    /// Credit statuses specify the credit-worthiness of a company and affect the
    /// ad serving of campaigns belonging to the company.
    ///
    /// In basic settings, only the
    /// [ACTIVE][google.ads.admanager.v1.CompanyCreditStatusEnum.CompanyCreditStatus.ACTIVE]
    /// and
    /// [INACTIVE][google.ads.admanager.v1.CompanyCreditStatusEnum.CompanyCreditStatus.INACTIVE]
    /// credit statuses are applicable. In advance settings, all credit statuses
    /// are applicable.
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
    pub enum CompanyCreditStatus {
        /// No value specified
        Unspecified = 0,
        /// The company's credit status is active.
        ///
        /// Line items belonging to the company can serve.
        ///
        /// This credit status is the default for basic settings and is available in
        /// both basic and advance settings.
        Active = 1,
        /// The company's credit status is inactive.
        ///
        /// Line items belonging to the company cannot be activated. However, line
        /// items that were activated before the credit status changed will remain
        /// active. New orders or line items belonging to the company cannot be
        /// created.
        ///
        /// Companies with this credit status will be hidden by default in company
        /// search results.
        ///
        /// This credit status is available in both basic and advance settings.
        Inactive = 2,
        /// The company's credit status is on hold.
        ///
        /// Line items belonging to the company cannot be activated. However, line
        /// items that were activated before the credit status changed will remain
        /// active. New orders or line items belonging to the company can be
        /// created.
        ///
        /// This credit status is the default in advance settings and is only
        /// available in advance settings.
        OnHold = 3,
        /// The company's credit status is stopped.
        ///
        /// Line items belonging to the company cannot be activated. However, line
        /// items that were activated before the credit status changed will remain
        /// active. New orders or line items belonging to the company cannot be
        /// created.
        ///
        /// This credit status is only available in advance settings.
        Stop = 4,
        /// The company's credit status is blocked.
        ///
        /// All active line items belonging to the company will stop serving with
        /// immediate effect. Line items belonging to the company cannot be
        /// activated, and new orders or line items belonging to the company cannot
        /// be created.
        ///
        /// This credit status is only available in advance settings.
        Blocked = 5,
    }
    impl CompanyCreditStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CompanyCreditStatus::Unspecified => "COMPANY_CREDIT_STATUS_UNSPECIFIED",
                CompanyCreditStatus::Active => "ACTIVE",
                CompanyCreditStatus::Inactive => "INACTIVE",
                CompanyCreditStatus::OnHold => "ON_HOLD",
                CompanyCreditStatus::Stop => "STOP",
                CompanyCreditStatus::Blocked => "BLOCKED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "COMPANY_CREDIT_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
                "ACTIVE" => Some(Self::Active),
                "INACTIVE" => Some(Self::Inactive),
                "ON_HOLD" => Some(Self::OnHold),
                "STOP" => Some(Self::Stop),
                "BLOCKED" => Some(Self::Blocked),
                _ => None,
            }
        }
    }
}
/// Wrapper message for
/// [CompanyType][google.ads.admanager.v1.CompanyTypeEnum.CompanyType]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompanyTypeEnum {}
/// Nested message and enum types in `CompanyTypeEnum`.
pub mod company_type_enum {
    /// The type of a company.
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
    pub enum CompanyType {
        /// No value specified
        Unspecified = 0,
        /// A business entity that purchases ad inventory.
        Advertiser = 1,
        /// A company representing the publisher's own advertiser for house ads.
        HouseAdvertiser = 2,
        /// An organization that manages ad accounts and offers services, such as ad
        /// creation, placement, and management to advertisers.
        Agency = 3,
        /// A company representing the publisher's own agency.
        HouseAgency = 4,
        /// A company representing multiple advertisers and agencies.
        AdNetwork = 5,
        /// A third-party that measures creative viewability.
        ViewabilityProvider = 6,
    }
    impl CompanyType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CompanyType::Unspecified => "COMPANY_TYPE_UNSPECIFIED",
                CompanyType::Advertiser => "ADVERTISER",
                CompanyType::HouseAdvertiser => "HOUSE_ADVERTISER",
                CompanyType::Agency => "AGENCY",
                CompanyType::HouseAgency => "HOUSE_AGENCY",
                CompanyType::AdNetwork => "AD_NETWORK",
                CompanyType::ViewabilityProvider => "VIEWABILITY_PROVIDER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "COMPANY_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "ADVERTISER" => Some(Self::Advertiser),
                "HOUSE_ADVERTISER" => Some(Self::HouseAdvertiser),
                "AGENCY" => Some(Self::Agency),
                "HOUSE_AGENCY" => Some(Self::HouseAgency),
                "AD_NETWORK" => Some(Self::AdNetwork),
                "VIEWABILITY_PROVIDER" => Some(Self::ViewabilityProvider),
                _ => None,
            }
        }
    }
}
/// The `Company` resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Company {
    /// Identifier. The resource name of the `Company`.
    /// Format: `networks/{network_code}/companies/{company_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. `Company` ID.
    #[prost(int64, tag = "2")]
    pub company_id: i64,
    /// Required. The display name of the `Company`.
    ///
    /// This value has a maximum length of 127 characters.
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
    /// Required. The type of the `Company`.
    #[prost(enumeration = "company_type_enum::CompanyType", tag = "4")]
    pub r#type: i32,
    /// Optional. The address for the `Company`.
    ///
    /// This value has a maximum length of 1024 characters.
    #[prost(string, tag = "5")]
    pub address: ::prost::alloc::string::String,
    /// Optional. The email for the `Company`.
    ///
    /// This value has a maximum length of 128 characters.
    #[prost(string, tag = "6")]
    pub email: ::prost::alloc::string::String,
    /// Optional. The fax number for the `Company`.
    ///
    /// This value has a maximum length of 63 characters.
    #[prost(string, tag = "7")]
    pub fax: ::prost::alloc::string::String,
    /// Optional. The phone number for the `Company`.
    ///
    /// This value has a maximum length of 63 characters.
    #[prost(string, tag = "8")]
    pub phone: ::prost::alloc::string::String,
    /// Optional. The external ID for the `Company`.
    ///
    /// This value has a maximum length of 255 characters.
    #[prost(string, tag = "9")]
    pub external_id: ::prost::alloc::string::String,
    /// Optional. Comments about the `Company`.
    ///
    /// This value has a maximum length of 1024 characters.
    #[prost(string, tag = "10")]
    pub comment: ::prost::alloc::string::String,
    /// Optional. The credit status of this company.
    ///
    /// This attribute defaults to `ACTIVE` if basic settings are enabled and
    /// `ON_HOLD` if advance settings are enabled.
    #[prost(enumeration = "company_credit_status_enum::CompanyCreditStatus", tag = "11")]
    pub credit_status: i32,
    /// Optional. The labels that are directly applied to this company.
    #[prost(message, repeated, tag = "12")]
    pub applied_labels: ::prost::alloc::vec::Vec<AppliedLabel>,
    /// Optional. The resource names of primary Contact of this company.
    /// Format: "networks/{network_code}/contacts/{contact_id}"
    #[prost(string, optional, tag = "13")]
    pub primary_contact: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional. The resource names of Teams that are directly associated with
    /// this company. Format: "networks/{network_code}/teams/{team_id}"
    #[prost(string, repeated, tag = "14")]
    pub applied_teams: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request object for `GetCompany` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCompanyRequest {
    /// Required. The resource name of the Company.
    /// Format: `networks/{network_code}/companies/{company_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request object for `ListCompanies` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCompaniesRequest {
    /// Required. The parent, which owns this collection of Companies.
    /// Format: `networks/{network_code}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of `Companies` to return. The service may
    /// return fewer than this value. If unspecified, at most 50 `Companies` will
    /// be returned. The maximum value is 1000; values above 1000 will be coerced
    /// to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListCompanies` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListCompanies` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Expression to filter the response.
    /// See syntax details at
    /// <https://developers.google.com/ad-manager/api/beta/filters>
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Expression to specify sorting order.
    /// See syntax details at
    /// <https://developers.google.com/ad-manager/api/beta/filters#order>
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
    /// Optional. Number of individual resources to skip while paginating.
    #[prost(int32, tag = "6")]
    pub skip: i32,
}
/// Response object for `ListCompaniesRequest` containing matching `Company`
/// resources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCompaniesResponse {
    /// The `Company` from the specified network.
    #[prost(message, repeated, tag = "1")]
    pub companies: ::prost::alloc::vec::Vec<Company>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Total number of `Companies`.
    /// If a filter was included in the request, this reflects the total number
    /// after the filtering is applied.
    ///
    /// `total_size` will not be calculated in the response unless it has been
    /// included in a response field mask. The response field mask can be provided
    /// to the method by using the URL parameter `$fields` or `fields`, or by using
    /// the HTTP/gRPC header `X-Goog-FieldMask`.
    ///
    /// For more information, see
    /// <https://developers.google.com/ad-manager/api/beta/field-masks>
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Generated client implementations.
pub mod company_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Provides methods for handling `Company` objects.
    #[derive(Debug, Clone)]
    pub struct CompanyServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CompanyServiceClient<tonic::transport::Channel> {
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
    impl<T> CompanyServiceClient<T>
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
        ) -> CompanyServiceClient<InterceptedService<T, F>>
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
            CompanyServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// API to retrieve a `Company` object.
        pub async fn get_company(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCompanyRequest>,
        ) -> std::result::Result<tonic::Response<super::Company>, tonic::Status> {
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
                "/google.ads.admanager.v1.CompanyService/GetCompany",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ads.admanager.v1.CompanyService",
                        "GetCompany",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// API to retrieve a list of `Company` objects.
        pub async fn list_companies(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCompaniesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCompaniesResponse>,
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
                "/google.ads.admanager.v1.CompanyService/ListCompanies",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ads.admanager.v1.CompanyService",
                        "ListCompanies",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Wrapper message for
/// [ComputedStatus][google.ads.admanager.v1.ComputedStatusEnum.ComputedStatus].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputedStatusEnum {}
/// Nested message and enum types in `ComputedStatusEnum`.
pub mod computed_status_enum {
    /// Describes the computed LineItem status that is derived from the
    /// current state of the LineItem.
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
    pub enum ComputedStatus {
        /// No value specified.
        Unspecified = 0,
        /// The LineItem has past its link LineItem#endDateTime with an auto
        /// extension, but hasn't met its goal.
        DeliveryExtended = 1,
        /// The LineItem has begun serving.
        Delivering = 2,
        /// The LineItem has been activated and is ready to serve.
        Ready = 3,
        /// The LineItem has been paused from serving.
        Paused = 4,
        /// The LineItem is inactive. It is either caused by missing creatives or
        /// the network disabling auto-activation.
        Inactive = 5,
        /// The LineItem has been paused and its reserved inventory has been
        /// released. The LineItem will not serve.
        PausedInventoryReleased = 6,
        /// The LineItem has been submitted for approval.
        PendingApproval = 7,
        /// The LineItem has completed its run.
        Completed = 8,
        /// The LineItem has been disapproved and is not eligible to serve.
        Disapproved = 9,
        /// The LineItem is still being drafted.
        Draft = 10,
        /// The LineItem has been canceled and is no longer eligible to serve.
        /// This is a legacy status imported from Google Ad Manager orders.
        Canceled = 11,
    }
    impl ComputedStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ComputedStatus::Unspecified => "COMPUTED_STATUS_UNSPECIFIED",
                ComputedStatus::DeliveryExtended => "DELIVERY_EXTENDED",
                ComputedStatus::Delivering => "DELIVERING",
                ComputedStatus::Ready => "READY",
                ComputedStatus::Paused => "PAUSED",
                ComputedStatus::Inactive => "INACTIVE",
                ComputedStatus::PausedInventoryReleased => "PAUSED_INVENTORY_RELEASED",
                ComputedStatus::PendingApproval => "PENDING_APPROVAL",
                ComputedStatus::Completed => "COMPLETED",
                ComputedStatus::Disapproved => "DISAPPROVED",
                ComputedStatus::Draft => "DRAFT",
                ComputedStatus::Canceled => "CANCELED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "COMPUTED_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
                "DELIVERY_EXTENDED" => Some(Self::DeliveryExtended),
                "DELIVERING" => Some(Self::Delivering),
                "READY" => Some(Self::Ready),
                "PAUSED" => Some(Self::Paused),
                "INACTIVE" => Some(Self::Inactive),
                "PAUSED_INVENTORY_RELEASED" => Some(Self::PausedInventoryReleased),
                "PENDING_APPROVAL" => Some(Self::PendingApproval),
                "COMPLETED" => Some(Self::Completed),
                "DISAPPROVED" => Some(Self::Disapproved),
                "DRAFT" => Some(Self::Draft),
                "CANCELED" => Some(Self::Canceled),
                _ => None,
            }
        }
    }
}
/// The Contact resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contact {
    /// Identifier. The resource name of the Contact.
    /// Format: `networks/{network_code}/contacts/{contact_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. `Contact` ID.
    #[prost(int64, tag = "2")]
    pub contact_id: i64,
}
/// Request object for GetContact method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetContactRequest {
    /// Required. The resource name of the Contact.
    /// Format: `networks/{network_code}/contacts/{contact_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request object for ListContacts method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListContactsRequest {
    /// Required. The parent, which owns this collection of Contacts.
    /// Format: `networks/{network_code}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of Contacts to return. The service may return
    /// fewer than this value. If unspecified, at most 50 contacts will be
    /// returned. The maximum value is 1000; values above 1000 will be coerced to
    /// 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListContacts` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListContacts` must match
    /// the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Expression to filter the response.
    /// See syntax details at
    /// <https://developers.google.com/ad-manager/api/beta/filters>
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Expression to specify sorting order.
    /// See syntax details at
    /// <https://developers.google.com/ad-manager/api/beta/filters#order>
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
    /// Optional. Number of individual resources to skip while paginating.
    #[prost(int32, tag = "6")]
    pub skip: i32,
}
/// Response object for ListContactsRequest containing matching Contact
/// resources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListContactsResponse {
    /// The Contact from the specified network.
    #[prost(message, repeated, tag = "1")]
    pub contacts: ::prost::alloc::vec::Vec<Contact>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Total number of Contacts.
    /// If a filter was included in the request, this reflects the total number
    /// after the filtering is applied.
    ///
    /// `total_size` will not be calculated in the response unless it has been
    /// included in a response field mask. The response field mask can be provided
    /// to the method by using the URL parameter `$fields` or `fields`, or by using
    /// the HTTP/gRPC header `X-Goog-FieldMask`.
    ///
    /// For more information, see
    /// <https://developers.google.com/ad-manager/api/beta/field-masks>
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Generated client implementations.
pub mod contact_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Provides methods for handling Contact objects.
    #[derive(Debug, Clone)]
    pub struct ContactServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ContactServiceClient<tonic::transport::Channel> {
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
    impl<T> ContactServiceClient<T>
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
        ) -> ContactServiceClient<InterceptedService<T, F>>
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
            ContactServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// API to retrieve a Contact object.
        pub async fn get_contact(
            &mut self,
            request: impl tonic::IntoRequest<super::GetContactRequest>,
        ) -> std::result::Result<tonic::Response<super::Contact>, tonic::Status> {
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
                "/google.ads.admanager.v1.ContactService/GetContact",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ads.admanager.v1.ContactService",
                        "GetContact",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// API to retrieve a list of Contact objects.
        pub async fn list_contacts(
            &mut self,
            request: impl tonic::IntoRequest<super::ListContactsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListContactsResponse>,
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
                "/google.ads.admanager.v1.ContactService/ListContacts",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ads.admanager.v1.ContactService",
                        "ListContacts",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Describes a slot that a creative is expected to fill. This is used in
/// forecasting and to validate that the correct creatives are associated with
/// the line item.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreativePlaceholder {
    /// Required. The size that the creative is expected to have.
    #[prost(message, optional, tag = "1")]
    pub size: ::core::option::Option<Size>,
    /// The companions that the creative is expected to have. This attribute can
    /// only be set if the line item it belongs to has an
    /// [EnvironmentType][google.ads.admanager.v1.EnvironmentTypeEnum.EnvironmentType]
    /// of VIDEO_PLAYER or [roadblocking_type][LineItem.roadblocking_type] of
    /// CREATIVE_SET.
    #[prost(message, repeated, tag = "2")]
    pub companion_sizes: ::prost::alloc::vec::Vec<Size>,
    /// Expected number of creatives that will be uploaded corresponding to this
    /// creative placeholder.  This estimate is used to improve the accuracy of
    /// forecasting; for example, if label frequency capping limits the number of
    /// times a creative may be served.
    #[prost(int32, tag = "3")]
    pub expected_creative_count: i32,
    /// Set of labels applied directly to this CreativePlaceholder.
    #[prost(message, repeated, tag = "4")]
    pub applied_labels: ::prost::alloc::vec::Vec<AppliedLabel>,
    /// Indicates if the expected creative of this placeholder has an AMP only
    /// variant. This is used to improve the accuracy of forecasting and has no
    /// effect on serving.
    #[prost(bool, tag = "5")]
    pub amp_only: bool,
    /// The display name of the creative targeting that this CreativePlaceholder
    /// represents.
    #[prost(string, tag = "6")]
    pub creative_targeting_display_name: ::prost::alloc::string::String,
}
/// The Creative resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Creative {
    /// Identifier. The resource name of the Creative.
    /// Format: `networks/{network_code}/creatives/{creative_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. `Creative` ID.
    #[prost(int64, tag = "7")]
    pub creative_id: i64,
    /// Optional. Display name of the `Creative`. This attribute has a maximum
    /// length of 255 characters.
    #[prost(string, tag = "8")]
    pub display_name: ::prost::alloc::string::String,
    /// Required. The resource name of the Company, which is of type
    /// Company.Type.ADVERTISER, to which this Creative belongs. Format:
    /// "networks/{network_code}/companies/{company_id}"
    #[prost(string, tag = "2")]
    pub advertiser: ::prost::alloc::string::String,
    /// Output only. The instant this Creative was last modified.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The URL of the creative for previewing the media.
    #[prost(string, tag = "4")]
    pub preview_url: ::prost::alloc::string::String,
    /// Output only. String representations of creative size.
    /// This field is temporarily available and will be deprecated when
    /// `Creative.size` becomes available.
    #[prost(string, tag = "9")]
    pub size_label: ::prost::alloc::string::String,
    /// Optional. The Ad Partners associated with this creative.
    /// This is distinct from any associated companies that Google may detect
    /// programmatically.
    #[prost(message, optional, tag = "6")]
    pub ad_partner_declaration: ::core::option::Option<AdPartnerDeclaration>,
}
/// Request object for GetCreative method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCreativeRequest {
    /// Required. The resource name of the Creative.
    /// Format: `networks/{network_code}/creatives/{creative_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request object for ListCreatives method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCreativesRequest {
    /// Required. The parent, which owns this collection of Creatives.
    /// Format: networks/{network_code}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of Creatives to return. The service may return
    /// fewer than this value. If unspecified, at most 50 creatives will be
    /// returned. The maximum value is 1000; values above 1000 will be coerced to
    /// 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListCreatives` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListCreatives` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Expression to filter the response.
    /// See syntax details at
    /// <https://developers.google.com/ad-manager/api/beta/filters>
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Expression to specify sorting order.
    /// See syntax details at
    /// <https://developers.google.com/ad-manager/api/beta/filters#order>
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
    /// Optional. Number of individual resources to skip while paginating.
    #[prost(int32, tag = "6")]
    pub skip: i32,
}
/// Response object for ListCreativesRequest containing matching Creative
/// resources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCreativesResponse {
    /// The Creative from the specified network.
    #[prost(message, repeated, tag = "1")]
    pub creatives: ::prost::alloc::vec::Vec<Creative>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Total number of Creatives.
    /// If a filter was included in the request, this reflects the total number
    /// after the filtering is applied.
    ///
    /// `total_size` will not be calculated in the response unless it has been
    /// included in a response field mask. The response field mask can be provided
    /// to the method by using the URL parameter `$fields` or `fields`, or by using
    /// the HTTP/gRPC header `X-Goog-FieldMask`.
    ///
    /// For more information, see
    /// <https://developers.google.com/ad-manager/api/beta/field-masks>
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Generated client implementations.
pub mod creative_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Provides methods for handling Creative objects.
    #[derive(Debug, Clone)]
    pub struct CreativeServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CreativeServiceClient<tonic::transport::Channel> {
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
    impl<T> CreativeServiceClient<T>
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
        ) -> CreativeServiceClient<InterceptedService<T, F>>
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
            CreativeServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// API to retrieve a Creative object.
        pub async fn get_creative(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCreativeRequest>,
        ) -> std::result::Result<tonic::Response<super::Creative>, tonic::Status> {
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
                "/google.ads.admanager.v1.CreativeService/GetCreative",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ads.admanager.v1.CreativeService",
                        "GetCreative",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// API to retrieve a list of Creative objects.
        pub async fn list_creatives(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCreativesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCreativesResponse>,
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
                "/google.ads.admanager.v1.CreativeService/ListCreatives",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ads.admanager.v1.CreativeService",
                        "ListCreatives",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Wrapper message for
/// [CustomFieldDataType][google.ads.admanager.v1.CustomFieldDataTypeEnum.CustomFieldDataType]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomFieldDataTypeEnum {}
/// Nested message and enum types in `CustomFieldDataTypeEnum`.
pub mod custom_field_data_type_enum {
    /// The data type for a CustomField.
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
    pub enum CustomFieldDataType {
        /// No value specified
        Unspecified = 0,
        /// A string field
        ///
        /// The max length is 255 characters.
        String = 1,
        /// A number field.
        Number = 2,
        /// A "Yes" or "No" toggle field.
        Toggle = 3,
        /// A drop-down field.
        DropDown = 4,
    }
    impl CustomFieldDataType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CustomFieldDataType::Unspecified => "CUSTOM_FIELD_DATA_TYPE_UNSPECIFIED",
                CustomFieldDataType::String => "STRING",
                CustomFieldDataType::Number => "NUMBER",
                CustomFieldDataType::Toggle => "TOGGLE",
                CustomFieldDataType::DropDown => "DROP_DOWN",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CUSTOM_FIELD_DATA_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "STRING" => Some(Self::String),
                "NUMBER" => Some(Self::Number),
                "TOGGLE" => Some(Self::Toggle),
                "DROP_DOWN" => Some(Self::DropDown),
                _ => None,
            }
        }
    }
}
/// Wrapper message for
/// [CustomFieldEntityType][google.ads.admanager.v1.CustomFieldEntityTypeEnum.CustomFieldEntityType]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomFieldEntityTypeEnum {}
/// Nested message and enum types in `CustomFieldEntityTypeEnum`.
pub mod custom_field_entity_type_enum {
    /// The types of entities that a CustomField can be applied to.
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
    pub enum CustomFieldEntityType {
        /// No value specified
        Unspecified = 0,
        /// The CustomField is applied to LineItems.
        LineItem = 1,
        /// The CustomField is applied to Orders.
        Order = 2,
        /// The CustomField is applied to Creatives.
        Creative = 3,
        /// The CustomField is applied to Proposals.
        Proposal = 4,
        /// The CustomField is applied to ProposalLineItems.
        ProposalLineItem = 5,
    }
    impl CustomFieldEntityType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CustomFieldEntityType::Unspecified => {
                    "CUSTOM_FIELD_ENTITY_TYPE_UNSPECIFIED"
                }
                CustomFieldEntityType::LineItem => "LINE_ITEM",
                CustomFieldEntityType::Order => "ORDER",
                CustomFieldEntityType::Creative => "CREATIVE",
                CustomFieldEntityType::Proposal => "PROPOSAL",
                CustomFieldEntityType::ProposalLineItem => "PROPOSAL_LINE_ITEM",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CUSTOM_FIELD_ENTITY_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "LINE_ITEM" => Some(Self::LineItem),
                "ORDER" => Some(Self::Order),
                "CREATIVE" => Some(Self::Creative),
                "PROPOSAL" => Some(Self::Proposal),
                "PROPOSAL_LINE_ITEM" => Some(Self::ProposalLineItem),
                _ => None,
            }
        }
    }
}
/// Wrapper message for
/// [CustomFieldStatus][google.ads.admanager.v1.CustomFieldStatusEnum.CustomFieldStatus]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomFieldStatusEnum {}
/// Nested message and enum types in `CustomFieldStatusEnum`.
pub mod custom_field_status_enum {
    /// The status of the CustomField.
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
    pub enum CustomFieldStatus {
        /// No value specified
        Unspecified = 0,
        /// The CustomField is active.
        Active = 1,
        /// The CustomField is inactive.
        Inactive = 2,
    }
    impl CustomFieldStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CustomFieldStatus::Unspecified => "CUSTOM_FIELD_STATUS_UNSPECIFIED",
                CustomFieldStatus::Active => "ACTIVE",
                CustomFieldStatus::Inactive => "INACTIVE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CUSTOM_FIELD_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
                "ACTIVE" => Some(Self::Active),
                "INACTIVE" => Some(Self::Inactive),
                _ => None,
            }
        }
    }
}
/// Wrapper message for
/// [CustomFieldVisibility][google.ads.admanager.v1.CustomFieldVisibilityEnum.CustomFieldVisibility]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomFieldVisibilityEnum {}
/// Nested message and enum types in `CustomFieldVisibilityEnum`.
pub mod custom_field_visibility_enum {
    /// The visibility level of a CustomField.
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
    pub enum CustomFieldVisibility {
        /// No value specified
        Unspecified = 0,
        /// The CustomField is not visible in the UI and only visible through the
        /// API.
        Hidden = 1,
        /// The CustomField is visible in the UI and only editable through the API.
        ReadOnly = 2,
        /// The CustomField is visible and editable in both the API and UI.
        Editable = 3,
    }
    impl CustomFieldVisibility {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CustomFieldVisibility::Unspecified => {
                    "CUSTOM_FIELD_VISIBILITY_UNSPECIFIED"
                }
                CustomFieldVisibility::Hidden => "HIDDEN",
                CustomFieldVisibility::ReadOnly => "READ_ONLY",
                CustomFieldVisibility::Editable => "EDITABLE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CUSTOM_FIELD_VISIBILITY_UNSPECIFIED" => Some(Self::Unspecified),
                "HIDDEN" => Some(Self::Hidden),
                "READ_ONLY" => Some(Self::ReadOnly),
                "EDITABLE" => Some(Self::Editable),
                _ => None,
            }
        }
    }
}
/// The `CustomField` resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomField {
    /// Identifier. The resource name of the `CustomField`.
    /// Format: `networks/{network_code}/customFields/{custom_field_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. `CustomField` ID.
    #[prost(int64, tag = "2")]
    pub custom_field_id: i64,
    /// Required. The display name of the `CustomField`.
    ///
    /// This value has a maximum length of 127 characters.
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. The description of the `CustomField`.
    ///
    /// This value has a maximum length of 511 characters.
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The status of the `CustomField`.
    #[prost(enumeration = "custom_field_status_enum::CustomFieldStatus", tag = "5")]
    pub status: i32,
    /// Required. The type of entity the `CustomField` can be applied to.
    #[prost(
        enumeration = "custom_field_entity_type_enum::CustomFieldEntityType",
        tag = "7"
    )]
    pub entity_type: i32,
    /// Required. The data type of the `CustomField`.
    #[prost(enumeration = "custom_field_data_type_enum::CustomFieldDataType", tag = "8")]
    pub data_type: i32,
    /// Required. The visibility of the `CustomField`.
    #[prost(
        enumeration = "custom_field_visibility_enum::CustomFieldVisibility",
        tag = "9"
    )]
    pub visibility: i32,
    /// Optional. The drop-down options for the `CustomField`.
    ///
    /// Only applicable for `CustomField` with the drop-down data type.
    #[prost(message, repeated, tag = "10")]
    pub options: ::prost::alloc::vec::Vec<CustomFieldOption>,
}
/// An option for a drop-down `CustomField`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomFieldOption {
    /// Output only. `CustomFieldOption` ID.
    #[prost(int64, tag = "1")]
    pub custom_field_option_id: i64,
    /// Required. The display name of the `CustomFieldOption`.
    ///
    /// This value has a maximum length of 127 characters.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
}
/// Request object for `GetCustomField` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomFieldRequest {
    /// Required. The resource name of the CustomField.
    /// Format: `networks/{network_code}/customFields/{custom_field_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request object for `ListCustomFields` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCustomFieldsRequest {
    /// Required. The parent, which owns this collection of CustomFields.
    /// Format: `networks/{network_code}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of `CustomFields` to return. The service may
    /// return fewer than this value. If unspecified, at most 50 `CustomFields`
    /// will be returned. The maximum value is 1000; values above 1000 will be
    /// coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListCustomFields` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListCustomFields` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Expression to filter the response.
    /// See syntax details at <https://google.aip.dev/160>
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Expression to specify sorting order.
    /// See syntax details at <https://google.aip.dev/132#ordering>
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
    /// Optional. Number of individual resources to skip while paginating.
    #[prost(int32, tag = "6")]
    pub skip: i32,
}
/// Response object for `ListCustomFieldsRequest` containing matching
/// `CustomField` objects.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCustomFieldsResponse {
    /// The `CustomField` objects from the specified network.
    #[prost(message, repeated, tag = "1")]
    pub custom_fields: ::prost::alloc::vec::Vec<CustomField>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Total number of `CustomField` objects.
    /// If a filter was included in the request, this reflects the total number
    /// after the filtering is applied.
    ///
    /// `total_size` will not be calculated in the response unless it has been
    /// included in a response field mask. The response field mask can be provided
    /// to the method by using the URL parameter `$fields` or `fields`, or by using
    /// the HTTP/gRPC header `X-Goog-FieldMask`.
    ///
    /// For more information, see
    /// [System Parameters](<https://cloud.google.com/apis/docs/system-parameters>).
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Generated client implementations.
pub mod custom_field_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Provides methods for handling `CustomField` objects.
    #[derive(Debug, Clone)]
    pub struct CustomFieldServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CustomFieldServiceClient<tonic::transport::Channel> {
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
    impl<T> CustomFieldServiceClient<T>
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
        ) -> CustomFieldServiceClient<InterceptedService<T, F>>
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
            CustomFieldServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// API to retrieve a `CustomField` object.
        pub async fn get_custom_field(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCustomFieldRequest>,
        ) -> std::result::Result<tonic::Response<super::CustomField>, tonic::Status> {
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
                "/google.ads.admanager.v1.CustomFieldService/GetCustomField",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ads.admanager.v1.CustomFieldService",
                        "GetCustomField",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// API to retrieve a list of `CustomField` objects.
        pub async fn list_custom_fields(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCustomFieldsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCustomFieldsResponse>,
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
                "/google.ads.admanager.v1.CustomFieldService/ListCustomFields",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ads.admanager.v1.CustomFieldService",
                        "ListCustomFields",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Wrapper message for
/// [CustomTargetingKeyStatus][google.ads.admanager.v1.CustomTargetingKeyStatusEnum.CustomTargetingKeyStatus]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomTargetingKeyStatusEnum {}
/// Nested message and enum types in `CustomTargetingKeyStatusEnum`.
pub mod custom_targeting_key_status_enum {
    /// Status of the custom targeting key.
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
    pub enum CustomTargetingKeyStatus {
        /// Not specified value.
        Unspecified = 0,
        /// Custom targeting key is active.
        Active = 1,
        /// Custom targeting key is inactive.
        Inactive = 2,
    }
    impl CustomTargetingKeyStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CustomTargetingKeyStatus::Unspecified => {
                    "CUSTOM_TARGETING_KEY_STATUS_UNSPECIFIED"
                }
                CustomTargetingKeyStatus::Active => "ACTIVE",
                CustomTargetingKeyStatus::Inactive => "INACTIVE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CUSTOM_TARGETING_KEY_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
                "ACTIVE" => Some(Self::Active),
                "INACTIVE" => Some(Self::Inactive),
                _ => None,
            }
        }
    }
}
/// Wrapper message for
/// [CustomTargetingKeyType][google.ads.admanager.v1.CustomTargetingKeyTypeEnum.CustomTargetingKeyType]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomTargetingKeyTypeEnum {}
/// Nested message and enum types in `CustomTargetingKeyTypeEnum`.
pub mod custom_targeting_key_type_enum {
    /// Type of the custom targeting key.
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
    pub enum CustomTargetingKeyType {
        /// Not specified value.
        Unspecified = 0,
        /// Key with a fixed set of values.
        Predefined = 1,
        /// Key without a fixed set of values
        Freeform = 2,
    }
    impl CustomTargetingKeyType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CustomTargetingKeyType::Unspecified => {
                    "CUSTOM_TARGETING_KEY_TYPE_UNSPECIFIED"
                }
                CustomTargetingKeyType::Predefined => "PREDEFINED",
                CustomTargetingKeyType::Freeform => "FREEFORM",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CUSTOM_TARGETING_KEY_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "PREDEFINED" => Some(Self::Predefined),
                "FREEFORM" => Some(Self::Freeform),
                _ => None,
            }
        }
    }
}
/// Wrapper message for
/// [CustomTargetingKeyReportableType][google.ads.admanager.v1.CustomTargetingKeyReportableTypeEnum.CustomTargetingKeyReportableType]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomTargetingKeyReportableTypeEnum {}
/// Nested message and enum types in `CustomTargetingKeyReportableTypeEnum`.
pub mod custom_targeting_key_reportable_type_enum {
    /// Reportable type of the custom targeting key.
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
    pub enum CustomTargetingKeyReportableType {
        /// Not specified value.
        Unspecified = 0,
        /// Not available for reporting in the Ad Manager query tool.
        Off = 1,
        /// Available for reporting in the Ad Manager query tool.
        On = 2,
        /// Custom dimension available for reporting in the AdManager query tool.
        CustomDimension = 3,
    }
    impl CustomTargetingKeyReportableType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CustomTargetingKeyReportableType::Unspecified => {
                    "CUSTOM_TARGETING_KEY_REPORTABLE_TYPE_UNSPECIFIED"
                }
                CustomTargetingKeyReportableType::Off => "OFF",
                CustomTargetingKeyReportableType::On => "ON",
                CustomTargetingKeyReportableType::CustomDimension => "CUSTOM_DIMENSION",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CUSTOM_TARGETING_KEY_REPORTABLE_TYPE_UNSPECIFIED" => {
                    Some(Self::Unspecified)
                }
                "OFF" => Some(Self::Off),
                "ON" => Some(Self::On),
                "CUSTOM_DIMENSION" => Some(Self::CustomDimension),
                _ => None,
            }
        }
    }
}
/// The `CustomTargetingKey` resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomTargetingKey {
    /// Identifier. The resource name of the `CustomTargetingKey`.
    /// Format:
    /// `networks/{network_code}/customTargetingKeys/{custom_targeting_key_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. `CustomTargetingKey` ID.
    #[prost(int64, tag = "2")]
    pub custom_targeting_key_id: i64,
    /// Immutable. Name of the key. Keys can contain up to 10 characters each. You
    /// can use alphanumeric characters and symbols other than the following:
    /// ", ', =, !, +, #, *, ~, ;, ^, (, ), <, >, \[, \], the white space character.
    #[prost(string, tag = "3")]
    pub ad_tag_name: ::prost::alloc::string::String,
    /// Optional. Descriptive name for the `CustomTargetingKey`.
    #[prost(string, tag = "4")]
    pub display_name: ::prost::alloc::string::String,
    /// Required. Indicates whether users will select from predefined values or
    /// create new targeting values, while specifying targeting criteria for a line
    /// item.
    #[prost(
        enumeration = "custom_targeting_key_type_enum::CustomTargetingKeyType",
        tag = "5"
    )]
    pub r#type: i32,
    /// Output only. Status of the `CustomTargetingKey`.
    #[prost(
        enumeration = "custom_targeting_key_status_enum::CustomTargetingKeyStatus",
        tag = "6"
    )]
    pub status: i32,
    /// Required. Reportable state of the `CustomTargetingKey`.
    #[prost(
        enumeration = "custom_targeting_key_reportable_type_enum::CustomTargetingKeyReportableType",
        tag = "7"
    )]
    pub reportable_type: i32,
}
/// Request object for `GetCustomTargetingKey` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomTargetingKeyRequest {
    /// Required. The resource name of the CustomTargetingKey.
    /// Format:
    /// `networks/{network_code}/customTargetingKeys/{custom_targeting_key_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request object for `ListCustomTargetingKeys` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCustomTargetingKeysRequest {
    /// Required. The parent, which owns this collection of CustomTargetingKeys.
    /// Format: `networks/{network_code}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of `CustomTargetingKeys` to return. The
    /// service may return fewer than this value. If unspecified, at most 50
    /// `CustomTargetingKeys` will be returned. The maximum value is 1000; values
    /// above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListCustomTargetingKeys`
    /// call. Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListCustomTargetingKeys`
    /// must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Expression to filter the response.
    /// See syntax details at
    /// <https://developers.google.com/ad-manager/api/beta/filters>
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Expression to specify sorting order.
    /// See syntax details at
    /// <https://developers.google.com/ad-manager/api/beta/filters#order>
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
    /// Optional. Number of individual resources to skip while paginating.
    #[prost(int32, tag = "6")]
    pub skip: i32,
}
/// Response object for `ListCustomTargetingKeysRequest` containing matching
/// `CustomTargetingKey` objects.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCustomTargetingKeysResponse {
    /// The `CustomTargetingKey` objects from the specified network.
    #[prost(message, repeated, tag = "1")]
    pub custom_targeting_keys: ::prost::alloc::vec::Vec<CustomTargetingKey>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Total number of `CustomTargetingKey` objects.
    /// If a filter was included in the request, this reflects the total number
    /// after the filtering is applied.
    ///
    /// `total_size` will not be calculated in the response unless it has been
    /// included in a response field mask. The response field mask can be provided
    /// to the method by using the URL parameter `$fields` or `fields`, or by using
    /// the HTTP/gRPC header `X-Goog-FieldMask`.
    ///
    /// For more information, see
    /// <https://developers.google.com/ad-manager/api/beta/field-masks>
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Generated client implementations.
pub mod custom_targeting_key_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Provides methods for handling `CustomTargetingKey` objects.
    #[derive(Debug, Clone)]
    pub struct CustomTargetingKeyServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CustomTargetingKeyServiceClient<tonic::transport::Channel> {
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
    impl<T> CustomTargetingKeyServiceClient<T>
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
        ) -> CustomTargetingKeyServiceClient<InterceptedService<T, F>>
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
            CustomTargetingKeyServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
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
        /// API to retrieve a `CustomTargetingKey` object.
        pub async fn get_custom_targeting_key(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCustomTargetingKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CustomTargetingKey>,
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
                "/google.ads.admanager.v1.CustomTargetingKeyService/GetCustomTargetingKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ads.admanager.v1.CustomTargetingKeyService",
                        "GetCustomTargetingKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// API to retrieve a list of `CustomTargetingKey` objects.
        pub async fn list_custom_targeting_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCustomTargetingKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCustomTargetingKeysResponse>,
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
                "/google.ads.admanager.v1.CustomTargetingKeyService/ListCustomTargetingKeys",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ads.admanager.v1.CustomTargetingKeyService",
                        "ListCustomTargetingKeys",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Wrapper message for
/// [CustomTargetingValueStatus][google.ads.admanager.v1.CustomTargetingValueStatusEnum.CustomTargetingValueStatus]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomTargetingValueStatusEnum {}
/// Nested message and enum types in `CustomTargetingValueStatusEnum`.
pub mod custom_targeting_value_status_enum {
    /// Status of the custom targeting value.
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
    pub enum CustomTargetingValueStatus {
        /// Not specified value.
        Unspecified = 0,
        /// Custom targeting value is active.
        Active = 1,
        /// Custom targeting value is inactive.
        Inactive = 2,
    }
    impl CustomTargetingValueStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CustomTargetingValueStatus::Unspecified => {
                    "CUSTOM_TARGETING_VALUE_STATUS_UNSPECIFIED"
                }
                CustomTargetingValueStatus::Active => "ACTIVE",
                CustomTargetingValueStatus::Inactive => "INACTIVE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CUSTOM_TARGETING_VALUE_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
                "ACTIVE" => Some(Self::Active),
                "INACTIVE" => Some(Self::Inactive),
                _ => None,
            }
        }
    }
}
/// Wrapper message for
/// [CustomTargetingValueMatchType][google.ads.admanager.v1.CustomTargetingValueMatchTypeEnum.CustomTargetingValueMatchType]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomTargetingValueMatchTypeEnum {}
/// Nested message and enum types in `CustomTargetingValueMatchTypeEnum`.
pub mod custom_targeting_value_match_type_enum {
    /// Match type of the custom targeting value.
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
    pub enum CustomTargetingValueMatchType {
        /// Not specified value.
        Unspecified = 0,
        /// Used for exact matching. For example, the targeting value car=honda will
        /// only match to the ad request car=honda.
        Exact = 1,
        /// Used for lenient matching when at least one of the words in the ad
        /// request matches the targeted value. The targeting value car=honda will
        /// match to ad requests containing the word honda. So ad requests car=honda
        /// or car=honda civic or car=buy honda or car=how much does a honda cost
        /// will all have the line item delivered.
        /// This match type can not be used within an audience segment rule.
        Broad = 2,
        /// Used for 'starts with' matching when the first few characters in the ad
        /// request match all of the characters in the targeted value. The targeting
        /// value car=honda will match to ad requests car=honda or car=hondas for
        /// sale but not to car=I want a honda.
        Prefix = 3,
        /// This is a combination of MatchType#BROAD and MatchType#PREFIX matching.
        /// The targeting value car=honda will match to ad requests that contain
        /// words that start with the characters in the targeted value, for example
        /// with car=civic hondas.
        /// This match type can not be used within an audience segment rule.
        BroadPrefix = 4,
        /// Used for 'ends with' matching when the last characters in the ad request
        /// match all of the characters in the targeted value. The targeting value
        /// car=honda will match with ad requests car=honda or car=I want a honda but
        /// not to car=hondas for sale.
        /// This match type can not be used within line item targeting.
        Suffix = 5,
        /// Used for 'within' matching when the string in the ad request contains the
        /// string in the targeted value. The targeting value car=honda will match
        /// with ad requests car=honda, car=I want a honda, and also with car=hondas
        /// for sale, but not with car=misspelled hond a. This match type can not be
        /// used within line item targeting.
        Contains = 6,
    }
    impl CustomTargetingValueMatchType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CustomTargetingValueMatchType::Unspecified => {
                    "CUSTOM_TARGETING_VALUE_MATCH_TYPE_UNSPECIFIED"
                }
                CustomTargetingValueMatchType::Exact => "EXACT",
                CustomTargetingValueMatchType::Broad => "BROAD",
                CustomTargetingValueMatchType::Prefix => "PREFIX",
                CustomTargetingValueMatchType::BroadPrefix => "BROAD_PREFIX",
                CustomTargetingValueMatchType::Suffix => "SUFFIX",
                CustomTargetingValueMatchType::Contains => "CONTAINS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CUSTOM_TARGETING_VALUE_MATCH_TYPE_UNSPECIFIED" => {
                    Some(Self::Unspecified)
                }
                "EXACT" => Some(Self::Exact),
                "BROAD" => Some(Self::Broad),
                "PREFIX" => Some(Self::Prefix),
                "BROAD_PREFIX" => Some(Self::BroadPrefix),
                "SUFFIX" => Some(Self::Suffix),
                "CONTAINS" => Some(Self::Contains),
                _ => None,
            }
        }
    }
}
/// The `CustomTargetingValue` resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomTargetingValue {
    /// Identifier. The resource name of the `CustomTargetingValue`.
    /// Format:
    /// `networks/{network_code}/customTargetingKeys/{custom_targeting_key_id}/customTargetingValues/{custom_targeting_value_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. Name of the `CustomTargetingValue`. Values can contain up to 40
    /// characters each. You can use alphanumeric characters and symbols other than
    /// the following: ", ', =, !, +, #, *, ~, ;, ^, (, ), <, >, \[, \]. Values are
    /// not data-specific; all values are treated as string. For example, instead
    /// of using "age>=18 AND <=34", try "18-34"
    #[prost(string, tag = "4")]
    pub ad_tag_name: ::prost::alloc::string::String,
    /// Optional. Descriptive name for the `CustomTargetingValue`.
    #[prost(string, tag = "5")]
    pub display_name: ::prost::alloc::string::String,
    /// Required. The way in which the CustomTargetingValue.name strings will be
    /// matched.
    #[prost(
        enumeration = "custom_targeting_value_match_type_enum::CustomTargetingValueMatchType",
        tag = "6"
    )]
    pub match_type: i32,
    /// Output only. Status of the `CustomTargetingValue`.
    #[prost(
        enumeration = "custom_targeting_value_status_enum::CustomTargetingValueStatus",
        tag = "7"
    )]
    pub status: i32,
}
/// Request object for `GetCustomTargetingValue` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomTargetingValueRequest {
    /// Required. The resource name of the CustomTargetingValue.
    /// Format:
    /// `networks/{network_code}/customTargetingKeys/{custom_targeting_key_id}/customTargetingValues/{custom_targeting_value_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request object for `ListCustomTargetingValues` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCustomTargetingValuesRequest {
    /// Required. The parent, which owns this collection of CustomTargetingValues.
    /// Format:
    /// `networks/{network_code}/customTargetingKeys/{custom_targeting_key_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of `CustomTargetingValues` to return. The
    /// service may return fewer than this value. If unspecified, at most 50
    /// `CustomTargetingValues` will be returned. The maximum value is 1000; values
    /// above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous
    /// `ListCustomTargetingValues` call. Provide this to retrieve the subsequent
    /// page.
    ///
    /// When paginating, all other parameters provided to
    /// `ListCustomTargetingValues` must match the call that provided the page
    /// token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Expression to filter the response.
    /// See syntax details at
    /// <https://developers.google.com/ad-manager/api/beta/filters>
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Expression to specify sorting order.
    /// See syntax details at
    /// <https://developers.google.com/ad-manager/api/beta/filters#order>
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
    /// Optional. Number of individual resources to skip while paginating.
    #[prost(int32, tag = "6")]
    pub skip: i32,
}
/// Response object for `ListCustomTargetingValuesRequest` containing matching
/// `CustomTargetingValue` objects.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCustomTargetingValuesResponse {
    /// The `CustomTargetingValue` objects from the specified network.
    #[prost(message, repeated, tag = "1")]
    pub custom_targeting_values: ::prost::alloc::vec::Vec<CustomTargetingValue>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Total number of `CustomTargetingValue` objects.
    /// If a filter was included in the request, this reflects the total number
    /// after the filtering is applied.
    ///
    /// `total_size` will not be calculated in the response unless it has been
    /// included in a response field mask. The response field mask can be provided
    /// to the method by using the URL parameter `$fields` or `fields`, or by using
    /// the HTTP/gRPC header `X-Goog-FieldMask`.
    ///
    /// For more information, see
    /// <https://developers.google.com/ad-manager/api/beta/field-masks>
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Generated client implementations.
pub mod custom_targeting_value_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Provides methods for handling `CustomTargetingValue` objects.
    #[derive(Debug, Clone)]
    pub struct CustomTargetingValueServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CustomTargetingValueServiceClient<tonic::transport::Channel> {
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
    impl<T> CustomTargetingValueServiceClient<T>
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
        ) -> CustomTargetingValueServiceClient<InterceptedService<T, F>>
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
            CustomTargetingValueServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
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
        /// API to retrieve a `CustomTargetingValue` object.
        pub async fn get_custom_targeting_value(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCustomTargetingValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CustomTargetingValue>,
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
                "/google.ads.admanager.v1.CustomTargetingValueService/GetCustomTargetingValue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ads.admanager.v1.CustomTargetingValueService",
                        "GetCustomTargetingValue",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// API to retrieve a list of `CustomTargetingValue` objects.
        pub async fn list_custom_targeting_values(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCustomTargetingValuesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCustomTargetingValuesResponse>,
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
                "/google.ads.admanager.v1.CustomTargetingValueService/ListCustomTargetingValues",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ads.admanager.v1.CustomTargetingValueService",
                        "ListCustomTargetingValues",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Defines the criteria a [LineItem][google.ads.admanager.v1.LineItem] needs to
/// satisfy to meet its delivery
///   goal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Goal {
    /// The type of the goal for the LineItem. It defines the period over which the
    /// goal should be reached.
    #[prost(enumeration = "goal_type_enum::GoalType", tag = "1")]
    pub goal_type: i32,
    /// The type of the goal unit for the LineItem.
    #[prost(enumeration = "unit_type_enum::UnitType", tag = "2")]
    pub unit_type: i32,
    /// If this is a primary goal, it represents the number or percentage of
    /// impressions or clicks that will be reserved. If the line item is of type
    /// [LineItemTypeEnum.LineItemType.SPONSORSHIP][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.SPONSORSHIP],
    /// it represents the percentage of available impressions reserved. If the line
    /// item is of type
    /// [LineItemTypeEnum.LineItemType.BULK][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.BULK]
    /// or
    /// [LineItemTypeEnum.LineItemType.PRICE_PRIORITY][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.PRICE_PRIORITY],
    /// it represents the number of remaining impressions reserved. If the line
    /// item is of type
    /// [LineItemTypeEnum.LineItemType.NETWORK][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.NETWORK]
    /// or
    /// [LineItemTypeEnum.LineItemType.HOUSE][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.HOUSE],
    /// it represents the percentage of remaining impressions reserved. <p>If this
    /// is an impression cap goal, it represents the number of impressions or
    /// conversions that the line item will stop serving at if reached. For valid
    /// line item types, see [LineItem.impressions_cap][].
    #[prost(int64, tag = "3")]
    pub units: i64,
}
/// Wrapper message for
/// [GoalType][google.ads.admanager.v1.GoalTypeEnum.GoalType].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoalTypeEnum {}
/// Nested message and enum types in `GoalTypeEnum`.
pub mod goal_type_enum {
    /// Specifies the type of the goal for a LineItem.
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
    pub enum GoalType {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// No goal is specified for the number of ads delivered.
        /// The line item [type][google.ads.admanager.v1.LineItem.line_item_type]
        /// must be one of:
        ///
        /// * [LineItemTypeEnum.LineItemType.PRICE_PRIORITY][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.PRICE_PRIORITY]
        /// * [LineItemTypeEnum.LineItemType.AD_EXCHANGE][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.AD_EXCHANGE]
        /// * [LineItemTypeEnum.LineItemType.CLICK_TRACKING][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.CLICK_TRACKING]
        None = 1,
        /// There is a goal on the number of ads delivered for this line item during
        /// its entire lifetime.
        /// The line item [type][google.ads.admanager.v1.LineItem.line_item_type]
        /// must be one of:
        ///
        /// * [LineItemTypeEnum.LineItemType.STANDARD][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.STANDARD]
        /// * [LineItemTypeEnum.LineItemType.BULK][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.BULK]
        /// * [LineItemTypeEnum.LineItemType.PRICE_PRIORITY][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.PRICE_PRIORITY]
        /// * [LineItemTypeEnum.LineItemType.ADSENSE][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.ADSENSE]
        /// * [LineItemTypeEnum.LineItemType.AD_EXCHANGE][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.AD_EXCHANGE]
        /// * [LineItemTypeEnum.LineItemType.ADMOB][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.ADMOB]
        /// * [LineItemTypeEnum.LineItemType.CLICK_TRACKING][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.CLICK_TRACKING]
        Lifetime = 2,
        /// There is a daily goal on the number of ads delivered for this line item.
        /// The line item [type][google.ads.admanager.v1.LineItem.line_item_type]
        /// must be one of:
        ///
        /// * [LineItemTypeEnum.LineItemType.SPONSORSHIP][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.SPONSORSHIP]
        /// * [LineItemTypeEnum.LineItemType.NETWORK][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.NETWORK]
        /// * [LineItemTypeEnum.LineItemType.PRICE_PRIORITY][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.PRICE_PRIORITY]
        /// * [LineItemTypeEnum.LineItemType.HOUSE][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.HOUSE]
        /// * [LineItemTypeEnum.LineItemType.ADSENSE][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.ADSENSE]
        /// * [LineItemTypeEnum.LineItemType.AD_EXCHANGE][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.AD_EXCHANGE]
        /// * [LineItemTypeEnum.LineItemType.ADMOB][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.ADMOB]
        /// * [LineItemTypeEnum.LineItemType.BUMPER][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.BUMPER]
        Daily = 3,
    }
    impl GoalType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                GoalType::Unspecified => "GOAL_TYPE_UNSPECIFIED",
                GoalType::None => "NONE",
                GoalType::Lifetime => "LIFETIME",
                GoalType::Daily => "DAILY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "GOAL_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "NONE" => Some(Self::None),
                "LIFETIME" => Some(Self::Lifetime),
                "DAILY" => Some(Self::Daily),
                _ => None,
            }
        }
    }
}
/// Wrapper message for
/// [UnitType][google.ads.admanager.v1.UnitTypeEnum.UnitType].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnitTypeEnum {}
/// Nested message and enum types in `UnitTypeEnum`.
pub mod unit_type_enum {
    /// Indicates the type of unit used for defining a reservation. The
    /// [LineItem.cost_type][google.ads.admanager.v1.LineItem.cost_type] can differ
    /// from the UnitType - an ad can have an impression goal, but be billed by its
    /// click. Usually CostType and UnitType will refer to the same unit.
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
    pub enum UnitType {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// The number of impressions served by creatives associated with the line
        /// item.
        Impressions = 1,
        /// The number of clicks reported by creatives associated with the line item.
        /// The line item [type][google.ads.admanager.v1.LineItem.line_item_type]
        /// must be one of:
        ///
        /// * [LineItemTypeEnum.LineItemType.STANDARD][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.STANDARD]
        /// * [LineItemTypeEnum.LineItemType.BULK][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.BULK]
        /// * [LineItemTypeEnum.LineItemType.PRICE_PRIORITY][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.PRICE_PRIORITY]
        Clicks = 2,
        /// The number of view-through Cost-Per-Action (CPA) conversions from
        /// creatives associated with the line item. This is only supported as
        /// secondary goal and the
        /// [LineItem.cost_type][google.ads.admanager.v1.LineItem.cost_type] must be
        /// [CostTypeEnum.CostType.CPA][].
        ClickThroughCpaConversions = 3,
        /// The number of view-through Cost-Per-Action (CPA) conversions from
        /// creatives associated with the line item. This is only supported as
        /// secondary goal and the
        /// [LineItem.cost_type][google.ads.admanager.v1.LineItem.cost_type] must be
        /// [CostTypeEnum.CostType.CPA}.
        ViewThroughCpaConversions = 4,
        /// The number of total Cost-Per-Action (CPA) conversions from creatives
        /// associated with the line item. This is only supported as secondary goal
        /// and the [LineItem.cost_type} must be [CostTypeEnum.CostType.CPA}.
        TotalCpaConversions = 5,
        /// The number of viewable impressions reported by creatives associated with
        /// the line item. The
        /// [LineItem.line_item_type][google.ads.admanager.v1.LineItem.line_item_type]
        /// must be
        /// [LineItemTypeEnum.LineItemType.STANDARD][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.STANDARD].
        ViewableImpressions = 6,
        /// The number of in-target impressions reported by third party measurements.
        /// The
        /// [LineItem.line_item_type][google.ads.admanager.v1.LineItem.line_item_type]
        /// must be
        /// [LineItemTypeEnum.LineItemType.STANDARD][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.STANDARD].
        InTargetImpressions = 7,
    }
    impl UnitType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                UnitType::Unspecified => "UNIT_TYPE_UNSPECIFIED",
                UnitType::Impressions => "IMPRESSIONS",
                UnitType::Clicks => "CLICKS",
                UnitType::ClickThroughCpaConversions => "CLICK_THROUGH_CPA_CONVERSIONS",
                UnitType::ViewThroughCpaConversions => "VIEW_THROUGH_CPA_CONVERSIONS",
                UnitType::TotalCpaConversions => "TOTAL_CPA_CONVERSIONS",
                UnitType::ViewableImpressions => "VIEWABLE_IMPRESSIONS",
                UnitType::InTargetImpressions => "IN_TARGET_IMPRESSIONS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNIT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "IMPRESSIONS" => Some(Self::Impressions),
                "CLICKS" => Some(Self::Clicks),
                "CLICK_THROUGH_CPA_CONVERSIONS" => Some(Self::ClickThroughCpaConversions),
                "VIEW_THROUGH_CPA_CONVERSIONS" => Some(Self::ViewThroughCpaConversions),
                "TOTAL_CPA_CONVERSIONS" => Some(Self::TotalCpaConversions),
                "VIEWABLE_IMPRESSIONS" => Some(Self::ViewableImpressions),
                "IN_TARGET_IMPRESSIONS" => Some(Self::InTargetImpressions),
                _ => None,
            }
        }
    }
}
/// The Label resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Label {
    /// Identifier. The resource name of the Label.
    /// Format: `networks/{network_code}/labels/{label_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request object for GetLabel method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLabelRequest {
    /// Required. The resource name of the Label.
    /// Format: `networks/{network_code}/labels/{label_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request object for ListLabels method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLabelsRequest {
    /// Required. The parent, which owns this collection of Labels.
    /// Format: `networks/{network_code}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of Labels to return. The service may return
    /// fewer than this value. If unspecified, at most 50 labels will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListLabels` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListLabels` must match
    /// the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Expression to filter the response.
    /// See syntax details at
    /// <https://developers.google.com/ad-manager/api/beta/filters>
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Expression to specify sorting order.
    /// See syntax details at
    /// <https://developers.google.com/ad-manager/api/beta/filters#order>
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
    /// Optional. Number of individual resources to skip while paginating.
    #[prost(int32, tag = "6")]
    pub skip: i32,
}
/// Response object for ListLabelsRequest containing matching Label
/// resources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLabelsResponse {
    /// The Label from the specified network.
    #[prost(message, repeated, tag = "1")]
    pub labels: ::prost::alloc::vec::Vec<Label>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Total number of Labels.
    /// If a filter was included in the request, this reflects the total number
    /// after the filtering is applied.
    ///
    /// `total_size` will not be calculated in the response unless it has been
    /// included in a response field mask. The response field mask can be provided
    /// to the method by using the URL parameter `$fields` or `fields`, or by using
    /// the HTTP/gRPC header `X-Goog-FieldMask`.
    ///
    /// For more information, see
    /// <https://developers.google.com/ad-manager/api/beta/field-masks>
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Generated client implementations.
pub mod label_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Provides methods for handling Label objects.
    #[derive(Debug, Clone)]
    pub struct LabelServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl LabelServiceClient<tonic::transport::Channel> {
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
    impl<T> LabelServiceClient<T>
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
        ) -> LabelServiceClient<InterceptedService<T, F>>
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
            LabelServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// API to retrieve a Label object.
        pub async fn get_label(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLabelRequest>,
        ) -> std::result::Result<tonic::Response<super::Label>, tonic::Status> {
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
                "/google.ads.admanager.v1.LabelService/GetLabel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.ads.admanager.v1.LabelService", "GetLabel"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// API to retrieve a list of Label objects.
        pub async fn list_labels(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLabelsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListLabelsResponse>,
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
                "/google.ads.admanager.v1.LabelService/ListLabels",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.ads.admanager.v1.LabelService", "ListLabels"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Wrapper message for
/// [LineItemCostType][google.ads.admanager.v1.LineItemCostTypeEnum.LineItemCostType].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LineItemCostTypeEnum {}
/// Nested message and enum types in `LineItemCostTypeEnum`.
pub mod line_item_cost_type_enum {
    /// Describes the LineItem actions that are billable.
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
    pub enum LineItemCostType {
        /// Not specified value.
        Unspecified = 0,
        /// Cost per action. The line item
        /// [type][google.ads.admanager.v1.LineItem.line_item_type] must be one of:
        ///
        /// * [LineItemTypeEnum.LineItemType.SPONSORSHIP][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.SPONSORSHIP]
        /// * [LineItemTypeEnum.LineItemType.STANDARD][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.STANDARD]
        /// * [LineItemTypeEnum.LineItemType.BULK][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.BULK]
        /// * [LineItemTypeEnum.LineItemType.NETWORK][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.NETWORK]
        Cpa = 1,
        /// Cost per click. The line item
        /// [type][google.ads.admanager.v1.LineItem.line_item_type] must be one of:
        ///
        /// * [LineItemTypeEnum.LineItemType.SPONSORSHIP][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.SPONSORSHIP]
        /// * [LineItemTypeEnum.LineItemType.STANDARD][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.STANDARD]
        /// * [LineItemTypeEnum.LineItemType.BULK][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.BULK]
        /// * [LineItemTypeEnum.LineItemType.NETWORK][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.NETWORK]
        /// * [LineItemTypeEnum.LineItemType.PRICE_PRIORITY][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.PRICE_PRIORITY]
        /// * [LineItemTypeEnum.LineItemType.HOUSE][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.HOUSE]
        Cpc = 2,
        /// Cost per day. The line item
        /// [type][google.ads.admanager.v1.LineItem.line_item_type] must be one of:
        ///
        /// * [LineItemTypeEnum.LineItemType.SPONSORSHIP][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.SPONSORSHIP]
        /// * [LineItemTypeEnum.LineItemType.NETWORK][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.NETWORK]
        Cpd = 3,
        /// Cost per mille (thousand) impressions. The line item
        /// [type][google.ads.admanager.v1.LineItem.line_item_type] must be one of:
        ///
        /// * [LineItemTypeEnum.LineItemType.SPONSORSHIP][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.SPONSORSHIP]
        /// * [LineItemTypeEnum.LineItemType.STANDARD][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.STANDARD]
        /// * [LineItemTypeEnum.LineItemType.BULK][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.BULK]
        /// * [LineItemTypeEnum.LineItemType.NETWORK][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.NETWORK]
        /// * [LineItemTypeEnum.LineItemType.PRICE_PRIORITY][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.PRICE_PRIORITY]
        /// * [LineItemTypeEnum.LineItemType.HOUSE][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.HOUSE]
        Cpm = 4,
        /// Cost per mille (thousand) Active View viewable impressions. The line item
        /// [type][google.ads.admanager.v1.LineItem.line_item_type] must be one of:
        ///
        /// * [LineItemTypeEnum.LineItemType.STANDARD][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.STANDARD]
        Vcpm = 5,
        /// Cost per millie (thousand) in-target impressions. The line item
        /// [type][google.ads.admanager.v1.LineItem.line_item_type] must be one of:
        ///
        /// * [LineItemTypeEnum.LineItemType.STANDARD][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.STANDARD]
        CpmInTarget = 6,
        /// Cost for the entire flight of the deal. The line item
        /// [type][google.ads.admanager.v1.LineItem.line_item_type] must be must be
        /// one of:
        ///
        /// * [LineItemTypeEnum.LineItemType.SPONSORSHIP][google.ads.admanager.v1.LineItemTypeEnum.LineItemType.SPONSORSHIP]
        Cpf = 7,
    }
    impl LineItemCostType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LineItemCostType::Unspecified => "LINE_ITEM_COST_TYPE_UNSPECIFIED",
                LineItemCostType::Cpa => "CPA",
                LineItemCostType::Cpc => "CPC",
                LineItemCostType::Cpd => "CPD",
                LineItemCostType::Cpm => "CPM",
                LineItemCostType::Vcpm => "VCPM",
                LineItemCostType::CpmInTarget => "CPM_IN_TARGET",
                LineItemCostType::Cpf => "CPF",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LINE_ITEM_COST_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "CPA" => Some(Self::Cpa),
                "CPC" => Some(Self::Cpc),
                "CPD" => Some(Self::Cpd),
                "CPM" => Some(Self::Cpm),
                "VCPM" => Some(Self::Vcpm),
                "CPM_IN_TARGET" => Some(Self::CpmInTarget),
                "CPF" => Some(Self::Cpf),
                _ => None,
            }
        }
    }
}
/// Wrapper message for
/// [CreativeRotationType][google.ads.admanager.v1.CreativeRotationTypeEnum.CreativeRotationType].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreativeRotationTypeEnum {}
/// Nested message and enum types in `CreativeRotationTypeEnum`.
pub mod creative_rotation_type_enum {
    /// The strategy to use for displaying multiple
    /// [creatives][google.ads.admanager.v1.Creative] that are associated with a
    /// line item.
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
    pub enum CreativeRotationType {
        /// Not specified value
        Unspecified = 0,
        /// Creatives are displayed approximately the same number of times over the
        /// duration of the line item.
        Evenly = 1,
        /// Creatives are served approximately proportionally to their performance.
        Optimized = 2,
        /// Creatives are served approximately proportionally to their weights, set
        /// on the `LineItemCreativeAssociation`.
        Weighted = 3,
        /// Creatives are served exactly in sequential order, aka Storyboarding. Set
        /// on the `LineItemCreativeAssociation`.
        Sequential = 4,
    }
    impl CreativeRotationType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CreativeRotationType::Unspecified => "CREATIVE_ROTATION_TYPE_UNSPECIFIED",
                CreativeRotationType::Evenly => "EVENLY",
                CreativeRotationType::Optimized => "OPTIMIZED",
                CreativeRotationType::Weighted => "WEIGHTED",
                CreativeRotationType::Sequential => "SEQUENTIAL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CREATIVE_ROTATION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "EVENLY" => Some(Self::Evenly),
                "OPTIMIZED" => Some(Self::Optimized),
                "WEIGHTED" => Some(Self::Weighted),
                "SEQUENTIAL" => Some(Self::Sequential),
                _ => None,
            }
        }
    }
}
/// Wrapper message for
/// [DeliveryRateType][google.ads.admanager.v1.DeliveryRateTypeEnum.DeliveryRateType].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeliveryRateTypeEnum {}
/// Nested message and enum types in `DeliveryRateTypeEnum`.
pub mod delivery_rate_type_enum {
    /// Possible delivery rates for a line item. It dictates the manner in which
    /// the line item is served.
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
    pub enum DeliveryRateType {
        /// Not specified value
        Unspecified = 0,
        /// Line items are served as evenly as possible across the number of days
        /// specified in a line item's [duration][LineItem.duration].
        Evenly = 1,
        /// Line items are served more aggressively in the beginning of the flight
        /// date.
        Frontloaded = 2,
        /// The booked impressions may delivered well before the
        /// [end_time][google.ads.admanager.v1.LineItem.end_time]. Other
        /// lower-priority or lower-value line items will be stopped from delivering
        /// until the line item meets the number of impressions or clicks it is
        /// booked for.
        AsFastAsPossible = 3,
    }
    impl DeliveryRateType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DeliveryRateType::Unspecified => "DELIVERY_RATE_TYPE_UNSPECIFIED",
                DeliveryRateType::Evenly => "EVENLY",
                DeliveryRateType::Frontloaded => "FRONTLOADED",
                DeliveryRateType::AsFastAsPossible => "AS_FAST_AS_POSSIBLE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DELIVERY_RATE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "EVENLY" => Some(Self::Evenly),
                "FRONTLOADED" => Some(Self::Frontloaded),
                "AS_FAST_AS_POSSIBLE" => Some(Self::AsFastAsPossible),
                _ => None,
            }
        }
    }
}
/// Wrapper message for
/// [LineItemDiscountType][google.ads.admanager.v1.LineItemDiscountTypeEnum.LineItemDiscountType].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LineItemDiscountTypeEnum {}
/// Nested message and enum types in `LineItemDiscountTypeEnum`.
pub mod line_item_discount_type_enum {
    /// Describes the possible discount types on the cost of booking a line item.
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
    pub enum LineItemDiscountType {
        /// No value specified
        Unspecified = 0,
        /// An absolute value will be discounted from the line item's cost.
        AbsoluteValue = 1,
        /// A percentage of the cost will be discounted for booking the line item.
        Percentage = 2,
    }
    impl LineItemDiscountType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LineItemDiscountType::Unspecified => {
                    "LINE_ITEM_DISCOUNT_TYPE_UNSPECIFIED"
                }
                LineItemDiscountType::AbsoluteValue => "ABSOLUTE_VALUE",
                LineItemDiscountType::Percentage => "PERCENTAGE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LINE_ITEM_DISCOUNT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "ABSOLUTE_VALUE" => Some(Self::AbsoluteValue),
                "PERCENTAGE" => Some(Self::Percentage),
                _ => None,
            }
        }
    }
}
/// Wrapper message for
/// [LineItemType][google.ads.admanager.v1.LineItemTypeEnum.LineItemType].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LineItemTypeEnum {}
/// Nested message and enum types in `LineItemTypeEnum`.
pub mod line_item_type_enum {
    /// Indicates the priority of a LineItem, determined by the way in which
    /// impressions are reserved to be served for it.
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
    pub enum LineItemType {
        /// Not specified value.
        Unspecified = 0,
        /// The type of LineItem for which a percentage of all the impressions that
        /// are being sold are reserved.
        Sponsorship = 12,
        /// The type of LineItem for which a fixed quantity of impressions or
        /// clicks are reserved.
        Standard = 13,
        /// The type of LineItem most commonly used to fill a site's unsold
        /// inventory if not contractually obligated to deliver a requested number
        /// of impressions. Uses daily percentage of unsold impressions or clicks.
        Network = 9,
        /// The type of LineItem for which a fixed quantity of impressions or
        /// clicks will be delivered at a priority lower than the STANDARD type.
        Bulk = 4,
        /// The type of LineItem most commonly used to fill a site's unsold
        /// inventory if not contractually obligated to deliver a requested number
        /// of impressions. Uses fixed quantity percentage of unsold impressions or
        /// clicks.
        PricePriority = 11,
        /// The type of LineItem typically used for ads that promote products and
        /// services chosen by the publisher.
        House = 7,
        /// Represents a legacy LineItem that has been migrated from the DFP
        /// system.
        LegacyDfp = 8,
        /// The type of LineItem used for ads that track ads being served
        /// externally of Ad Manager.
        ClickTracking = 6,
        /// A LineItem using dynamic allocation backed by AdSense.
        Adsense = 2,
        /// A LineItem using dynamic allocation backed by the Google Ad Exchange.
        AdExchange = 3,
        /// Represents a non-monetizable video LineItem that targets one or more
        /// bumper positions, which are short house video messages used by
        /// publishers to separate content from ad breaks.
        Bumper = 5,
        /// A LineItem using dynamic allocation backed by AdMob.
        Admob = 1,
        /// The type of LineItem for which there are no impressions reserved, and
        /// will serve for a second price bid.
        PreferredDeal = 10,
    }
    impl LineItemType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LineItemType::Unspecified => "LINE_ITEM_TYPE_UNSPECIFIED",
                LineItemType::Sponsorship => "SPONSORSHIP",
                LineItemType::Standard => "STANDARD",
                LineItemType::Network => "NETWORK",
                LineItemType::Bulk => "BULK",
                LineItemType::PricePriority => "PRICE_PRIORITY",
                LineItemType::House => "HOUSE",
                LineItemType::LegacyDfp => "LEGACY_DFP",
                LineItemType::ClickTracking => "CLICK_TRACKING",
                LineItemType::Adsense => "ADSENSE",
                LineItemType::AdExchange => "AD_EXCHANGE",
                LineItemType::Bumper => "BUMPER",
                LineItemType::Admob => "ADMOB",
                LineItemType::PreferredDeal => "PREFERRED_DEAL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LINE_ITEM_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "SPONSORSHIP" => Some(Self::Sponsorship),
                "STANDARD" => Some(Self::Standard),
                "NETWORK" => Some(Self::Network),
                "BULK" => Some(Self::Bulk),
                "PRICE_PRIORITY" => Some(Self::PricePriority),
                "HOUSE" => Some(Self::House),
                "LEGACY_DFP" => Some(Self::LegacyDfp),
                "CLICK_TRACKING" => Some(Self::ClickTracking),
                "ADSENSE" => Some(Self::Adsense),
                "AD_EXCHANGE" => Some(Self::AdExchange),
                "BUMPER" => Some(Self::Bumper),
                "ADMOB" => Some(Self::Admob),
                "PREFERRED_DEAL" => Some(Self::PreferredDeal),
                _ => None,
            }
        }
    }
}
/// Wrapper message for
/// [ReservationStatus][google.ads.admanager.v1.ReservationStatusEnum.ReservationStatus].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReservationStatusEnum {}
/// Nested message and enum types in `ReservationStatusEnum`.
pub mod reservation_status_enum {
    /// Defines the different reservation statuses of a line item.
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
    pub enum ReservationStatus {
        /// No value specified
        Unspecified = 0,
        /// Indicates that inventory has been reserved for the line item.
        Reserved = 1,
        /// Indicates that inventory has not been reserved for the line item.
        Unreserved = 2,
    }
    impl ReservationStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ReservationStatus::Unspecified => "RESERVATION_STATUS_UNSPECIFIED",
                ReservationStatus::Reserved => "RESERVED",
                ReservationStatus::Unreserved => "UNRESERVED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RESERVATION_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
                "RESERVED" => Some(Self::Reserved),
                "UNRESERVED" => Some(Self::Unreserved),
                _ => None,
            }
        }
    }
}
/// The LineItem resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LineItem {
    /// Identifier. The resource name of the LineItem.
    /// Format:
    /// `networks/{network_code}/orders/{order_id}/lineItems/{line_item_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Display name of the LineItem. This attribute has a maximum length
    /// of 255 characters.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. The archival status of the LineItem.
    #[prost(bool, tag = "14")]
    pub archived: bool,
    /// Optional. This attribute is only applicable for certain
    /// [line item types][LineItemType] and acts as an "FYI" or note, which does
    /// not impact ad-serving or other backend systems.
    ///
    /// For [SPONSORSHIP][LineItemType.SPONSORSHIP] line items, this represents
    /// the minimum quantity, which is a lifetime impression volume goal for
    /// reporting purposes.
    ///
    /// For [STANDARD][LineItemType.STANDARD] line items, this represents the
    /// contracted quantity, which is the number of units specified in the contract
    /// that the advertiser has bought for this line item. This attribute is only
    /// available if you have this feature enabled on your network.
    #[prost(int64, tag = "18")]
    pub contracted_units_bought: i64,
    /// Required. The amount of money to spend per impression or click.
    #[prost(message, optional, tag = "15")]
    pub cost_per_unit: ::core::option::Option<super::super::super::r#type::Money>,
    /// Required. The method used for billing this line item.
    #[prost(enumeration = "line_item_cost_type_enum::LineItemCostType", tag = "19")]
    pub cost_type: i32,
    /// Output only. The instant at which the LineItem was created. This attribute
    /// may be null for line items created before this feature was introduced.
    #[prost(message, optional, tag = "12")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The instant at which the LineItem was last updated
    #[prost(message, optional, tag = "31")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. The strategy used for displaying multiple
    /// [creatives][google.ads.admanager.v1.Creative] that are associated with the
    /// line item.
    #[prost(
        enumeration = "creative_rotation_type_enum::CreativeRotationType",
        tag = "22"
    )]
    pub creative_rotation_type: i32,
    /// Non-empty default. The strategy for delivering ads over the duration of the
    /// line item. Defaults to [EVENLY][DeliveryRateType.EVENLY] or
    /// [FRONTLOADED][DeliveryRatetype.FRONTLOADED] depending on the network's
    /// configuration.
    #[prost(enumeration = "delivery_rate_type_enum::DeliveryRateType", tag = "23")]
    pub delivery_rate_type: i32,
    /// Optional. The number here is either a percentage or an absolute value
    /// depending on the
    /// [discount_type][google.ads.admanager.v1.LineItem.discount_type]. If it is
    /// [PERCENTAGE][LineItemDiscountType.PERCENTAGE], then only non-fractional
    /// values are supported.
    #[prost(double, tag = "13")]
    pub discount: f64,
    /// Non-empty default. The type of discount applied to the line item. Defaults
    /// to [PERCENTAGE][LineItemDiscountType.PERCENTAGE].
    #[prost(
        enumeration = "line_item_discount_type_enum::LineItemDiscountType",
        tag = "24"
    )]
    pub discount_type: i32,
    /// Non-empty default. The environment that the line item is targeting. The
    /// default value is [BROWSER][EnvironmentType.BROWSER]. If this value is
    /// [VIDEO_PLAYER][EnvironmentType.VIDEO_PLAYER], then this line item can only
    /// target [AdUnits][google.ads.admanager.v1.AdUnit] that have `AdUnitSizes`
    /// whose `environment_type` is also `VIDEO_PLAYER`.
    #[prost(enumeration = "environment_type_enum::EnvironmentType", tag = "25")]
    pub environment_type: i32,
    /// Optional. Identifier for the LineItem that is meaningful to the publisher.
    /// This attribute has a maximum length of 255 characters.
    #[prost(string, tag = "5")]
    pub external_id: ::prost::alloc::string::String,
    /// Required. Time at which the LineItem will begin serving. This attribute
    /// must be in the future when creating a LineItem.
    #[prost(message, optional, tag = "6")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Time at which the LineItem will stop serving. This attribute is
    /// ignored when
    /// [unlimited_end_time][google.ads.admanager.v1.LineItem.unlimited_end_time]
    /// is `true`. If specified, it must be after
    /// [start_time][google.ads.admanager.v1.LineItem.start_time]. This end time
    /// does not include
    /// [auto_extension_days][google.ads.admanager.v1.LineItem.auto_extension_days].
    #[prost(message, optional, tag = "7")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Number of days to allow a LineItem to deliver past its
    /// [end_time][google.ads.admanager.v1.LineItem.end_time]. A maximum of 7 days
    /// is allowed. This feature is only available for Ad Manager 360 accounts.
    #[prost(int32, tag = "8")]
    pub auto_extension_days: i32,
    /// Optional. Whether the LineItem has an
    /// [end_time][google.ads.admanager.v1.LineItem.end_time]. This attribute can
    /// be set to `true` for only LineItems with
    /// [line_item_type][google.ads.admanager.v1.LineItem.line_item_type]
    /// [SPONSORSHIP][LineItemType.SPONSORSHIP], [NETWORK][LineItemType.NETWORK],
    /// [PRICE_PRIORITY][LineItemType.PRICE_PRIORITY] and
    /// [HOUSE][LineItemType.HOUSE].
    #[prost(bool, tag = "9")]
    pub unlimited_end_time: bool,
    /// Output only. The application that last modified this line item.
    #[prost(string, tag = "17")]
    pub last_modified_by_app: ::prost::alloc::string::String,
    /// Required. Determines the default priority of the LineItem for delivery.
    /// More information can be found on the [Ad Manager Help
    /// Center](<https://support.google.com/dfp_premium/answer/177279>).
    #[prost(enumeration = "line_item_type_enum::LineItemType", tag = "10")]
    pub line_item_type: i32,
    /// Output only. Indicates if a line item is missing any
    /// [creatives][google.ads.admanager.v1.Creative] for the
    /// [creative_placeholders][google.ads.admanager.v1.LineItem.creative_placeholders]
    /// specified.
    ///
    /// [Creatives][google.ads.admanager.v1.Creative] can be considered missing for
    /// several reasons:
    ///
    /// * Not enough [creatives][google.ads.admanager.v1.Creative] of a certain
    /// size have been uploaded,
    ///    as determined by
    ///    [expectedCreativeCount][google.ads.admanager.v1.CreativePlaceholder.expected_creative_count].
    ///    For example a line item specifies 750x350, 400x200, but only a 750x350
    ///    was uploaded. Or line item specifies 750x350 with an expected count of 2,
    ///    but only one was uploaded.
    /// * The [appliedLabels][Creative.applied_labels] of an associated
    /// [Creative][google.ads.admanager.v1.Creative]
    ///    do not match the
    ///    [effectiveAppliedLabels][CreativePlaceholder.effective_applied_labels] of
    ///    the line item. For example if a line item specifies 750x350 with a foo
    ///    applied label, but a 750x350 creative without an applied label was
    ///    uploaded.
    #[prost(bool, tag = "16")]
    pub missing_creatives: bool,
    /// Optional. Provides any additional notes that may annotate LineItem. This
    /// field has a maximum length of 65,535 characters.
    #[prost(string, tag = "20")]
    pub notes: ::prost::alloc::string::String,
    /// Optional. Priority of the LineItem for delivery. Valid values range from 1
    /// to 16. This field can only be changed by certain networks, otherwise a
    /// `PERMISSION_DENIED` error will occur.
    ///
    /// The following list shows the default, minimum, and maximum priority values
    /// for each [LineItemType][LineItemType]: formatted as `LineItemType`: default
    /// priority (minimum priority, maximum priority):
    ///
    /// * `SPONSORSHIP`: 4 (2,5)
    /// * `STANDARD`: 8 (6,10)
    /// * `NETWORK`: 12 (11, 14)
    /// * `BULK`: 12 (11, 14)
    /// * `PRICE_PRIORITY`: 12 (11, 14)
    /// * `HOUSE`: 16 (15, 16)
    /// * `CLICK_TRACKING`: 16 (1, 16)
    /// * `AD_EXCHANGE`: 12 (1, 16)
    /// * `ADSENSE`: 12 (1, 16)
    /// * `BUMPER`: 16 (15, 16)
    #[prost(int64, tag = "11")]
    pub priority: i64,
    /// Output only. Describes whether or not inventory has been reserved for the
    /// line item.
    #[prost(enumeration = "reservation_status_enum::ReservationStatus", tag = "26")]
    pub reservation_status: i32,
    /// Optional. The web property code used for dynamic allocation line items.
    /// This web property is only required with line item types
    /// [AD_EXCHANGE][LineItemType.AD_EXCHANGE] and
    /// [ADSENSE][LineItemType.ADSENSE].
    #[prost(string, tag = "21")]
    pub web_property_code: ::prost::alloc::string::String,
    /// Required. Details about the creatives that are expected to serve through
    /// this LineItem.
    #[prost(message, repeated, tag = "27")]
    pub creative_placeholders: ::prost::alloc::vec::Vec<CreativePlaceholder>,
    /// Output only. The status of the LineItem.
    #[prost(enumeration = "computed_status_enum::ComputedStatus", tag = "28")]
    pub status: i32,
    /// Required. The primary goal that this LineItem is associated with, which is
    /// used in its pacing and budgeting.
    #[prost(message, optional, tag = "29")]
    pub primary_goal: ::core::option::Option<Goal>,
    /// Optional. The impression limit for the LineItem. This field is meaningful
    /// only if the
    /// [LineItem.line_item_type][google.ads.admanager.v1.LineItem.line_item_type]
    /// is [LineItemType.SPONSORSHIP][] and
    /// [LineItem.cost_type][google.ads.admanager.v1.LineItem.cost_type] is
    /// [CostType.CPM][].
    #[prost(message, optional, tag = "30")]
    pub impression_limit: ::core::option::Option<Goal>,
}
/// Request object for GetLineItem method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLineItemRequest {
    /// Required. The resource name of the LineItem.
    /// Format:
    /// `networks/{network_code}/orders/{order_id}/lineItems/{line_item_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request object for ListLineItems method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLineItemsRequest {
    /// Required. The parent, which owns this collection of LineItems.
    /// Format: networks/{network_code}/orders/{order_id}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of LineItems to return. The service may return
    /// fewer than this value. If unspecified, at most 50 line items will be
    /// returned. The maximum value is 1000; values above 1000 will be coerced to
    /// 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListLineItems` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListLineItems` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Expression to filter the response.
    /// See syntax details at
    /// <https://developers.google.com/ad-manager/api/beta/filters>
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Expression to specify sorting order.
    /// See syntax details at
    /// <https://developers.google.com/ad-manager/api/beta/filters#order>
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
    /// Optional. Number of individual resources to skip while paginating.
    #[prost(int32, tag = "6")]
    pub skip: i32,
}
/// Response object for ListLineItemsRequest containing matching LineItem
/// resources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLineItemsResponse {
    /// The LineItem from the specified network.
    #[prost(message, repeated, tag = "1")]
    pub line_items: ::prost::alloc::vec::Vec<LineItem>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Total number of LineItems.
    /// If a filter was included in the request, this reflects the total number
    /// after the filtering is applied.
    ///
    /// `total_size` will not be calculated in the response unless it has been
    /// included in a response field mask. The response field mask can be provided
    /// to the method by using the URL parameter `$fields` or `fields`, or by using
    /// the HTTP/gRPC header `X-Goog-FieldMask`.
    ///
    /// For more information, see
    /// <https://developers.google.com/ad-manager/api/beta/field-masks>
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Generated client implementations.
pub mod line_item_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Provides methods for handling LineItem objects.
    #[derive(Debug, Clone)]
    pub struct LineItemServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl LineItemServiceClient<tonic::transport::Channel> {
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
    impl<T> LineItemServiceClient<T>
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
        ) -> LineItemServiceClient<InterceptedService<T, F>>
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
            LineItemServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// API to retrieve a LineItem object.
        pub async fn get_line_item(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLineItemRequest>,
        ) -> std::result::Result<tonic::Response<super::LineItem>, tonic::Status> {
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
                "/google.ads.admanager.v1.LineItemService/GetLineItem",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ads.admanager.v1.LineItemService",
                        "GetLineItem",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// API to retrieve a list of LineItem objects.
        pub async fn list_line_items(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLineItemsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListLineItemsResponse>,
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
                "/google.ads.admanager.v1.LineItemService/ListLineItems",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ads.admanager.v1.LineItemService",
                        "ListLineItems",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// The Network resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Network {
    /// Identifier. The resource name of the Network.
    /// Format: networks/{network_code}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Display name for Network.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. Network Code.
    #[prost(string, tag = "3")]
    pub network_code: ::prost::alloc::string::String,
    /// Output only. Property code.
    #[prost(string, tag = "4")]
    pub property_code: ::prost::alloc::string::String,
    /// Output only. Time zone associated with the delivery of orders and
    /// reporting.
    #[prost(string, tag = "5")]
    pub time_zone: ::prost::alloc::string::String,
    /// Output only. Primary currency code, in ISO-4217 format.
    #[prost(string, tag = "6")]
    pub currency_code: ::prost::alloc::string::String,
    /// Optional. Currency codes that can be used as an alternative to the primary
    /// currency code for trafficking Line Items.
    #[prost(string, repeated, tag = "7")]
    pub secondary_currency_codes: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// Output only. Top most [Ad Unit](google.ads.admanager.v1.AdUnit) to which
    /// descendant Ad Units can be added.
    /// Format: networks/{network_code}/adUnit/{ad_unit_id}
    #[prost(string, tag = "8")]
    pub effective_root_ad_unit: ::prost::alloc::string::String,
    /// Output only. Whether this is a test network.
    #[prost(bool, tag = "10")]
    pub test_network: bool,
    /// Output only. Network ID.
    #[prost(int64, tag = "11")]
    pub network_id: i64,
}
/// Request to get Network
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNetworkRequest {
    /// Required. Resource name of Network.
    /// Format: networks/{network_code}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod network_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Provides methods for handling Network objects.
    #[derive(Debug, Clone)]
    pub struct NetworkServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl NetworkServiceClient<tonic::transport::Channel> {
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
    impl<T> NetworkServiceClient<T>
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
        ) -> NetworkServiceClient<InterceptedService<T, F>>
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
            NetworkServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// API to retrieve a Network object.
        pub async fn get_network(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNetworkRequest>,
        ) -> std::result::Result<tonic::Response<super::Network>, tonic::Status> {
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
                "/google.ads.admanager.v1.NetworkService/GetNetwork",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ads.admanager.v1.NetworkService",
                        "GetNetwork",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// The `Order` resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Order {
    /// Identifier. The resource name of the `Order`.
    /// Format: `networks/{network_code}/orders/{order_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Order ID.
    #[prost(int64, tag = "4")]
    pub order_id: i64,
    /// Required. The display name of the Order.  This value is required to create
    /// an order and has a maximum length of 255 characters.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. Specifies whether or not the Order is a programmatic order.
    #[prost(bool, tag = "3")]
    pub programmatic: bool,
    /// Required. The resource name of the User responsible for trafficking the
    /// Order. Format: "networks/{network_code}/users/{user_id}"
    #[prost(string, tag = "23")]
    pub trafficker: ::prost::alloc::string::String,
    /// Optional. The resource names of Contacts from the advertiser of this Order.
    /// Format: "networks/{network_code}/contacts/{contact_id}"
    #[prost(string, repeated, tag = "5")]
    pub advertiser_contacts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. The resource name of the Company, which is of type
    /// Company.Type.ADVERTISER, to which this order belongs. This attribute is
    /// required. Format: "networks/{network_code}/companies/{company_id}"
    #[prost(string, tag = "6")]
    pub advertiser: ::prost::alloc::string::String,
    /// Optional. The resource names of Contacts from the advertising Agency of
    /// this Order. Format: "networks/{network_code}/contacts/{contact_id}"
    #[prost(string, repeated, tag = "7")]
    pub agency_contacts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. The resource name of the Company, which is of type
    /// Company.Type.AGENCY, with which this order is associated. Format:
    /// "networks/{network_code}/companies/{company_id}"
    #[prost(string, tag = "8")]
    pub agency: ::prost::alloc::string::String,
    /// Optional. The resource names of Teams directly applied to this Order.
    /// Format: "networks/{network_code}/teams/{team_id}"
    #[prost(string, repeated, tag = "9")]
    pub applied_teams: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. The resource names of Teams applied to this Order including
    /// inherited values. Format: "networks/{network_code}/teams/{team_id}"
    #[prost(string, repeated, tag = "28")]
    pub effective_teams: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. The resource name of the User who created the Order on behalf
    /// of the advertiser. This value is assigned by Google. Format:
    /// "networks/{network_code}/users/{user_id}"
    #[prost(string, tag = "10")]
    pub creator: ::prost::alloc::string::String,
    /// Output only. The ISO 4217 3-letter currency code for the currency used by
    /// the Order. This value is the network's currency code.
    #[prost(string, tag = "11")]
    pub currency_code: ::prost::alloc::string::String,
    /// Output only. The instant at which the Order and its associated line items
    /// are eligible to begin serving. This attribute is derived from the line item
    /// of the order that has the earliest LineItem.start_time.
    #[prost(message, optional, tag = "19")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The instant at which the Order and its associated line items
    /// stop being served. This attribute is derived from the line item of the
    /// order that has the latest LineItem.end_time.
    #[prost(message, optional, tag = "12")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. An arbitrary ID to associate to the Order, which can be used as a
    /// key to an external system.
    #[prost(int64, tag = "13")]
    pub external_order_id: i64,
    /// Output only. The archival status of the Order.
    #[prost(bool, tag = "14")]
    pub archived: bool,
    /// Output only. The application which modified this order. This attribute is
    /// assigned by Google.
    #[prost(string, tag = "15")]
    pub last_modified_by_app: ::prost::alloc::string::String,
    /// Output only. The instant this Order was last modified.
    #[prost(message, optional, tag = "16")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Provides any additional notes that may annotate the Order. This
    /// attribute has a maximum length of 65,535 characters.
    #[prost(string, tag = "17")]
    pub notes: ::prost::alloc::string::String,
    /// Optional. The purchase order number for the Order. This value has a maximum
    /// length of 63 characters.
    #[prost(string, tag = "18")]
    pub po_number: ::prost::alloc::string::String,
    /// Output only. The status of the Order.
    #[prost(enumeration = "order::Status", tag = "20")]
    pub status: i32,
    /// Optional. The resource name of the User responsible for the sales of the
    /// Order. Format: "networks/{network_code}/users/{user_id}"
    #[prost(string, tag = "21")]
    pub salesperson: ::prost::alloc::string::String,
    /// Optional. The resource names of the secondary salespeople associated with
    /// the order. Format: "networks/{network_code}/users/{user_id}"
    #[prost(string, repeated, tag = "22")]
    pub secondary_salespeople: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. The resource names of the secondary traffickers associated with
    /// the order. Format: "networks/{network_code}/users/{user_id}"
    #[prost(string, repeated, tag = "24")]
    pub secondary_traffickers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. The set of labels applied directly to this order.
    #[prost(message, repeated, tag = "25")]
    pub applied_labels: ::prost::alloc::vec::Vec<AppliedLabel>,
    /// Output only. Contains the set of labels applied directly to the order as
    /// well as those inherited from the company that owns the order. If a label
    /// has been negated, only the negated label is returned. This field is
    /// assigned by Google.
    #[prost(message, repeated, tag = "26")]
    pub effective_applied_labels: ::prost::alloc::vec::Vec<AppliedLabel>,
}
/// Nested message and enum types in `Order`.
pub mod order {
    /// The status of an Order.
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
    pub enum Status {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// Indicates that the Order has just been created but no approval has been
        /// requested yet.
        Draft = 2,
        /// Indicates that a request for approval for the Order has been made.
        PendingApproval = 3,
        /// Indicates that the Order has been approved and is ready to serve.
        Approved = 4,
        /// Indicates that the Order has been disapproved and is not eligible to
        /// serve.
        Disapproved = 5,
        /// This is a legacy state. Paused status should be checked on LineItems
        /// within the order.
        Paused = 6,
        /// Indicates that the Order has been canceled and cannot serve.
        Canceled = 7,
        /// Indicates that the Order has been deleted.
        Deleted = 8,
    }
    impl Status {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Status::Unspecified => "STATUS_UNSPECIFIED",
                Status::Draft => "DRAFT",
                Status::PendingApproval => "PENDING_APPROVAL",
                Status::Approved => "APPROVED",
                Status::Disapproved => "DISAPPROVED",
                Status::Paused => "PAUSED",
                Status::Canceled => "CANCELED",
                Status::Deleted => "DELETED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATUS_UNSPECIFIED" => Some(Self::Unspecified),
                "DRAFT" => Some(Self::Draft),
                "PENDING_APPROVAL" => Some(Self::PendingApproval),
                "APPROVED" => Some(Self::Approved),
                "DISAPPROVED" => Some(Self::Disapproved),
                "PAUSED" => Some(Self::Paused),
                "CANCELED" => Some(Self::Canceled),
                "DELETED" => Some(Self::Deleted),
                _ => None,
            }
        }
    }
}
/// Request object for `GetOrder` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrderRequest {
    /// Required. The resource name of the Order.
    /// Format: `networks/{network_code}/orders/{order_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request object for `ListOrders` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrdersRequest {
    /// Required. The parent, which owns this collection of Orders.
    /// Format: `networks/{network_code}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of `Orders` to return. The service may return
    /// fewer than this value. If unspecified, at most 50 `Orders` will be
    /// returned. The maximum value is 1000; values above 1000 will be coerced to
    /// 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListOrders` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListOrders` must match
    /// the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Expression to filter the response.
    /// See syntax details at
    /// <https://developers.google.com/ad-manager/api/beta/filters>
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Expression to specify sorting order.
    /// See syntax details at
    /// <https://developers.google.com/ad-manager/api/beta/filters#order>
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
    /// Optional. Number of individual resources to skip while paginating.
    #[prost(int32, tag = "6")]
    pub skip: i32,
}
/// Response object for `ListOrdersRequest` containing matching `Order`
/// resources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrdersResponse {
    /// The `Order` from the specified network.
    #[prost(message, repeated, tag = "1")]
    pub orders: ::prost::alloc::vec::Vec<Order>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Total number of `Orders`.
    /// If a filter was included in the request, this reflects the total number
    /// after the filtering is applied.
    ///
    /// `total_size` will not be calculated in the response unless it has been
    /// included in a response field mask. The response field mask can be provided
    /// to the method by using the URL parameter `$fields` or `fields`, or by using
    /// the HTTP/gRPC header `X-Goog-FieldMask`.
    ///
    /// For more information, see
    /// <https://developers.google.com/ad-manager/api/beta/field-masks>
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Generated client implementations.
pub mod order_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Provides methods for handling `Order` objects.
    #[derive(Debug, Clone)]
    pub struct OrderServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OrderServiceClient<tonic::transport::Channel> {
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
    impl<T> OrderServiceClient<T>
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
        ) -> OrderServiceClient<InterceptedService<T, F>>
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
            OrderServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// API to retrieve an Order object.
        pub async fn get_order(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrderRequest>,
        ) -> std::result::Result<tonic::Response<super::Order>, tonic::Status> {
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
                "/google.ads.admanager.v1.OrderService/GetOrder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.ads.admanager.v1.OrderService", "GetOrder"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// API to retrieve a list of `Order` objects.
        ///
        /// Fields used for literal matching in filter string:
        /// * `order_id`
        /// * `display_name`
        /// * `external_order_id`
        pub async fn list_orders(
            &mut self,
            request: impl tonic::IntoRequest<super::ListOrdersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListOrdersResponse>,
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
                "/google.ads.admanager.v1.OrderService/ListOrders",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.ads.admanager.v1.OrderService", "ListOrders"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Wrapper message for
/// [PlacementStatus][google.ads.admanager.v1.PlacementStatusEnum.PlacementStatus]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlacementStatusEnum {}
/// Nested message and enum types in `PlacementStatusEnum`.
pub mod placement_status_enum {
    /// Status of the placement.
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
    pub enum PlacementStatus {
        /// Not specified value.
        Unspecified = 0,
        /// Stats are collected, user-visible.
        Active = 1,
        /// No stats collected, not user-visible.
        Inactive = 2,
        /// No stats collected, user-visible.
        Archived = 3,
    }
    impl PlacementStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PlacementStatus::Unspecified => "PLACEMENT_STATUS_UNSPECIFIED",
                PlacementStatus::Active => "ACTIVE",
                PlacementStatus::Inactive => "INACTIVE",
                PlacementStatus::Archived => "ARCHIVED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PLACEMENT_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
                "ACTIVE" => Some(Self::Active),
                "INACTIVE" => Some(Self::Inactive),
                "ARCHIVED" => Some(Self::Archived),
                _ => None,
            }
        }
    }
}
/// The `Placement` resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Placement {
    /// Identifier. The resource name of the `Placement`.
    /// Format: `networks/{network_code}/placements/{placement_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. `Placement` ID.
    #[prost(int64, tag = "2")]
    pub placement_id: i64,
    /// Required. The display name of the placement. Its maximum length is 255
    /// characters.
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. A description of the Placement. This value is optional and its
    /// maximum length is 65,535 characters.
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    /// Output only. A string used to uniquely identify the Placement for purposes
    /// of serving the ad. This attribute is read-only and is assigned by Google
    /// when a placement is created.
    #[prost(string, tag = "5")]
    pub placement_code: ::prost::alloc::string::String,
    /// Output only. The status of the Placement. This attribute is read-only.
    #[prost(enumeration = "placement_status_enum::PlacementStatus", tag = "6")]
    pub status: i32,
    /// Optional. The resource names of AdUnits that constitute the Placement.
    /// Format: "networks/{network_code}/adUnits/{ad_unit_id}"
    #[prost(string, repeated, tag = "7")]
    pub targeted_ad_units: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. The instant this Placement was last modified.
    #[prost(message, optional, tag = "9")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request object for `GetPlacement` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPlacementRequest {
    /// Required. The resource name of the Placement.
    /// Format: `networks/{network_code}/placements/{placement_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request object for `ListPlacements` method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPlacementsRequest {
    /// Required. The parent, which owns this collection of Placements.
    /// Format: `networks/{network_code}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of `Placements` to return. The service may
    /// return fewer than this value. If unspecified, at most 50 `Placements` will
    /// be returned. The maximum value is 1000; values above 1000 will be coerced
    /// to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListPlacements` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListPlacements` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Expression to filter the response.
    /// See syntax details at
    /// <https://developers.google.com/ad-manager/api/beta/filters>
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Expression to specify sorting order.
    /// See syntax details at
    /// <https://developers.google.com/ad-manager/api/beta/filters#order>
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
    /// Optional. Number of individual resources to skip while paginating.
    #[prost(int32, tag = "6")]
    pub skip: i32,
}
/// Response object for `ListPlacementsRequest` containing matching `Placement`
/// objects.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPlacementsResponse {
    /// The `Placement` objects from the specified network.
    #[prost(message, repeated, tag = "1")]
    pub placements: ::prost::alloc::vec::Vec<Placement>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Total number of `Placement` objects.
    /// If a filter was included in the request, this reflects the total number
    /// after the filtering is applied.
    ///
    /// `total_size` will not be calculated in the response unless it has been
    /// included in a response field mask. The response field mask can be provided
    /// to the method by using the URL parameter `$fields` or `fields`, or by using
    /// the HTTP/gRPC header `X-Goog-FieldMask`.
    ///
    /// For more information, see
    /// <https://developers.google.com/ad-manager/api/beta/field-masks>
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Generated client implementations.
pub mod placement_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Provides methods for handling `Placement` objects.
    #[derive(Debug, Clone)]
    pub struct PlacementServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PlacementServiceClient<tonic::transport::Channel> {
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
    impl<T> PlacementServiceClient<T>
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
        ) -> PlacementServiceClient<InterceptedService<T, F>>
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
            PlacementServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// API to retrieve a `Placement` object.
        pub async fn get_placement(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPlacementRequest>,
        ) -> std::result::Result<tonic::Response<super::Placement>, tonic::Status> {
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
                "/google.ads.admanager.v1.PlacementService/GetPlacement",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ads.admanager.v1.PlacementService",
                        "GetPlacement",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// API to retrieve a list of `Placement` objects.
        pub async fn list_placements(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPlacementsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListPlacementsResponse>,
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
                "/google.ads.admanager.v1.PlacementService/ListPlacements",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ads.admanager.v1.PlacementService",
                        "ListPlacements",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// The Report resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Report {
    /// Identifier. The resource name of the Report.
    /// Report resource name have the form:
    /// `networks/{network_code}/reports/{report_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request proto for the configuration of a report run.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportSavedReportRequest {
    /// The name of a particular saved report resource.
    ///
    /// A report will be run based on the specification of this saved report.
    /// It must have the format of
    /// "networks/{network_code}/reports/{report_id}"
    #[prost(string, tag = "1")]
    pub report: ::prost::alloc::string::String,
    /// Required. The export format requested.
    #[prost(enumeration = "export_saved_report_request::Format", tag = "2")]
    pub format: i32,
    /// Whether or not to include the report properties (e.g. network, user, date
    /// generated...) in the generated report.
    #[prost(bool, tag = "3")]
    pub include_report_properties: bool,
    /// Whether or not to include the IDs if there are any (e.g. advertiser ID,
    ///   order ID...) present in the report.
    #[prost(bool, tag = "4")]
    pub include_ids: bool,
    /// Whether or not to include a row containing metric totals.
    #[prost(bool, tag = "5")]
    pub include_totals_row: bool,
    /// The file name of report download. The file extension is determined by
    /// export_format and gzip_compressed.
    ///
    /// Defaults to "DFP Report" if not specified.
    #[prost(string, tag = "6")]
    pub file_name: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ExportSavedReportRequest`.
pub mod export_saved_report_request {
    /// Supported file formats.
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
    pub enum Format {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// Comma separated values meant to be used by automated machine
        /// processing.
        ///
        /// Unlike other formats, the output is not localized and there is no
        /// totals row by default.
        CsvDump = 2,
        /// The report file is generated as an Office Open XML spreadsheet designed
        /// for Excel 2007+.
        Xlsx = 5,
        /// The report is generated as XML.
        Xml = 6,
    }
    impl Format {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Format::Unspecified => "FORMAT_UNSPECIFIED",
                Format::CsvDump => "CSV_DUMP",
                Format::Xlsx => "XLSX",
                Format::Xml => "XML",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FORMAT_UNSPECIFIED" => Some(Self::Unspecified),
                "CSV_DUMP" => Some(Self::CsvDump),
                "XLSX" => Some(Self::Xlsx),
                "XML" => Some(Self::Xml),
                _ => None,
            }
        }
    }
}
/// The message stored in the google.longrunning.Operation.metadata field.
/// Contains metadata regarding this execution.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportSavedReportMetadata {
    /// The result generated in this report run.
    #[prost(int64, tag = "1")]
    pub result_id: i64,
}
/// Message included in the longrunning Operation result.response field when
/// the report completes successfully.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportSavedReportResponse {
    /// The link to the exported file.
    #[prost(string, tag = "1")]
    pub download_url: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod report_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Provides methods for interacting with Reports.
    #[derive(Debug, Clone)]
    pub struct ReportServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ReportServiceClient<tonic::transport::Channel> {
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
    impl<T> ReportServiceClient<T>
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
        ) -> ReportServiceClient<InterceptedService<T, F>>
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
            ReportServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Initiates the execution and export of a report asynchronously. Users can
        /// get the report by polling this operation via
        /// OperationsService.GetOperation.
        /// Intervals of at least 2 seconds are recommended, with an exponential
        /// backoff. Once a report is complete, the operation will contain a
        /// ExportSavedReportResponse in its response field.
        pub async fn export_saved_report(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportSavedReportRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.ads.admanager.v1.ReportService/ExportSavedReport",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.ads.admanager.v1.ReportService",
                        "ExportSavedReport",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// The Role resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Role {
    /// Identifier. The resource name of the Role.
    /// Format: `networks/{network_code}/roles/{role_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request object for GetRole method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRoleRequest {
    /// Required. The resource name of the Role.
    /// Format: `networks/{network_code}/roles/{role_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request object for ListRoles method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRolesRequest {
    /// Required. The parent, which owns this collection of Roles.
    /// Format: `networks/{network_code}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of Roles to return. The service may return
    /// fewer than this value. If unspecified, at most 50 roles will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListRoles` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListRoles` must match
    /// the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Expression to filter the response.
    /// See syntax details at
    /// <https://developers.google.com/ad-manager/api/beta/filters>
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Expression to specify sorting order.
    /// See syntax details at
    /// <https://developers.google.com/ad-manager/api/beta/filters#order>
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
    /// Optional. Number of individual resources to skip while paginating.
    #[prost(int32, tag = "6")]
    pub skip: i32,
}
/// Response object for ListRolesRequest containing matching Role
/// resources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRolesResponse {
    /// The Role from the specified network.
    #[prost(message, repeated, tag = "1")]
    pub roles: ::prost::alloc::vec::Vec<Role>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Total number of Roles.
    /// If a filter was included in the request, this reflects the total number
    /// after the filtering is applied.
    ///
    /// `total_size` will not be calculated in the response unless it has been
    /// included in a response field mask. The response field mask can be provided
    /// to the method by using the URL parameter `$fields` or `fields`, or by using
    /// the HTTP/gRPC header `X-Goog-FieldMask`.
    ///
    /// For more information, see
    /// <https://developers.google.com/ad-manager/api/beta/field-masks>
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Generated client implementations.
pub mod role_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Provides methods for handling Role objects.
    #[derive(Debug, Clone)]
    pub struct RoleServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RoleServiceClient<tonic::transport::Channel> {
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
    impl<T> RoleServiceClient<T>
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
        ) -> RoleServiceClient<InterceptedService<T, F>>
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
            RoleServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// API to retrieve a Role object.
        pub async fn get_role(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRoleRequest>,
        ) -> std::result::Result<tonic::Response<super::Role>, tonic::Status> {
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
                "/google.ads.admanager.v1.RoleService/GetRole",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.ads.admanager.v1.RoleService", "GetRole"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// API to retrieve a list of Role objects.
        pub async fn list_roles(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRolesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRolesResponse>,
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
                "/google.ads.admanager.v1.RoleService/ListRoles",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.ads.admanager.v1.RoleService", "ListRoles"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// The Team resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Team {
    /// Identifier. The resource name of the Team.
    /// Format: `networks/{network_code}/teams/{team_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request object for GetTeam method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTeamRequest {
    /// Required. The resource name of the Team.
    /// Format: `networks/{network_code}/teams/{team_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request object for ListTeams method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTeamsRequest {
    /// Required. The parent, which owns this collection of Teams.
    /// Format: `networks/{network_code}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of Teams to return. The service may return
    /// fewer than this value. If unspecified, at most 50 teams will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListTeams` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListTeams` must match
    /// the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Expression to filter the response.
    /// See syntax details at
    /// <https://developers.google.com/ad-manager/api/beta/filters>
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Expression to specify sorting order.
    /// See syntax details at
    /// <https://developers.google.com/ad-manager/api/beta/filters#order>
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
    /// Optional. Number of individual resources to skip while paginating.
    #[prost(int32, tag = "6")]
    pub skip: i32,
}
/// Response object for ListTeamsRequest containing matching Team
/// resources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTeamsResponse {
    /// The Team from the specified network.
    #[prost(message, repeated, tag = "1")]
    pub teams: ::prost::alloc::vec::Vec<Team>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Total number of Teams.
    /// If a filter was included in the request, this reflects the total number
    /// after the filtering is applied.
    ///
    /// `total_size` will not be calculated in the response unless it has been
    /// included in a response field mask. The response field mask can be provided
    /// to the method by using the URL parameter `$fields` or `fields`, or by using
    /// the HTTP/gRPC header `X-Goog-FieldMask`.
    ///
    /// For more information, see
    /// <https://developers.google.com/ad-manager/api/beta/field-masks>
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Generated client implementations.
pub mod team_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Provides methods for handling Team objects.
    #[derive(Debug, Clone)]
    pub struct TeamServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TeamServiceClient<tonic::transport::Channel> {
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
    impl<T> TeamServiceClient<T>
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
        ) -> TeamServiceClient<InterceptedService<T, F>>
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
            TeamServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// API to retrieve a Team object.
        pub async fn get_team(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTeamRequest>,
        ) -> std::result::Result<tonic::Response<super::Team>, tonic::Status> {
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
                "/google.ads.admanager.v1.TeamService/GetTeam",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.ads.admanager.v1.TeamService", "GetTeam"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// API to retrieve a list of Team objects.
        pub async fn list_teams(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTeamsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListTeamsResponse>,
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
                "/google.ads.admanager.v1.TeamService/ListTeams",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.ads.admanager.v1.TeamService", "ListTeams"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// The User resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    /// Identifier. The resource name of the User.
    /// Format: `networks/{network_code}/users/{user_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. `User` ID.
    #[prost(int64, tag = "10")]
    pub user_id: i64,
    /// Required. The name of the User. It has a maximum length of 128 characters.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Required. The email or login of the User. In order to create a new user,
    /// you must already have a Google Account.
    #[prost(string, tag = "3")]
    pub email: ::prost::alloc::string::String,
    /// Required. The unique Role ID of the User. Roles that are created by Google
    /// will have negative IDs.
    #[prost(string, tag = "4")]
    pub role: ::prost::alloc::string::String,
    /// Output only. Specifies whether or not the User is active. An inactive user
    /// cannot log in to the system or perform any operations.
    #[prost(bool, tag = "6")]
    pub active: bool,
    /// Optional. An identifier for the User that is meaningful to the publisher.
    /// This attribute has a maximum length of 255 characters.
    #[prost(string, tag = "7")]
    pub external_id: ::prost::alloc::string::String,
    /// Output only. Whether the user is an OAuth2 service account user.
    /// Service account users can only be added through the UI.
    #[prost(bool, tag = "8")]
    pub service_account: bool,
    /// Optional. The IANA Time Zone Database time zone, e.g. "America/New_York",
    /// used in the orders and line items UI for this User. If not provided, the UI
    /// then defaults to using the Network's timezone. This setting only affects
    /// the UI for this user and does not affect the timezone of any dates and
    /// times returned in API responses.
    #[prost(string, tag = "9")]
    pub orders_ui_local_time_zone: ::prost::alloc::string::String,
}
/// Request object for GetUser method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserRequest {
    /// Required. The resource name of the User.
    /// Format: `networks/{network_code}/users/{user_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request object for ListUsers method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUsersRequest {
    /// Required. The parent, which owns this collection of Users.
    /// Format: `networks/{network_code}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of Users to return. The service may return
    /// fewer than this value. If unspecified, at most 50 users will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListUsers` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListUsers` must match
    /// the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Expression to filter the response.
    /// See syntax details at
    /// <https://developers.google.com/ad-manager/api/beta/filters>
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Expression to specify sorting order.
    /// See syntax details at
    /// <https://developers.google.com/ad-manager/api/beta/filters#order>
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
    /// Optional. Number of individual resources to skip while paginating.
    #[prost(int32, tag = "6")]
    pub skip: i32,
}
/// Response object for ListUsersRequest containing matching User resources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUsersResponse {
    /// The User from the specified network.
    #[prost(message, repeated, tag = "1")]
    pub users: ::prost::alloc::vec::Vec<User>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Total number of Users.
    /// If a filter was included in the request, this reflects the total number
    /// after the filtering is applied.
    ///
    /// `total_size` will not be calculated in the response unless it has been
    /// included in a response field mask. The response field mask can be provided
    /// to the method by using the URL parameter `$fields` or `fields`, or by using
    /// the HTTP/gRPC header `X-Goog-FieldMask`.
    ///
    /// For more information, see
    /// <https://developers.google.com/ad-manager/api/beta/field-masks>
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Generated client implementations.
pub mod user_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Provides methods for handling User objects.
    #[derive(Debug, Clone)]
    pub struct UserServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl UserServiceClient<tonic::transport::Channel> {
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
    impl<T> UserServiceClient<T>
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
        ) -> UserServiceClient<InterceptedService<T, F>>
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
            UserServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// API to retrieve a User object.
        pub async fn get_user(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserRequest>,
        ) -> std::result::Result<tonic::Response<super::User>, tonic::Status> {
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
                "/google.ads.admanager.v1.UserService/GetUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.ads.admanager.v1.UserService", "GetUser"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// API to retrieve a list of User objects.
        pub async fn list_users(
            &mut self,
            request: impl tonic::IntoRequest<super::ListUsersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListUsersResponse>,
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
                "/google.ads.admanager.v1.UserService/ListUsers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.ads.admanager.v1.UserService", "ListUsers"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
