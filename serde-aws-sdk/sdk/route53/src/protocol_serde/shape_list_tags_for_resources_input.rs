// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_tags_for_resources_input_input(
    input: &crate::operation::list_tags_for_resources::ListTagsForResourcesInput,
    writer: aws_smithy_xml::encode::ElWriter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.resource_ids {
        let mut inner_writer = scope.start_el("ResourceIds").finish();
        for list_item_2 in var_1 {
            {
                let mut inner_writer = inner_writer.start_el("ResourceId").finish();
                inner_writer.data(list_item_2.as_str());
            }
        }
    }
    scope.finish();
    Ok(())
}