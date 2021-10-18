#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(rustdoc::bare_urls)]
//! <p>Amazon Lightsail is the easiest way to get started with Amazon Web Services (AWS) for developers
//! who need to build websites or web applications. It includes everything you need to launch your
//! project quickly - instances (virtual private servers), container services, storage buckets,
//! managed databases, SSD-based block storage, static IP addresses, load balancers, content
//! delivery network (CDN) distributions, DNS management of registered domains, and resource
//! snapshots (backups) - for a low, predictable monthly price.</p>
//! <p>You can manage your Lightsail resources using the Lightsail console, Lightsail API,
//! AWS Command Line Interface (AWS CLI), or SDKs. For more information about Lightsail concepts
//! and tasks, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en_us/articles/lightsail-how-to-set-up-access-keys-to-use-sdk-api-cli">Amazon Lightsail Developer Guide</a>.</p>
//! <p>This API Reference provides detailed information about the actions, data types,
//! parameters, and errors of the Lightsail service. For more information about the supported
//! AWS Regions, endpoints, and service quotas of the Lightsail service, see <a href="https://docs.aws.amazon.com/general/latest/gr/lightsail.html">Amazon Lightsail Endpoints and
//! Quotas</a> in the <i>AWS General Reference</i>.</p>

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

pub use config::Config;

mod aws_endpoint;
#[cfg(feature = "client")]
pub mod client;
pub mod config;
pub mod error;
mod error_meta;
pub mod input;
mod json_deser;
mod json_errors;
mod json_ser;
pub mod model;
mod no_credentials;
pub mod operation;
mod operation_deser;
mod operation_ser;
pub mod output;
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use smithy_http::byte_stream::ByteStream;
pub use smithy_http::result::SdkError;
pub use smithy_types::Blob;
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("lightsail", PKG_VERSION);
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[cfg(feature = "client")]
pub use client::Client;
pub use smithy_http::endpoint::Endpoint;
pub use smithy_types::retry::RetryConfig;