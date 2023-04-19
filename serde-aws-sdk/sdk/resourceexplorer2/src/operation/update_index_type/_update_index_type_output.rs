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
pub struct UpdateIndexTypeOutput {
    /// <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon resource name (ARN)</a> of the index that you updated.</p>
    #[doc(hidden)]
    pub arn: std::option::Option<std::string::String>,
    /// <p>Specifies the type of the specified index after the operation completes.</p>
    #[doc(hidden)]
    pub r#type: std::option::Option<crate::types::IndexType>,
    /// <p>Indicates the state of the request to update the index. This operation is asynchronous. Call the <code>GetIndex</code> operation to check for changes.</p>
    #[doc(hidden)]
    pub state: std::option::Option<crate::types::IndexState>,
    /// <p>The date and timestamp when the index was last updated.</p>
    #[doc(hidden)]
    pub last_updated_at: std::option::Option<aws_smithy_types::DateTime>,
    _request_id: Option<String>,
}
impl UpdateIndexTypeOutput {
    /// <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon resource name (ARN)</a> of the index that you updated.</p>
    pub fn arn(&self) -> std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>Specifies the type of the specified index after the operation completes.</p>
    pub fn r#type(&self) -> std::option::Option<&crate::types::IndexType> {
        self.r#type.as_ref()
    }
    /// <p>Indicates the state of the request to update the index. This operation is asynchronous. Call the <code>GetIndex</code> operation to check for changes.</p>
    pub fn state(&self) -> std::option::Option<&crate::types::IndexState> {
        self.state.as_ref()
    }
    /// <p>The date and timestamp when the index was last updated.</p>
    pub fn last_updated_at(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.last_updated_at.as_ref()
    }
}
impl aws_http::request_id::RequestId for UpdateIndexTypeOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl UpdateIndexTypeOutput {
    /// Creates a new builder-style object to manufacture [`UpdateIndexTypeOutput`](crate::operation::update_index_type::UpdateIndexTypeOutput).
    pub fn builder() -> crate::operation::update_index_type::builders::UpdateIndexTypeOutputBuilder
    {
        crate::operation::update_index_type::builders::UpdateIndexTypeOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::update_index_type::UpdateIndexTypeOutput;
/// A builder for [`UpdateIndexTypeOutput`](crate::operation::update_index_type::UpdateIndexTypeOutput).
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
pub struct UpdateIndexTypeOutputBuilder {
    pub(crate) arn: std::option::Option<std::string::String>,
    pub(crate) r#type: std::option::Option<crate::types::IndexType>,
    pub(crate) state: std::option::Option<crate::types::IndexState>,
    pub(crate) last_updated_at: std::option::Option<aws_smithy_types::DateTime>,
    _request_id: Option<String>,
}
impl UpdateIndexTypeOutputBuilder {
    /// <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon resource name (ARN)</a> of the index that you updated.</p>
    pub fn arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.arn = Some(input.into());
        self
    }
    /// <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon resource name (ARN)</a> of the index that you updated.</p>
    pub fn set_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>Specifies the type of the specified index after the operation completes.</p>
    pub fn r#type(mut self, input: crate::types::IndexType) -> Self {
        self.r#type = Some(input);
        self
    }
    /// <p>Specifies the type of the specified index after the operation completes.</p>
    pub fn set_type(mut self, input: std::option::Option<crate::types::IndexType>) -> Self {
        self.r#type = input;
        self
    }
    /// <p>Indicates the state of the request to update the index. This operation is asynchronous. Call the <code>GetIndex</code> operation to check for changes.</p>
    pub fn state(mut self, input: crate::types::IndexState) -> Self {
        self.state = Some(input);
        self
    }
    /// <p>Indicates the state of the request to update the index. This operation is asynchronous. Call the <code>GetIndex</code> operation to check for changes.</p>
    pub fn set_state(mut self, input: std::option::Option<crate::types::IndexState>) -> Self {
        self.state = input;
        self
    }
    /// <p>The date and timestamp when the index was last updated.</p>
    pub fn last_updated_at(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.last_updated_at = Some(input);
        self
    }
    /// <p>The date and timestamp when the index was last updated.</p>
    pub fn set_last_updated_at(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.last_updated_at = input;
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
    /// Consumes the builder and constructs a [`UpdateIndexTypeOutput`](crate::operation::update_index_type::UpdateIndexTypeOutput).
    pub fn build(self) -> crate::operation::update_index_type::UpdateIndexTypeOutput {
        crate::operation::update_index_type::UpdateIndexTypeOutput {
            arn: self.arn,
            r#type: self.r#type,
            state: self.state,
            last_updated_at: self.last_updated_at,
            _request_id: self._request_id,
        }
    }
}