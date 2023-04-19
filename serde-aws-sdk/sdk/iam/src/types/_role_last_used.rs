// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information about the last time that an IAM role was used. This includes the date and time and the Region in which the role was last used. Activity is only reported for the trailing 400 days. This period can be shorter if your Region began supporting these features within the last year. The role might have been used more than 400 days ago. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_access-advisor.html#access-advisor_tracking-period">Regions where data is tracked</a> in the <i>IAM user Guide</i>.</p>
/// <p>This data type is returned as a response element in the <code>GetRole</code> and <code>GetAccountAuthorizationDetails</code> operations.</p>
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
pub struct RoleLastUsed {
    /// <p>The date and time, in&nbsp;<a href="http://www.iso.org/iso/iso8601">ISO 8601 date-time format</a> that the role was last used.</p>
    /// <p>This field is null if the role has not been used within the IAM tracking period. For more information about the tracking period, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_access-advisor.html#access-advisor_tracking-period">Regions where data is tracked</a> in the <i>IAM User Guide</i>. </p>
    #[doc(hidden)]
    pub last_used_date: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The name of the Amazon Web Services Region in which the role was last used.</p>
    #[doc(hidden)]
    pub region: std::option::Option<std::string::String>,
}
impl RoleLastUsed {
    /// <p>The date and time, in&nbsp;<a href="http://www.iso.org/iso/iso8601">ISO 8601 date-time format</a> that the role was last used.</p>
    /// <p>This field is null if the role has not been used within the IAM tracking period. For more information about the tracking period, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_access-advisor.html#access-advisor_tracking-period">Regions where data is tracked</a> in the <i>IAM User Guide</i>. </p>
    pub fn last_used_date(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.last_used_date.as_ref()
    }
    /// <p>The name of the Amazon Web Services Region in which the role was last used.</p>
    pub fn region(&self) -> std::option::Option<&str> {
        self.region.as_deref()
    }
}
impl RoleLastUsed {
    /// Creates a new builder-style object to manufacture [`RoleLastUsed`](crate::types::RoleLastUsed).
    pub fn builder() -> crate::types::builders::RoleLastUsedBuilder {
        crate::types::builders::RoleLastUsedBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::RoleLastUsed;
/// A builder for [`RoleLastUsed`](crate::types::RoleLastUsed).
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
pub struct RoleLastUsedBuilder {
    pub(crate) last_used_date: std::option::Option<aws_smithy_types::DateTime>,
    pub(crate) region: std::option::Option<std::string::String>,
}
impl RoleLastUsedBuilder {
    /// <p>The date and time, in&nbsp;<a href="http://www.iso.org/iso/iso8601">ISO 8601 date-time format</a> that the role was last used.</p>
    /// <p>This field is null if the role has not been used within the IAM tracking period. For more information about the tracking period, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_access-advisor.html#access-advisor_tracking-period">Regions where data is tracked</a> in the <i>IAM User Guide</i>. </p>
    pub fn last_used_date(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.last_used_date = Some(input);
        self
    }
    /// <p>The date and time, in&nbsp;<a href="http://www.iso.org/iso/iso8601">ISO 8601 date-time format</a> that the role was last used.</p>
    /// <p>This field is null if the role has not been used within the IAM tracking period. For more information about the tracking period, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_access-advisor.html#access-advisor_tracking-period">Regions where data is tracked</a> in the <i>IAM User Guide</i>. </p>
    pub fn set_last_used_date(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.last_used_date = input;
        self
    }
    /// <p>The name of the Amazon Web Services Region in which the role was last used.</p>
    pub fn region(mut self, input: impl Into<std::string::String>) -> Self {
        self.region = Some(input.into());
        self
    }
    /// <p>The name of the Amazon Web Services Region in which the role was last used.</p>
    pub fn set_region(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.region = input;
        self
    }
    /// Consumes the builder and constructs a [`RoleLastUsed`](crate::types::RoleLastUsed).
    pub fn build(self) -> crate::types::RoleLastUsed {
        crate::types::RoleLastUsed {
            last_used_date: self.last_used_date,
            region: self.region,
        }
    }
}