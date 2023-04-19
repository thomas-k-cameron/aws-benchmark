// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the configuration settings for the modified Reserved Instances.</p>
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
pub struct ReservedInstancesConfiguration {
    /// <p>The Availability Zone for the modified Reserved Instances.</p>
    #[doc(hidden)]
    pub availability_zone: std::option::Option<std::string::String>,
    /// <p>The number of modified Reserved Instances.</p> <note>
    /// <p>This is a required field for a request.</p>
    /// </note>
    #[doc(hidden)]
    pub instance_count: std::option::Option<i32>,
    /// <p>The instance type for the modified Reserved Instances.</p>
    #[doc(hidden)]
    pub instance_type: std::option::Option<crate::types::InstanceType>,
    /// <p>The network platform of the modified Reserved Instances, which is either EC2-Classic or EC2-VPC.</p>
    #[doc(hidden)]
    pub platform: std::option::Option<std::string::String>,
    /// <p>Whether the Reserved Instance is applied to instances in a Region or instances in a specific Availability Zone.</p>
    #[doc(hidden)]
    pub scope: std::option::Option<crate::types::Scope>,
}
impl ReservedInstancesConfiguration {
    /// <p>The Availability Zone for the modified Reserved Instances.</p>
    pub fn availability_zone(&self) -> std::option::Option<&str> {
        self.availability_zone.as_deref()
    }
    /// <p>The number of modified Reserved Instances.</p> <note>
    /// <p>This is a required field for a request.</p>
    /// </note>
    pub fn instance_count(&self) -> std::option::Option<i32> {
        self.instance_count
    }
    /// <p>The instance type for the modified Reserved Instances.</p>
    pub fn instance_type(&self) -> std::option::Option<&crate::types::InstanceType> {
        self.instance_type.as_ref()
    }
    /// <p>The network platform of the modified Reserved Instances, which is either EC2-Classic or EC2-VPC.</p>
    pub fn platform(&self) -> std::option::Option<&str> {
        self.platform.as_deref()
    }
    /// <p>Whether the Reserved Instance is applied to instances in a Region or instances in a specific Availability Zone.</p>
    pub fn scope(&self) -> std::option::Option<&crate::types::Scope> {
        self.scope.as_ref()
    }
}
impl ReservedInstancesConfiguration {
    /// Creates a new builder-style object to manufacture [`ReservedInstancesConfiguration`](crate::types::ReservedInstancesConfiguration).
    pub fn builder() -> crate::types::builders::ReservedInstancesConfigurationBuilder {
        crate::types::builders::ReservedInstancesConfigurationBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::ReservedInstancesConfiguration;
/// A builder for [`ReservedInstancesConfiguration`](crate::types::ReservedInstancesConfiguration).
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
pub struct ReservedInstancesConfigurationBuilder {
    pub(crate) availability_zone: std::option::Option<std::string::String>,
    pub(crate) instance_count: std::option::Option<i32>,
    pub(crate) instance_type: std::option::Option<crate::types::InstanceType>,
    pub(crate) platform: std::option::Option<std::string::String>,
    pub(crate) scope: std::option::Option<crate::types::Scope>,
}
impl ReservedInstancesConfigurationBuilder {
    /// <p>The Availability Zone for the modified Reserved Instances.</p>
    pub fn availability_zone(mut self, input: impl Into<std::string::String>) -> Self {
        self.availability_zone = Some(input.into());
        self
    }
    /// <p>The Availability Zone for the modified Reserved Instances.</p>
    pub fn set_availability_zone(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.availability_zone = input;
        self
    }
    /// <p>The number of modified Reserved Instances.</p> <note>
    /// <p>This is a required field for a request.</p>
    /// </note>
    pub fn instance_count(mut self, input: i32) -> Self {
        self.instance_count = Some(input);
        self
    }
    /// <p>The number of modified Reserved Instances.</p> <note>
    /// <p>This is a required field for a request.</p>
    /// </note>
    pub fn set_instance_count(mut self, input: std::option::Option<i32>) -> Self {
        self.instance_count = input;
        self
    }
    /// <p>The instance type for the modified Reserved Instances.</p>
    pub fn instance_type(mut self, input: crate::types::InstanceType) -> Self {
        self.instance_type = Some(input);
        self
    }
    /// <p>The instance type for the modified Reserved Instances.</p>
    pub fn set_instance_type(
        mut self,
        input: std::option::Option<crate::types::InstanceType>,
    ) -> Self {
        self.instance_type = input;
        self
    }
    /// <p>The network platform of the modified Reserved Instances, which is either EC2-Classic or EC2-VPC.</p>
    pub fn platform(mut self, input: impl Into<std::string::String>) -> Self {
        self.platform = Some(input.into());
        self
    }
    /// <p>The network platform of the modified Reserved Instances, which is either EC2-Classic or EC2-VPC.</p>
    pub fn set_platform(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.platform = input;
        self
    }
    /// <p>Whether the Reserved Instance is applied to instances in a Region or instances in a specific Availability Zone.</p>
    pub fn scope(mut self, input: crate::types::Scope) -> Self {
        self.scope = Some(input);
        self
    }
    /// <p>Whether the Reserved Instance is applied to instances in a Region or instances in a specific Availability Zone.</p>
    pub fn set_scope(mut self, input: std::option::Option<crate::types::Scope>) -> Self {
        self.scope = input;
        self
    }
    /// Consumes the builder and constructs a [`ReservedInstancesConfiguration`](crate::types::ReservedInstancesConfiguration).
    pub fn build(self) -> crate::types::ReservedInstancesConfiguration {
        crate::types::ReservedInstancesConfiguration {
            availability_zone: self.availability_zone,
            instance_count: self.instance_count,
            instance_type: self.instance_type,
            platform: self.platform,
            scope: self.scope,
        }
    }
}