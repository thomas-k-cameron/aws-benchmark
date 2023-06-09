// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies the default server-side-encryption configuration.</p>
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
pub struct ServerSideEncryptionConfiguration {
    /// <p>Container for information about a particular server-side encryption configuration rule.</p>
    #[doc(hidden)]
    pub rules: std::option::Option<std::vec::Vec<crate::types::ServerSideEncryptionRule>>,
}
impl ServerSideEncryptionConfiguration {
    /// <p>Container for information about a particular server-side encryption configuration rule.</p>
    pub fn rules(&self) -> std::option::Option<&[crate::types::ServerSideEncryptionRule]> {
        self.rules.as_deref()
    }
}
impl ServerSideEncryptionConfiguration {
    /// Creates a new builder-style object to manufacture [`ServerSideEncryptionConfiguration`](crate::types::ServerSideEncryptionConfiguration).
    pub fn builder() -> crate::types::builders::ServerSideEncryptionConfigurationBuilder {
        crate::types::builders::ServerSideEncryptionConfigurationBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::ServerSideEncryptionConfiguration;
/// A builder for [`ServerSideEncryptionConfiguration`](crate::types::ServerSideEncryptionConfiguration).
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
pub struct ServerSideEncryptionConfigurationBuilder {
    pub(crate) rules: std::option::Option<std::vec::Vec<crate::types::ServerSideEncryptionRule>>,
}
impl ServerSideEncryptionConfigurationBuilder {
    /// Appends an item to `rules`.
    ///
    /// To override the contents of this collection use [`set_rules`](Self::set_rules).
    ///
    /// <p>Container for information about a particular server-side encryption configuration rule.</p>
    pub fn rules(mut self, input: crate::types::ServerSideEncryptionRule) -> Self {
        let mut v = self.rules.unwrap_or_default();
        v.push(input);
        self.rules = Some(v);
        self
    }
    /// <p>Container for information about a particular server-side encryption configuration rule.</p>
    pub fn set_rules(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::ServerSideEncryptionRule>>,
    ) -> Self {
        self.rules = input;
        self
    }
    /// Consumes the builder and constructs a [`ServerSideEncryptionConfiguration`](crate::types::ServerSideEncryptionConfiguration).
    pub fn build(self) -> crate::types::ServerSideEncryptionConfiguration {
        crate::types::ServerSideEncryptionConfiguration { rules: self.rules }
    }
}
