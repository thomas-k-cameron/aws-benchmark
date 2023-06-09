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
pub struct DescribeInstanceEventNotificationAttributesInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
}
impl DescribeInstanceEventNotificationAttributesInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
}
impl DescribeInstanceEventNotificationAttributesInput {
    /// Creates a new builder-style object to manufacture [`DescribeInstanceEventNotificationAttributesInput`](crate::operation::describe_instance_event_notification_attributes::DescribeInstanceEventNotificationAttributesInput).
    pub fn builder() -> crate::operation::describe_instance_event_notification_attributes::builders::DescribeInstanceEventNotificationAttributesInputBuilder{
        crate::operation::describe_instance_event_notification_attributes::builders::DescribeInstanceEventNotificationAttributesInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::describe_instance_event_notification_attributes::DescribeInstanceEventNotificationAttributesInput;
/// A builder for [`DescribeInstanceEventNotificationAttributesInput`](crate::operation::describe_instance_event_notification_attributes::DescribeInstanceEventNotificationAttributesInput).
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
pub struct DescribeInstanceEventNotificationAttributesInputBuilder {
    pub(crate) dry_run: std::option::Option<bool>,
}
impl DescribeInstanceEventNotificationAttributesInputBuilder {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeInstanceEventNotificationAttributesInput`](crate::operation::describe_instance_event_notification_attributes::DescribeInstanceEventNotificationAttributesInput).
    pub fn build(self) -> Result<crate::operation::describe_instance_event_notification_attributes::DescribeInstanceEventNotificationAttributesInput, aws_smithy_http::operation::error::BuildError>{
        Ok(
            crate::operation::describe_instance_event_notification_attributes::DescribeInstanceEventNotificationAttributesInput {
                dry_run: self.dry_run
                ,
            }
        )
    }
}
