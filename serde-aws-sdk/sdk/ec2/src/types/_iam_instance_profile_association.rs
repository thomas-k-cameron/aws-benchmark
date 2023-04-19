// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes an association between an IAM instance profile and an instance.</p>
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
pub struct IamInstanceProfileAssociation {
    /// <p>The ID of the association.</p>
    #[doc(hidden)]
    pub association_id: std::option::Option<std::string::String>,
    /// <p>The ID of the instance.</p>
    #[doc(hidden)]
    pub instance_id: std::option::Option<std::string::String>,
    /// <p>The IAM instance profile.</p>
    #[doc(hidden)]
    pub iam_instance_profile: std::option::Option<crate::types::IamInstanceProfile>,
    /// <p>The state of the association.</p>
    #[doc(hidden)]
    pub state: std::option::Option<crate::types::IamInstanceProfileAssociationState>,
    /// <p>The time the IAM instance profile was associated with the instance.</p>
    #[doc(hidden)]
    pub timestamp: std::option::Option<aws_smithy_types::DateTime>,
}
impl IamInstanceProfileAssociation {
    /// <p>The ID of the association.</p>
    pub fn association_id(&self) -> std::option::Option<&str> {
        self.association_id.as_deref()
    }
    /// <p>The ID of the instance.</p>
    pub fn instance_id(&self) -> std::option::Option<&str> {
        self.instance_id.as_deref()
    }
    /// <p>The IAM instance profile.</p>
    pub fn iam_instance_profile(&self) -> std::option::Option<&crate::types::IamInstanceProfile> {
        self.iam_instance_profile.as_ref()
    }
    /// <p>The state of the association.</p>
    pub fn state(&self) -> std::option::Option<&crate::types::IamInstanceProfileAssociationState> {
        self.state.as_ref()
    }
    /// <p>The time the IAM instance profile was associated with the instance.</p>
    pub fn timestamp(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.timestamp.as_ref()
    }
}
impl IamInstanceProfileAssociation {
    /// Creates a new builder-style object to manufacture [`IamInstanceProfileAssociation`](crate::types::IamInstanceProfileAssociation).
    pub fn builder() -> crate::types::builders::IamInstanceProfileAssociationBuilder {
        crate::types::builders::IamInstanceProfileAssociationBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::IamInstanceProfileAssociation;
/// A builder for [`IamInstanceProfileAssociation`](crate::types::IamInstanceProfileAssociation).
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
pub struct IamInstanceProfileAssociationBuilder {
    pub(crate) association_id: std::option::Option<std::string::String>,
    pub(crate) instance_id: std::option::Option<std::string::String>,
    pub(crate) iam_instance_profile: std::option::Option<crate::types::IamInstanceProfile>,
    pub(crate) state: std::option::Option<crate::types::IamInstanceProfileAssociationState>,
    pub(crate) timestamp: std::option::Option<aws_smithy_types::DateTime>,
}
impl IamInstanceProfileAssociationBuilder {
    /// <p>The ID of the association.</p>
    pub fn association_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.association_id = Some(input.into());
        self
    }
    /// <p>The ID of the association.</p>
    pub fn set_association_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.association_id = input;
        self
    }
    /// <p>The ID of the instance.</p>
    pub fn instance_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.instance_id = Some(input.into());
        self
    }
    /// <p>The ID of the instance.</p>
    pub fn set_instance_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.instance_id = input;
        self
    }
    /// <p>The IAM instance profile.</p>
    pub fn iam_instance_profile(mut self, input: crate::types::IamInstanceProfile) -> Self {
        self.iam_instance_profile = Some(input);
        self
    }
    /// <p>The IAM instance profile.</p>
    pub fn set_iam_instance_profile(
        mut self,
        input: std::option::Option<crate::types::IamInstanceProfile>,
    ) -> Self {
        self.iam_instance_profile = input;
        self
    }
    /// <p>The state of the association.</p>
    pub fn state(mut self, input: crate::types::IamInstanceProfileAssociationState) -> Self {
        self.state = Some(input);
        self
    }
    /// <p>The state of the association.</p>
    pub fn set_state(
        mut self,
        input: std::option::Option<crate::types::IamInstanceProfileAssociationState>,
    ) -> Self {
        self.state = input;
        self
    }
    /// <p>The time the IAM instance profile was associated with the instance.</p>
    pub fn timestamp(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.timestamp = Some(input);
        self
    }
    /// <p>The time the IAM instance profile was associated with the instance.</p>
    pub fn set_timestamp(mut self, input: std::option::Option<aws_smithy_types::DateTime>) -> Self {
        self.timestamp = input;
        self
    }
    /// Consumes the builder and constructs a [`IamInstanceProfileAssociation`](crate::types::IamInstanceProfileAssociation).
    pub fn build(self) -> crate::types::IamInstanceProfileAssociation {
        crate::types::IamInstanceProfileAssociation {
            association_id: self.association_id,
            instance_id: self.instance_id,
            iam_instance_profile: self.iam_instance_profile,
            state: self.state,
            timestamp: self.timestamp,
        }
    }
}