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
pub struct CreateCidrCollectionInput {
    /// <p>A unique identifier for the account that can be used to reference the collection from other API calls.</p>
    #[doc(hidden)]
    pub name: std::option::Option<std::string::String>,
    /// <p>A client-specific token that allows requests to be securely retried so that the intended outcome will only occur once, retries receive a similar response, and there are no additional edge cases to handle.</p>
    #[doc(hidden)]
    pub caller_reference: std::option::Option<std::string::String>,
}
impl CreateCidrCollectionInput {
    /// <p>A unique identifier for the account that can be used to reference the collection from other API calls.</p>
    pub fn name(&self) -> std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>A client-specific token that allows requests to be securely retried so that the intended outcome will only occur once, retries receive a similar response, and there are no additional edge cases to handle.</p>
    pub fn caller_reference(&self) -> std::option::Option<&str> {
        self.caller_reference.as_deref()
    }
}
impl CreateCidrCollectionInput {
    /// Creates a new builder-style object to manufacture [`CreateCidrCollectionInput`](crate::operation::create_cidr_collection::CreateCidrCollectionInput).
    pub fn builder(
    ) -> crate::operation::create_cidr_collection::builders::CreateCidrCollectionInputBuilder {
        crate::operation::create_cidr_collection::builders::CreateCidrCollectionInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::create_cidr_collection::CreateCidrCollectionInput;
/// A builder for [`CreateCidrCollectionInput`](crate::operation::create_cidr_collection::CreateCidrCollectionInput).
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
pub struct CreateCidrCollectionInputBuilder {
    pub(crate) name: std::option::Option<std::string::String>,
    pub(crate) caller_reference: std::option::Option<std::string::String>,
}
impl CreateCidrCollectionInputBuilder {
    /// <p>A unique identifier for the account that can be used to reference the collection from other API calls.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.name = Some(input.into());
        self
    }
    /// <p>A unique identifier for the account that can be used to reference the collection from other API calls.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>A client-specific token that allows requests to be securely retried so that the intended outcome will only occur once, retries receive a similar response, and there are no additional edge cases to handle.</p>
    pub fn caller_reference(mut self, input: impl Into<std::string::String>) -> Self {
        self.caller_reference = Some(input.into());
        self
    }
    /// <p>A client-specific token that allows requests to be securely retried so that the intended outcome will only occur once, retries receive a similar response, and there are no additional edge cases to handle.</p>
    pub fn set_caller_reference(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.caller_reference = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateCidrCollectionInput`](crate::operation::create_cidr_collection::CreateCidrCollectionInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::create_cidr_collection::CreateCidrCollectionInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::create_cidr_collection::CreateCidrCollectionInput {
                name: self.name,
                caller_reference: self.caller_reference,
            },
        )
    }
}