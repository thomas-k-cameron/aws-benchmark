// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_tracked_action_last_accessed(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::TrackedActionLastAccessed, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::TrackedActionLastAccessed::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("ActionName") /* ActionName com.amazonaws.iam#TrackedActionLastAccessed$ActionName */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_action_name(var_1);
            }
            ,
            s if s.matches("LastAccessedEntity") /* LastAccessedEntity com.amazonaws.iam#TrackedActionLastAccessed$LastAccessedEntity */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_last_accessed_entity(var_2);
            }
            ,
            s if s.matches("LastAccessedTime") /* LastAccessedTime com.amazonaws.iam#TrackedActionLastAccessed$LastAccessedTime */ =>  {
                let var_3 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.iam#dateType`)"))
                        ?
                    )
                ;
                builder = builder.set_last_accessed_time(var_3);
            }
            ,
            s if s.matches("LastAccessedRegion") /* LastAccessedRegion com.amazonaws.iam#TrackedActionLastAccessed$LastAccessedRegion */ =>  {
                let var_4 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_last_accessed_region(var_4);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}