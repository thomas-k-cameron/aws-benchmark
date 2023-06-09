// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_deprovision_ipam_pool_cidr_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::deprovision_ipam_pool_cidr::DeprovisionIpamPoolCidrOutput,
    crate::operation::deprovision_ipam_pool_cidr::DeprovisionIpamPoolCidrError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::deprovision_ipam_pool_cidr::DeprovisionIpamPoolCidrError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(
        crate::operation::deprovision_ipam_pool_cidr::DeprovisionIpamPoolCidrError::generic(
            generic,
        ),
    )
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_deprovision_ipam_pool_cidr_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::deprovision_ipam_pool_cidr::DeprovisionIpamPoolCidrOutput,
    crate::operation::deprovision_ipam_pool_cidr::DeprovisionIpamPoolCidrError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::deprovision_ipam_pool_cidr::builders::DeprovisionIpamPoolCidrOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_deprovision_ipam_pool_cidr::de_deprovision_ipam_pool_cidr(response.body().as_ref(), output).map_err(crate::operation::deprovision_ipam_pool_cidr::DeprovisionIpamPoolCidrError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_deprovision_ipam_pool_cidr(
    inp: &[u8],
    mut builder: crate::operation::deprovision_ipam_pool_cidr::builders::DeprovisionIpamPoolCidrOutputBuilder,
) -> Result<
    crate::operation::deprovision_ipam_pool_cidr::builders::DeprovisionIpamPoolCidrOutputBuilder,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DeprovisionIpamPoolCidrResponse")) {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DeprovisionIpamPoolCidrResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("ipamPoolCidr") /* IpamPoolCidr com.amazonaws.ec2.synthetic#DeprovisionIpamPoolCidrOutput$IpamPoolCidr */ =>  {
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
