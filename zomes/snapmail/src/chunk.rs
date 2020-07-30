use hdk3::prelude::*;
use test_wasm_common::*;
use holochain_wasmer_guest::*;
use holochain_zome_types::*;
use holo_hash::HoloHash;
use holo_hash::hash_type;

holochain_wasmer_guest::holochain_externs!();

pub const CHUNK_MAX_SIZE: usize = 200 * 1024;

map_extern!(write_chunk, _write_chunk);
map_extern!(get_chunk, _get_chunk);

#[derive(Shrinkwrap, Clone, Debug, PartialEq, Default, Serialize, Deserialize, SerializedBytes)]
pub struct MyString(String);

#[derive(Shrinkwrap, Clone, Debug, PartialEq, Default, Serialize, Deserialize, SerializedBytes)]
pub struct MyRaw(Vec<u8>);

//-------------------------------------------------------------------------------------------------
// Definition
//-------------------------------------------------------------------------------------------------

/// Entry representing a file chunk.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, SerializedBytes)]
pub struct FileChunk {
    pub data_hash: String,
    pub chunk_index: usize,
    pub chunk: String,
}

impl FileChunk {
    pub fn entry_def() -> EntryDef {
        EntryDef {
            id: "file_chunk".into(),
            visibility: EntryVisibility::Private,
            crdt_type: CrdtType,
            required_validations: 8.into(),
        }
    }
}

impl From<&FileChunk> for EntryDefId {
    fn from(_: &FileChunk) -> Self {
        "file_chunk".into()
    }
}

impl From<&FileChunk> for EntryVisibility {
    fn from(_: &FileChunk) -> Self {
        Self::Private
    }
}

impl From<&FileChunk> for CrdtType {
    fn from(_: &FileChunk) -> Self {
        Self
    }
}

impl From<&FileChunk> for RequiredValidations {
    fn from(_: &FileChunk) -> Self {
        8.into()
    }
}

impl From<&FileChunk> for EntryDef {
    fn from(post: &FileChunk) -> Self {
        Self {
            id: post.into(),
            visibility: post.into(),
            crdt_type: post.into(),
            required_validations: post.into(),
        }
    }
}

impl TryFrom<&FileChunk> for Entry {
    type Error = SerializedBytesError;
    fn try_from(post: &FileChunk) -> Result<Self, Self::Error> {
        Ok(Entry::App(post.try_into()?))
    }
}

// pub(crate) fn validate_chunk(validation_data: hdk::EntryValidationData<FileChunk>) -> Result<(), String> {
//     match validation_data {
//         EntryValidationData::Create{entry: file, validation_data: _} => {
//             // Check size
//             if file.chunk.len() > CHUNK_MAX_SIZE {
//                 return Err(format!("A file chunk can't be bigger than {} KiB", CHUNK_MAX_SIZE / 1024));
//             }
//             return Ok(());
//         },
//         EntryValidationData::Modify{new_entry: _, old_entry: _, old_entry_header:_, validation_data: _} => {
//             return Err("Update chunk not allowed".into());
//         },
//         EntryValidationData::Delete{old_entry: _, old_entry_header: _, validation_data:_} => {
//             return Ok(());
//         }
//     }
// }

impl FileChunk {
    pub fn new(data_hash: String, chunk_index: usize, chunk: String) -> Self {
        Self {
            data_hash,
            chunk_index,
            chunk,
        }
    }
}

/// Zome function
/// Write base64 file as string to source chain
pub fn _write_chunk(
    fileChunk: FileChunk
) -> Result<EntryHash, WasmError> {
    //debug!(format!("fileChunk: {:?}", fileChunk)).ok();
    let res: EntryHash = host_call!(
        __commit_entry,
        CommitEntryInput::new(((&fileChunk).into(), (&fileChunk).try_into()?))
    )?;
    debug!(format!("commit_result: {:?}", res)).ok();
    Ok(res)
}

/// Zome function
/// Get chunk index and chunk as base64 string in local source chain at given address
pub fn _get_chunk(chunk_address_raw: /*EntryHash*/ MyRaw) -> Result<MyString, WasmError> {
    debug!(format!("chunk_address_raw: {:?}", chunk_address_raw)).ok();
    let chunk_address = HoloHash::<hash_type::Entry>::from_raw_bytes_and_type(chunk_address_raw.to_vec(), hash_type::Entry::Content);
        debug!(format!("chunk_address: {:?}", chunk_address)).ok();
    let maybe_entry = get_entry!(chunk_address)
        .expect("No reason for get_entry() to crash");
    if maybe_entry.is_none() {
        return Ok(MyString("".into()));
    }

    let chunk = match maybe_entry.unwrap() {
        Entry::App(entry_value) => FileChunk::try_from(entry_value).expect("should convert"),
        _ =>  return Ok(MyString(String::new().into())),
    };
    // Ok((chunk.chunk_index, chunk.chunk))
    Ok(MyString(chunk.chunk.into()))
}
