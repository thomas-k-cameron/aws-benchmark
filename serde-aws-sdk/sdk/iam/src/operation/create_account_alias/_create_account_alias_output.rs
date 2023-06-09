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
pub struct CreateAccountAliasOutput {
    _request_id: Option<String>,
}
impl aws_http::request_id::RequestId for CreateAccountAliasOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateAccountAliasOutput {
    /// Creates a new builder-style object to manufacture [`CreateAccountAliasOutput`](crate::operation::create_account_alias::CreateAccountAliasOutput).
    pub fn builder(
    ) -> crate::operation::create_account_alias::builders::CreateAccountAliasOutputBuilder {
        crate::operation::create_account_alias::builders::CreateAccountAliasOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::create_account_alias::CreateAccountAliasOutput;
/// A builder for [`CreateAccountAliasOutput`](crate::operation::create_account_alias::CreateAccountAliasOutput).
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
pub struct CreateAccountAliasOutputBuilder {
    _request_id: Option<String>,
}
impl CreateAccountAliasOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`CreateAccountAliasOutput`](crate::operation::create_account_alias::CreateAccountAliasOutput).
    pub fn build(self) -> crate::operation::create_account_alias::CreateAccountAliasOutput {
        crate::operation::create_account_alias::CreateAccountAliasOutput {
            _request_id: self._request_id,
        }
    }
}
