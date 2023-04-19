// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeImport`](crate::operation::describe_import::builders::DescribeImportFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`import_arn(impl Into<String>)`](crate::operation::describe_import::builders::DescribeImportFluentBuilder::import_arn) / [`set_import_arn(Option<String>)`](crate::operation::describe_import::builders::DescribeImportFluentBuilder::set_import_arn): <p> The Amazon Resource Name (ARN) associated with the table you're importing to. </p>
    /// - On success, responds with [`DescribeImportOutput`](crate::operation::describe_import::DescribeImportOutput) with field(s):
    ///   - [`import_table_description(Option<ImportTableDescription>)`](crate::operation::describe_import::DescribeImportOutput::import_table_description): <p> Represents the properties of the table created for the import, and parameters of the import. The import parameters include import status, how many items were processed, and how many errors were encountered. </p>
    /// - On failure, responds with [`SdkError<DescribeImportError>`](crate::operation::describe_import::DescribeImportError)
    pub fn describe_import(
        &self,
    ) -> crate::operation::describe_import::builders::DescribeImportFluentBuilder {
        crate::operation::describe_import::builders::DescribeImportFluentBuilder::new(
            self.handle.clone(),
        )
    }
}