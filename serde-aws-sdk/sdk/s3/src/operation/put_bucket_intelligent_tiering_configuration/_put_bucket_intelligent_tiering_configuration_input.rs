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
pub struct PutBucketIntelligentTieringConfigurationInput {
    /// <p>The name of the Amazon S3 bucket whose configuration you want to modify or retrieve.</p>
    #[doc(hidden)]
    pub bucket: std::option::Option<std::string::String>,
    /// <p>The ID used to identify the S3 Intelligent-Tiering configuration.</p>
    #[doc(hidden)]
    pub id: std::option::Option<std::string::String>,
    /// <p>Container for S3 Intelligent-Tiering configuration.</p>
    #[doc(hidden)]
    pub intelligent_tiering_configuration:
        std::option::Option<crate::types::IntelligentTieringConfiguration>,
}
impl PutBucketIntelligentTieringConfigurationInput {
    /// <p>The name of the Amazon S3 bucket whose configuration you want to modify or retrieve.</p>
    pub fn bucket(&self) -> std::option::Option<&str> {
        self.bucket.as_deref()
    }
    /// <p>The ID used to identify the S3 Intelligent-Tiering configuration.</p>
    pub fn id(&self) -> std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>Container for S3 Intelligent-Tiering configuration.</p>
    pub fn intelligent_tiering_configuration(
        &self,
    ) -> std::option::Option<&crate::types::IntelligentTieringConfiguration> {
        self.intelligent_tiering_configuration.as_ref()
    }
}
impl PutBucketIntelligentTieringConfigurationInput {
    /// Creates a new builder-style object to manufacture [`PutBucketIntelligentTieringConfigurationInput`](crate::operation::put_bucket_intelligent_tiering_configuration::PutBucketIntelligentTieringConfigurationInput).
    pub fn builder() -> crate::operation::put_bucket_intelligent_tiering_configuration::builders::PutBucketIntelligentTieringConfigurationInputBuilder{
        crate::operation::put_bucket_intelligent_tiering_configuration::builders::PutBucketIntelligentTieringConfigurationInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::put_bucket_intelligent_tiering_configuration::PutBucketIntelligentTieringConfigurationInput;
/// A builder for [`PutBucketIntelligentTieringConfigurationInput`](crate::operation::put_bucket_intelligent_tiering_configuration::PutBucketIntelligentTieringConfigurationInput).
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
pub struct PutBucketIntelligentTieringConfigurationInputBuilder {
    pub(crate) bucket: std::option::Option<std::string::String>,
    pub(crate) id: std::option::Option<std::string::String>,
    pub(crate) intelligent_tiering_configuration:
        std::option::Option<crate::types::IntelligentTieringConfiguration>,
}
impl PutBucketIntelligentTieringConfigurationInputBuilder {
    /// <p>The name of the Amazon S3 bucket whose configuration you want to modify or retrieve.</p>
    pub fn bucket(mut self, input: impl Into<std::string::String>) -> Self {
        self.bucket = Some(input.into());
        self
    }
    /// <p>The name of the Amazon S3 bucket whose configuration you want to modify or retrieve.</p>
    pub fn set_bucket(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.bucket = input;
        self
    }
    /// <p>The ID used to identify the S3 Intelligent-Tiering configuration.</p>
    pub fn id(mut self, input: impl Into<std::string::String>) -> Self {
        self.id = Some(input.into());
        self
    }
    /// <p>The ID used to identify the S3 Intelligent-Tiering configuration.</p>
    pub fn set_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>Container for S3 Intelligent-Tiering configuration.</p>
    pub fn intelligent_tiering_configuration(
        mut self,
        input: crate::types::IntelligentTieringConfiguration,
    ) -> Self {
        self.intelligent_tiering_configuration = Some(input);
        self
    }
    /// <p>Container for S3 Intelligent-Tiering configuration.</p>
    pub fn set_intelligent_tiering_configuration(
        mut self,
        input: std::option::Option<crate::types::IntelligentTieringConfiguration>,
    ) -> Self {
        self.intelligent_tiering_configuration = input;
        self
    }
    /// Consumes the builder and constructs a [`PutBucketIntelligentTieringConfigurationInput`](crate::operation::put_bucket_intelligent_tiering_configuration::PutBucketIntelligentTieringConfigurationInput).
    pub fn build(self) -> Result<crate::operation::put_bucket_intelligent_tiering_configuration::PutBucketIntelligentTieringConfigurationInput, aws_smithy_http::operation::error::BuildError>{
        Ok(
            crate::operation::put_bucket_intelligent_tiering_configuration::PutBucketIntelligentTieringConfigurationInput {
                bucket: self.bucket
                ,
                id: self.id
                ,
                intelligent_tiering_configuration: self.intelligent_tiering_configuration
                ,
            }
        )
    }
}