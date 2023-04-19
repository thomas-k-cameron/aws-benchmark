// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_load_permission(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::LoadPermission, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::LoadPermission::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("userId") /* UserId com.amazonaws.ec2#LoadPermission$UserId */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_user_id(var_1);
            }
            ,
            s if s.matches("group") /* Group com.amazonaws.ec2#LoadPermission$Group */ =>  {
                let var_2 =
                    Some(
                        Result::<crate::types::PermissionGroup, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::PermissionGroup::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_group(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}