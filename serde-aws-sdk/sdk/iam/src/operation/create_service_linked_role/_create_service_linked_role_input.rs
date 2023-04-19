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
pub struct CreateServiceLinkedRoleInput {
    /// <p>The service principal for the Amazon Web Services service to which this role is attached. You use a string similar to a URL but without the http:// in front. For example: <code>elasticbeanstalk.amazonaws.com</code>. </p>
    /// <p>Service principals are unique and case-sensitive. To find the exact service principal for your service-linked role, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_aws-services-that-work-with-iam.html">Amazon Web Services services that work with IAM</a> in the <i>IAM User Guide</i>. Look for the services that have <b>Yes </b>in the <b>Service-Linked Role</b> column. Choose the <b>Yes</b> link to view the service-linked role documentation for that service.</p>
    #[doc(hidden)]
    pub aws_service_name: std::option::Option<std::string::String>,
    /// <p>The description of the role.</p>
    #[doc(hidden)]
    pub description: std::option::Option<std::string::String>,
    /// <p></p>
    /// <p>A string that you provide, which is combined with the service-provided prefix to form the complete role name. If you make multiple requests for the same service, then you must supply a different <code>CustomSuffix</code> for each request. Otherwise the request fails with a duplicate role name error. For example, you could add <code>-1</code> or <code>-debug</code> to the suffix.</p>
    /// <p>Some services do not support the <code>CustomSuffix</code> parameter. If you provide an optional suffix and the operation fails, try the operation again without the suffix.</p>
    #[doc(hidden)]
    pub custom_suffix: std::option::Option<std::string::String>,
}
impl CreateServiceLinkedRoleInput {
    /// <p>The service principal for the Amazon Web Services service to which this role is attached. You use a string similar to a URL but without the http:// in front. For example: <code>elasticbeanstalk.amazonaws.com</code>. </p>
    /// <p>Service principals are unique and case-sensitive. To find the exact service principal for your service-linked role, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_aws-services-that-work-with-iam.html">Amazon Web Services services that work with IAM</a> in the <i>IAM User Guide</i>. Look for the services that have <b>Yes </b>in the <b>Service-Linked Role</b> column. Choose the <b>Yes</b> link to view the service-linked role documentation for that service.</p>
    pub fn aws_service_name(&self) -> std::option::Option<&str> {
        self.aws_service_name.as_deref()
    }
    /// <p>The description of the role.</p>
    pub fn description(&self) -> std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p></p>
    /// <p>A string that you provide, which is combined with the service-provided prefix to form the complete role name. If you make multiple requests for the same service, then you must supply a different <code>CustomSuffix</code> for each request. Otherwise the request fails with a duplicate role name error. For example, you could add <code>-1</code> or <code>-debug</code> to the suffix.</p>
    /// <p>Some services do not support the <code>CustomSuffix</code> parameter. If you provide an optional suffix and the operation fails, try the operation again without the suffix.</p>
    pub fn custom_suffix(&self) -> std::option::Option<&str> {
        self.custom_suffix.as_deref()
    }
}
impl CreateServiceLinkedRoleInput {
    /// Creates a new builder-style object to manufacture [`CreateServiceLinkedRoleInput`](crate::operation::create_service_linked_role::CreateServiceLinkedRoleInput).
    pub fn builder(
    ) -> crate::operation::create_service_linked_role::builders::CreateServiceLinkedRoleInputBuilder
    {
        crate::operation::create_service_linked_role::builders::CreateServiceLinkedRoleInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::create_service_linked_role::CreateServiceLinkedRoleInput;
/// A builder for [`CreateServiceLinkedRoleInput`](crate::operation::create_service_linked_role::CreateServiceLinkedRoleInput).
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
pub struct CreateServiceLinkedRoleInputBuilder {
    pub(crate) aws_service_name: std::option::Option<std::string::String>,
    pub(crate) description: std::option::Option<std::string::String>,
    pub(crate) custom_suffix: std::option::Option<std::string::String>,
}
impl CreateServiceLinkedRoleInputBuilder {
    /// <p>The service principal for the Amazon Web Services service to which this role is attached. You use a string similar to a URL but without the http:// in front. For example: <code>elasticbeanstalk.amazonaws.com</code>. </p>
    /// <p>Service principals are unique and case-sensitive. To find the exact service principal for your service-linked role, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_aws-services-that-work-with-iam.html">Amazon Web Services services that work with IAM</a> in the <i>IAM User Guide</i>. Look for the services that have <b>Yes </b>in the <b>Service-Linked Role</b> column. Choose the <b>Yes</b> link to view the service-linked role documentation for that service.</p>
    pub fn aws_service_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.aws_service_name = Some(input.into());
        self
    }
    /// <p>The service principal for the Amazon Web Services service to which this role is attached. You use a string similar to a URL but without the http:// in front. For example: <code>elasticbeanstalk.amazonaws.com</code>. </p>
    /// <p>Service principals are unique and case-sensitive. To find the exact service principal for your service-linked role, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_aws-services-that-work-with-iam.html">Amazon Web Services services that work with IAM</a> in the <i>IAM User Guide</i>. Look for the services that have <b>Yes </b>in the <b>Service-Linked Role</b> column. Choose the <b>Yes</b> link to view the service-linked role documentation for that service.</p>
    pub fn set_aws_service_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.aws_service_name = input;
        self
    }
    /// <p>The description of the role.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.description = Some(input.into());
        self
    }
    /// <p>The description of the role.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p></p>
    /// <p>A string that you provide, which is combined with the service-provided prefix to form the complete role name. If you make multiple requests for the same service, then you must supply a different <code>CustomSuffix</code> for each request. Otherwise the request fails with a duplicate role name error. For example, you could add <code>-1</code> or <code>-debug</code> to the suffix.</p>
    /// <p>Some services do not support the <code>CustomSuffix</code> parameter. If you provide an optional suffix and the operation fails, try the operation again without the suffix.</p>
    pub fn custom_suffix(mut self, input: impl Into<std::string::String>) -> Self {
        self.custom_suffix = Some(input.into());
        self
    }
    /// <p></p>
    /// <p>A string that you provide, which is combined with the service-provided prefix to form the complete role name. If you make multiple requests for the same service, then you must supply a different <code>CustomSuffix</code> for each request. Otherwise the request fails with a duplicate role name error. For example, you could add <code>-1</code> or <code>-debug</code> to the suffix.</p>
    /// <p>Some services do not support the <code>CustomSuffix</code> parameter. If you provide an optional suffix and the operation fails, try the operation again without the suffix.</p>
    pub fn set_custom_suffix(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.custom_suffix = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateServiceLinkedRoleInput`](crate::operation::create_service_linked_role::CreateServiceLinkedRoleInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::create_service_linked_role::CreateServiceLinkedRoleInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::create_service_linked_role::CreateServiceLinkedRoleInput {
                aws_service_name: self.aws_service_name,
                description: self.description,
                custom_suffix: self.custom_suffix,
            },
        )
    }
}