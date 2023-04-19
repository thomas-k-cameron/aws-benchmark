// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A complex type that contains information about the health checks or hosted zones for which you want to list tags.</p>
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
pub struct ListTagsForResourcesInput {
    /// <p>The type of the resources.</p>
    /// <ul>
    /// <li> <p>The resource type for health checks is <code>healthcheck</code>.</p> </li>
    /// <li> <p>The resource type for hosted zones is <code>hostedzone</code>.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub resource_type: std::option::Option<crate::types::TagResourceType>,
    /// <p>A complex type that contains the ResourceId element for each resource for which you want to get a list of tags.</p>
    #[doc(hidden)]
    pub resource_ids: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl ListTagsForResourcesInput {
    /// <p>The type of the resources.</p>
    /// <ul>
    /// <li> <p>The resource type for health checks is <code>healthcheck</code>.</p> </li>
    /// <li> <p>The resource type for hosted zones is <code>hostedzone</code>.</p> </li>
    /// </ul>
    pub fn resource_type(&self) -> std::option::Option<&crate::types::TagResourceType> {
        self.resource_type.as_ref()
    }
    /// <p>A complex type that contains the ResourceId element for each resource for which you want to get a list of tags.</p>
    pub fn resource_ids(&self) -> std::option::Option<&[std::string::String]> {
        self.resource_ids.as_deref()
    }
}
impl ListTagsForResourcesInput {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourcesInput`](crate::operation::list_tags_for_resources::ListTagsForResourcesInput).
    pub fn builder(
    ) -> crate::operation::list_tags_for_resources::builders::ListTagsForResourcesInputBuilder {
        crate::operation::list_tags_for_resources::builders::ListTagsForResourcesInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::list_tags_for_resources::ListTagsForResourcesInput;
/// A builder for [`ListTagsForResourcesInput`](crate::operation::list_tags_for_resources::ListTagsForResourcesInput).
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
pub struct ListTagsForResourcesInputBuilder {
    pub(crate) resource_type: std::option::Option<crate::types::TagResourceType>,
    pub(crate) resource_ids: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl ListTagsForResourcesInputBuilder {
    /// <p>The type of the resources.</p>
    /// <ul>
    /// <li> <p>The resource type for health checks is <code>healthcheck</code>.</p> </li>
    /// <li> <p>The resource type for hosted zones is <code>hostedzone</code>.</p> </li>
    /// </ul>
    pub fn resource_type(mut self, input: crate::types::TagResourceType) -> Self {
        self.resource_type = Some(input);
        self
    }
    /// <p>The type of the resources.</p>
    /// <ul>
    /// <li> <p>The resource type for health checks is <code>healthcheck</code>.</p> </li>
    /// <li> <p>The resource type for hosted zones is <code>hostedzone</code>.</p> </li>
    /// </ul>
    pub fn set_resource_type(
        mut self,
        input: std::option::Option<crate::types::TagResourceType>,
    ) -> Self {
        self.resource_type = input;
        self
    }
    /// Appends an item to `resource_ids`.
    ///
    /// To override the contents of this collection use [`set_resource_ids`](Self::set_resource_ids).
    ///
    /// <p>A complex type that contains the ResourceId element for each resource for which you want to get a list of tags.</p>
    pub fn resource_ids(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.resource_ids.unwrap_or_default();
        v.push(input.into());
        self.resource_ids = Some(v);
        self
    }
    /// <p>A complex type that contains the ResourceId element for each resource for which you want to get a list of tags.</p>
    pub fn set_resource_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.resource_ids = input;
        self
    }
    /// Consumes the builder and constructs a [`ListTagsForResourcesInput`](crate::operation::list_tags_for_resources::ListTagsForResourcesInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::list_tags_for_resources::ListTagsForResourcesInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::list_tags_for_resources::ListTagsForResourcesInput {
                resource_type: self.resource_type,
                resource_ids: self.resource_ids,
            },
        )
    }
}