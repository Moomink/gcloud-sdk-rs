/// A request to the SnapToRoads method, requesting that a sequence of points be
/// snapped to road segments.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapToRoadsRequest {
    /// The path to be snapped as a series of lat, lng points. Specified as
    /// a string of the format: lat,lng|lat,lng|...
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    /// Whether to interpolate the points to return full road geometry.
    #[prost(bool, tag = "2")]
    pub interpolate: bool,
    /// The asset ID of the asset to which this path relates. This is used for
    /// abuse detection purposes for clients with asset-based SKUs.
    #[prost(string, tag = "3")]
    pub asset_id: ::prost::alloc::string::String,
    /// The type of travel being tracked. This will constrain the paths we snap to.
    #[prost(enumeration = "TravelMode", tag = "4")]
    pub travel_mode: i32,
}
/// A snapped point object, representing the result of snapping.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnappedPoint {
    /// The lat,lng of the snapped location.
    #[prost(message, optional, tag = "1")]
    pub location: ::core::option::Option<super::super::super::r#type::LatLng>,
    /// The index into the original path of the equivalent pre-snapped point.
    /// This allows for identification of points which have been interpolated if
    /// this index is missing.
    #[prost(message, optional, tag = "2")]
    pub original_index: ::core::option::Option<u32>,
    /// The place ID for this snapped location (road segment). These are the same
    /// as are currently used by the Places API.
    #[prost(string, tag = "3")]
    pub place_id: ::prost::alloc::string::String,
}
/// The response from the SnapToRoads method, returning a sequence of snapped
/// points.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapToRoadsResponse {
    /// A list of snapped points.
    #[prost(message, repeated, tag = "1")]
    pub snapped_points: ::prost::alloc::vec::Vec<SnappedPoint>,
    /// User-visible warning message, if any, which can be shown alongside a valid
    /// result.
    #[prost(string, tag = "2")]
    pub warning_message: ::prost::alloc::string::String,
}
/// A request to the ListNearestRoads method, requesting that a sequence of
/// points be snapped individually to the road segment that each is closest to.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNearestRoadsRequest {
    /// The points to be snapped as a series of lat, lng points. Specified as
    /// a string of the format: lat,lng|lat,lng|...
    #[prost(string, tag = "1")]
    pub points: ::prost::alloc::string::String,
    /// The type of travel being tracked. This will constrain the roads we snap to.
    #[prost(enumeration = "TravelMode", tag = "2")]
    pub travel_mode: i32,
}
/// The response from the ListNearestRoads method, returning a list of snapped
/// points.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNearestRoadsResponse {
    /// A list of snapped points.
    #[prost(message, repeated, tag = "1")]
    pub snapped_points: ::prost::alloc::vec::Vec<SnappedPoint>,
}
/// An enum representing the mode of travel used for snapping.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TravelMode {
    Unspecified = 0,
    Driving = 1,
    Cycling = 2,
    Walking = 3,
}
impl TravelMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TravelMode::Unspecified => "TRAVEL_MODE_UNSPECIFIED",
            TravelMode::Driving => "DRIVING",
            TravelMode::Cycling => "CYCLING",
            TravelMode::Walking => "WALKING",
        }
    }
}
/// Generated client implementations.
pub mod roads_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct RoadsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RoadsServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> RoadsServiceClient<T>
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
        ) -> RoadsServiceClient<InterceptedService<T, F>>
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
            RoadsServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// This method takes a sequence of latitude,longitude points and snaps them to
        /// the most likely road segments. Optionally returns additional points giving
        /// the full road geometry. Also returns a place ID for each snapped point.
        pub async fn snap_to_roads(
            &mut self,
            request: impl tonic::IntoRequest<super::SnapToRoadsRequest>,
        ) -> Result<tonic::Response<super::SnapToRoadsResponse>, tonic::Status> {
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
                "/google.maps.roads.v1op.RoadsService/SnapToRoads",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// This method takes a list of latitude,longitude points and snaps them each
        /// to their nearest road. Also returns a place ID for each snapped point.
        pub async fn list_nearest_roads(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNearestRoadsRequest>,
        ) -> Result<tonic::Response<super::ListNearestRoadsResponse>, tonic::Status> {
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
                "/google.maps.roads.v1op.RoadsService/ListNearestRoads",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
