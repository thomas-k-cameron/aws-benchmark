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
pub struct AssociateDefaultViewInput {
    /// <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon resource name (ARN)</a> of the view to set as the default for the Amazon Web Services Region and Amazon Web Services account in which you call this operation. The specified view must already exist in the called Region.</p>
    #[doc(hidden)]
    pub view_arn: std::option::Option<std::string::String>,
}
impl AssociateDefaultViewInput {
    /// <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon resource name (ARN)</a> of the view to set as the default for the Amazon Web Services Region and Amazon Web Services account in which you call this operation. The specified view must already exist in the called Region.</p>
    pub fn view_arn(&self) -> std::option::Option<&str> {
        self.view_arn.as_deref()
    }
}
impl AssociateDefaultViewInput {
    /// Creates a new builder-style object to manufacture [`AssociateDefaultViewInput`](crate::operation::associate_default_view::AssociateDefaultViewInput).
    pub fn builder(
    ) -> crate::operation::associate_default_view::builders::AssociateDefaultViewInputBuilder {
        crate::operation::associate_default_view::builders::AssociateDefaultViewInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::associate_default_view::AssociateDefaultViewInput;
/// A builder for [`AssociateDefaultViewInput`](crate::operation::associate_default_view::AssociateDefaultViewInput).
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
pub struct AssociateDefaultViewInputBuilder {
    pub(crate) view_arn: std::option::Option<std::string::String>,
}
impl AssociateDefaultViewInputBuilder {
    /// <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon resource name (ARN)</a> of the view to set as the default for the Amazon Web Services Region and Amazon Web Services account in which you call this operation. The specified view must already exist in the called Region.</p>
    pub fn view_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.view_arn = Some(input.into());
        self
    }
    /// <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon resource name (ARN)</a> of the view to set as the default for the Amazon Web Services Region and Amazon Web Services account in which you call this operation. The specified view must already exist in the called Region.</p>
    pub fn set_view_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.view_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`AssociateDefaultViewInput`](crate::operation::associate_default_view::AssociateDefaultViewInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::associate_default_view::AssociateDefaultViewInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::associate_default_view::AssociateDefaultViewInput {
                view_arn: self.view_arn,
            },
        )
    }
}