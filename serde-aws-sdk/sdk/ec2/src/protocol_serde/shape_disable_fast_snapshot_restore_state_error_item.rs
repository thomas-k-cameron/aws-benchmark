// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_disable_fast_snapshot_restore_state_error_item(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<
    crate::types::DisableFastSnapshotRestoreStateErrorItem,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    #[allow(unused_mut)]
    let mut builder = crate::types::DisableFastSnapshotRestoreStateErrorItem::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("availabilityZone") /* AvailabilityZone com.amazonaws.ec2#DisableFastSnapshotRestoreStateErrorItem$AvailabilityZone */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_availability_zone(var_1);
            }
            ,
            s if s.matches("error") /* Error com.amazonaws.ec2#DisableFastSnapshotRestoreStateErrorItem$Error */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_disable_fast_snapshot_restore_state_error::de_disable_fast_snapshot_restore_state_error(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_error(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}