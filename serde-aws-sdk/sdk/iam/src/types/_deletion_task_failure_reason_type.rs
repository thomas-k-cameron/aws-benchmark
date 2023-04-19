// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The reason that the service-linked role deletion failed.</p>
/// <p>This data type is used as a response element in the <code>GetServiceLinkedRoleDeletionStatus</code> operation.</p>
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
pub struct DeletionTaskFailureReasonType {
    /// <p>A short description of the reason that the service-linked role deletion failed.</p>
    #[doc(hidden)]
    pub reason: std::option::Option<std::string::String>,
    /// <p>A list of objects that contains details about the service-linked role deletion failure, if that information is returned by the service. If the service-linked role has active sessions or if any resources that were used by the role have not been deleted from the linked service, the role can't be deleted. This parameter includes a list of the resources that are associated with the role and the Region in which the resources are being used.</p>
    #[doc(hidden)]
    pub role_usage_list: std::option::Option<std::vec::Vec<crate::types::RoleUsageType>>,
}
impl DeletionTaskFailureReasonType {
    /// <p>A short description of the reason that the service-linked role deletion failed.</p>
    pub fn reason(&self) -> std::option::Option<&str> {
        self.reason.as_deref()
    }
    /// <p>A list of objects that contains details about the service-linked role deletion failure, if that information is returned by the service. If the service-linked role has active sessions or if any resources that were used by the role have not been deleted from the linked service, the role can't be deleted. This parameter includes a list of the resources that are associated with the role and the Region in which the resources are being used.</p>
    pub fn role_usage_list(&self) -> std::option::Option<&[crate::types::RoleUsageType]> {
        self.role_usage_list.as_deref()
    }
}
impl DeletionTaskFailureReasonType {
    /// Creates a new builder-style object to manufacture [`DeletionTaskFailureReasonType`](crate::types::DeletionTaskFailureReasonType).
    pub fn builder() -> crate::types::builders::DeletionTaskFailureReasonTypeBuilder {
        crate::types::builders::DeletionTaskFailureReasonTypeBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::DeletionTaskFailureReasonType;
/// A builder for [`DeletionTaskFailureReasonType`](crate::types::DeletionTaskFailureReasonType).
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
pub struct DeletionTaskFailureReasonTypeBuilder {
    pub(crate) reason: std::option::Option<std::string::String>,
    pub(crate) role_usage_list: std::option::Option<std::vec::Vec<crate::types::RoleUsageType>>,
}
impl DeletionTaskFailureReasonTypeBuilder {
    /// <p>A short description of the reason that the service-linked role deletion failed.</p>
    pub fn reason(mut self, input: impl Into<std::string::String>) -> Self {
        self.reason = Some(input.into());
        self
    }
    /// <p>A short description of the reason that the service-linked role deletion failed.</p>
    pub fn set_reason(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.reason = input;
        self
    }
    /// Appends an item to `role_usage_list`.
    ///
    /// To override the contents of this collection use [`set_role_usage_list`](Self::set_role_usage_list).
    ///
    /// <p>A list of objects that contains details about the service-linked role deletion failure, if that information is returned by the service. If the service-linked role has active sessions or if any resources that were used by the role have not been deleted from the linked service, the role can't be deleted. This parameter includes a list of the resources that are associated with the role and the Region in which the resources are being used.</p>
    pub fn role_usage_list(mut self, input: crate::types::RoleUsageType) -> Self {
        let mut v = self.role_usage_list.unwrap_or_default();
        v.push(input);
        self.role_usage_list = Some(v);
        self
    }
    /// <p>A list of objects that contains details about the service-linked role deletion failure, if that information is returned by the service. If the service-linked role has active sessions or if any resources that were used by the role have not been deleted from the linked service, the role can't be deleted. This parameter includes a list of the resources that are associated with the role and the Region in which the resources are being used.</p>
    pub fn set_role_usage_list(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::RoleUsageType>>,
    ) -> Self {
        self.role_usage_list = input;
        self
    }
    /// Consumes the builder and constructs a [`DeletionTaskFailureReasonType`](crate::types::DeletionTaskFailureReasonType).
    pub fn build(self) -> crate::types::DeletionTaskFailureReasonType {
        crate::types::DeletionTaskFailureReasonType {
            reason: self.reason,
            role_usage_list: self.role_usage_list,
        }
    }
}