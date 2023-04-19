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
pub struct CreateGrantOutput {
    /// <p>The grant token.</p>
    /// <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grant-manage.html#using-grant-token">Using a grant token</a> in the <i>Key Management Service Developer Guide</i>.</p>
    #[doc(hidden)]
    pub grant_token: std::option::Option<std::string::String>,
    /// <p>The unique identifier for the grant.</p>
    /// <p>You can use the <code>GrantId</code> in a <code>ListGrants</code>, <code>RetireGrant</code>, or <code>RevokeGrant</code> operation.</p>
    #[doc(hidden)]
    pub grant_id: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl CreateGrantOutput {
    /// <p>The grant token.</p>
    /// <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grant-manage.html#using-grant-token">Using a grant token</a> in the <i>Key Management Service Developer Guide</i>.</p>
    pub fn grant_token(&self) -> std::option::Option<&str> {
        self.grant_token.as_deref()
    }
    /// <p>The unique identifier for the grant.</p>
    /// <p>You can use the <code>GrantId</code> in a <code>ListGrants</code>, <code>RetireGrant</code>, or <code>RevokeGrant</code> operation.</p>
    pub fn grant_id(&self) -> std::option::Option<&str> {
        self.grant_id.as_deref()
    }
}
impl aws_http::request_id::RequestId for CreateGrantOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateGrantOutput {
    /// Creates a new builder-style object to manufacture [`CreateGrantOutput`](crate::operation::create_grant::CreateGrantOutput).
    pub fn builder() -> crate::operation::create_grant::builders::CreateGrantOutputBuilder {
        crate::operation::create_grant::builders::CreateGrantOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::create_grant::CreateGrantOutput;
/// A builder for [`CreateGrantOutput`](crate::operation::create_grant::CreateGrantOutput).
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
pub struct CreateGrantOutputBuilder {
    pub(crate) grant_token: std::option::Option<std::string::String>,
    pub(crate) grant_id: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl CreateGrantOutputBuilder {
    /// <p>The grant token.</p>
    /// <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grant-manage.html#using-grant-token">Using a grant token</a> in the <i>Key Management Service Developer Guide</i>.</p>
    pub fn grant_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.grant_token = Some(input.into());
        self
    }
    /// <p>The grant token.</p>
    /// <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grant-manage.html#using-grant-token">Using a grant token</a> in the <i>Key Management Service Developer Guide</i>.</p>
    pub fn set_grant_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.grant_token = input;
        self
    }
    /// <p>The unique identifier for the grant.</p>
    /// <p>You can use the <code>GrantId</code> in a <code>ListGrants</code>, <code>RetireGrant</code>, or <code>RevokeGrant</code> operation.</p>
    pub fn grant_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.grant_id = Some(input.into());
        self
    }
    /// <p>The unique identifier for the grant.</p>
    /// <p>You can use the <code>GrantId</code> in a <code>ListGrants</code>, <code>RetireGrant</code>, or <code>RevokeGrant</code> operation.</p>
    pub fn set_grant_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.grant_id = input;
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
    /// Consumes the builder and constructs a [`CreateGrantOutput`](crate::operation::create_grant::CreateGrantOutput).
    pub fn build(self) -> crate::operation::create_grant::CreateGrantOutput {
        crate::operation::create_grant::CreateGrantOutput {
            grant_token: self.grant_token,
            grant_id: self.grant_id,
            _request_id: self._request_id,
        }
    }
}