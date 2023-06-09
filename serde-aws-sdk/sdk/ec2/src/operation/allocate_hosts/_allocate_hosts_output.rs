// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the output of AllocateHosts.</p>
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
pub struct AllocateHostsOutput {
    /// <p>The ID of the allocated Dedicated Host. This is used to launch an instance onto a specific host.</p>
    #[doc(hidden)]
    pub host_ids: std::option::Option<std::vec::Vec<std::string::String>>,
    _request_id: Option<String>,
}
impl AllocateHostsOutput {
    /// <p>The ID of the allocated Dedicated Host. This is used to launch an instance onto a specific host.</p>
    pub fn host_ids(&self) -> std::option::Option<&[std::string::String]> {
        self.host_ids.as_deref()
    }
}
impl aws_http::request_id::RequestId for AllocateHostsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl AllocateHostsOutput {
    /// Creates a new builder-style object to manufacture [`AllocateHostsOutput`](crate::operation::allocate_hosts::AllocateHostsOutput).
    pub fn builder() -> crate::operation::allocate_hosts::builders::AllocateHostsOutputBuilder {
        crate::operation::allocate_hosts::builders::AllocateHostsOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::allocate_hosts::AllocateHostsOutput;
/// A builder for [`AllocateHostsOutput`](crate::operation::allocate_hosts::AllocateHostsOutput).
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
pub struct AllocateHostsOutputBuilder {
    pub(crate) host_ids: std::option::Option<std::vec::Vec<std::string::String>>,
    _request_id: Option<String>,
}
impl AllocateHostsOutputBuilder {
    /// Appends an item to `host_ids`.
    ///
    /// To override the contents of this collection use [`set_host_ids`](Self::set_host_ids).
    ///
    /// <p>The ID of the allocated Dedicated Host. This is used to launch an instance onto a specific host.</p>
    pub fn host_ids(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.host_ids.unwrap_or_default();
        v.push(input.into());
        self.host_ids = Some(v);
        self
    }
    /// <p>The ID of the allocated Dedicated Host. This is used to launch an instance onto a specific host.</p>
    pub fn set_host_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.host_ids = input;
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
    /// Consumes the builder and constructs a [`AllocateHostsOutput`](crate::operation::allocate_hosts::AllocateHostsOutput).
    pub fn build(self) -> crate::operation::allocate_hosts::AllocateHostsOutput {
        crate::operation::allocate_hosts::AllocateHostsOutput {
            host_ids: self.host_ids,
            _request_id: self._request_id,
        }
    }
}
