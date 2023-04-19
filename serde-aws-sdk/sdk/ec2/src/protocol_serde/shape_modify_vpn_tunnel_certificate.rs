// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_vpn_tunnel_certificate_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::modify_vpn_tunnel_certificate::ModifyVpnTunnelCertificateOutput,
    crate::operation::modify_vpn_tunnel_certificate::ModifyVpnTunnelCertificateError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::modify_vpn_tunnel_certificate::ModifyVpnTunnelCertificateError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(
        crate::operation::modify_vpn_tunnel_certificate::ModifyVpnTunnelCertificateError::generic(
            generic,
        ),
    )
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_vpn_tunnel_certificate_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::modify_vpn_tunnel_certificate::ModifyVpnTunnelCertificateOutput,
    crate::operation::modify_vpn_tunnel_certificate::ModifyVpnTunnelCertificateError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::modify_vpn_tunnel_certificate::builders::ModifyVpnTunnelCertificateOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_modify_vpn_tunnel_certificate::de_modify_vpn_tunnel_certificate(response.body().as_ref(), output).map_err(crate::operation::modify_vpn_tunnel_certificate::ModifyVpnTunnelCertificateError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_modify_vpn_tunnel_certificate(inp: &[u8], mut builder: crate::operation::modify_vpn_tunnel_certificate::builders::ModifyVpnTunnelCertificateOutputBuilder) -> Result<crate::operation::modify_vpn_tunnel_certificate::builders::ModifyVpnTunnelCertificateOutputBuilder, aws_smithy_xml::decode::XmlDecodeError>{
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("ModifyVpnTunnelCertificateResponse")) {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected ModifyVpnTunnelCertificateResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("vpnConnection") /* VpnConnection com.amazonaws.ec2.synthetic#ModifyVpnTunnelCertificateOutput$VpnConnection */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_vpn_connection::de_vpn_connection(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_vpn_connection(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}