// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Object Identifier is unique value to identify objects.</p>
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
pub struct ObjectIdentifier {
    /// <p>Key name of the object.</p> <important>
    /// <p>Replacement must be made for object keys containing special characters (such as carriage returns) when using XML requests. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints"> XML related object key constraints</a>.</p>
    /// </important>
    #[doc(hidden)]
    pub key: std::option::Option<std::string::String>,
    /// <p>VersionId for the specific version of the object to delete.</p>
    #[doc(hidden)]
    pub version_id: std::option::Option<std::string::String>,
}
impl ObjectIdentifier {
    /// <p>Key name of the object.</p> <important>
    /// <p>Replacement must be made for object keys containing special characters (such as carriage returns) when using XML requests. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints"> XML related object key constraints</a>.</p>
    /// </important>
    pub fn key(&self) -> std::option::Option<&str> {
        self.key.as_deref()
    }
    /// <p>VersionId for the specific version of the object to delete.</p>
    pub fn version_id(&self) -> std::option::Option<&str> {
        self.version_id.as_deref()
    }
}
impl ObjectIdentifier {
    /// Creates a new builder-style object to manufacture [`ObjectIdentifier`](crate::types::ObjectIdentifier).
    pub fn builder() -> crate::types::builders::ObjectIdentifierBuilder {
        crate::types::builders::ObjectIdentifierBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::ObjectIdentifier;
/// A builder for [`ObjectIdentifier`](crate::types::ObjectIdentifier).
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
pub struct ObjectIdentifierBuilder {
    pub(crate) key: std::option::Option<std::string::String>,
    pub(crate) version_id: std::option::Option<std::string::String>,
}
impl ObjectIdentifierBuilder {
    /// <p>Key name of the object.</p> <important>
    /// <p>Replacement must be made for object keys containing special characters (such as carriage returns) when using XML requests. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints"> XML related object key constraints</a>.</p>
    /// </important>
    pub fn key(mut self, input: impl Into<std::string::String>) -> Self {
        self.key = Some(input.into());
        self
    }
    /// <p>Key name of the object.</p> <important>
    /// <p>Replacement must be made for object keys containing special characters (such as carriage returns) when using XML requests. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints"> XML related object key constraints</a>.</p>
    /// </important>
    pub fn set_key(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.key = input;
        self
    }
    /// <p>VersionId for the specific version of the object to delete.</p>
    pub fn version_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.version_id = Some(input.into());
        self
    }
    /// <p>VersionId for the specific version of the object to delete.</p>
    pub fn set_version_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.version_id = input;
        self
    }
    /// Consumes the builder and constructs a [`ObjectIdentifier`](crate::types::ObjectIdentifier).
    pub fn build(self) -> crate::types::ObjectIdentifier {
        crate::types::ObjectIdentifier {
            key: self.key,
            version_id: self.version_id,
        }
    }
}