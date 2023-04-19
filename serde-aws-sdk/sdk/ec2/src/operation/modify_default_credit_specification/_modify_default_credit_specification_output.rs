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
pub struct ModifyDefaultCreditSpecificationOutput {
    /// <p>The default credit option for CPU usage of the instance family.</p>
    #[doc(hidden)]
    pub instance_family_credit_specification:
        std::option::Option<crate::types::InstanceFamilyCreditSpecification>,
    _request_id: Option<String>,
}
impl ModifyDefaultCreditSpecificationOutput {
    /// <p>The default credit option for CPU usage of the instance family.</p>
    pub fn instance_family_credit_specification(
        &self,
    ) -> std::option::Option<&crate::types::InstanceFamilyCreditSpecification> {
        self.instance_family_credit_specification.as_ref()
    }
}
impl aws_http::request_id::RequestId for ModifyDefaultCreditSpecificationOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ModifyDefaultCreditSpecificationOutput {
    /// Creates a new builder-style object to manufacture [`ModifyDefaultCreditSpecificationOutput`](crate::operation::modify_default_credit_specification::ModifyDefaultCreditSpecificationOutput).
    pub fn builder() -> crate::operation::modify_default_credit_specification::builders::ModifyDefaultCreditSpecificationOutputBuilder{
        crate::operation::modify_default_credit_specification::builders::ModifyDefaultCreditSpecificationOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::modify_default_credit_specification::ModifyDefaultCreditSpecificationOutput;
/// A builder for [`ModifyDefaultCreditSpecificationOutput`](crate::operation::modify_default_credit_specification::ModifyDefaultCreditSpecificationOutput).
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
pub struct ModifyDefaultCreditSpecificationOutputBuilder {
    pub(crate) instance_family_credit_specification:
        std::option::Option<crate::types::InstanceFamilyCreditSpecification>,
    _request_id: Option<String>,
}
impl ModifyDefaultCreditSpecificationOutputBuilder {
    /// <p>The default credit option for CPU usage of the instance family.</p>
    pub fn instance_family_credit_specification(
        mut self,
        input: crate::types::InstanceFamilyCreditSpecification,
    ) -> Self {
        self.instance_family_credit_specification = Some(input);
        self
    }
    /// <p>The default credit option for CPU usage of the instance family.</p>
    pub fn set_instance_family_credit_specification(
        mut self,
        input: std::option::Option<crate::types::InstanceFamilyCreditSpecification>,
    ) -> Self {
        self.instance_family_credit_specification = input;
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
    /// Consumes the builder and constructs a [`ModifyDefaultCreditSpecificationOutput`](crate::operation::modify_default_credit_specification::ModifyDefaultCreditSpecificationOutput).
    pub fn build(
        self,
    ) -> crate::operation::modify_default_credit_specification::ModifyDefaultCreditSpecificationOutput
    {
        crate::operation::modify_default_credit_specification::ModifyDefaultCreditSpecificationOutput {
            instance_family_credit_specification: self.instance_family_credit_specification
            ,
            _request_id: self._request_id,
        }
    }
}