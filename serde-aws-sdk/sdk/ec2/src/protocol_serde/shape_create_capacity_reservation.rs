// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_create_capacity_reservation_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::create_capacity_reservation::CreateCapacityReservationOutput,
    crate::operation::create_capacity_reservation::CreateCapacityReservationError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::create_capacity_reservation::CreateCapacityReservationError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(
        crate::operation::create_capacity_reservation::CreateCapacityReservationError::generic(
            generic,
        ),
    )
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_capacity_reservation_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::create_capacity_reservation::CreateCapacityReservationOutput,
    crate::operation::create_capacity_reservation::CreateCapacityReservationError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::create_capacity_reservation::builders::CreateCapacityReservationOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_create_capacity_reservation::de_create_capacity_reservation(response.body().as_ref(), output).map_err(crate::operation::create_capacity_reservation::CreateCapacityReservationError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_create_capacity_reservation(
    inp: &[u8],
    mut builder: crate::operation::create_capacity_reservation::builders::CreateCapacityReservationOutputBuilder,
) -> Result<
    crate::operation::create_capacity_reservation::builders::CreateCapacityReservationOutputBuilder,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("CreateCapacityReservationResponse")) {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected CreateCapacityReservationResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("capacityReservation") /* CapacityReservation com.amazonaws.ec2.synthetic#CreateCapacityReservationOutput$CapacityReservation */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_capacity_reservation::de_capacity_reservation(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_capacity_reservation(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
