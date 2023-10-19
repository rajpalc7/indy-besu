pub mod cl;
pub mod did;

pub use cl::{CredentialDefinition, CredentialDefinitionRegistry, Schema, SchemaRegistry};
pub use did::{
    DidDocument, DidDocumentWithMeta, DidRegistry, Service, ServiceEndpoint, StringOrVector,
    VerificationMethod, VerificationMethodOrReference,
};