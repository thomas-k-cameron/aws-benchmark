// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AllocateAddress`](crate::operation::allocate_address::builders::AllocateAddressFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`domain(DomainType)`](crate::operation::allocate_address::builders::AllocateAddressFluentBuilder::domain) / [`set_domain(Option<DomainType>)`](crate::operation::allocate_address::builders::AllocateAddressFluentBuilder::set_domain): <p>Indicates whether the Elastic IP address is for use with instances in a VPC or instances in EC2-Classic.</p>  <p>Default: If the Region supports EC2-Classic, the default is <code>standard</code>. Otherwise, the default is <code>vpc</code>.</p>
    ///   - [`address(impl Into<String>)`](crate::operation::allocate_address::builders::AllocateAddressFluentBuilder::address) / [`set_address(Option<String>)`](crate::operation::allocate_address::builders::AllocateAddressFluentBuilder::set_address): <p>[EC2-VPC] The Elastic IP address to recover or an IPv4 address from an address pool.</p>
    ///   - [`public_ipv4_pool(impl Into<String>)`](crate::operation::allocate_address::builders::AllocateAddressFluentBuilder::public_ipv4_pool) / [`set_public_ipv4_pool(Option<String>)`](crate::operation::allocate_address::builders::AllocateAddressFluentBuilder::set_public_ipv4_pool): <p>The ID of an address pool that you own. Use this parameter to let Amazon EC2 select an address from the address pool. To specify a specific address from the address pool, use the <code>Address</code> parameter instead.</p>
    ///   - [`network_border_group(impl Into<String>)`](crate::operation::allocate_address::builders::AllocateAddressFluentBuilder::network_border_group) / [`set_network_border_group(Option<String>)`](crate::operation::allocate_address::builders::AllocateAddressFluentBuilder::set_network_border_group): <p> A unique set of Availability Zones, Local Zones, or Wavelength Zones from which Amazon Web Services advertises IP addresses. Use this parameter to limit the IP address to this location. IP addresses cannot move between network border groups.</p>  <p>Use <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeAvailabilityZones.html">DescribeAvailabilityZones</a> to view the network border groups.</p>  <p>You cannot use a network border group with EC2 Classic. If you attempt this operation on EC2 Classic, you receive an <code>InvalidParameterCombination</code> error.</p>
    ///   - [`customer_owned_ipv4_pool(impl Into<String>)`](crate::operation::allocate_address::builders::AllocateAddressFluentBuilder::customer_owned_ipv4_pool) / [`set_customer_owned_ipv4_pool(Option<String>)`](crate::operation::allocate_address::builders::AllocateAddressFluentBuilder::set_customer_owned_ipv4_pool): <p>The ID of a customer-owned address pool. Use this parameter to let Amazon EC2 select an address from the address pool. Alternatively, specify a specific address from the address pool.</p>
    ///   - [`dry_run(bool)`](crate::operation::allocate_address::builders::AllocateAddressFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::allocate_address::builders::AllocateAddressFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`tag_specifications(Vec<TagSpecification>)`](crate::operation::allocate_address::builders::AllocateAddressFluentBuilder::tag_specifications) / [`set_tag_specifications(Option<Vec<TagSpecification>>)`](crate::operation::allocate_address::builders::AllocateAddressFluentBuilder::set_tag_specifications): <p>The tags to assign to the Elastic IP address.</p>
    /// - On success, responds with [`AllocateAddressOutput`](crate::operation::allocate_address::AllocateAddressOutput) with field(s):
    ///   - [`public_ip(Option<String>)`](crate::operation::allocate_address::AllocateAddressOutput::public_ip): <p>The Elastic IP address.</p>
    ///   - [`allocation_id(Option<String>)`](crate::operation::allocate_address::AllocateAddressOutput::allocation_id): <p>[EC2-VPC] The ID that Amazon Web Services assigns to represent the allocation of the Elastic IP address for use with instances in a VPC.</p>
    ///   - [`public_ipv4_pool(Option<String>)`](crate::operation::allocate_address::AllocateAddressOutput::public_ipv4_pool): <p>The ID of an address pool.</p>
    ///   - [`network_border_group(Option<String>)`](crate::operation::allocate_address::AllocateAddressOutput::network_border_group): <p>The set of Availability Zones, Local Zones, or Wavelength Zones from which Amazon Web Services advertises IP addresses.</p>
    ///   - [`domain(Option<DomainType>)`](crate::operation::allocate_address::AllocateAddressOutput::domain): <p>Indicates whether the Elastic IP address is for use with instances in a VPC (<code>vpc</code>) or instances in EC2-Classic (<code>standard</code>).</p>
    ///   - [`customer_owned_ip(Option<String>)`](crate::operation::allocate_address::AllocateAddressOutput::customer_owned_ip): <p>The customer-owned IP address.</p>
    ///   - [`customer_owned_ipv4_pool(Option<String>)`](crate::operation::allocate_address::AllocateAddressOutput::customer_owned_ipv4_pool): <p>The ID of the customer-owned address pool.</p>
    ///   - [`carrier_ip(Option<String>)`](crate::operation::allocate_address::AllocateAddressOutput::carrier_ip): <p>The carrier IP address. This option is only available for network interfaces which reside in a subnet in a Wavelength Zone (for example an EC2 instance). </p>
    /// - On failure, responds with [`SdkError<AllocateAddressError>`](crate::operation::allocate_address::AllocateAddressError)
    pub fn allocate_address(
        &self,
    ) -> crate::operation::allocate_address::builders::AllocateAddressFluentBuilder {
        crate::operation::allocate_address::builders::AllocateAddressFluentBuilder::new(
            self.handle.clone(),
        )
    }
}