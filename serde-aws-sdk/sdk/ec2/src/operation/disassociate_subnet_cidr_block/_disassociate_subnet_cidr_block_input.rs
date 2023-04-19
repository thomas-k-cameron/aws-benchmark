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
pub struct DisassociateSubnetCidrBlockInput {
    /// <p>The association ID for the CIDR block.</p>
    #[doc(hidden)]
    pub association_id: std::option::Option<std::string::String>,
}
impl DisassociateSubnetCidrBlockInput {
    /// <p>The association ID for the CIDR block.</p>
    pub fn association_id(&self) -> std::option::Option<&str> {
        self.association_id.as_deref()
    }
}
impl DisassociateSubnetCidrBlockInput {
    /// Creates a new builder-style object to manufacture [`DisassociateSubnetCidrBlockInput`](crate::operation::disassociate_subnet_cidr_block::DisassociateSubnetCidrBlockInput).
    pub fn builder() -> crate::operation::disassociate_subnet_cidr_block::builders::DisassociateSubnetCidrBlockInputBuilder{
        crate::operation::disassociate_subnet_cidr_block::builders::DisassociateSubnetCidrBlockInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::disassociate_subnet_cidr_block::DisassociateSubnetCidrBlockInput;
/// A builder for [`DisassociateSubnetCidrBlockInput`](crate::operation::disassociate_subnet_cidr_block::DisassociateSubnetCidrBlockInput).
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
pub struct DisassociateSubnetCidrBlockInputBuilder {
    pub(crate) association_id: std::option::Option<std::string::String>,
}
impl DisassociateSubnetCidrBlockInputBuilder {
    /// <p>The association ID for the CIDR block.</p>
    pub fn association_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.association_id = Some(input.into());
        self
    }
    /// <p>The association ID for the CIDR block.</p>
    pub fn set_association_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.association_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DisassociateSubnetCidrBlockInput`](crate::operation::disassociate_subnet_cidr_block::DisassociateSubnetCidrBlockInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::disassociate_subnet_cidr_block::DisassociateSubnetCidrBlockInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::disassociate_subnet_cidr_block::DisassociateSubnetCidrBlockInput {
                association_id: self.association_id,
            },
        )
    }
}