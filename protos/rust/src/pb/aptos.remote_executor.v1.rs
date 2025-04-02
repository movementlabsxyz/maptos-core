// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkMessage {
    #[prost(bytes="vec", tag="1")]
    pub message: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub message_type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Empty {
}
/// Encoded file descriptor set for the `aptos.remote_executor.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xa0, 0x06, 0x0a, 0x2a, 0x61, 0x70, 0x74, 0x6f, 0x73, 0x2f, 0x72, 0x65, 0x6d, 0x6f, 0x74,
    0x65, 0x5f, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x2f, 0x76, 0x31, 0x2f, 0x6e, 0x65,
    0x74, 0x77, 0x6f, 0x72, 0x6b, 0x5f, 0x6d, 0x73, 0x67, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x18, 0x61, 0x70, 0x74, 0x6f, 0x73, 0x2e, 0x72, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x5f, 0x65, 0x78,
    0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x2e, 0x76, 0x31, 0x22, 0x4d, 0x0a, 0x0e, 0x4e, 0x65, 0x74,
    0x77, 0x6f, 0x72, 0x6b, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x6d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x07, 0x6d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x21, 0x0a, 0x0c, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x6d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x22, 0x07, 0x0a, 0x05, 0x45, 0x6d, 0x70, 0x74,
    0x79, 0x32, 0x77, 0x0a, 0x15, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x4d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x12, 0x5e, 0x0a, 0x11, 0x53, 0x69,
    0x6d, 0x70, 0x6c, 0x65, 0x4d, 0x73, 0x67, 0x45, 0x78, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x12,
    0x28, 0x2e, 0x61, 0x70, 0x74, 0x6f, 0x73, 0x2e, 0x72, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x5f, 0x65,
    0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x2e, 0x76, 0x31, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f,
    0x72, 0x6b, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x1a, 0x1f, 0x2e, 0x61, 0x70, 0x74, 0x6f,
    0x73, 0x2e, 0x72, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x5f, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f,
    0x72, 0x2e, 0x76, 0x31, 0x2e, 0x45, 0x6d, 0x70, 0x74, 0x79, 0x42, 0xad, 0x01, 0x0a, 0x1c, 0x63,
    0x6f, 0x6d, 0x2e, 0x61, 0x70, 0x74, 0x6f, 0x73, 0x2e, 0x72, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x5f,
    0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x2e, 0x76, 0x31, 0x42, 0x0f, 0x4e, 0x65, 0x74,
    0x77, 0x6f, 0x72, 0x6b, 0x4d, 0x73, 0x67, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0xa2, 0x02,
    0x03, 0x41, 0x52, 0x58, 0xaa, 0x02, 0x17, 0x41, 0x70, 0x74, 0x6f, 0x73, 0x2e, 0x52, 0x65, 0x6d,
    0x6f, 0x74, 0x65, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x2e, 0x56, 0x31, 0xca, 0x02,
    0x17, 0x41, 0x70, 0x74, 0x6f, 0x73, 0x5c, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x45, 0x78, 0x65,
    0x63, 0x75, 0x74, 0x6f, 0x72, 0x5c, 0x56, 0x31, 0xe2, 0x02, 0x23, 0x41, 0x70, 0x74, 0x6f, 0x73,
    0x5c, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x5c,
    0x56, 0x31, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02,
    0x19, 0x41, 0x70, 0x74, 0x6f, 0x73, 0x3a, 0x3a, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x45, 0x78,
    0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x3a, 0x3a, 0x56, 0x31, 0x4a, 0xce, 0x02, 0x0a, 0x06, 0x12,
    0x04, 0x03, 0x00, 0x10, 0x01, 0x0a, 0x4e, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x03, 0x00, 0x12, 0x32,
    0x44, 0x20, 0x43, 0x6f, 0x70, 0x79, 0x72, 0x69, 0x67, 0x68, 0x74, 0x20, 0xc2, 0xa9, 0x20, 0x41,
    0x70, 0x74, 0x6f, 0x73, 0x20, 0x46, 0x6f, 0x75, 0x6e, 0x64, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x0a,
    0x20, 0x53, 0x50, 0x44, 0x58, 0x2d, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x2d, 0x49, 0x64,
    0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72, 0x3a, 0x20, 0x41, 0x70, 0x61, 0x63, 0x68, 0x65,
    0x2d, 0x32, 0x2e, 0x30, 0x0a, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x05, 0x00, 0x21, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x07, 0x00, 0x0a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x00, 0x01, 0x12, 0x03, 0x07, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x08, 0x02, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x08,
    0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x08, 0x08, 0x0f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x08, 0x12, 0x13, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x09, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x09, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x09, 0x09, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x09, 0x18, 0x19, 0x0a, 0x09, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x03, 0x0c, 0x00, 0x10,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0c, 0x08, 0x0d, 0x0a, 0x0a, 0x0a, 0x02,
    0x06, 0x00, 0x12, 0x04, 0x0e, 0x00, 0x10, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00, 0x01, 0x12,
    0x03, 0x0e, 0x08, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0f, 0x02,
    0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0f, 0x06, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0f, 0x18, 0x26, 0x0a, 0x0c, 0x0a,
    0x05, 0x06, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0f, 0x31, 0x36, 0x62, 0x06, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x33,
];
include!("aptos.remote_executor.v1.serde.rs");
include!("aptos.remote_executor.v1.tonic.rs");
// @@protoc_insertion_point(module)