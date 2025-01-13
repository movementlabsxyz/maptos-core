// Copyright (c) Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

// @generated
// This file is @generated by prost-build.
// Transaction data is transferred via 1 stream with batches until terminated.
// One stream consists:
//   StreamStatus: INIT with version x
//   loop k:
//     TransactionOutput data(size n)
//     StreamStatus: BATCH_END with version x + (k + 1) * n - 1

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionsOutput {
    #[prost(message, repeated, tag="1")]
    pub transactions: ::prost::alloc::vec::Vec<super::super::super::transaction::v1::Transaction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct StreamStatus {
    #[prost(enumeration="stream_status::StatusType", tag="1")]
    pub r#type: i32,
    /// Required. Start version of current batch/stream, inclusive.
    #[prost(uint64, tag="2")]
    pub start_version: u64,
    /// End version of current *batch*, inclusive.
    #[prost(uint64, optional, tag="3")]
    pub end_version: ::core::option::Option<u64>,
}
/// Nested message and enum types in `StreamStatus`.
pub mod stream_status {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum StatusType {
        Unspecified = 0,
        /// Signal for the start of the stream.
        Init = 1,
        /// Signal for the end of the batch.
        BatchEnd = 2,
    }
    impl StatusType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                StatusType::Unspecified => "STATUS_TYPE_UNSPECIFIED",
                StatusType::Init => "STATUS_TYPE_INIT",
                StatusType::BatchEnd => "STATUS_TYPE_BATCH_END",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATUS_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "STATUS_TYPE_INIT" => Some(Self::Init),
                "STATUS_TYPE_BATCH_END" => Some(Self::BatchEnd),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct GetTransactionsFromNodeRequest {
    /// Required; start version of current stream.
    /// If not set will panic somewhere
    #[prost(uint64, optional, tag="1")]
    pub starting_version: ::core::option::Option<u64>,
    /// Optional; number of transactions to return in current stream.
    /// If not set, response streams infinitely.
    #[prost(uint64, optional, tag="2")]
    pub transactions_count: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionsFromNodeResponse {
    /// Making sure that all the responses include a chain id
    #[prost(uint32, tag="3")]
    pub chain_id: u32,
    #[prost(oneof="transactions_from_node_response::Response", tags="1, 2")]
    pub response: ::core::option::Option<transactions_from_node_response::Response>,
}
/// Nested message and enum types in `TransactionsFromNodeResponse`.
pub mod transactions_from_node_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag="1")]
        Status(super::StreamStatus),
        #[prost(message, tag="2")]
        Data(super::TransactionsOutput),
    }
}
/// Encoded file descriptor set for the `aptos.internal.fullnode.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xf1, 0x17, 0x0a, 0x2e, 0x61, 0x70, 0x74, 0x6f, 0x73, 0x2f, 0x69, 0x6e, 0x74, 0x65, 0x72,
    0x6e, 0x61, 0x6c, 0x2f, 0x66, 0x75, 0x6c, 0x6c, 0x6e, 0x6f, 0x64, 0x65, 0x2f, 0x76, 0x31, 0x2f,
    0x66, 0x75, 0x6c, 0x6c, 0x6e, 0x6f, 0x64, 0x65, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x1a, 0x61, 0x70, 0x74, 0x6f, 0x73, 0x2e, 0x69, 0x6e, 0x74, 0x65, 0x72,
    0x6e, 0x61, 0x6c, 0x2e, 0x66, 0x75, 0x6c, 0x6c, 0x6e, 0x6f, 0x64, 0x65, 0x2e, 0x76, 0x31, 0x1a,
    0x26, 0x61, 0x70, 0x74, 0x6f, 0x73, 0x2f, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x2f, 0x76, 0x31, 0x2f, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x5b, 0x0a, 0x12, 0x54, 0x72, 0x61, 0x6e, 0x73,
    0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x4f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x12, 0x45, 0x0a,
    0x0c, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x01, 0x20,
    0x03, 0x28, 0x0b, 0x32, 0x21, 0x2e, 0x61, 0x70, 0x74, 0x6f, 0x73, 0x2e, 0x74, 0x72, 0x61, 0x6e,
    0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x76, 0x31, 0x2e, 0x54, 0x72, 0x61, 0x6e, 0x73,
    0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x0c, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x73, 0x22, 0x92, 0x02, 0x0a, 0x0c, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x53,
    0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x47, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0e, 0x32, 0x33, 0x2e, 0x61, 0x70, 0x74, 0x6f, 0x73, 0x2e, 0x69, 0x6e, 0x74, 0x65,
    0x72, 0x6e, 0x61, 0x6c, 0x2e, 0x66, 0x75, 0x6c, 0x6c, 0x6e, 0x6f, 0x64, 0x65, 0x2e, 0x76, 0x31,
    0x2e, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x53, 0x74,
    0x61, 0x74, 0x75, 0x73, 0x54, 0x79, 0x70, 0x65, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x23,
    0x0a, 0x0d, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0c, 0x73, 0x74, 0x61, 0x72, 0x74, 0x56, 0x65, 0x72, 0x73,
    0x69, 0x6f, 0x6e, 0x12, 0x28, 0x0a, 0x0b, 0x65, 0x6e, 0x64, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69,
    0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x42, 0x02, 0x30, 0x01, 0x48, 0x00, 0x52, 0x0a,
    0x65, 0x6e, 0x64, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x88, 0x01, 0x01, 0x22, 0x5a, 0x0a,
    0x0a, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x54, 0x79, 0x70, 0x65, 0x12, 0x1b, 0x0a, 0x17, 0x53,
    0x54, 0x41, 0x54, 0x55, 0x53, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x55, 0x4e, 0x53, 0x50, 0x45,
    0x43, 0x49, 0x46, 0x49, 0x45, 0x44, 0x10, 0x00, 0x12, 0x14, 0x0a, 0x10, 0x53, 0x54, 0x41, 0x54,
    0x55, 0x53, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x49, 0x4e, 0x49, 0x54, 0x10, 0x01, 0x12, 0x19,
    0x0a, 0x15, 0x53, 0x54, 0x41, 0x54, 0x55, 0x53, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x42, 0x41,
    0x54, 0x43, 0x48, 0x5f, 0x45, 0x4e, 0x44, 0x10, 0x02, 0x42, 0x0e, 0x0a, 0x0c, 0x5f, 0x65, 0x6e,
    0x64, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x22, 0xb8, 0x01, 0x0a, 0x1e, 0x47, 0x65,
    0x74, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x46, 0x72, 0x6f,
    0x6d, 0x4e, 0x6f, 0x64, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x32, 0x0a, 0x10,
    0x73, 0x74, 0x61, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x42, 0x02, 0x30, 0x01, 0x48, 0x00, 0x52, 0x0f, 0x73, 0x74,
    0x61, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x88, 0x01, 0x01,
    0x12, 0x36, 0x0a, 0x12, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73,
    0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x42, 0x02, 0x30, 0x01,
    0x48, 0x01, 0x52, 0x11, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73,
    0x43, 0x6f, 0x75, 0x6e, 0x74, 0x88, 0x01, 0x01, 0x42, 0x13, 0x0a, 0x11, 0x5f, 0x73, 0x74, 0x61,
    0x72, 0x74, 0x69, 0x6e, 0x67, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x42, 0x15, 0x0a,
    0x13, 0x5f, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x5f, 0x63,
    0x6f, 0x75, 0x6e, 0x74, 0x22, 0xcf, 0x01, 0x0a, 0x1c, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63,
    0x74, 0x69, 0x6f, 0x6e, 0x73, 0x46, 0x72, 0x6f, 0x6d, 0x4e, 0x6f, 0x64, 0x65, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x42, 0x0a, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x28, 0x2e, 0x61, 0x70, 0x74, 0x6f, 0x73, 0x2e, 0x69, 0x6e,
    0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x2e, 0x66, 0x75, 0x6c, 0x6c, 0x6e, 0x6f, 0x64, 0x65, 0x2e,
    0x76, 0x31, 0x2e, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x48,
    0x00, 0x52, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x44, 0x0a, 0x04, 0x64, 0x61, 0x74,
    0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2e, 0x2e, 0x61, 0x70, 0x74, 0x6f, 0x73, 0x2e,
    0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x2e, 0x66, 0x75, 0x6c, 0x6c, 0x6e, 0x6f, 0x64,
    0x65, 0x2e, 0x76, 0x31, 0x2e, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x73, 0x4f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x48, 0x00, 0x52, 0x04, 0x64, 0x61, 0x74, 0x61, 0x12,
    0x19, 0x0a, 0x08, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x07, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x49, 0x64, 0x42, 0x0a, 0x0a, 0x08, 0x72, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x32, 0xa2, 0x01, 0x0a, 0x0c, 0x46, 0x75, 0x6c, 0x6c, 0x6e,
    0x6f, 0x64, 0x65, 0x44, 0x61, 0x74, 0x61, 0x12, 0x91, 0x01, 0x0a, 0x17, 0x47, 0x65, 0x74, 0x54,
    0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x46, 0x72, 0x6f, 0x6d, 0x4e,
    0x6f, 0x64, 0x65, 0x12, 0x3a, 0x2e, 0x61, 0x70, 0x74, 0x6f, 0x73, 0x2e, 0x69, 0x6e, 0x74, 0x65,
    0x72, 0x6e, 0x61, 0x6c, 0x2e, 0x66, 0x75, 0x6c, 0x6c, 0x6e, 0x6f, 0x64, 0x65, 0x2e, 0x76, 0x31,
    0x2e, 0x47, 0x65, 0x74, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73,
    0x46, 0x72, 0x6f, 0x6d, 0x4e, 0x6f, 0x64, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a,
    0x38, 0x2e, 0x61, 0x70, 0x74, 0x6f, 0x73, 0x2e, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c,
    0x2e, 0x66, 0x75, 0x6c, 0x6c, 0x6e, 0x6f, 0x64, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x54, 0x72, 0x61,
    0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x46, 0x72, 0x6f, 0x6d, 0x4e, 0x6f, 0x64,
    0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x30, 0x01, 0x42, 0xbe, 0x01, 0x0a, 0x1e,
    0x63, 0x6f, 0x6d, 0x2e, 0x61, 0x70, 0x74, 0x6f, 0x73, 0x2e, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e,
    0x61, 0x6c, 0x2e, 0x66, 0x75, 0x6c, 0x6c, 0x6e, 0x6f, 0x64, 0x65, 0x2e, 0x76, 0x31, 0x42, 0x11,
    0x46, 0x75, 0x6c, 0x6c, 0x6e, 0x6f, 0x64, 0x65, 0x44, 0x61, 0x74, 0x61, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x50, 0x01, 0xa2, 0x02, 0x03, 0x41, 0x49, 0x46, 0xaa, 0x02, 0x1a, 0x41, 0x70, 0x74, 0x6f,
    0x73, 0x2e, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x2e, 0x46, 0x75, 0x6c, 0x6c, 0x6e,
    0x6f, 0x64, 0x65, 0x2e, 0x56, 0x31, 0xca, 0x02, 0x1a, 0x41, 0x70, 0x74, 0x6f, 0x73, 0x5c, 0x49,
    0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x5c, 0x46, 0x75, 0x6c, 0x6c, 0x6e, 0x6f, 0x64, 0x65,
    0x5c, 0x56, 0x31, 0xe2, 0x02, 0x26, 0x41, 0x70, 0x74, 0x6f, 0x73, 0x5c, 0x49, 0x6e, 0x74, 0x65,
    0x72, 0x6e, 0x61, 0x6c, 0x5c, 0x46, 0x75, 0x6c, 0x6c, 0x6e, 0x6f, 0x64, 0x65, 0x5c, 0x56, 0x31,
    0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02, 0x1d, 0x41,
    0x70, 0x74, 0x6f, 0x73, 0x3a, 0x3a, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x3a, 0x3a,
    0x46, 0x75, 0x6c, 0x6c, 0x6e, 0x6f, 0x64, 0x65, 0x3a, 0x3a, 0x56, 0x31, 0x4a, 0x8d, 0x0e, 0x0a,
    0x06, 0x12, 0x04, 0x03, 0x00, 0x38, 0x01, 0x0a, 0x4e, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x03, 0x00,
    0x12, 0x32, 0x44, 0x20, 0x43, 0x6f, 0x70, 0x79, 0x72, 0x69, 0x67, 0x68, 0x74, 0x20, 0xc2, 0xa9,
    0x20, 0x41, 0x70, 0x74, 0x6f, 0x73, 0x20, 0x46, 0x6f, 0x75, 0x6e, 0x64, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x0a, 0x20, 0x53, 0x50, 0x44, 0x58, 0x2d, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x2d,
    0x49, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72, 0x3a, 0x20, 0x41, 0x70, 0x61, 0x63,
    0x68, 0x65, 0x2d, 0x32, 0x2e, 0x30, 0x0a, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x05, 0x00,
    0x23, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x07, 0x00, 0x30, 0x0a, 0xfe, 0x01, 0x0a,
    0x02, 0x04, 0x00, 0x12, 0x04, 0x10, 0x00, 0x12, 0x01, 0x32, 0xf1, 0x01, 0x20, 0x54, 0x72, 0x61,
    0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x69, 0x73,
    0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x65, 0x72, 0x72, 0x65, 0x64, 0x20, 0x76, 0x69, 0x61,
    0x20, 0x31, 0x20, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x62,
    0x61, 0x74, 0x63, 0x68, 0x65, 0x73, 0x20, 0x75, 0x6e, 0x74, 0x69, 0x6c, 0x20, 0x74, 0x65, 0x72,
    0x6d, 0x69, 0x6e, 0x61, 0x74, 0x65, 0x64, 0x2e, 0x0a, 0x20, 0x4f, 0x6e, 0x65, 0x20, 0x73, 0x74,
    0x72, 0x65, 0x61, 0x6d, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x69, 0x73, 0x74, 0x73, 0x3a, 0x0a, 0x20,
    0x20, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x3a, 0x20, 0x49,
    0x4e, 0x49, 0x54, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e,
    0x20, 0x78, 0x0a, 0x20, 0x20, 0x6c, 0x6f, 0x6f, 0x70, 0x20, 0x6b, 0x3a, 0x0a, 0x20, 0x20, 0x20,
    0x20, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x4f, 0x75, 0x74, 0x70,
    0x75, 0x74, 0x20, 0x64, 0x61, 0x74, 0x61, 0x28, 0x73, 0x69, 0x7a, 0x65, 0x20, 0x6e, 0x29, 0x0a,
    0x20, 0x20, 0x20, 0x20, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73,
    0x3a, 0x20, 0x42, 0x41, 0x54, 0x43, 0x48, 0x5f, 0x45, 0x4e, 0x44, 0x20, 0x77, 0x69, 0x74, 0x68,
    0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x78, 0x20, 0x2b, 0x20, 0x28, 0x6b, 0x20,
    0x2b, 0x20, 0x31, 0x29, 0x20, 0x2a, 0x20, 0x6e, 0x20, 0x2d, 0x20, 0x31, 0x0a, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x10, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x11, 0x02, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x11, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x11,
    0x0b, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x11, 0x2c, 0x38,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x11, 0x3b, 0x3c, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x14, 0x00, 0x21, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01,
    0x01, 0x12, 0x03, 0x14, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x01, 0x04, 0x00, 0x12, 0x04,
    0x15, 0x02, 0x1b, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x04, 0x00, 0x01, 0x12, 0x03, 0x15,
    0x07, 0x11, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x16, 0x04,
    0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x16, 0x04,
    0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x16, 0x1e,
    0x1f, 0x0a, 0x34, 0x0a, 0x06, 0x04, 0x01, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x18, 0x04, 0x19,
    0x1a, 0x25, 0x20, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x73, 0x74, 0x61, 0x72, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73,
    0x74, 0x72, 0x65, 0x61, 0x6d, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x18, 0x04, 0x14, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02,
    0x01, 0x02, 0x12, 0x03, 0x18, 0x17, 0x18, 0x0a, 0x31, 0x0a, 0x06, 0x04, 0x01, 0x04, 0x00, 0x02,
    0x02, 0x12, 0x03, 0x1a, 0x04, 0x1e, 0x1a, 0x22, 0x20, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x20,
    0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x6e, 0x64, 0x20, 0x6f, 0x66, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x62, 0x61, 0x74, 0x63, 0x68, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01,
    0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1a, 0x04, 0x19, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01,
    0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x1a, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x00, 0x12, 0x03, 0x1c, 0x02, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x1c, 0x02, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x1c, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1c, 0x14,
    0x15, 0x0a, 0x4a, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x1e, 0x02, 0x1b, 0x1a, 0x3d,
    0x20, 0x52, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x2e, 0x20, 0x53, 0x74, 0x61, 0x72, 0x74,
    0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x63, 0x75, 0x72, 0x72,
    0x65, 0x6e, 0x74, 0x20, 0x62, 0x61, 0x74, 0x63, 0x68, 0x2f, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d,
    0x2c, 0x20, 0x69, 0x6e, 0x63, 0x6c, 0x75, 0x73, 0x69, 0x76, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x1e, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1e, 0x09, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x1e, 0x19, 0x1a, 0x0a, 0x39, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12,
    0x03, 0x20, 0x02, 0x37, 0x1a, 0x2c, 0x20, 0x45, 0x6e, 0x64, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69,
    0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x2a, 0x62,
    0x61, 0x74, 0x63, 0x68, 0x2a, 0x2c, 0x20, 0x69, 0x6e, 0x63, 0x6c, 0x75, 0x73, 0x69, 0x76, 0x65,
    0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03, 0x20, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x20, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x20, 0x12, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x20, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x02, 0x08, 0x12, 0x03, 0x20, 0x22, 0x36, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x02, 0x02,
    0x08, 0x06, 0x12, 0x03, 0x20, 0x23, 0x35, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x23,
    0x00, 0x2b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x23, 0x08, 0x26, 0x0a,
    0x5a, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x26, 0x02, 0x3c, 0x1a, 0x4d, 0x20, 0x52,
    0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x3b, 0x20, 0x73, 0x74, 0x61, 0x72, 0x74, 0x20, 0x76,
    0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e,
    0x74, 0x20, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x2e, 0x0a, 0x20, 0x49, 0x66, 0x20, 0x6e, 0x6f,
    0x74, 0x20, 0x73, 0x65, 0x74, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x70, 0x61, 0x6e, 0x69, 0x63,
    0x20, 0x73, 0x6f, 0x6d, 0x65, 0x77, 0x68, 0x65, 0x72, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x26, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x26, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x26, 0x12, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x26, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x08, 0x12, 0x03, 0x26, 0x27,
    0x3b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x02, 0x02, 0x00, 0x08, 0x06, 0x12, 0x03, 0x26, 0x28, 0x3a,
    0x0a, 0x76, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2a, 0x02, 0x3e, 0x1a, 0x69, 0x20,
    0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x3b, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72,
    0x20, 0x6f, 0x66, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73,
    0x20, 0x74, 0x6f, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x20, 0x69, 0x6e, 0x20, 0x63, 0x75,
    0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x2e, 0x0a, 0x20, 0x49,
    0x66, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x73, 0x65, 0x74, 0x2c, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x20, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x73, 0x20, 0x69, 0x6e, 0x66, 0x69,
    0x6e, 0x69, 0x74, 0x65, 0x6c, 0x79, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x2a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x2a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2a,
    0x12, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2a, 0x27, 0x28,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x08, 0x12, 0x03, 0x2a, 0x29, 0x3d, 0x0a, 0x0d,
    0x0a, 0x06, 0x04, 0x02, 0x02, 0x01, 0x08, 0x06, 0x12, 0x03, 0x2a, 0x2a, 0x3c, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x03, 0x12, 0x04, 0x2d, 0x00, 0x34, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01,
    0x12, 0x03, 0x2d, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x03, 0x08, 0x00, 0x12, 0x04, 0x2e,
    0x02, 0x31, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x08, 0x00, 0x01, 0x12, 0x03, 0x2e, 0x08,
    0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x2f, 0x04, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x06, 0x12, 0x03, 0x2f, 0x04, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2f, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x2f, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01,
    0x12, 0x03, 0x30, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x06, 0x12, 0x03,
    0x30, 0x04, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x30, 0x17,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x30, 0x1e, 0x1f, 0x0a,
    0x44, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x33, 0x02, 0x16, 0x1a, 0x37, 0x20, 0x4d,
    0x61, 0x6b, 0x69, 0x6e, 0x67, 0x20, 0x73, 0x75, 0x72, 0x65, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20,
    0x61, 0x6c, 0x6c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x73, 0x20, 0x69, 0x6e, 0x63, 0x6c, 0x75, 0x64, 0x65, 0x20, 0x61, 0x20, 0x63, 0x68, 0x61, 0x69,
    0x6e, 0x20, 0x69, 0x64, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x33, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x33, 0x09,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x33, 0x14, 0x15, 0x0a,
    0x0a, 0x0a, 0x02, 0x06, 0x00, 0x12, 0x04, 0x36, 0x00, 0x38, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x06,
    0x00, 0x01, 0x12, 0x03, 0x36, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x37, 0x02, 0x6c, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x37,
    0x06, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x37, 0x1e, 0x3c,
    0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x37, 0x47, 0x4d, 0x0a, 0x0c,
    0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x37, 0x4e, 0x6a, 0x62, 0x06, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x33,
];
include!("aptos.internal.fullnode.v1.serde.rs");
include!("aptos.internal.fullnode.v1.tonic.rs");
// @@protoc_insertion_point(module)
