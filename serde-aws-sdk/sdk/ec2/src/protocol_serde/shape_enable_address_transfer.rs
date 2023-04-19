// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_enable_address_transfer_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::enable_address_transfer::EnableAddressTransferOutput,
    crate::operation::enable_address_transfer::EnableAddressTransferError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::enable_address_transfer::EnableAddressTransferError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::operation::enable_address_transfer::EnableAddressTransferError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_enable_address_transfer_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::enable_address_transfer::EnableAddressTransferOutput,
    crate::operation::enable_address_transfer::EnableAddressTransferError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::enable_address_transfer::builders::EnableAddressTransferOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_enable_address_transfer::de_enable_address_transfer(
            response.body().as_ref(),
            output,
        )
        .map_err(
            crate::operation::enable_address_transfer::EnableAddressTransferError::unhandled,
        )?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_enable_address_transfer(
    inp: &[u8],
    mut builder: crate::operation::enable_address_transfer::builders::EnableAddressTransferOutputBuilder,
) -> Result<
    crate::operation::enable_address_transfer::builders::EnableAddressTransferOutputBuilder,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("EnableAddressTransferResponse")) {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected EnableAddressTransferResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("addressTransfer") /* AddressTransfer com.amazonaws.ec2.synthetic#EnableAddressTransferOutput$AddressTransfer */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_address_transfer::de_address_transfer(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_address_transfer(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}