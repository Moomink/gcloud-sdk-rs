/// Specifies how to process the `ConverseRequest` messages.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConverseConfig {
    /// *Required* Specifies how to process the subsequent incoming audio.
    #[prost(message, optional, tag = "1")]
    pub audio_in_config: ::core::option::Option<AudioInConfig>,
    /// *Required* Specifies how to format the audio that will be returned.
    #[prost(message, optional, tag = "2")]
    pub audio_out_config: ::core::option::Option<AudioOutConfig>,
    /// *Required* Represents the current dialog state.
    #[prost(message, optional, tag = "3")]
    pub converse_state: ::core::option::Option<ConverseState>,
}
/// Specifies how to process the `audio_in` data that will be provided in
/// subsequent requests. For recommended settings, see the Google Assistant SDK
/// [best
/// practices](<https://developers.google.com/assistant/sdk/develop/grpc/best-practices/audio>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudioInConfig {
    /// *Required* Encoding of audio data sent in all `audio_in` messages.
    #[prost(enumeration = "audio_in_config::Encoding", tag = "1")]
    pub encoding: i32,
    /// *Required* Sample rate (in Hertz) of the audio data sent in all `audio_in`
    /// messages. Valid values are from 16000-24000, but 16000 is optimal.
    /// For best results, set the sampling rate of the audio source to 16000 Hz.
    /// If that's not possible, use the native sample rate of the audio source
    /// (instead of re-sampling).
    #[prost(int32, tag = "2")]
    pub sample_rate_hertz: i32,
}
/// Nested message and enum types in `AudioInConfig`.
pub mod audio_in_config {
    /// Audio encoding of the data sent in the audio message.
    /// Audio must be one-channel (mono). The only language supported is "en-US".
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
    pub enum Encoding {
        /// Not specified. Will return result \[google.rpc.Code.INVALID_ARGUMENT][\].
        Unspecified = 0,
        /// Uncompressed 16-bit signed little-endian samples (Linear PCM).
        /// This encoding includes no header, only the raw audio bytes.
        Linear16 = 1,
        /// \[`FLAC`\](<https://xiph.org/flac/documentation.html>) (Free Lossless Audio
        /// Codec) is the recommended encoding because it is
        /// lossless--therefore recognition is not compromised--and
        /// requires only about half the bandwidth of `LINEAR16`. This encoding
        /// includes the `FLAC` stream header followed by audio data. It supports
        /// 16-bit and 24-bit samples, however, not all fields in `STREAMINFO` are
        /// supported.
        Flac = 2,
    }
    impl Encoding {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Encoding::Unspecified => "ENCODING_UNSPECIFIED",
                Encoding::Linear16 => "LINEAR16",
                Encoding::Flac => "FLAC",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ENCODING_UNSPECIFIED" => Some(Self::Unspecified),
                "LINEAR16" => Some(Self::Linear16),
                "FLAC" => Some(Self::Flac),
                _ => None,
            }
        }
    }
}
/// Specifies the desired format for the server to use when it returns
/// `audio_out` messages.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudioOutConfig {
    /// *Required* The encoding of audio data to be returned in all `audio_out`
    /// messages.
    #[prost(enumeration = "audio_out_config::Encoding", tag = "1")]
    pub encoding: i32,
    /// *Required* The sample rate in Hertz of the audio data returned in
    /// `audio_out` messages. Valid values are: 16000-24000.
    #[prost(int32, tag = "2")]
    pub sample_rate_hertz: i32,
    /// *Required* Current volume setting of the device's audio output.
    /// Valid values are 1 to 100 (corresponding to 1% to 100%).
    #[prost(int32, tag = "3")]
    pub volume_percentage: i32,
}
/// Nested message and enum types in `AudioOutConfig`.
pub mod audio_out_config {
    /// Audio encoding of the data returned in the audio message. All encodings are
    /// raw audio bytes with no header, except as indicated below.
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
    pub enum Encoding {
        /// Not specified. Will return result \[google.rpc.Code.INVALID_ARGUMENT][\].
        Unspecified = 0,
        /// Uncompressed 16-bit signed little-endian samples (Linear PCM).
        Linear16 = 1,
        /// MP3 audio encoding. The sample rate is encoded in the payload.
        Mp3 = 2,
        /// Opus-encoded audio wrapped in an ogg container. The result will be a
        /// file which can be played natively on Android and in some browsers (such
        /// as Chrome). The quality of the encoding is considerably higher than MP3
        /// while using the same bitrate. The sample rate is encoded in the payload.
        OpusInOgg = 3,
    }
    impl Encoding {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Encoding::Unspecified => "ENCODING_UNSPECIFIED",
                Encoding::Linear16 => "LINEAR16",
                Encoding::Mp3 => "MP3",
                Encoding::OpusInOgg => "OPUS_IN_OGG",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ENCODING_UNSPECIFIED" => Some(Self::Unspecified),
                "LINEAR16" => Some(Self::Linear16),
                "MP3" => Some(Self::Mp3),
                "OPUS_IN_OGG" => Some(Self::OpusInOgg),
                _ => None,
            }
        }
    }
}
/// Provides information about the current dialog state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConverseState {
    /// *Required* The `conversation_state` value returned in the prior
    /// `ConverseResponse`. Omit (do not set the field) if there was no prior
    /// `ConverseResponse`. If there was a prior `ConverseResponse`, do not omit
    /// this field; doing so will end that conversation (and this new request will
    /// start a new conversation).
    #[prost(bytes = "vec", tag = "1")]
    pub conversation_state: ::prost::alloc::vec::Vec<u8>,
}
/// The audio containing the assistant's response to the query. Sequential chunks
/// of audio data are received in sequential `ConverseResponse` messages.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudioOut {
    /// *Output-only* The audio data containing the assistant's response to the
    /// query. Sequential chunks of audio data are received in sequential
    /// `ConverseResponse` messages.
    #[prost(bytes = "vec", tag = "1")]
    pub audio_data: ::prost::alloc::vec::Vec<u8>,
}
/// The semantic result for the user's spoken query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConverseResult {
    /// *Output-only* The recognized transcript of what the user said.
    #[prost(string, tag = "1")]
    pub spoken_request_text: ::prost::alloc::string::String,
    /// *Output-only* The text of the assistant's spoken response. This is only
    /// returned for an IFTTT action.
    #[prost(string, tag = "2")]
    pub spoken_response_text: ::prost::alloc::string::String,
    /// *Output-only* State information for subsequent `ConverseRequest`. This
    /// value should be saved in the client and returned in the
    /// `conversation_state` with the next `ConverseRequest`. (The client does not
    /// need to interpret or otherwise use this value.) There is no need to save
    /// this information across device restarts.
    #[prost(bytes = "vec", tag = "3")]
    pub conversation_state: ::prost::alloc::vec::Vec<u8>,
    /// *Output-only* Specifies the mode of the microphone after this `Converse`
    /// RPC is processed.
    #[prost(enumeration = "converse_result::MicrophoneMode", tag = "4")]
    pub microphone_mode: i32,
    /// *Output-only* Updated volume level. The value will be 0 or omitted
    /// (indicating no change) unless a voice command such as "Increase the volume"
    /// or "Set volume level 4" was recognized, in which case the value will be
    /// between 1 and 100 (corresponding to the new volume level of 1% to 100%).
    /// Typically, a client should use this volume level when playing the
    /// `audio_out` data, and retain this value as the current volume level and
    /// supply it in the `AudioOutConfig` of the next `ConverseRequest`. (Some
    /// clients may also implement other ways to allow the current volume level to
    /// be changed, for example, by providing a knob that the user can turn.)
    #[prost(int32, tag = "5")]
    pub volume_percentage: i32,
}
/// Nested message and enum types in `ConverseResult`.
pub mod converse_result {
    /// Possible states of the microphone after a `Converse` RPC completes.
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
    pub enum MicrophoneMode {
        /// No mode specified.
        Unspecified = 0,
        /// The service is not expecting a follow-on question from the user.
        /// The microphone should remain off until the user re-activates it.
        CloseMicrophone = 1,
        /// The service is expecting a follow-on question from the user. The
        /// microphone should be re-opened when the `AudioOut` playback completes
        /// (by starting a new `Converse` RPC call to send the new audio).
        DialogFollowOn = 2,
    }
    impl MicrophoneMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MicrophoneMode::Unspecified => "MICROPHONE_MODE_UNSPECIFIED",
                MicrophoneMode::CloseMicrophone => "CLOSE_MICROPHONE",
                MicrophoneMode::DialogFollowOn => "DIALOG_FOLLOW_ON",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MICROPHONE_MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "CLOSE_MICROPHONE" => Some(Self::CloseMicrophone),
                "DIALOG_FOLLOW_ON" => Some(Self::DialogFollowOn),
                _ => None,
            }
        }
    }
}
/// The top-level message sent by the client. Clients must send at least two, and
/// typically numerous `ConverseRequest` messages. The first message must
/// contain a `config` message and must not contain `audio_in` data. All
/// subsequent messages must contain `audio_in` data and must not contain a
/// `config` message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConverseRequest {
    /// Exactly one of these fields must be specified in each `ConverseRequest`.
    #[prost(oneof = "converse_request::ConverseRequest", tags = "1, 2")]
    pub converse_request: ::core::option::Option<converse_request::ConverseRequest>,
}
/// Nested message and enum types in `ConverseRequest`.
pub mod converse_request {
    /// Exactly one of these fields must be specified in each `ConverseRequest`.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConverseRequest {
        /// The `config` message provides information to the recognizer that
        /// specifies how to process the request.
        /// The first `ConverseRequest` message must contain a `config` message.
        #[prost(message, tag = "1")]
        Config(super::ConverseConfig),
        /// The audio data to be recognized. Sequential chunks of audio data are sent
        /// in sequential `ConverseRequest` messages. The first `ConverseRequest`
        /// message must not contain `audio_in` data and all subsequent
        /// `ConverseRequest` messages must contain `audio_in` data. The audio bytes
        /// must be encoded as specified in `AudioInConfig`.
        /// Audio must be sent at approximately real-time (16000 samples per second).
        /// An error will be returned if audio is sent significantly faster or
        /// slower.
        #[prost(bytes, tag = "2")]
        AudioIn(::prost::alloc::vec::Vec<u8>),
    }
}
/// The top-level message received by the client. A series of one or more
/// `ConverseResponse` messages are streamed back to the client.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConverseResponse {
    /// Exactly one of these fields will be populated in each `ConverseResponse`.
    #[prost(oneof = "converse_response::ConverseResponse", tags = "1, 2, 3, 5")]
    pub converse_response: ::core::option::Option<converse_response::ConverseResponse>,
}
/// Nested message and enum types in `ConverseResponse`.
pub mod converse_response {
    /// Indicates the type of event.
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
    pub enum EventType {
        /// No event specified.
        Unspecified = 0,
        /// This event indicates that the server has detected the end of the user's
        /// speech utterance and expects no additional speech. Therefore, the server
        /// will not process additional audio (although it may subsequently return
        /// additional results). The client should stop sending additional audio
        /// data, half-close the gRPC connection, and wait for any additional results
        /// until the server closes the gRPC connection.
        EndOfUtterance = 1,
    }
    impl EventType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EventType::Unspecified => "EVENT_TYPE_UNSPECIFIED",
                EventType::EndOfUtterance => "END_OF_UTTERANCE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "EVENT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "END_OF_UTTERANCE" => Some(Self::EndOfUtterance),
                _ => None,
            }
        }
    }
    /// Exactly one of these fields will be populated in each `ConverseResponse`.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConverseResponse {
        /// *Output-only* If set, returns a \[google.rpc.Status][google.rpc.Status\]
        /// message that specifies the error for the operation. If an error occurs
        /// during processing, this message will be set and there will be no further
        /// messages sent.
        #[prost(message, tag = "1")]
        Error(super::super::super::super::rpc::Status),
        /// *Output-only* Indicates the type of event.
        #[prost(enumeration = "EventType", tag = "2")]
        EventType(i32),
        /// *Output-only* The audio containing the assistant's response to the query.
        #[prost(message, tag = "3")]
        AudioOut(super::AudioOut),
        /// *Output-only* The semantic result for the user's spoken query.
        #[prost(message, tag = "5")]
        Result(super::ConverseResult),
    }
}
/// Generated client implementations.
pub mod embedded_assistant_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service that implements Google Assistant API.
    #[derive(Debug, Clone)]
    pub struct EmbeddedAssistantClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl EmbeddedAssistantClient<tonic::transport::Channel> {
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
    impl<T> EmbeddedAssistantClient<T>
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
        ) -> EmbeddedAssistantClient<InterceptedService<T, F>>
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
            EmbeddedAssistantClient::new(InterceptedService::new(inner, interceptor))
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
        /// Initiates or continues a conversation with the embedded assistant service.
        /// Each call performs one round-trip, sending an audio request to the service
        /// and receiving the audio response. Uses bidirectional streaming to receive
        /// results, such as the `END_OF_UTTERANCE` event, while sending audio.
        ///
        /// A conversation is one or more gRPC connections, each consisting of several
        /// streamed requests and responses.
        /// For example, the user says *Add to my shopping list* and the assistant
        /// responds *What do you want to add?*. The sequence of streamed requests and
        /// responses in the first gRPC message could be:
        ///
        /// *   ConverseRequest.config
        /// *   ConverseRequest.audio_in
        /// *   ConverseRequest.audio_in
        /// *   ConverseRequest.audio_in
        /// *   ConverseRequest.audio_in
        /// *   ConverseResponse.event_type.END_OF_UTTERANCE
        /// *   ConverseResponse.result.microphone_mode.DIALOG_FOLLOW_ON
        /// *   ConverseResponse.audio_out
        /// *   ConverseResponse.audio_out
        /// *   ConverseResponse.audio_out
        ///
        /// The user then says *bagels* and the assistant responds
        /// *OK, I've added bagels to your shopping list*. This is sent as another gRPC
        /// connection call to the `Converse` method, again with streamed requests and
        /// responses, such as:
        ///
        /// *   ConverseRequest.config
        /// *   ConverseRequest.audio_in
        /// *   ConverseRequest.audio_in
        /// *   ConverseRequest.audio_in
        /// *   ConverseResponse.event_type.END_OF_UTTERANCE
        /// *   ConverseResponse.result.microphone_mode.CLOSE_MICROPHONE
        /// *   ConverseResponse.audio_out
        /// *   ConverseResponse.audio_out
        /// *   ConverseResponse.audio_out
        /// *   ConverseResponse.audio_out
        ///
        /// Although the precise order of responses is not guaranteed, sequential
        /// ConverseResponse.audio_out messages will always contain sequential portions
        /// of audio.
        pub async fn converse(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::ConverseRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::ConverseResponse>>,
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
                "/google.assistant.embedded.v1alpha1.EmbeddedAssistant/Converse",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.assistant.embedded.v1alpha1.EmbeddedAssistant",
                        "Converse",
                    ),
                );
            self.inner.streaming(req, path, codec).await
        }
    }
}
