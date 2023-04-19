// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the default credit option for CPU usage of a burstable performance instance family.</p>
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
pub struct InstanceFamilyCreditSpecification {
    /// <p>The instance family.</p>
    #[doc(hidden)]
    pub instance_family: std::option::Option<crate::types::UnlimitedSupportedInstanceFamily>,
    /// <p>The default credit option for CPU usage of the instance family. Valid values are <code>standard</code> and <code>unlimited</code>.</p>
    #[doc(hidden)]
    pub cpu_credits: std::option::Option<std::string::String>,
}
impl InstanceFamilyCreditSpecification {
    /// <p>The instance family.</p>
    pub fn instance_family(
        &self,
    ) -> std::option::Option<&crate::types::UnlimitedSupportedInstanceFamily> {
        self.instance_family.as_ref()
    }
    /// <p>The default credit option for CPU usage of the instance family. Valid values are <code>standard</code> and <code>unlimited</code>.</p>
    pub fn cpu_credits(&self) -> std::option::Option<&str> {
        self.cpu_credits.as_deref()
    }
}
impl InstanceFamilyCreditSpecification {
    /// Creates a new builder-style object to manufacture [`InstanceFamilyCreditSpecification`](crate::types::InstanceFamilyCreditSpecification).
    pub fn builder() -> crate::types::builders::InstanceFamilyCreditSpecificationBuilder {
        crate::types::builders::InstanceFamilyCreditSpecificationBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::InstanceFamilyCreditSpecification;
/// A builder for [`InstanceFamilyCreditSpecification`](crate::types::InstanceFamilyCreditSpecification).
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
pub struct InstanceFamilyCreditSpecificationBuilder {
    pub(crate) instance_family: std::option::Option<crate::types::UnlimitedSupportedInstanceFamily>,
    pub(crate) cpu_credits: std::option::Option<std::string::String>,
}
impl InstanceFamilyCreditSpecificationBuilder {
    /// <p>The instance family.</p>
    pub fn instance_family(
        mut self,
        input: crate::types::UnlimitedSupportedInstanceFamily,
    ) -> Self {
        self.instance_family = Some(input);
        self
    }
    /// <p>The instance family.</p>
    pub fn set_instance_family(
        mut self,
        input: std::option::Option<crate::types::UnlimitedSupportedInstanceFamily>,
    ) -> Self {
        self.instance_family = input;
        self
    }
    /// <p>The default credit option for CPU usage of the instance family. Valid values are <code>standard</code> and <code>unlimited</code>.</p>
    pub fn cpu_credits(mut self, input: impl Into<std::string::String>) -> Self {
        self.cpu_credits = Some(input.into());
        self
    }
    /// <p>The default credit option for CPU usage of the instance family. Valid values are <code>standard</code> and <code>unlimited</code>.</p>
    pub fn set_cpu_credits(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.cpu_credits = input;
        self
    }
    /// Consumes the builder and constructs a [`InstanceFamilyCreditSpecification`](crate::types::InstanceFamilyCreditSpecification).
    pub fn build(self) -> crate::types::InstanceFamilyCreditSpecification {
        crate::types::InstanceFamilyCreditSpecification {
            instance_family: self.instance_family,
            cpu_credits: self.cpu_credits,
        }
    }
}