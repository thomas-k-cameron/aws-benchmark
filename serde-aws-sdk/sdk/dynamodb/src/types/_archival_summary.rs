// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains details of a table archival operation.</p>
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
pub struct ArchivalSummary {
    /// <p>The date and time when table archival was initiated by DynamoDB, in UNIX epoch time format.</p>
    #[doc(hidden)]
    pub archival_date_time: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The reason DynamoDB archived the table. Currently, the only possible value is:</p>
    /// <ul>
    /// <li> <p> <code>INACCESSIBLE_ENCRYPTION_CREDENTIALS</code> - The table was archived due to the table's KMS key being inaccessible for more than seven days. An On-Demand backup was created at the archival time.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub archival_reason: std::option::Option<std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the backup the table was archived to, when applicable in the archival reason. If you wish to restore this backup to the same table name, you will need to delete the original table.</p>
    #[doc(hidden)]
    pub archival_backup_arn: std::option::Option<std::string::String>,
}
impl ArchivalSummary {
    /// <p>The date and time when table archival was initiated by DynamoDB, in UNIX epoch time format.</p>
    pub fn archival_date_time(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.archival_date_time.as_ref()
    }
    /// <p>The reason DynamoDB archived the table. Currently, the only possible value is:</p>
    /// <ul>
    /// <li> <p> <code>INACCESSIBLE_ENCRYPTION_CREDENTIALS</code> - The table was archived due to the table's KMS key being inaccessible for more than seven days. An On-Demand backup was created at the archival time.</p> </li>
    /// </ul>
    pub fn archival_reason(&self) -> std::option::Option<&str> {
        self.archival_reason.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the backup the table was archived to, when applicable in the archival reason. If you wish to restore this backup to the same table name, you will need to delete the original table.</p>
    pub fn archival_backup_arn(&self) -> std::option::Option<&str> {
        self.archival_backup_arn.as_deref()
    }
}
impl ArchivalSummary {
    /// Creates a new builder-style object to manufacture [`ArchivalSummary`](crate::types::ArchivalSummary).
    pub fn builder() -> crate::types::builders::ArchivalSummaryBuilder {
        crate::types::builders::ArchivalSummaryBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::ArchivalSummary;
/// A builder for [`ArchivalSummary`](crate::types::ArchivalSummary).
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
pub struct ArchivalSummaryBuilder {
    pub(crate) archival_date_time: std::option::Option<aws_smithy_types::DateTime>,
    pub(crate) archival_reason: std::option::Option<std::string::String>,
    pub(crate) archival_backup_arn: std::option::Option<std::string::String>,
}
impl ArchivalSummaryBuilder {
    /// <p>The date and time when table archival was initiated by DynamoDB, in UNIX epoch time format.</p>
    pub fn archival_date_time(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.archival_date_time = Some(input);
        self
    }
    /// <p>The date and time when table archival was initiated by DynamoDB, in UNIX epoch time format.</p>
    pub fn set_archival_date_time(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.archival_date_time = input;
        self
    }
    /// <p>The reason DynamoDB archived the table. Currently, the only possible value is:</p>
    /// <ul>
    /// <li> <p> <code>INACCESSIBLE_ENCRYPTION_CREDENTIALS</code> - The table was archived due to the table's KMS key being inaccessible for more than seven days. An On-Demand backup was created at the archival time.</p> </li>
    /// </ul>
    pub fn archival_reason(mut self, input: impl Into<std::string::String>) -> Self {
        self.archival_reason = Some(input.into());
        self
    }
    /// <p>The reason DynamoDB archived the table. Currently, the only possible value is:</p>
    /// <ul>
    /// <li> <p> <code>INACCESSIBLE_ENCRYPTION_CREDENTIALS</code> - The table was archived due to the table's KMS key being inaccessible for more than seven days. An On-Demand backup was created at the archival time.</p> </li>
    /// </ul>
    pub fn set_archival_reason(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.archival_reason = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the backup the table was archived to, when applicable in the archival reason. If you wish to restore this backup to the same table name, you will need to delete the original table.</p>
    pub fn archival_backup_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.archival_backup_arn = Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the backup the table was archived to, when applicable in the archival reason. If you wish to restore this backup to the same table name, you will need to delete the original table.</p>
    pub fn set_archival_backup_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.archival_backup_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`ArchivalSummary`](crate::types::ArchivalSummary).
    pub fn build(self) -> crate::types::ArchivalSummary {
        crate::types::ArchivalSummary {
            archival_date_time: self.archival_date_time,
            archival_reason: self.archival_reason,
            archival_backup_arn: self.archival_backup_arn,
        }
    }
}