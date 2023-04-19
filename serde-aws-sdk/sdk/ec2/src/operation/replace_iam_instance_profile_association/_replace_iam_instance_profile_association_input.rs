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
pub struct ReplaceIamInstanceProfileAssociationInput {
    /// <p>The IAM instance profile.</p>
    #[doc(hidden)]
    pub iam_instance_profile: std::option::Option<crate::types::IamInstanceProfileSpecification>,
    /// <p>The ID of the existing IAM instance profile association.</p>
    #[doc(hidden)]
    pub association_id: std::option::Option<std::string::String>,
}
impl ReplaceIamInstanceProfileAssociationInput {
    /// <p>The IAM instance profile.</p>
    pub fn iam_instance_profile(
        &self,
    ) -> std::option::Option<&crate::types::IamInstanceProfileSpecification> {
        self.iam_instance_profile.as_ref()
    }
    /// <p>The ID of the existing IAM instance profile association.</p>
    pub fn association_id(&self) -> std::option::Option<&str> {
        self.association_id.as_deref()
    }
}
impl ReplaceIamInstanceProfileAssociationInput {
    /// Creates a new builder-style object to manufacture [`ReplaceIamInstanceProfileAssociationInput`](crate::operation::replace_iam_instance_profile_association::ReplaceIamInstanceProfileAssociationInput).
    pub fn builder() -> crate::operation::replace_iam_instance_profile_association::builders::ReplaceIamInstanceProfileAssociationInputBuilder{
        crate::operation::replace_iam_instance_profile_association::builders::ReplaceIamInstanceProfileAssociationInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::replace_iam_instance_profile_association::ReplaceIamInstanceProfileAssociationInput;
/// A builder for [`ReplaceIamInstanceProfileAssociationInput`](crate::operation::replace_iam_instance_profile_association::ReplaceIamInstanceProfileAssociationInput).
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
pub struct ReplaceIamInstanceProfileAssociationInputBuilder {
    pub(crate) iam_instance_profile:
        std::option::Option<crate::types::IamInstanceProfileSpecification>,
    pub(crate) association_id: std::option::Option<std::string::String>,
}
impl ReplaceIamInstanceProfileAssociationInputBuilder {
    /// <p>The IAM instance profile.</p>
    pub fn iam_instance_profile(
        mut self,
        input: crate::types::IamInstanceProfileSpecification,
    ) -> Self {
        self.iam_instance_profile = Some(input);
        self
    }
    /// <p>The IAM instance profile.</p>
    pub fn set_iam_instance_profile(
        mut self,
        input: std::option::Option<crate::types::IamInstanceProfileSpecification>,
    ) -> Self {
        self.iam_instance_profile = input;
        self
    }
    /// <p>The ID of the existing IAM instance profile association.</p>
    pub fn association_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.association_id = Some(input.into());
        self
    }
    /// <p>The ID of the existing IAM instance profile association.</p>
    pub fn set_association_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.association_id = input;
        self
    }
    /// Consumes the builder and constructs a [`ReplaceIamInstanceProfileAssociationInput`](crate::operation::replace_iam_instance_profile_association::ReplaceIamInstanceProfileAssociationInput).
    pub fn build(self) -> Result<crate::operation::replace_iam_instance_profile_association::ReplaceIamInstanceProfileAssociationInput, aws_smithy_http::operation::error::BuildError>{
        Ok(
            crate::operation::replace_iam_instance_profile_association::ReplaceIamInstanceProfileAssociationInput {
                iam_instance_profile: self.iam_instance_profile
                ,
                association_id: self.association_id
                ,
            }
        )
    }
}