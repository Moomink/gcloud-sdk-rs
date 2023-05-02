/// The request message for the PublishChannelConnectionEvents method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublishChannelConnectionEventsRequest {
    /// The channel_connection that the events are published from. For example:
    /// `projects/{partner_project_id}/locations/{location}/channelConnections/{channel_connection_id}`.
    #[prost(string, tag = "1")]
    pub channel_connection: ::prost::alloc::string::String,
    /// The CloudEvents v1.0 events to publish. No other types are allowed.
    /// If this field is set, then the `text_events` fields must not be set.
    #[prost(message, repeated, tag = "2")]
    pub events: ::prost::alloc::vec::Vec<::prost_types::Any>,
    /// The text representation of events to publish.
    /// CloudEvent v1.0 in JSON format is the only allowed type. Refer to
    /// <https://github.com/cloudevents/spec/blob/v1.0.2/cloudevents/formats/json-format.md>
    /// for specification.
    /// If this field is set, then the `events` fields must not be set.
    #[prost(string, repeated, tag = "3")]
    pub text_events: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The response message for the PublishChannelConnectionEvents method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublishChannelConnectionEventsResponse {}
/// The request message for the PublishEvents method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublishEventsRequest {
    /// The full name of the channel to publish to. For example:
    /// `projects/{project}/locations/{location}/channels/{channel-id}`.
    #[prost(string, tag = "1")]
    pub channel: ::prost::alloc::string::String,
    /// The CloudEvents v1.0 events to publish. No other types are allowed.
    /// If this field is set, then the `text_events` fields must not be set.
    #[prost(message, repeated, tag = "2")]
    pub events: ::prost::alloc::vec::Vec<::prost_types::Any>,
    /// The text representation of events to publish.
    /// CloudEvent v1.0 in JSON format is the only allowed type. Refer to
    /// <https://github.com/cloudevents/spec/blob/v1.0.2/cloudevents/formats/json-format.md>
    /// for specification.
    /// If this field is set, then the `events` fields must not be set.
    #[prost(string, repeated, tag = "3")]
    pub text_events: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The response message for the PublishEvents method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublishEventsResponse {}
/// Generated client implementations.
pub mod publisher_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Eventarc processes events generated by an event provider and delivers them to
    /// a subscriber.
    ///
    /// An event provider is a software-as-a-service (SaaS) system or
    /// product that can generate and deliver events through Eventarc.
    ///
    /// A third-party event provider is an event provider from outside of Google.
    ///
    /// A partner is a third-party event provider that is integrated with Eventarc.
    ///
    /// A subscriber is a GCP customer interested in receiving events.
    ///
    /// Channel is a first-class Eventarc resource that is created and managed
    /// by the subscriber in their GCP project. A Channel represents a subscriber's
    /// intent to receive events from an event provider. A Channel is associated with
    /// exactly one event provider.
    ///
    /// ChannelConnection is a first-class Eventarc resource that
    /// is created and managed by the partner in their GCP project. A
    /// ChannelConnection represents a connection between a partner and a
    /// subscriber's Channel. A ChannelConnection has a one-to-one mapping with a
    /// Channel.
    ///
    /// Publisher allows an event provider to publish events to Eventarc.
    #[derive(Debug, Clone)]
    pub struct PublisherClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PublisherClient<tonic::transport::Channel> {
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
    impl<T> PublisherClient<T>
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
        ) -> PublisherClient<InterceptedService<T, F>>
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
            PublisherClient::new(InterceptedService::new(inner, interceptor))
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
        /// Publish events to a ChannelConnection in a partner's project.
        pub async fn publish_channel_connection_events(
            &mut self,
            request: impl tonic::IntoRequest<
                super::PublishChannelConnectionEventsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::PublishChannelConnectionEventsResponse>,
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
                "/google.cloud.eventarc.publishing.v1.Publisher/PublishChannelConnectionEvents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.eventarc.publishing.v1.Publisher",
                        "PublishChannelConnectionEvents",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Publish events to a subscriber's channel.
        pub async fn publish_events(
            &mut self,
            request: impl tonic::IntoRequest<super::PublishEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PublishEventsResponse>,
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
                "/google.cloud.eventarc.publishing.v1.Publisher/PublishEvents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.eventarc.publishing.v1.Publisher",
                        "PublishEvents",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
