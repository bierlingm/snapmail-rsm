use hdk3::prelude::*;

use crate::{
    ZomeString,
    file::FileChunk,
    utils::*,
};

/// Zome function
/// Get chunk index and chunk as base64 string in local source chain at given address
/// Must be a valid address
/// TODO try using a AnyDht hash
#[hdk_extern]
pub fn get_chunk(chunk_eh: EntryHash) -> ExternResult<ZomeString> {
    debug!("get_chunk(): {}", chunk_eh);
    /// Look for element
    let element = match get(chunk_eh, GetOptions::content())? {
        Some(element) => element,
        None => return error("No element found at given address"),
    };
    /// Check if element is a Manifest
    let maybe_FileChunk: ExternResult<FileChunk> = get_typed_from_el(element.clone());
    if let Ok(chunk) = maybe_FileChunk {
        return Ok(ZomeString(chunk.chunk));
    }
    /// Done
    return error("Element at given address is not a FileChunk");
}
