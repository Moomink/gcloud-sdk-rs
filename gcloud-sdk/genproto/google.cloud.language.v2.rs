/// Represents the input to API methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Document {
    /// Required. If the type is not set or is `TYPE_UNSPECIFIED`,
    /// returns an `INVALID_ARGUMENT` error.
    #[prost(enumeration = "document::Type", tag = "1")]
    pub r#type: i32,
    /// Optional. The language of the document (if not specified, the language is
    /// automatically detected). Both ISO and BCP-47 language codes are
    /// accepted.<br>
    /// [Language
    /// Support](<https://cloud.google.com/natural-language/docs/languages>) lists
    /// currently supported languages for each API method. If the language (either
    /// specified by the caller or automatically detected) is not supported by the
    /// called API method, an `INVALID_ARGUMENT` error is returned.
    #[prost(string, tag = "4")]
    pub language_code: ::prost::alloc::string::String,
    /// The source of the document: a string containing the content or a
    /// Google Cloud Storage URI.
    #[prost(oneof = "document::Source", tags = "2, 3")]
    pub source: ::core::option::Option<document::Source>,
}
/// Nested message and enum types in `Document`.
pub mod document {
    /// The document types enum.
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
    pub enum Type {
        /// The content type is not specified.
        Unspecified = 0,
        /// Plain text
        PlainText = 1,
        /// HTML
        Html = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::PlainText => "PLAIN_TEXT",
                Type::Html => "HTML",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "PLAIN_TEXT" => Some(Self::PlainText),
                "HTML" => Some(Self::Html),
                _ => None,
            }
        }
    }
    /// The source of the document: a string containing the content or a
    /// Google Cloud Storage URI.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// The content of the input in string format.
        /// Cloud audit logging exempt since it is based on user data.
        #[prost(string, tag = "2")]
        Content(::prost::alloc::string::String),
        /// The Google Cloud Storage URI where the file content is located.
        /// This URI must be of the form: gs://bucket_name/object_name. For more
        /// details, see <https://cloud.google.com/storage/docs/reference-uris.>
        /// NOTE: Cloud Storage object versioning is not supported.
        #[prost(string, tag = "3")]
        GcsContentUri(::prost::alloc::string::String),
    }
}
/// Represents a sentence in the input document.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sentence {
    /// The sentence text.
    #[prost(message, optional, tag = "1")]
    pub text: ::core::option::Option<TextSpan>,
    /// For calls to [AnalyzeSentiment][] or if
    /// [AnnotateTextRequest.Features.extract_document_sentiment][google.cloud.language.v2.AnnotateTextRequest.Features.extract_document_sentiment]
    /// is set to true, this field will contain the sentiment for the sentence.
    #[prost(message, optional, tag = "2")]
    pub sentiment: ::core::option::Option<Sentiment>,
}
/// Represents a phrase in the text that is a known entity, such as
/// a person, an organization, or location. The API associates information, such
/// as probability and mentions, with entities.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Entity {
    /// The representative name for the entity.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The entity type.
    #[prost(enumeration = "entity::Type", tag = "2")]
    pub r#type: i32,
    /// Metadata associated with the entity.
    ///
    /// For most entity types, the metadata is a Wikipedia URL (`wikipedia_url`)
    /// and Knowledge Graph MID (`mid`), if they are available. For the metadata
    /// associated with other entity types, see the Type table below.
    #[prost(map = "string, string", tag = "3")]
    pub metadata: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The mentions of this entity in the input document. The API currently
    /// supports proper noun mentions.
    #[prost(message, repeated, tag = "5")]
    pub mentions: ::prost::alloc::vec::Vec<EntityMention>,
    /// For calls to [AnalyzeEntitySentiment][] or if
    /// [AnnotateTextRequest.Features.extract_entity_sentiment][google.cloud.language.v2.AnnotateTextRequest.Features.extract_entity_sentiment]
    /// is set to true, this field will contain the aggregate sentiment expressed
    /// for this entity in the provided document.
    #[prost(message, optional, tag = "6")]
    pub sentiment: ::core::option::Option<Sentiment>,
}
/// Nested message and enum types in `Entity`.
pub mod entity {
    /// The type of the entity. For most entity types, the associated metadata is a
    /// Wikipedia URL (`wikipedia_url`) and Knowledge Graph MID (`mid`). The table
    /// below lists the associated fields for entities that have different
    /// metadata.
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
    pub enum Type {
        /// Unknown
        Unknown = 0,
        /// Person
        Person = 1,
        /// Location
        Location = 2,
        /// Organization
        Organization = 3,
        /// Event
        Event = 4,
        /// Artwork
        WorkOfArt = 5,
        /// Consumer product
        ConsumerGood = 6,
        /// Other types of entities
        Other = 7,
        /// Phone number
        ///
        /// The metadata lists the phone number, formatted according to local
        /// convention, plus whichever additional elements appear in the text:
        ///
        /// * `number` - the actual number, broken down into sections as per local
        /// convention
        /// * `national_prefix` - country code, if detected
        /// * `area_code` - region or area code, if detected
        /// * `extension` - phone extension (to be dialed after connection), if
        /// detected
        PhoneNumber = 9,
        /// Address
        ///
        /// The metadata identifies the street number and locality plus whichever
        /// additional elements appear in the text:
        ///
        /// * `street_number` - street number
        /// * `locality` - city or town
        /// * `street_name` - street/route name, if detected
        /// * `postal_code` - postal code, if detected
        /// * `country` - country, if detected<
        /// * `broad_region` - administrative area, such as the state, if detected
        /// * `narrow_region` - smaller administrative area, such as county, if
        /// detected
        /// * `sublocality` - used in Asian addresses to demark a district within a
        /// city, if detected
        Address = 10,
        /// Date
        ///
        /// The metadata identifies the components of the date:
        ///
        /// * `year` - four digit year, if detected
        /// * `month` - two digit month number, if detected
        /// * `day` - two digit day number, if detected
        Date = 11,
        /// Number
        ///
        /// The metadata is the number itself.
        Number = 12,
        /// Price
        ///
        /// The metadata identifies the `value` and `currency`.
        Price = 13,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unknown => "UNKNOWN",
                Type::Person => "PERSON",
                Type::Location => "LOCATION",
                Type::Organization => "ORGANIZATION",
                Type::Event => "EVENT",
                Type::WorkOfArt => "WORK_OF_ART",
                Type::ConsumerGood => "CONSUMER_GOOD",
                Type::Other => "OTHER",
                Type::PhoneNumber => "PHONE_NUMBER",
                Type::Address => "ADDRESS",
                Type::Date => "DATE",
                Type::Number => "NUMBER",
                Type::Price => "PRICE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN" => Some(Self::Unknown),
                "PERSON" => Some(Self::Person),
                "LOCATION" => Some(Self::Location),
                "ORGANIZATION" => Some(Self::Organization),
                "EVENT" => Some(Self::Event),
                "WORK_OF_ART" => Some(Self::WorkOfArt),
                "CONSUMER_GOOD" => Some(Self::ConsumerGood),
                "OTHER" => Some(Self::Other),
                "PHONE_NUMBER" => Some(Self::PhoneNumber),
                "ADDRESS" => Some(Self::Address),
                "DATE" => Some(Self::Date),
                "NUMBER" => Some(Self::Number),
                "PRICE" => Some(Self::Price),
                _ => None,
            }
        }
    }
}
/// Represents the feeling associated with the entire text or entities in
/// the text.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sentiment {
    /// A non-negative number in the [0, +inf) range, which represents
    /// the absolute magnitude of sentiment regardless of score (positive or
    /// negative).
    #[prost(float, tag = "1")]
    pub magnitude: f32,
    /// Sentiment score between -1.0 (negative sentiment) and 1.0
    /// (positive sentiment).
    #[prost(float, tag = "2")]
    pub score: f32,
}
/// Represents a mention for an entity in the text. Currently, proper noun
/// mentions are supported.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityMention {
    /// The mention text.
    #[prost(message, optional, tag = "1")]
    pub text: ::core::option::Option<TextSpan>,
    /// The type of the entity mention.
    #[prost(enumeration = "entity_mention::Type", tag = "2")]
    pub r#type: i32,
    /// For calls to [AnalyzeEntitySentiment][] or if
    /// [AnnotateTextRequest.Features.extract_entity_sentiment][google.cloud.language.v2.AnnotateTextRequest.Features.extract_entity_sentiment]
    /// is set to true, this field will contain the sentiment expressed for this
    /// mention of the entity in the provided document.
    #[prost(message, optional, tag = "3")]
    pub sentiment: ::core::option::Option<Sentiment>,
    /// Probability score associated with the entity.
    ///
    /// The score shows the probability of the entity mention being the entity
    /// type. The score is in (0, 1] range.
    #[prost(float, tag = "4")]
    pub probability: f32,
}
/// Nested message and enum types in `EntityMention`.
pub mod entity_mention {
    /// The supported types of mentions.
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
    pub enum Type {
        /// Unknown
        Unknown = 0,
        /// Proper name
        Proper = 1,
        /// Common noun (or noun compound)
        Common = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unknown => "TYPE_UNKNOWN",
                Type::Proper => "PROPER",
                Type::Common => "COMMON",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNKNOWN" => Some(Self::Unknown),
                "PROPER" => Some(Self::Proper),
                "COMMON" => Some(Self::Common),
                _ => None,
            }
        }
    }
}
/// Represents a text span in the input document.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextSpan {
    /// The content of the text span, which is a substring of the document.
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    /// The API calculates the beginning offset of the content in the original
    /// document according to the
    /// [EncodingType][google.cloud.language.v2.EncodingType] specified in the API
    /// request.
    #[prost(int32, tag = "2")]
    pub begin_offset: i32,
}
/// Represents a category returned from the text classifier.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClassificationCategory {
    /// The name of the category representing the document.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The classifier's confidence of the category. Number represents how certain
    /// the classifier is that this category represents the given text.
    #[prost(float, tag = "2")]
    pub confidence: f32,
}
/// The sentiment analysis request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeSentimentRequest {
    /// Required. Input document.
    #[prost(message, optional, tag = "1")]
    pub document: ::core::option::Option<Document>,
    /// The encoding type used by the API to calculate sentence offsets.
    #[prost(enumeration = "EncodingType", tag = "2")]
    pub encoding_type: i32,
}
/// The sentiment analysis response message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeSentimentResponse {
    /// The overall sentiment of the input document.
    #[prost(message, optional, tag = "1")]
    pub document_sentiment: ::core::option::Option<Sentiment>,
    /// The language of the text, which will be the same as the language specified
    /// in the request or, if not specified, the automatically-detected language.
    /// See [Document.language][] field for more details.
    #[prost(string, tag = "2")]
    pub language_code: ::prost::alloc::string::String,
    /// The sentiment for all the sentences in the document.
    #[prost(message, repeated, tag = "3")]
    pub sentences: ::prost::alloc::vec::Vec<Sentence>,
    /// Whether the language is officially supported. The API may still return a
    /// response when the language is not supported, but it is on a best effort
    /// basis.
    #[prost(bool, tag = "4")]
    pub language_supported: bool,
}
/// The entity analysis request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeEntitiesRequest {
    /// Required. Input document.
    #[prost(message, optional, tag = "1")]
    pub document: ::core::option::Option<Document>,
    /// The encoding type used by the API to calculate offsets.
    #[prost(enumeration = "EncodingType", tag = "2")]
    pub encoding_type: i32,
}
/// The entity analysis response message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeEntitiesResponse {
    /// The recognized entities in the input document.
    #[prost(message, repeated, tag = "1")]
    pub entities: ::prost::alloc::vec::Vec<Entity>,
    /// The language of the text, which will be the same as the language specified
    /// in the request or, if not specified, the automatically-detected language.
    /// See [Document.language][] field for more details.
    #[prost(string, tag = "2")]
    pub language_code: ::prost::alloc::string::String,
    /// Whether the language is officially supported. The API may still return a
    /// response when the language is not supported, but it is on a best effort
    /// basis.
    #[prost(bool, tag = "3")]
    pub language_supported: bool,
}
/// The document classification request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClassifyTextRequest {
    /// Required. Input document.
    #[prost(message, optional, tag = "1")]
    pub document: ::core::option::Option<Document>,
}
/// The document classification response message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClassifyTextResponse {
    /// Categories representing the input document.
    #[prost(message, repeated, tag = "1")]
    pub categories: ::prost::alloc::vec::Vec<ClassificationCategory>,
    /// The language of the text, which will be the same as the language specified
    /// in the request or, if not specified, the automatically-detected language.
    /// See [Document.language][] field for more details.
    #[prost(string, tag = "2")]
    pub language_code: ::prost::alloc::string::String,
    /// Whether the language is officially supported. The API may still return a
    /// response when the language is not supported, but it is on a best effort
    /// basis.
    #[prost(bool, tag = "3")]
    pub language_supported: bool,
}
/// The document moderation request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModerateTextRequest {
    /// Required. Input document.
    #[prost(message, optional, tag = "1")]
    pub document: ::core::option::Option<Document>,
}
/// The document moderation response message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModerateTextResponse {
    /// Harmful and sensitive categories representing the input document.
    #[prost(message, repeated, tag = "1")]
    pub moderation_categories: ::prost::alloc::vec::Vec<ClassificationCategory>,
    /// The language of the text, which will be the same as the language specified
    /// in the request or, if not specified, the automatically-detected language.
    /// See [Document.language][] field for more details.
    #[prost(string, tag = "2")]
    pub language_code: ::prost::alloc::string::String,
    /// Whether the language is officially supported. The API may still return a
    /// response when the language is not supported, but it is on a best effort
    /// basis.
    #[prost(bool, tag = "3")]
    pub language_supported: bool,
}
/// The request message for the text annotation API, which can perform multiple
/// analysis types in one call.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotateTextRequest {
    /// Required. Input document.
    #[prost(message, optional, tag = "1")]
    pub document: ::core::option::Option<Document>,
    /// Required. The enabled features.
    #[prost(message, optional, tag = "2")]
    pub features: ::core::option::Option<annotate_text_request::Features>,
    /// The encoding type used by the API to calculate offsets.
    #[prost(enumeration = "EncodingType", tag = "3")]
    pub encoding_type: i32,
}
/// Nested message and enum types in `AnnotateTextRequest`.
pub mod annotate_text_request {
    /// All available features.
    /// Setting each one to true will enable that specific analysis for the input.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Features {
        /// Optional. Extract entities.
        #[prost(bool, tag = "1")]
        pub extract_entities: bool,
        /// Optional. Extract document-level sentiment.
        #[prost(bool, tag = "2")]
        pub extract_document_sentiment: bool,
        /// Optional. Classify the full document into categories.
        #[prost(bool, tag = "4")]
        pub classify_text: bool,
        /// Optional. Moderate the document for harmful and sensitive categories.
        #[prost(bool, tag = "5")]
        pub moderate_text: bool,
    }
}
/// The text annotations response message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotateTextResponse {
    /// Sentences in the input document. Populated if the user enables
    /// [AnnotateTextRequest.Features.extract_document_sentiment][google.cloud.language.v2.AnnotateTextRequest.Features.extract_document_sentiment].
    #[prost(message, repeated, tag = "1")]
    pub sentences: ::prost::alloc::vec::Vec<Sentence>,
    /// Entities, along with their semantic information, in the input document.
    /// Populated if the user enables
    /// [AnnotateTextRequest.Features.extract_entities][google.cloud.language.v2.AnnotateTextRequest.Features.extract_entities]
    /// or
    /// [AnnotateTextRequest.Features.extract_entity_sentiment][google.cloud.language.v2.AnnotateTextRequest.Features.extract_entity_sentiment].
    #[prost(message, repeated, tag = "2")]
    pub entities: ::prost::alloc::vec::Vec<Entity>,
    /// The overall sentiment for the document. Populated if the user enables
    /// [AnnotateTextRequest.Features.extract_document_sentiment][google.cloud.language.v2.AnnotateTextRequest.Features.extract_document_sentiment].
    #[prost(message, optional, tag = "3")]
    pub document_sentiment: ::core::option::Option<Sentiment>,
    /// The language of the text, which will be the same as the language specified
    /// in the request or, if not specified, the automatically-detected language.
    /// See [Document.language][] field for more details.
    #[prost(string, tag = "4")]
    pub language_code: ::prost::alloc::string::String,
    /// Categories identified in the input document.
    #[prost(message, repeated, tag = "5")]
    pub categories: ::prost::alloc::vec::Vec<ClassificationCategory>,
    /// Harmful and sensitive categories identified in the input document.
    #[prost(message, repeated, tag = "6")]
    pub moderation_categories: ::prost::alloc::vec::Vec<ClassificationCategory>,
    /// Whether the language is officially supported by all requested features.
    /// The API may still return a response when the language is not supported, but
    /// it is on a best effort basis.
    #[prost(bool, tag = "7")]
    pub language_supported: bool,
}
/// Represents the text encoding that the caller uses to process the output.
/// Providing an `EncodingType` is recommended because the API provides the
/// beginning offsets for various outputs, such as tokens and mentions, and
/// languages that natively use different text encodings may access offsets
/// differently.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EncodingType {
    /// If `EncodingType` is not specified, encoding-dependent information (such as
    /// `begin_offset`) will be set at `-1`.
    None = 0,
    /// Encoding-dependent information (such as `begin_offset`) is calculated based
    /// on the UTF-8 encoding of the input. C++ and Go are examples of languages
    /// that use this encoding natively.
    Utf8 = 1,
    /// Encoding-dependent information (such as `begin_offset`) is calculated based
    /// on the UTF-16 encoding of the input. Java and JavaScript are examples of
    /// languages that use this encoding natively.
    Utf16 = 2,
    /// Encoding-dependent information (such as `begin_offset`) is calculated based
    /// on the UTF-32 encoding of the input. Python is an example of a language
    /// that uses this encoding natively.
    Utf32 = 3,
}
impl EncodingType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EncodingType::None => "NONE",
            EncodingType::Utf8 => "UTF8",
            EncodingType::Utf16 => "UTF16",
            EncodingType::Utf32 => "UTF32",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NONE" => Some(Self::None),
            "UTF8" => Some(Self::Utf8),
            "UTF16" => Some(Self::Utf16),
            "UTF32" => Some(Self::Utf32),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod language_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Provides text analysis operations such as sentiment analysis and entity
    /// recognition.
    #[derive(Debug, Clone)]
    pub struct LanguageServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl LanguageServiceClient<tonic::transport::Channel> {
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
    impl<T> LanguageServiceClient<T>
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
        ) -> LanguageServiceClient<InterceptedService<T, F>>
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
            LanguageServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Analyzes the sentiment of the provided text.
        pub async fn analyze_sentiment(
            &mut self,
            request: impl tonic::IntoRequest<super::AnalyzeSentimentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AnalyzeSentimentResponse>,
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
                "/google.cloud.language.v2.LanguageService/AnalyzeSentiment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.language.v2.LanguageService",
                        "AnalyzeSentiment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Finds named entities (currently proper names and common nouns) in the text
        /// along with entity types, probability, mentions for each entity, and
        /// other properties.
        pub async fn analyze_entities(
            &mut self,
            request: impl tonic::IntoRequest<super::AnalyzeEntitiesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AnalyzeEntitiesResponse>,
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
                "/google.cloud.language.v2.LanguageService/AnalyzeEntities",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.language.v2.LanguageService",
                        "AnalyzeEntities",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Classifies a document into categories.
        pub async fn classify_text(
            &mut self,
            request: impl tonic::IntoRequest<super::ClassifyTextRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ClassifyTextResponse>,
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
                "/google.cloud.language.v2.LanguageService/ClassifyText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.language.v2.LanguageService",
                        "ClassifyText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Moderates a document for harmful and sensitive categories.
        pub async fn moderate_text(
            &mut self,
            request: impl tonic::IntoRequest<super::ModerateTextRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ModerateTextResponse>,
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
                "/google.cloud.language.v2.LanguageService/ModerateText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.language.v2.LanguageService",
                        "ModerateText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// A convenience method that provides all features in one call.
        pub async fn annotate_text(
            &mut self,
            request: impl tonic::IntoRequest<super::AnnotateTextRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AnnotateTextResponse>,
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
                "/google.cloud.language.v2.LanguageService/AnnotateText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.language.v2.LanguageService",
                        "AnnotateText",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
