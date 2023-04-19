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
pub struct SubmitJobOutput {
    /// <p>The Amazon Resource Name (ARN) for the job.</p>
    #[doc(hidden)]
    pub job_arn: std::option::Option<std::string::String>,
    /// <p>The name of the job.</p>
    #[doc(hidden)]
    pub job_name: std::option::Option<std::string::String>,
    /// <p>The unique identifier for the job.</p>
    #[doc(hidden)]
    pub job_id: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl SubmitJobOutput {
    /// <p>The Amazon Resource Name (ARN) for the job.</p>
    pub fn job_arn(&self) -> std::option::Option<&str> {
        self.job_arn.as_deref()
    }
    /// <p>The name of the job.</p>
    pub fn job_name(&self) -> std::option::Option<&str> {
        self.job_name.as_deref()
    }
    /// <p>The unique identifier for the job.</p>
    pub fn job_id(&self) -> std::option::Option<&str> {
        self.job_id.as_deref()
    }
}
impl aws_http::request_id::RequestId for SubmitJobOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl SubmitJobOutput {
    /// Creates a new builder-style object to manufacture [`SubmitJobOutput`](crate::operation::submit_job::SubmitJobOutput).
    pub fn builder() -> crate::operation::submit_job::builders::SubmitJobOutputBuilder {
        crate::operation::submit_job::builders::SubmitJobOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::submit_job::SubmitJobOutput;
/// A builder for [`SubmitJobOutput`](crate::operation::submit_job::SubmitJobOutput).
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
pub struct SubmitJobOutputBuilder {
    pub(crate) job_arn: std::option::Option<std::string::String>,
    pub(crate) job_name: std::option::Option<std::string::String>,
    pub(crate) job_id: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl SubmitJobOutputBuilder {
    /// <p>The Amazon Resource Name (ARN) for the job.</p>
    pub fn job_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.job_arn = Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) for the job.</p>
    pub fn set_job_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.job_arn = input;
        self
    }
    /// <p>The name of the job.</p>
    pub fn job_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.job_name = Some(input.into());
        self
    }
    /// <p>The name of the job.</p>
    pub fn set_job_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.job_name = input;
        self
    }
    /// <p>The unique identifier for the job.</p>
    pub fn job_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.job_id = Some(input.into());
        self
    }
    /// <p>The unique identifier for the job.</p>
    pub fn set_job_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.job_id = input;
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
    /// Consumes the builder and constructs a [`SubmitJobOutput`](crate::operation::submit_job::SubmitJobOutput).
    pub fn build(self) -> crate::operation::submit_job::SubmitJobOutput {
        crate::operation::submit_job::SubmitJobOutput {
            job_arn: self.job_arn,
            job_name: self.job_name,
            job_id: self.job_id,
            _request_id: self._request_id,
        }
    }
}