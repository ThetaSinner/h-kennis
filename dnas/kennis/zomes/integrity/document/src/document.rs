use hdi::prelude::*;
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct Document {
    pub title: String,
    pub content: String,
}
pub fn validate_create_document(
    _action: EntryCreationAction,
    _document: Document,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_document(
    _action: Update,
    _document: Document,
    _original_action: EntryCreationAction,
    _original_document: Document,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_document(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_document: Document,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_create_link_document_updates(
    _action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // Check the entry type for the given action hash
    let action_hash = base_address
        .into_action_hash()
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("No action hash associated with link"))
            ),
        )?;
    let record = must_get_valid_record(action_hash)?;
    let _document: crate::Document = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    // Check the entry type for the given action hash
    let action_hash = target_address
        .into_action_hash()
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("No action hash associated with link"))
            ),
        )?;
    let record = must_get_valid_record(action_hash)?;
    let _document: crate::Document = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_document_updates(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("DocumentUpdates links cannot be deleted"),
        ),
    )
}
