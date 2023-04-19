// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteAttributes`](crate::operation::delete_attributes::builders::DeleteAttributesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`cluster(impl Into<String>)`](crate::operation::delete_attributes::builders::DeleteAttributesFluentBuilder::cluster) / [`set_cluster(Option<String>)`](crate::operation::delete_attributes::builders::DeleteAttributesFluentBuilder::set_cluster): <p>The short name or full Amazon Resource Name (ARN) of the cluster that contains the resource to delete attributes. If you do not specify a cluster, the default cluster is assumed.</p>
    ///   - [`attributes(Vec<Attribute>)`](crate::operation::delete_attributes::builders::DeleteAttributesFluentBuilder::attributes) / [`set_attributes(Option<Vec<Attribute>>)`](crate::operation::delete_attributes::builders::DeleteAttributesFluentBuilder::set_attributes): <p>The attributes to delete from your resource. You can specify up to 10 attributes for each request. For custom attributes, specify the attribute name and target ID, but don't specify the value. If you specify the target ID using the short form, you must also specify the target type.</p>
    /// - On success, responds with [`DeleteAttributesOutput`](crate::operation::delete_attributes::DeleteAttributesOutput) with field(s):
    ///   - [`attributes(Option<Vec<Attribute>>)`](crate::operation::delete_attributes::DeleteAttributesOutput::attributes): <p>A list of attribute objects that were successfully deleted from your resource.</p>
    /// - On failure, responds with [`SdkError<DeleteAttributesError>`](crate::operation::delete_attributes::DeleteAttributesError)
    pub fn delete_attributes(
        &self,
    ) -> crate::operation::delete_attributes::builders::DeleteAttributesFluentBuilder {
        crate::operation::delete_attributes::builders::DeleteAttributesFluentBuilder::new(
            self.handle.clone(),
        )
    }
}