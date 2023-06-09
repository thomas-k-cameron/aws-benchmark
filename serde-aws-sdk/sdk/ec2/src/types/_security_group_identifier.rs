// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a security group.</p>
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
pub struct SecurityGroupIdentifier {
    /// <p>The ID of the security group.</p>
    #[doc(hidden)]
    pub group_id: std::option::Option<std::string::String>,
    /// <p>The name of the security group.</p>
    #[doc(hidden)]
    pub group_name: std::option::Option<std::string::String>,
}
impl SecurityGroupIdentifier {
    /// <p>The ID of the security group.</p>
    pub fn group_id(&self) -> std::option::Option<&str> {
        self.group_id.as_deref()
    }
    /// <p>The name of the security group.</p>
    pub fn group_name(&self) -> std::option::Option<&str> {
        self.group_name.as_deref()
    }
}
impl SecurityGroupIdentifier {
    /// Creates a new builder-style object to manufacture [`SecurityGroupIdentifier`](crate::types::SecurityGroupIdentifier).
    pub fn builder() -> crate::types::builders::SecurityGroupIdentifierBuilder {
        crate::types::builders::SecurityGroupIdentifierBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::SecurityGroupIdentifier;
/// A builder for [`SecurityGroupIdentifier`](crate::types::SecurityGroupIdentifier).
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
pub struct SecurityGroupIdentifierBuilder {
    pub(crate) group_id: std::option::Option<std::string::String>,
    pub(crate) group_name: std::option::Option<std::string::String>,
}
impl SecurityGroupIdentifierBuilder {
    /// <p>The ID of the security group.</p>
    pub fn group_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.group_id = Some(input.into());
        self
    }
    /// <p>The ID of the security group.</p>
    pub fn set_group_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.group_id = input;
        self
    }
    /// <p>The name of the security group.</p>
    pub fn group_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.group_name = Some(input.into());
        self
    }
    /// <p>The name of the security group.</p>
    pub fn set_group_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.group_name = input;
        self
    }
    /// Consumes the builder and constructs a [`SecurityGroupIdentifier`](crate::types::SecurityGroupIdentifier).
    pub fn build(self) -> crate::types::SecurityGroupIdentifier {
        crate::types::SecurityGroupIdentifier {
            group_id: self.group_id,
            group_name: self.group_name,
        }
    }
}
