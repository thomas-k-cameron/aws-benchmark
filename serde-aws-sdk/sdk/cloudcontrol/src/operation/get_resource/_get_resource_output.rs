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
pub struct GetResourceOutput {
    /// <p>The name of the resource type.</p>
    #[doc(hidden)]
    pub type_name: std::option::Option<std::string::String>,
    /// <p>Represents information about a provisioned resource.</p>
    #[doc(hidden)]
    pub resource_description: std::option::Option<crate::types::ResourceDescription>,
    _request_id: Option<String>,
}
impl GetResourceOutput {
    /// <p>The name of the resource type.</p>
    pub fn type_name(&self) -> std::option::Option<&str> {
        self.type_name.as_deref()
    }
    /// <p>Represents information about a provisioned resource.</p>
    pub fn resource_description(&self) -> std::option::Option<&crate::types::ResourceDescription> {
        self.resource_description.as_ref()
    }
}
impl aws_http::request_id::RequestId for GetResourceOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetResourceOutput {
    /// Creates a new builder-style object to manufacture [`GetResourceOutput`](crate::operation::get_resource::GetResourceOutput).
    pub fn builder() -> crate::operation::get_resource::builders::GetResourceOutputBuilder {
        crate::operation::get_resource::builders::GetResourceOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_resource::GetResourceOutput;
/// A builder for [`GetResourceOutput`](crate::operation::get_resource::GetResourceOutput).
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
pub struct GetResourceOutputBuilder {
    pub(crate) type_name: std::option::Option<std::string::String>,
    pub(crate) resource_description: std::option::Option<crate::types::ResourceDescription>,
    _request_id: Option<String>,
}
impl GetResourceOutputBuilder {
    /// <p>The name of the resource type.</p>
    pub fn type_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.type_name = Some(input.into());
        self
    }
    /// <p>The name of the resource type.</p>
    pub fn set_type_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.type_name = input;
        self
    }
    /// <p>Represents information about a provisioned resource.</p>
    pub fn resource_description(mut self, input: crate::types::ResourceDescription) -> Self {
        self.resource_description = Some(input);
        self
    }
    /// <p>Represents information about a provisioned resource.</p>
    pub fn set_resource_description(
        mut self,
        input: std::option::Option<crate::types::ResourceDescription>,
    ) -> Self {
        self.resource_description = input;
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
    /// Consumes the builder and constructs a [`GetResourceOutput`](crate::operation::get_resource::GetResourceOutput).
    pub fn build(self) -> crate::operation::get_resource::GetResourceOutput {
        crate::operation::get_resource::GetResourceOutput {
            type_name: self.type_name,
            resource_description: self.resource_description,
            _request_id: self._request_id,
        }
    }
}