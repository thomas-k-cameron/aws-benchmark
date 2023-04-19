// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeReservedInstancesListings`](crate::operation::describe_reserved_instances_listings::builders::DescribeReservedInstancesListingsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`filters(Vec<Filter>)`](crate::operation::describe_reserved_instances_listings::builders::DescribeReservedInstancesListingsFluentBuilder::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::operation::describe_reserved_instances_listings::builders::DescribeReservedInstancesListingsFluentBuilder::set_filters): <p>One or more filters.</p>  <ul>   <li> <p> <code>reserved-instances-id</code> - The ID of the Reserved Instances.</p> </li>   <li> <p> <code>reserved-instances-listing-id</code> - The ID of the Reserved Instances listing.</p> </li>   <li> <p> <code>status</code> - The status of the Reserved Instance listing (<code>pending</code> | <code>active</code> | <code>cancelled</code> | <code>closed</code>).</p> </li>   <li> <p> <code>status-message</code> - The reason for the status.</p> </li>  </ul>
    ///   - [`reserved_instances_id(impl Into<String>)`](crate::operation::describe_reserved_instances_listings::builders::DescribeReservedInstancesListingsFluentBuilder::reserved_instances_id) / [`set_reserved_instances_id(Option<String>)`](crate::operation::describe_reserved_instances_listings::builders::DescribeReservedInstancesListingsFluentBuilder::set_reserved_instances_id): <p>One or more Reserved Instance IDs.</p>
    ///   - [`reserved_instances_listing_id(impl Into<String>)`](crate::operation::describe_reserved_instances_listings::builders::DescribeReservedInstancesListingsFluentBuilder::reserved_instances_listing_id) / [`set_reserved_instances_listing_id(Option<String>)`](crate::operation::describe_reserved_instances_listings::builders::DescribeReservedInstancesListingsFluentBuilder::set_reserved_instances_listing_id): <p>One or more Reserved Instance listing IDs.</p>
    /// - On success, responds with [`DescribeReservedInstancesListingsOutput`](crate::operation::describe_reserved_instances_listings::DescribeReservedInstancesListingsOutput) with field(s):
    ///   - [`reserved_instances_listings(Option<Vec<ReservedInstancesListing>>)`](crate::operation::describe_reserved_instances_listings::DescribeReservedInstancesListingsOutput::reserved_instances_listings): <p>Information about the Reserved Instance listing.</p>
    /// - On failure, responds with [`SdkError<DescribeReservedInstancesListingsError>`](crate::operation::describe_reserved_instances_listings::DescribeReservedInstancesListingsError)
    pub fn describe_reserved_instances_listings(&self) -> crate::operation::describe_reserved_instances_listings::builders::DescribeReservedInstancesListingsFluentBuilder{
        crate::operation::describe_reserved_instances_listings::builders::DescribeReservedInstancesListingsFluentBuilder::new(self.handle.clone())
    }
}