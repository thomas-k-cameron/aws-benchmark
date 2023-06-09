// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_monitor_instances_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::monitor_instances::MonitorInstancesOutput,
    crate::operation::monitor_instances::MonitorInstancesError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::monitor_instances::MonitorInstancesError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::operation::monitor_instances::MonitorInstancesError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_monitor_instances_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::monitor_instances::MonitorInstancesOutput,
    crate::operation::monitor_instances::MonitorInstancesError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::monitor_instances::builders::MonitorInstancesOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_monitor_instances::de_monitor_instances(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::operation::monitor_instances::MonitorInstancesError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_monitor_instances(
    inp: &[u8],
    mut builder: crate::operation::monitor_instances::builders::MonitorInstancesOutputBuilder,
) -> Result<
    crate::operation::monitor_instances::builders::MonitorInstancesOutputBuilder,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("MonitorInstancesResponse")) {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected MonitorInstancesResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("instancesSet") /* InstanceMonitorings com.amazonaws.ec2.synthetic#MonitorInstancesOutput$InstanceMonitorings */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_instance_monitoring_list::de_instance_monitoring_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_instance_monitorings(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
