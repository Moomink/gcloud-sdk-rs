/// A notification object for notifying customers about security and privacy
/// issues.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Notification {
    /// The resource name of the notification.
    /// Format:
    /// organizations/{organization}/locations/{location}/notifications/{notification}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The subject line of the notification.
    #[prost(message, optional, tag = "2")]
    pub subject: ::core::option::Option<Subject>,
    /// A list of messages in the notification.
    #[prost(message, repeated, tag = "3")]
    pub messages: ::prost::alloc::vec::Vec<Message>,
    /// Output only. Time the notification was created.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Type of notification
    #[prost(enumeration = "NotificationType", tag = "12")]
    pub notification_type: i32,
}
/// A text object containing the English text and its localized copies.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Text {
    /// The English copy.
    #[prost(string, tag = "1")]
    pub en_text: ::prost::alloc::string::String,
    /// The requested localized copy (if applicable).
    #[prost(string, tag = "2")]
    pub localized_text: ::prost::alloc::string::String,
    /// Status of the localization.
    #[prost(enumeration = "LocalizationState", tag = "3")]
    pub localization_state: i32,
}
/// A subject line of a notification.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Subject {
    /// The text content.
    #[prost(message, optional, tag = "1")]
    pub text: ::core::option::Option<Text>,
}
/// A message which contains notification details.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    /// The message content.
    #[prost(message, optional, tag = "1")]
    pub body: ::core::option::Option<message::Body>,
    /// The attachments to download.
    #[prost(message, repeated, tag = "2")]
    pub attachments: ::prost::alloc::vec::Vec<Attachment>,
    /// The Message creation timestamp.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Time when Message was localized
    #[prost(message, optional, tag = "4")]
    pub localization_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `Message`.
pub mod message {
    /// A message body containing text.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Body {
        /// The text content of the message body.
        #[prost(message, optional, tag = "1")]
        pub text: ::core::option::Option<super::Text>,
    }
}
/// Attachment with specific information about the issue.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Attachment {
    /// The title of the attachment.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// Data type of the attachment.
    #[prost(oneof = "attachment::Data", tags = "2")]
    pub data: ::core::option::Option<attachment::Data>,
}
/// Nested message and enum types in `Attachment`.
pub mod attachment {
    /// Data type of the attachment.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        /// A CSV file attachment. Max size is 10 MB.
        #[prost(message, tag = "2")]
        Csv(super::Csv),
    }
}
/// A representation of a CSV file attachment, as a list of column headers and
/// a list of data rows.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Csv {
    /// The list of headers for data columns in a CSV file.
    #[prost(string, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The list of data rows in a CSV file, as string arrays rather than as a
    /// single comma-separated string.
    #[prost(message, repeated, tag = "2")]
    pub data_rows: ::prost::alloc::vec::Vec<csv::CsvRow>,
}
/// Nested message and enum types in `Csv`.
pub mod csv {
    /// A representation of a single data row in a CSV file.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CsvRow {
        /// The data entries in a CSV file row, as a string array rather than a
        /// single comma-separated string.
        #[prost(string, repeated, tag = "1")]
        pub entries: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
/// Request for fetching all notifications for a given parent.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNotificationsRequest {
    /// Required. The parent, which owns this collection of notifications.
    /// Must be of the form "organizations/{organization}/locations/{location}".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of notifications to return. The service may return
    /// fewer than this value. If unspecified or equal to 0, at most 50
    /// notifications will be returned. The maximum value is 50; values above 50
    /// will be coerced to 50.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token returned from a previous request.
    /// When paginating, all other parameters provided in the request
    /// must match the call that returned the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Specifies which parts of the notification resource should be returned
    /// in the response.
    #[prost(enumeration = "NotificationView", tag = "4")]
    pub view: i32,
    /// ISO code for requested localization language.  If unset, will be
    /// interpereted as "en". If the requested language is valid, but not supported
    /// for this notification, English will be returned with an "Not applicable"
    /// LocalizationState. If the ISO code is invalid (i.e. not a real language),
    /// this RPC will throw an error.
    #[prost(string, tag = "5")]
    pub language_code: ::prost::alloc::string::String,
}
/// Response of ListNotifications endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNotificationsResponse {
    /// List of notifications under a given parent.
    #[prost(message, repeated, tag = "1")]
    pub notifications: ::prost::alloc::vec::Vec<Notification>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Estimation of a total number of notifications.
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Request for fetching a notification.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNotificationRequest {
    /// Required. A name of the notification to retrieve.
    /// Format:
    /// organizations/{organization}/locations/{location}/notifications/{notification}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// ISO code for requested localization language. If unset, will be
    /// interpereted as "en". If the requested language is valid, but not supported
    /// for this notification, English will be returned with an "Not applicable"
    /// LocalizationState. If the ISO code is invalid (i.e. not a real language),
    /// this RPC will throw an error.
    #[prost(string, tag = "5")]
    pub language_code: ::prost::alloc::string::String,
}
/// Settings for Advisory Notifications.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Settings {
    /// Output only. The resource name of the settings to retrieve.
    /// Format:
    /// organizations/{organization}/locations/{location}/settings.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Map of each notification type and its settings to get/set all
    /// settings at once. The server will validate the value for each notification
    /// type.
    #[prost(map = "string, message", tag = "2")]
    pub notification_settings: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        NotificationSettings,
    >,
    /// Required. Fingerprint for optimistic concurrency returned in Get requests.
    /// Must be provided for Update requests. If the value provided does not match
    /// the value known to the server, ABORTED will be thrown, and the client
    /// should retry the read-modify-write cycle.
    #[prost(string, tag = "3")]
    pub etag: ::prost::alloc::string::String,
}
/// Settings for each NotificationType.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationSettings {
    /// Whether the associated NotificationType is enabled.
    #[prost(bool, tag = "1")]
    pub enabled: bool,
}
/// Request of GetSettings endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSettingsRequest {
    /// Required. The resource name of the settings to retrieve.
    /// Format:
    /// organizations/{organization}/locations/{location}/settings.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request of UpdateSettings endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSettingsRequest {
    /// Required. New settings.
    #[prost(message, optional, tag = "1")]
    pub settings: ::core::option::Option<Settings>,
}
/// Notification view.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NotificationView {
    /// Not specified, equivalent to BASIC.
    Unspecified = 0,
    /// Server responses only include title, creation time and Notification ID.
    /// Note: for internal use responses also include the last update time,
    /// the latest message text and whether notification has attachments.
    Basic = 1,
    /// Include everything.
    Full = 2,
}
impl NotificationView {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            NotificationView::Unspecified => "NOTIFICATION_VIEW_UNSPECIFIED",
            NotificationView::Basic => "BASIC",
            NotificationView::Full => "FULL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NOTIFICATION_VIEW_UNSPECIFIED" => Some(Self::Unspecified),
            "BASIC" => Some(Self::Basic),
            "FULL" => Some(Self::Full),
            _ => None,
        }
    }
}
/// Status of localized text.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LocalizationState {
    /// Not used.
    Unspecified = 0,
    /// Localization is not applicable for requested language. This can happen
    /// when:
    /// - The requested language was not supported by Advisory Notifications at the
    /// time of localization (including notifications created before the
    /// localization feature was launched).
    /// - The requested language is English, so only the English text is returned.
    NotApplicable = 1,
    /// Localization for requested language is in progress, and not ready yet.
    Pending = 2,
    /// Localization for requested language is completed.
    Completed = 3,
}
impl LocalizationState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LocalizationState::Unspecified => "LOCALIZATION_STATE_UNSPECIFIED",
            LocalizationState::NotApplicable => "LOCALIZATION_STATE_NOT_APPLICABLE",
            LocalizationState::Pending => "LOCALIZATION_STATE_PENDING",
            LocalizationState::Completed => "LOCALIZATION_STATE_COMPLETED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LOCALIZATION_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "LOCALIZATION_STATE_NOT_APPLICABLE" => Some(Self::NotApplicable),
            "LOCALIZATION_STATE_PENDING" => Some(Self::Pending),
            "LOCALIZATION_STATE_COMPLETED" => Some(Self::Completed),
            _ => None,
        }
    }
}
/// Type of notification
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NotificationType {
    /// Default type
    Unspecified = 0,
    /// Security and privacy advisory notifications
    SecurityPrivacyAdvisory = 1,
    /// Sensitive action notifications
    SensitiveActions = 2,
    /// General security MSA
    SecurityMsa = 3,
    /// Threat horizons MSA
    ThreatHorizons = 4,
}
impl NotificationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            NotificationType::Unspecified => "NOTIFICATION_TYPE_UNSPECIFIED",
            NotificationType::SecurityPrivacyAdvisory => {
                "NOTIFICATION_TYPE_SECURITY_PRIVACY_ADVISORY"
            }
            NotificationType::SensitiveActions => "NOTIFICATION_TYPE_SENSITIVE_ACTIONS",
            NotificationType::SecurityMsa => "NOTIFICATION_TYPE_SECURITY_MSA",
            NotificationType::ThreatHorizons => "NOTIFICATION_TYPE_THREAT_HORIZONS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NOTIFICATION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "NOTIFICATION_TYPE_SECURITY_PRIVACY_ADVISORY" => {
                Some(Self::SecurityPrivacyAdvisory)
            }
            "NOTIFICATION_TYPE_SENSITIVE_ACTIONS" => Some(Self::SensitiveActions),
            "NOTIFICATION_TYPE_SECURITY_MSA" => Some(Self::SecurityMsa),
            "NOTIFICATION_TYPE_THREAT_HORIZONS" => Some(Self::ThreatHorizons),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod advisory_notifications_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service to manage Security and Privacy Notifications.
    #[derive(Debug, Clone)]
    pub struct AdvisoryNotificationsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AdvisoryNotificationsServiceClient<tonic::transport::Channel> {
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
    impl<T> AdvisoryNotificationsServiceClient<T>
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
        ) -> AdvisoryNotificationsServiceClient<InterceptedService<T, F>>
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
            AdvisoryNotificationsServiceClient::new(
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
        /// Lists notifications under a given parent.
        pub async fn list_notifications(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNotificationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListNotificationsResponse>,
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
                "/google.cloud.advisorynotifications.v1.AdvisoryNotificationsService/ListNotifications",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.advisorynotifications.v1.AdvisoryNotificationsService",
                        "ListNotifications",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets a notification.
        pub async fn get_notification(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNotificationRequest>,
        ) -> std::result::Result<tonic::Response<super::Notification>, tonic::Status> {
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
                "/google.cloud.advisorynotifications.v1.AdvisoryNotificationsService/GetNotification",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.advisorynotifications.v1.AdvisoryNotificationsService",
                        "GetNotification",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get notification settings.
        pub async fn get_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSettingsRequest>,
        ) -> std::result::Result<tonic::Response<super::Settings>, tonic::Status> {
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
                "/google.cloud.advisorynotifications.v1.AdvisoryNotificationsService/GetSettings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.advisorynotifications.v1.AdvisoryNotificationsService",
                        "GetSettings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Update notification settings.
        pub async fn update_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSettingsRequest>,
        ) -> std::result::Result<tonic::Response<super::Settings>, tonic::Status> {
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
                "/google.cloud.advisorynotifications.v1.AdvisoryNotificationsService/UpdateSettings",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.advisorynotifications.v1.AdvisoryNotificationsService",
                        "UpdateSettings",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
