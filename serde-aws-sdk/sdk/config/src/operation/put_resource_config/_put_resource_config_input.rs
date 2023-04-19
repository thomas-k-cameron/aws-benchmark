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
pub struct PutResourceConfigInput {
    /// <p>The type of the resource. The custom resource type must be registered with CloudFormation. </p> <note>
    /// <p>You cannot use the organization names “amzn”, “amazon”, “alexa”, “custom” with custom resource types. It is the first part of the ResourceType up to the first ::.</p>
    /// </note>
    #[doc(hidden)]
    pub resource_type: std::option::Option<std::string::String>,
    /// <p>Version of the schema registered for the ResourceType in CloudFormation.</p>
    #[doc(hidden)]
    pub schema_version_id: std::option::Option<std::string::String>,
    /// <p>Unique identifier of the resource.</p>
    #[doc(hidden)]
    pub resource_id: std::option::Option<std::string::String>,
    /// <p>Name of the resource.</p>
    #[doc(hidden)]
    pub resource_name: std::option::Option<std::string::String>,
    /// <p>The configuration object of the resource in valid JSON format. It must match the schema registered with CloudFormation.</p> <note>
    /// <p>The configuration JSON must not exceed 64 KB.</p>
    /// </note>
    #[doc(hidden)]
    pub configuration: std::option::Option<std::string::String>,
    /// <p>Tags associated with the resource.</p> <note>
    /// <p>This field is not to be confused with the Amazon Web Services-wide tag feature for Amazon Web Services resources. Tags for <code>PutResourceConfig</code> are tags that you supply for the configuration items of your custom resources.</p>
    /// </note>
    #[doc(hidden)]
    pub tags:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl PutResourceConfigInput {
    /// <p>The type of the resource. The custom resource type must be registered with CloudFormation. </p> <note>
    /// <p>You cannot use the organization names “amzn”, “amazon”, “alexa”, “custom” with custom resource types. It is the first part of the ResourceType up to the first ::.</p>
    /// </note>
    pub fn resource_type(&self) -> std::option::Option<&str> {
        self.resource_type.as_deref()
    }
    /// <p>Version of the schema registered for the ResourceType in CloudFormation.</p>
    pub fn schema_version_id(&self) -> std::option::Option<&str> {
        self.schema_version_id.as_deref()
    }
    /// <p>Unique identifier of the resource.</p>
    pub fn resource_id(&self) -> std::option::Option<&str> {
        self.resource_id.as_deref()
    }
    /// <p>Name of the resource.</p>
    pub fn resource_name(&self) -> std::option::Option<&str> {
        self.resource_name.as_deref()
    }
    /// <p>The configuration object of the resource in valid JSON format. It must match the schema registered with CloudFormation.</p> <note>
    /// <p>The configuration JSON must not exceed 64 KB.</p>
    /// </note>
    pub fn configuration(&self) -> std::option::Option<&str> {
        self.configuration.as_deref()
    }
    /// <p>Tags associated with the resource.</p> <note>
    /// <p>This field is not to be confused with the Amazon Web Services-wide tag feature for Amazon Web Services resources. Tags for <code>PutResourceConfig</code> are tags that you supply for the configuration items of your custom resources.</p>
    /// </note>
    pub fn tags(
        &self,
    ) -> std::option::Option<&std::collections::HashMap<std::string::String, std::string::String>>
    {
        self.tags.as_ref()
    }
}
impl PutResourceConfigInput {
    /// Creates a new builder-style object to manufacture [`PutResourceConfigInput`](crate::operation::put_resource_config::PutResourceConfigInput).
    pub fn builder(
    ) -> crate::operation::put_resource_config::builders::PutResourceConfigInputBuilder {
        crate::operation::put_resource_config::builders::PutResourceConfigInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::put_resource_config::PutResourceConfigInput;
/// A builder for [`PutResourceConfigInput`](crate::operation::put_resource_config::PutResourceConfigInput).
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
pub struct PutResourceConfigInputBuilder {
    pub(crate) resource_type: std::option::Option<std::string::String>,
    pub(crate) schema_version_id: std::option::Option<std::string::String>,
    pub(crate) resource_id: std::option::Option<std::string::String>,
    pub(crate) resource_name: std::option::Option<std::string::String>,
    pub(crate) configuration: std::option::Option<std::string::String>,
    pub(crate) tags:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl PutResourceConfigInputBuilder {
    /// <p>The type of the resource. The custom resource type must be registered with CloudFormation. </p> <note>
    /// <p>You cannot use the organization names “amzn”, “amazon”, “alexa”, “custom” with custom resource types. It is the first part of the ResourceType up to the first ::.</p>
    /// </note>
    pub fn resource_type(mut self, input: impl Into<std::string::String>) -> Self {
        self.resource_type = Some(input.into());
        self
    }
    /// <p>The type of the resource. The custom resource type must be registered with CloudFormation. </p> <note>
    /// <p>You cannot use the organization names “amzn”, “amazon”, “alexa”, “custom” with custom resource types. It is the first part of the ResourceType up to the first ::.</p>
    /// </note>
    pub fn set_resource_type(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.resource_type = input;
        self
    }
    /// <p>Version of the schema registered for the ResourceType in CloudFormation.</p>
    pub fn schema_version_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.schema_version_id = Some(input.into());
        self
    }
    /// <p>Version of the schema registered for the ResourceType in CloudFormation.</p>
    pub fn set_schema_version_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.schema_version_id = input;
        self
    }
    /// <p>Unique identifier of the resource.</p>
    pub fn resource_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.resource_id = Some(input.into());
        self
    }
    /// <p>Unique identifier of the resource.</p>
    pub fn set_resource_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.resource_id = input;
        self
    }
    /// <p>Name of the resource.</p>
    pub fn resource_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.resource_name = Some(input.into());
        self
    }
    /// <p>Name of the resource.</p>
    pub fn set_resource_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.resource_name = input;
        self
    }
    /// <p>The configuration object of the resource in valid JSON format. It must match the schema registered with CloudFormation.</p> <note>
    /// <p>The configuration JSON must not exceed 64 KB.</p>
    /// </note>
    pub fn configuration(mut self, input: impl Into<std::string::String>) -> Self {
        self.configuration = Some(input.into());
        self
    }
    /// <p>The configuration object of the resource in valid JSON format. It must match the schema registered with CloudFormation.</p> <note>
    /// <p>The configuration JSON must not exceed 64 KB.</p>
    /// </note>
    pub fn set_configuration(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.configuration = input;
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Tags associated with the resource.</p> <note>
    /// <p>This field is not to be confused with the Amazon Web Services-wide tag feature for Amazon Web Services resources. Tags for <code>PutResourceConfig</code> are tags that you supply for the configuration items of your custom resources.</p>
    /// </note>
    pub fn tags(
        mut self,
        k: impl Into<std::string::String>,
        v: impl Into<std::string::String>,
    ) -> Self {
        let mut hash_map = self.tags.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.tags = Some(hash_map);
        self
    }
    /// <p>Tags associated with the resource.</p> <note>
    /// <p>This field is not to be confused with the Amazon Web Services-wide tag feature for Amazon Web Services resources. Tags for <code>PutResourceConfig</code> are tags that you supply for the configuration items of your custom resources.</p>
    /// </note>
    pub fn set_tags(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`PutResourceConfigInput`](crate::operation::put_resource_config::PutResourceConfigInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::put_resource_config::PutResourceConfigInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::put_resource_config::PutResourceConfigInput {
                resource_type: self.resource_type,
                schema_version_id: self.schema_version_id,
                resource_id: self.resource_id,
                resource_name: self.resource_name,
                configuration: self.configuration,
                tags: self.tags,
            },
        )
    }
}