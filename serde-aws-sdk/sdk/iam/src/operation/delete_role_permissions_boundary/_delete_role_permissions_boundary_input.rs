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
pub struct DeleteRolePermissionsBoundaryInput {
    /// <p>The name (friendly name, not ARN) of the IAM role from which you want to remove the permissions boundary.</p>
    #[doc(hidden)]
    pub role_name: std::option::Option<std::string::String>,
}
impl DeleteRolePermissionsBoundaryInput {
    /// <p>The name (friendly name, not ARN) of the IAM role from which you want to remove the permissions boundary.</p>
    pub fn role_name(&self) -> std::option::Option<&str> {
        self.role_name.as_deref()
    }
}
impl DeleteRolePermissionsBoundaryInput {
    /// Creates a new builder-style object to manufacture [`DeleteRolePermissionsBoundaryInput`](crate::operation::delete_role_permissions_boundary::DeleteRolePermissionsBoundaryInput).
    pub fn builder() -> crate::operation::delete_role_permissions_boundary::builders::DeleteRolePermissionsBoundaryInputBuilder{
        crate::operation::delete_role_permissions_boundary::builders::DeleteRolePermissionsBoundaryInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::delete_role_permissions_boundary::DeleteRolePermissionsBoundaryInput;
/// A builder for [`DeleteRolePermissionsBoundaryInput`](crate::operation::delete_role_permissions_boundary::DeleteRolePermissionsBoundaryInput).
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
pub struct DeleteRolePermissionsBoundaryInputBuilder {
    pub(crate) role_name: std::option::Option<std::string::String>,
}
impl DeleteRolePermissionsBoundaryInputBuilder {
    /// <p>The name (friendly name, not ARN) of the IAM role from which you want to remove the permissions boundary.</p>
    pub fn role_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.role_name = Some(input.into());
        self
    }
    /// <p>The name (friendly name, not ARN) of the IAM role from which you want to remove the permissions boundary.</p>
    pub fn set_role_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.role_name = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteRolePermissionsBoundaryInput`](crate::operation::delete_role_permissions_boundary::DeleteRolePermissionsBoundaryInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::delete_role_permissions_boundary::DeleteRolePermissionsBoundaryInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::delete_role_permissions_boundary::DeleteRolePermissionsBoundaryInput {
                role_name: self.role_name
                ,
            }
        )
    }
}