use hdk3::prelude::*;

use crate::{
    ZomeU32,
    file::{FileManifest, dm::request_chunk_by_dm},
    utils::*,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, SerializedBytes)]
pub struct GetMissingChunksInput {
    pub from: AgentPubKey,
    pub manifest_eh: EntryHash,
}

/// Zome Function
/// Request missing chunks for an attachment
/// Returns number of remaining missing chunks
/// TODO: Return vec of missing chunk HeaderHash
#[hdk_extern]
pub fn get_missing_chunks(input: GetMissingChunksInput) -> ExternResult<ZomeU32> {
    let (_eh, manifest) = get_typed_from_eh::<FileManifest>(input.manifest_eh.clone())?;
    let chunk_count = manifest.chunks.len();
    let mut missing = 0;
    let mut i = -1;
    for chunk_eh in manifest.chunks {
        i += 1;
        let chunk_str = format!("Chunk {}/{}", i, chunk_count);
        /// Skip if chunk already held
        let maybe_el = get(chunk_eh.clone(), GetOptions::content())?;
        if let Some(_) = maybe_el {
            debug!("{} already held", chunk_str);
            continue;
        }
        /// Request missing chunk
        let maybe_maybe_chunk = request_chunk_by_dm(input.from.clone(), chunk_eh);
        /// Notify failure
        if let Err(err) = maybe_maybe_chunk {
            let response_str = format!("{} failed", chunk_str);
            debug!("{}: {}", response_str, err);
            missing += 1;
            continue;
        }
        if let None = maybe_maybe_chunk.unwrap() {
            debug!("{} unknown from source agent", chunk_str);
            missing += 1;
            continue;
        }
    }
    /// Done
    Ok(ZomeU32(missing))
}
