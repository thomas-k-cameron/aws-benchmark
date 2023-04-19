// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateVpc`](crate::operation::create_vpc::builders::CreateVpcFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`cidr_block(impl Into<String>)`](crate::operation::create_vpc::builders::CreateVpcFluentBuilder::cidr_block) / [`set_cidr_block(Option<String>)`](crate::operation::create_vpc::builders::CreateVpcFluentBuilder::set_cidr_block): <p>The IPv4 network range for the VPC, in CIDR notation. For example, <code>10.0.0.0/16</code>. We modify the specified CIDR block to its canonical form; for example, if you specify <code>100.68.0.18/18</code>, we modify it to <code>100.68.0.0/18</code>.</p>
    ///   - [`amazon_provided_ipv6_cidr_block(bool)`](crate::operation::create_vpc::builders::CreateVpcFluentBuilder::amazon_provided_ipv6_cidr_block) / [`set_amazon_provided_ipv6_cidr_block(Option<bool>)`](crate::operation::create_vpc::builders::CreateVpcFluentBuilder::set_amazon_provided_ipv6_cidr_block): <p>Requests an Amazon-provided IPv6 CIDR block with a /56 prefix length for the VPC. You cannot specify the range of IP addresses, or the size of the CIDR block.</p>
    ///   - [`ipv6_pool(impl Into<String>)`](crate::operation::create_vpc::builders::CreateVpcFluentBuilder::ipv6_pool) / [`set_ipv6_pool(Option<String>)`](crate::operation::create_vpc::builders::CreateVpcFluentBuilder::set_ipv6_pool): <p>The ID of an IPv6 address pool from which to allocate the IPv6 CIDR block.</p>
    ///   - [`ipv6_cidr_block(impl Into<String>)`](crate::operation::create_vpc::builders::CreateVpcFluentBuilder::ipv6_cidr_block) / [`set_ipv6_cidr_block(Option<String>)`](crate::operation::create_vpc::builders::CreateVpcFluentBuilder::set_ipv6_cidr_block): <p>The IPv6 CIDR block from the IPv6 address pool. You must also specify <code>Ipv6Pool</code> in the request.</p>  <p>To let Amazon choose the IPv6 CIDR block for you, omit this parameter.</p>
    ///   - [`ipv4_ipam_pool_id(impl Into<String>)`](crate::operation::create_vpc::builders::CreateVpcFluentBuilder::ipv4_ipam_pool_id) / [`set_ipv4_ipam_pool_id(Option<String>)`](crate::operation::create_vpc::builders::CreateVpcFluentBuilder::set_ipv4_ipam_pool_id): <p>The ID of an IPv4 IPAM pool you want to use for allocating this VPC's CIDR. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/what-is-it-ipam.html">What is IPAM?</a> in the <i>Amazon VPC IPAM User Guide</i>. </p>
    ///   - [`ipv4_netmask_length(i32)`](crate::operation::create_vpc::builders::CreateVpcFluentBuilder::ipv4_netmask_length) / [`set_ipv4_netmask_length(Option<i32>)`](crate::operation::create_vpc::builders::CreateVpcFluentBuilder::set_ipv4_netmask_length): <p>The netmask length of the IPv4 CIDR you want to allocate to this VPC from an Amazon VPC IP Address Manager (IPAM) pool. For more information about IPAM, see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/what-is-it-ipam.html">What is IPAM?</a> in the <i>Amazon VPC IPAM User Guide</i>.</p>
    ///   - [`ipv6_ipam_pool_id(impl Into<String>)`](crate::operation::create_vpc::builders::CreateVpcFluentBuilder::ipv6_ipam_pool_id) / [`set_ipv6_ipam_pool_id(Option<String>)`](crate::operation::create_vpc::builders::CreateVpcFluentBuilder::set_ipv6_ipam_pool_id): <p>The ID of an IPv6 IPAM pool which will be used to allocate this VPC an IPv6 CIDR. IPAM is a VPC feature that you can use to automate your IP address management workflows including assigning, tracking, troubleshooting, and auditing IP addresses across Amazon Web Services Regions and accounts throughout your Amazon Web Services Organization. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/what-is-it-ipam.html">What is IPAM?</a> in the <i>Amazon VPC IPAM User Guide</i>.</p>
    ///   - [`ipv6_netmask_length(i32)`](crate::operation::create_vpc::builders::CreateVpcFluentBuilder::ipv6_netmask_length) / [`set_ipv6_netmask_length(Option<i32>)`](crate::operation::create_vpc::builders::CreateVpcFluentBuilder::set_ipv6_netmask_length): <p>The netmask length of the IPv6 CIDR you want to allocate to this VPC from an Amazon VPC IP Address Manager (IPAM) pool. For more information about IPAM, see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/what-is-it-ipam.html">What is IPAM?</a> in the <i>Amazon VPC IPAM User Guide</i>.</p>
    ///   - [`dry_run(bool)`](crate::operation::create_vpc::builders::CreateVpcFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::create_vpc::builders::CreateVpcFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`instance_tenancy(Tenancy)`](crate::operation::create_vpc::builders::CreateVpcFluentBuilder::instance_tenancy) / [`set_instance_tenancy(Option<Tenancy>)`](crate::operation::create_vpc::builders::CreateVpcFluentBuilder::set_instance_tenancy): <p>The tenancy options for instances launched into the VPC. For <code>default</code>, instances are launched with shared tenancy by default. You can launch instances with any tenancy into a shared tenancy VPC. For <code>dedicated</code>, instances are launched as dedicated tenancy instances by default. You can only launch instances with a tenancy of <code>dedicated</code> or <code>host</code> into a dedicated tenancy VPC. </p>  <p> <b>Important:</b> The <code>host</code> value cannot be used with this parameter. Use the <code>default</code> or <code>dedicated</code> values only.</p>  <p>Default: <code>default</code> </p>
    ///   - [`ipv6_cidr_block_network_border_group(impl Into<String>)`](crate::operation::create_vpc::builders::CreateVpcFluentBuilder::ipv6_cidr_block_network_border_group) / [`set_ipv6_cidr_block_network_border_group(Option<String>)`](crate::operation::create_vpc::builders::CreateVpcFluentBuilder::set_ipv6_cidr_block_network_border_group): <p>The name of the location from which we advertise the IPV6 CIDR block. Use this parameter to limit the address to this location.</p>  <p> You must set <code>AmazonProvidedIpv6CidrBlock</code> to <code>true</code> to use this parameter.</p>
    ///   - [`tag_specifications(Vec<TagSpecification>)`](crate::operation::create_vpc::builders::CreateVpcFluentBuilder::tag_specifications) / [`set_tag_specifications(Option<Vec<TagSpecification>>)`](crate::operation::create_vpc::builders::CreateVpcFluentBuilder::set_tag_specifications): <p>The tags to assign to the VPC.</p>
    /// - On success, responds with [`CreateVpcOutput`](crate::operation::create_vpc::CreateVpcOutput) with field(s):
    ///   - [`vpc(Option<Vpc>)`](crate::operation::create_vpc::CreateVpcOutput::vpc): <p>Information about the VPC.</p>
    /// - On failure, responds with [`SdkError<CreateVpcError>`](crate::operation::create_vpc::CreateVpcError)
    pub fn create_vpc(&self) -> crate::operation::create_vpc::builders::CreateVpcFluentBuilder {
        crate::operation::create_vpc::builders::CreateVpcFluentBuilder::new(self.handle.clone())
    }
}