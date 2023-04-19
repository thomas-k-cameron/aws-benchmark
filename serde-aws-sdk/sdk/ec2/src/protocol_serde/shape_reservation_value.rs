// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_reservation_value(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::ReservationValue, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::ReservationValue::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("hourlyPrice") /* HourlyPrice com.amazonaws.ec2#ReservationValue$HourlyPrice */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_hourly_price(var_1);
            }
            ,
            s if s.matches("remainingTotalValue") /* RemainingTotalValue com.amazonaws.ec2#ReservationValue$RemainingTotalValue */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_remaining_total_value(var_2);
            }
            ,
            s if s.matches("remainingUpfrontValue") /* RemainingUpfrontValue com.amazonaws.ec2#ReservationValue$RemainingUpfrontValue */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_remaining_upfront_value(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}