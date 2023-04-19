// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_provision_ipam_pool_cidr_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::provision_ipam_pool_cidr::ProvisionIpamPoolCidrOutput,
    crate::operation::provision_ipam_pool_cidr::ProvisionIpamPoolCidrError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::provision_ipam_pool_cidr::ProvisionIpamPoolCidrError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::operation::provision_ipam_pool_cidr::ProvisionIpamPoolCidrError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_provision_ipam_pool_cidr_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::provision_ipam_pool_cidr::ProvisionIpamPoolCidrOutput,
    crate::operation::provision_ipam_pool_cidr::ProvisionIpamPoolCidrError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::provision_ipam_pool_cidr::builders::ProvisionIpamPoolCidrOutputBuilder::default();
        let _ = response;
        output =
            crate::protocol_serde::shape_provision_ipam_pool_cidr::de_provision_ipam_pool_cidr(
                response.body().as_ref(),
                output,
            )
            .map_err(
                crate::operation::provision_ipam_pool_cidr::ProvisionIpamPoolCidrError::unhandled,
            )?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_provision_ipam_pool_cidr(
    inp: &[u8],
    mut builder: crate::operation::provision_ipam_pool_cidr::builders::ProvisionIpamPoolCidrOutputBuilder,
) -> Result<
    crate::operation::provision_ipam_pool_cidr::builders::ProvisionIpamPoolCidrOutputBuilder,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("ProvisionIpamPoolCidrResponse")) {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected ProvisionIpamPoolCidrResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("ipamPoolCidr") /* IpamPoolCidr com.amazonaws.ec2.synthetic#ProvisionIpamPoolCidrOutput$IpamPoolCidr */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_ipam_pool_cidr::de_ipam_pool_cidr(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_ipam_pool_cidr(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}