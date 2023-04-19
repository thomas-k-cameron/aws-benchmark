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
pub struct DeleteLexiconInput {
    /// <p>The name of the lexicon to delete. Must be an existing lexicon in the region.</p>
    #[doc(hidden)]
    pub name: std::option::Option<std::string::String>,
}
impl DeleteLexiconInput {
    /// <p>The name of the lexicon to delete. Must be an existing lexicon in the region.</p>
    pub fn name(&self) -> std::option::Option<&str> {
        self.name.as_deref()
    }
}
impl DeleteLexiconInput {
    /// Creates a new builder-style object to manufacture [`DeleteLexiconInput`](crate::operation::delete_lexicon::DeleteLexiconInput).
    pub fn builder() -> crate::operation::delete_lexicon::builders::DeleteLexiconInputBuilder {
        crate::operation::delete_lexicon::builders::DeleteLexiconInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::delete_lexicon::DeleteLexiconInput;
/// A builder for [`DeleteLexiconInput`](crate::operation::delete_lexicon::DeleteLexiconInput).
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
pub struct DeleteLexiconInputBuilder {
    pub(crate) name: std::option::Option<std::string::String>,
}
impl DeleteLexiconInputBuilder {
    /// <p>The name of the lexicon to delete. Must be an existing lexicon in the region.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.name = Some(input.into());
        self
    }
    /// <p>The name of the lexicon to delete. Must be an existing lexicon in the region.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteLexiconInput`](crate::operation::delete_lexicon::DeleteLexiconInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::delete_lexicon::DeleteLexiconInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::delete_lexicon::DeleteLexiconInput { name: self.name })
    }
}