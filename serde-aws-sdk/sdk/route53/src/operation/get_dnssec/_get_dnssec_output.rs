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
pub struct GetDnssecOutput {
    /// <p>A string repesenting the status of DNSSEC.</p>
    #[doc(hidden)]
    pub status: std::option::Option<crate::types::DnssecStatus>,
    /// <p>The key-signing keys (KSKs) in your account.</p>
    #[doc(hidden)]
    pub key_signing_keys: std::option::Option<std::vec::Vec<crate::types::KeySigningKey>>,
    _request_id: Option<String>,
}
impl GetDnssecOutput {
    /// <p>A string repesenting the status of DNSSEC.</p>
    pub fn status(&self) -> std::option::Option<&crate::types::DnssecStatus> {
        self.status.as_ref()
    }
    /// <p>The key-signing keys (KSKs) in your account.</p>
    pub fn key_signing_keys(&self) -> std::option::Option<&[crate::types::KeySigningKey]> {
        self.key_signing_keys.as_deref()
    }
}
impl aws_http::request_id::RequestId for GetDnssecOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetDnssecOutput {
    /// Creates a new builder-style object to manufacture [`GetDnssecOutput`](crate::operation::get_dnssec::GetDnssecOutput).
    pub fn builder() -> crate::operation::get_dnssec::builders::GetDnssecOutputBuilder {
        crate::operation::get_dnssec::builders::GetDnssecOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_dnssec::GetDnssecOutput;
/// A builder for [`GetDnssecOutput`](crate::operation::get_dnssec::GetDnssecOutput).
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
pub struct GetDnssecOutputBuilder {
    pub(crate) status: std::option::Option<crate::types::DnssecStatus>,
    pub(crate) key_signing_keys: std::option::Option<std::vec::Vec<crate::types::KeySigningKey>>,
    _request_id: Option<String>,
}
impl GetDnssecOutputBuilder {
    /// <p>A string repesenting the status of DNSSEC.</p>
    pub fn status(mut self, input: crate::types::DnssecStatus) -> Self {
        self.status = Some(input);
        self
    }
    /// <p>A string repesenting the status of DNSSEC.</p>
    pub fn set_status(mut self, input: std::option::Option<crate::types::DnssecStatus>) -> Self {
        self.status = input;
        self
    }
    /// Appends an item to `key_signing_keys`.
    ///
    /// To override the contents of this collection use [`set_key_signing_keys`](Self::set_key_signing_keys).
    ///
    /// <p>The key-signing keys (KSKs) in your account.</p>
    pub fn key_signing_keys(mut self, input: crate::types::KeySigningKey) -> Self {
        let mut v = self.key_signing_keys.unwrap_or_default();
        v.push(input);
        self.key_signing_keys = Some(v);
        self
    }
    /// <p>The key-signing keys (KSKs) in your account.</p>
    pub fn set_key_signing_keys(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::KeySigningKey>>,
    ) -> Self {
        self.key_signing_keys = input;
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
    /// Consumes the builder and constructs a [`GetDnssecOutput`](crate::operation::get_dnssec::GetDnssecOutput).
    pub fn build(self) -> crate::operation::get_dnssec::GetDnssecOutput {
        crate::operation::get_dnssec::GetDnssecOutput {
            status: self.status,
            key_signing_keys: self.key_signing_keys,
            _request_id: self._request_id,
        }
    }
}