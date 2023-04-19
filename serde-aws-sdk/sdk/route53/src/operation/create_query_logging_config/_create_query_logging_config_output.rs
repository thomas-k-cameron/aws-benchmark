// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct CreateQueryLoggingConfigOutput {
    /// <p>A complex type that contains the ID for a query logging configuration, the ID of the hosted zone that you want to log queries for, and the ARN for the log group that you want Amazon Route 53 to send query logs to.</p>
    #[doc(hidden)]
    pub query_logging_config: std::option::Option<crate::types::QueryLoggingConfig>,
    /// <p>The unique URL representing the new query logging configuration.</p>
    #[doc(hidden)]
    pub location: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl CreateQueryLoggingConfigOutput {
    /// <p>A complex type that contains the ID for a query logging configuration, the ID of the hosted zone that you want to log queries for, and the ARN for the log group that you want Amazon Route 53 to send query logs to.</p>
    pub fn query_logging_config(&self) -> std::option::Option<&crate::types::QueryLoggingConfig> {
        self.query_logging_config.as_ref()
    }
    /// <p>The unique URL representing the new query logging configuration.</p>
    pub fn location(&self) -> std::option::Option<&str> {
        self.location.as_deref()
    }
}
impl aws_http::request_id::RequestId for CreateQueryLoggingConfigOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateQueryLoggingConfigOutput {
    /// Creates a new builder-style object to manufacture [`CreateQueryLoggingConfigOutput`](crate::operation::create_query_logging_config::CreateQueryLoggingConfigOutput).
    pub fn builder() -> crate::operation::create_query_logging_config::builders::CreateQueryLoggingConfigOutputBuilder{
        crate::operation::create_query_logging_config::builders::CreateQueryLoggingConfigOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::create_query_logging_config::CreateQueryLoggingConfigOutput;
/// A builder for [`CreateQueryLoggingConfigOutput`](crate::operation::create_query_logging_config::CreateQueryLoggingConfigOutput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
pub struct CreateQueryLoggingConfigOutputBuilder {
    pub(crate) query_logging_config: std::option::Option<crate::types::QueryLoggingConfig>,
    pub(crate) location: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl CreateQueryLoggingConfigOutputBuilder {
    /// <p>A complex type that contains the ID for a query logging configuration, the ID of the hosted zone that you want to log queries for, and the ARN for the log group that you want Amazon Route 53 to send query logs to.</p>
    pub fn query_logging_config(mut self, input: crate::types::QueryLoggingConfig) -> Self {
        self.query_logging_config = Some(input);
        self
    }
    /// <p>A complex type that contains the ID for a query logging configuration, the ID of the hosted zone that you want to log queries for, and the ARN for the log group that you want Amazon Route 53 to send query logs to.</p>
    pub fn set_query_logging_config(
        mut self,
        input: std::option::Option<crate::types::QueryLoggingConfig>,
    ) -> Self {
        self.query_logging_config = input;
        self
    }
    /// <p>The unique URL representing the new query logging configuration.</p>
    pub fn location(mut self, input: impl Into<std::string::String>) -> Self {
        self.location = Some(input.into());
        self
    }
    /// <p>The unique URL representing the new query logging configuration.</p>
    pub fn set_location(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.location = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`CreateQueryLoggingConfigOutput`](crate::operation::create_query_logging_config::CreateQueryLoggingConfigOutput).
    pub fn build(
        self,
    ) -> crate::operation::create_query_logging_config::CreateQueryLoggingConfigOutput {
        crate::operation::create_query_logging_config::CreateQueryLoggingConfigOutput {
            query_logging_config: self.query_logging_config,
            location: self.location,
            _request_id: self._request_id,
        }
    }
}