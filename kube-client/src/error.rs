//! Error handling in [`kube`][crate]
use thiserror::Error;

pub use kube_core::ErrorResponse;

/// Possible errors when working with [`kube`][crate]
#[cfg_attr(docsrs, doc(cfg(any(feature = "config", feature = "client"))))]
#[derive(Error, Debug)]
pub enum Error {
    /// ApiError for when things fail
    ///
    /// This can be parsed into as an error handling fallback.
    /// It's also used in `WatchEvent` from watch calls.
    ///
    /// It's quite common to get a `410 Gone` when the `resourceVersion` is too old.
    #[error("ApiError: {0} ({0:?})")]
    Api(#[source] ErrorResponse),

    /// Hyper error
    #[cfg(feature = "client")]
    #[error("HyperError: {0}")]
    HyperError(#[source] hyper::Error),
    /// Service error
    #[cfg(feature = "client")]
    #[error("ServiceError: {0}")]
    Service(#[source] tower::BoxError),

    /// UTF-8 Error
    #[error("UTF-8 Error: {0}")]
    FromUtf8(#[source] std::string::FromUtf8Error),

    /// Returned when failed to find a newline character within max length.
    /// Only returned by `Client::request_events` and this should never happen as
    /// the max is `usize::MAX`.
    #[error("Error finding newline character")]
    LinesCodecMaxLineLengthExceeded,

    /// Returned on `std::io::Error` when reading event stream.
    #[error("Error reading events stream: {0}")]
    ReadEvents(#[source] std::io::Error),

    /// Http based error
    #[error("HttpError: {0}")]
    HttpError(#[source] http::Error),

    /// Common error case when requesting parsing into own structs
    #[error("Error deserializing response")]
    SerdeError(#[source] serde_json::Error),

    /// Failed to build request
    #[error("Failed to build request: {0}")]
    BuildRequest(#[source] kube_core::request::Error),

    /// Failed to infer config
    #[error("Failed to infer configuration: {0}")]
    InferConfig(#[source] crate::config::InferConfigError),

    /// Discovery errors
    #[error("Error from discovery: {0}")]
    Discovery(#[source] DiscoveryError),

    /// An error with configuring SSL occured
    #[error("SslError: {0}")]
    SslError(String),

    /// An error from openssl when handling configuration
    #[cfg(feature = "native-tls")]
    #[cfg_attr(docsrs, doc(cfg(feature = "native-tls")))]
    #[error("OpensslError: {0}")]
    OpensslError(#[source] openssl::error::ErrorStack),

    /// The server did not respond with [`SWITCHING_PROTOCOLS`] status when upgrading the
    /// connection.
    ///
    /// [`SWITCHING_PROTOCOLS`]: http::status::StatusCode::SWITCHING_PROTOCOLS
    #[cfg(feature = "ws")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ws")))]
    #[error("Failed to switch protocol. Status code: {0}")]
    ProtocolSwitch(http::status::StatusCode),

    /// `Upgrade` header was not set to `websocket` (case insensitive)
    #[cfg(feature = "ws")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ws")))]
    #[error("Upgrade header was not set to websocket")]
    MissingUpgradeWebSocketHeader,

    /// `Connection` header was not set to `Upgrade` (case insensitive)
    #[cfg(feature = "ws")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ws")))]
    #[error("Connection header was not set to Upgrade")]
    MissingConnectionUpgradeHeader,

    /// `Sec-WebSocket-Accept` key mismatched.
    #[cfg(feature = "ws")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ws")))]
    #[error("Sec-WebSocket-Accept key mismatched")]
    SecWebSocketAcceptKeyMismatch,

    /// `Sec-WebSocket-Protocol` mismatched.
    #[cfg(feature = "ws")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ws")))]
    #[error("Sec-WebSocket-Protocol mismatched")]
    SecWebSocketProtocolMismatch,

    /// Errors related to client auth
    #[cfg(feature = "client")]
    #[cfg_attr(docsrs, doc(cfg(feature = "client")))]
    #[error("auth error: {0}")]
    Auth(#[source] crate::client::AuthError),
}

#[derive(Error, Debug)]
// Redundant with the error messages and machine names
#[allow(missing_docs)]
/// Possible errors when using API discovery
pub enum DiscoveryError {
    #[error("Invalid GroupVersion: {0}")]
    InvalidGroupVersion(String),
    #[error("Missing Kind: {0}")]
    MissingKind(String),
    #[error("Missing Api Group: {0}")]
    MissingApiGroup(String),
    #[error("Missing MissingResource: {0}")]
    MissingResource(String),
    #[error("Empty Api Group: {0}")]
    EmptyApiGroup(String),
}
