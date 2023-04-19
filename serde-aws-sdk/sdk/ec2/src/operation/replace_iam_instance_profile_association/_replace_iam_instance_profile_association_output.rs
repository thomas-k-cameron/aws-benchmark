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
pub struct ReplaceIamInstanceProfileAssociationOutput {
    /// <p>Information about the IAM instance profile association.</p>
    #[doc(hidden)]
    pub iam_instance_profile_association:
        std::option::Option<crate::types::IamInstanceProfileAssociation>,
    _request_id: Option<String>,
}
impl ReplaceIamInstanceProfileAssociationOutput {
    /// <p>Information about the IAM instance profile association.</p>
    pub fn iam_instance_profile_association(
        &self,
    ) -> std::option::Option<&crate::types::IamInstanceProfileAssociation> {
        self.iam_instance_profile_association.as_ref()
    }
}
impl aws_http::request_id::RequestId for ReplaceIamInstanceProfileAssociationOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ReplaceIamInstanceProfileAssociationOutput {
    /// Creates a new builder-style object to manufacture [`ReplaceIamInstanceProfileAssociationOutput`](crate::operation::replace_iam_instance_profile_association::ReplaceIamInstanceProfileAssociationOutput).
    pub fn builder() -> crate::operation::replace_iam_instance_profile_association::builders::ReplaceIamInstanceProfileAssociationOutputBuilder{
        crate::operation::replace_iam_instance_profile_association::builders::ReplaceIamInstanceProfileAssociationOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::replace_iam_instance_profile_association::ReplaceIamInstanceProfileAssociationOutput;
/// A builder for [`ReplaceIamInstanceProfileAssociationOutput`](crate::operation::replace_iam_instance_profile_association::ReplaceIamInstanceProfileAssociationOutput).
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
pub struct ReplaceIamInstanceProfileAssociationOutputBuilder {
    pub(crate) iam_instance_profile_association:
        std::option::Option<crate::types::IamInstanceProfileAssociation>,
    _request_id: Option<String>,
}
impl ReplaceIamInstanceProfileAssociationOutputBuilder {
    /// <p>Information about the IAM instance profile association.</p>
    pub fn iam_instance_profile_association(
        mut self,
        input: crate::types::IamInstanceProfileAssociation,
    ) -> Self {
        self.iam_instance_profile_association = Some(input);
        self
    }
    /// <p>Information about the IAM instance profile association.</p>
    pub fn set_iam_instance_profile_association(
        mut self,
        input: std::option::Option<crate::types::IamInstanceProfileAssociation>,
    ) -> Self {
        self.iam_instance_profile_association = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ReplaceIamInstanceProfileAssociationOutput`](crate::operation::replace_iam_instance_profile_association::ReplaceIamInstanceProfileAssociationOutput).
    pub fn build(self) -> crate::operation::replace_iam_instance_profile_association::ReplaceIamInstanceProfileAssociationOutput{
        crate::operation::replace_iam_instance_profile_association::ReplaceIamInstanceProfileAssociationOutput {
            iam_instance_profile_association: self.iam_instance_profile_association
            ,
            _request_id: self._request_id,
        }
    }
}