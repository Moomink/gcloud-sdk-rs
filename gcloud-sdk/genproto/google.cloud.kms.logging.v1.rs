/// Log message used to send to Platform Logging for asynchronous
/// CryptoKey events.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CryptoKeyEvent {
    /// An event for rotating the primary CryptoKeyVersion of a CryptoKey.
    #[prost(message, optional, tag = "1")]
    pub rotation_event: ::core::option::Option<crypto_key_event::RotationEvent>,
}
/// Nested message and enum types in `CryptoKeyEvent`.
pub mod crypto_key_event {
    /// The event emitted by KMS when performing a scheduled automatic CryptoKey
    /// rotation. See <https://cloud.google.com/kms/docs/rotating-keys#automatic>
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RotationEvent {
        /// The result of the scheduled key rotation. The 'details' field of the
        /// status may contain a google.rpc.PreconditionFailure.
        #[prost(message, optional, tag = "1")]
        pub status: ::core::option::Option<
            super::super::super::super::super::rpc::Status,
        >,
    }
}
/// Log message used to send to Platform Logging for asynchronous
/// CryptoKeyVersion events.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CryptoKeyVersionEvent {
    #[prost(oneof = "crypto_key_version_event::Event", tags = "1, 2, 3")]
    pub event: ::core::option::Option<crypto_key_version_event::Event>,
}
/// Nested message and enum types in `CryptoKeyVersionEvent`.
pub mod crypto_key_version_event {
    /// The event emitted by KMS when destroying a CryptoKeyVersion scheduled for
    /// destruction. See <https://cloud.google.com/kms/docs/destroy-restore#destroy>
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ScheduledDestructionEvent {
        /// The result of the scheduled key version destruction. The 'details' field
        /// of the status may contain a google.rpc.PreconditionFailure. For EKM keys,
        /// the 'violations' field of a PreconditionFailure will also include EKM
        /// errors.
        #[prost(message, optional, tag = "1")]
        pub status: ::core::option::Option<
            super::super::super::super::super::rpc::Status,
        >,
        /// The Key Access Justification (KAJ) reason associated with the request.
        /// This field is only populated for KAJ enrolled customers for EKM keys.
        #[prost(string, tag = "2")]
        pub key_access_justification_reason: ::prost::alloc::string::String,
    }
    /// The event emitted by KMS when generating a CryptoKeyVersion. See
    /// <https://cloud.google.com/kms/docs/key-states>
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KeyGenerationEvent {
        /// The result of the key version generation. The 'details' field of the
        /// status may contain a google.rpc.PreconditionFailure. For EKM keys, the
        /// 'violations' field of a PreconditionFailure will also include EKM errors.
        #[prost(message, optional, tag = "1")]
        pub status: ::core::option::Option<
            super::super::super::super::super::rpc::Status,
        >,
        /// The Key Access Justification (KAJ) reason associated with the request.
        /// This field is only populated for KAJ enrolled customers for EKM keys.
        #[prost(string, tag = "2")]
        pub key_access_justification_reason: ::prost::alloc::string::String,
    }
    /// The event emitted by KMS when importing a CryptoKeyVersion. See
    /// <https://cloud.google.com/kms/docs/importing-a-key>
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ImportEvent {
        /// The result of the key version import. The 'details' field of the status
        /// may contain a google.rpc.PreconditionFailure.
        #[prost(message, optional, tag = "1")]
        pub status: ::core::option::Option<
            super::super::super::super::super::rpc::Status,
        >,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        /// An event for the scheduled destruction of a CryptoKeyVersion.
        #[prost(message, tag = "1")]
        ScheduledDestructionEvent(ScheduledDestructionEvent),
        /// An event for the generation of a CryptoKeyVersion.
        #[prost(message, tag = "2")]
        KeyGenerationEvent(KeyGenerationEvent),
        /// An event for the import of key material for a CryptoKeyVersion.
        #[prost(message, tag = "3")]
        ImportEvent(ImportEvent),
    }
}
