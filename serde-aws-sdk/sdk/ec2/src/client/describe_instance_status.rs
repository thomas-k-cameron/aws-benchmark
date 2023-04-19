// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeInstanceStatus`](crate::operation::describe_instance_status::builders::DescribeInstanceStatusFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_instance_status::builders::DescribeInstanceStatusFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`filters(Vec<Filter>)`](crate::operation::describe_instance_status::builders::DescribeInstanceStatusFluentBuilder::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::operation::describe_instance_status::builders::DescribeInstanceStatusFluentBuilder::set_filters): <p>The filters.</p>  <ul>   <li> <p> <code>availability-zone</code> - The Availability Zone of the instance.</p> </li>   <li> <p> <code>event.code</code> - The code for the scheduled event (<code>instance-reboot</code> | <code>system-reboot</code> | <code>system-maintenance</code> | <code>instance-retirement</code> | <code>instance-stop</code>).</p> </li>   <li> <p> <code>event.description</code> - A description of the event.</p> </li>   <li> <p> <code>event.instance-event-id</code> - The ID of the event whose date and time you are modifying.</p> </li>   <li> <p> <code>event.not-after</code> - The latest end time for the scheduled event (for example, <code>2014-09-15T17:15:20.000Z</code>).</p> </li>   <li> <p> <code>event.not-before</code> - The earliest start time for the scheduled event (for example, <code>2014-09-15T17:15:20.000Z</code>).</p> </li>   <li> <p> <code>event.not-before-deadline</code> - The deadline for starting the event (for example, <code>2014-09-15T17:15:20.000Z</code>).</p> </li>   <li> <p> <code>instance-state-code</code> - The code for the instance state, as a 16-bit unsigned integer. The high byte is used for internal purposes and should be ignored. The low byte is set based on the state represented. The valid values are 0 (pending), 16 (running), 32 (shutting-down), 48 (terminated), 64 (stopping), and 80 (stopped).</p> </li>   <li> <p> <code>instance-state-name</code> - The state of the instance (<code>pending</code> | <code>running</code> | <code>shutting-down</code> | <code>terminated</code> | <code>stopping</code> | <code>stopped</code>).</p> </li>   <li> <p> <code>instance-status.reachability</code> - Filters on instance status where the name is <code>reachability</code> (<code>passed</code> | <code>failed</code> | <code>initializing</code> | <code>insufficient-data</code>).</p> </li>   <li> <p> <code>instance-status.status</code> - The status of the instance (<code>ok</code> | <code>impaired</code> | <code>initializing</code> | <code>insufficient-data</code> | <code>not-applicable</code>).</p> </li>   <li> <p> <code>system-status.reachability</code> - Filters on system status where the name is <code>reachability</code> (<code>passed</code> | <code>failed</code> | <code>initializing</code> | <code>insufficient-data</code>).</p> </li>   <li> <p> <code>system-status.status</code> - The system status of the instance (<code>ok</code> | <code>impaired</code> | <code>initializing</code> | <code>insufficient-data</code> | <code>not-applicable</code>).</p> </li>  </ul>
    ///   - [`instance_ids(Vec<String>)`](crate::operation::describe_instance_status::builders::DescribeInstanceStatusFluentBuilder::instance_ids) / [`set_instance_ids(Option<Vec<String>>)`](crate::operation::describe_instance_status::builders::DescribeInstanceStatusFluentBuilder::set_instance_ids): <p>The instance IDs.</p>  <p>Default: Describes all your instances.</p>  <p>Constraints: Maximum 100 explicitly specified instance IDs.</p>
    ///   - [`max_results(i32)`](crate::operation::describe_instance_status::builders::DescribeInstanceStatusFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_instance_status::builders::DescribeInstanceStatusFluentBuilder::set_max_results): <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>  <p>You cannot specify this parameter and the instance IDs parameter in the same request.</p>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_instance_status::builders::DescribeInstanceStatusFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_instance_status::builders::DescribeInstanceStatusFluentBuilder::set_next_token): <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    ///   - [`dry_run(bool)`](crate::operation::describe_instance_status::builders::DescribeInstanceStatusFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::describe_instance_status::builders::DescribeInstanceStatusFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`include_all_instances(bool)`](crate::operation::describe_instance_status::builders::DescribeInstanceStatusFluentBuilder::include_all_instances) / [`set_include_all_instances(Option<bool>)`](crate::operation::describe_instance_status::builders::DescribeInstanceStatusFluentBuilder::set_include_all_instances): <p>When <code>true</code>, includes the health status for all instances. When <code>false</code>, includes the health status for running instances only.</p>  <p>Default: <code>false</code> </p>
    /// - On success, responds with [`DescribeInstanceStatusOutput`](crate::operation::describe_instance_status::DescribeInstanceStatusOutput) with field(s):
    ///   - [`instance_statuses(Option<Vec<InstanceStatus>>)`](crate::operation::describe_instance_status::DescribeInstanceStatusOutput::instance_statuses): <p>Information about the status of the instances.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_instance_status::DescribeInstanceStatusOutput::next_token): <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    /// - On failure, responds with [`SdkError<DescribeInstanceStatusError>`](crate::operation::describe_instance_status::DescribeInstanceStatusError)
    pub fn describe_instance_status(
        &self,
    ) -> crate::operation::describe_instance_status::builders::DescribeInstanceStatusFluentBuilder
    {
        crate::operation::describe_instance_status::builders::DescribeInstanceStatusFluentBuilder::new(self.handle.clone())
    }
}