//! Fill-blank API client — list and AI generation.

use crate::api::{endpoints, helpers};
use crate::fill_blanks::types::{
    CreateFillBlankResponse, FillBlankSet,
    FillBlankSetListResponse,
};
use crate::game_engine::GenerateFormData;

/// `output_game` value sent to the backend.
const OUTPUT_GAME: &str = "fill_blanks";
/// Form field name for the JSON metadata blob.
const METADATA_FIELD: &str = "metadata";
/// Filename used when uploading the metadata blob.
const METADATA_NAME: &str = "metadata.json";
/// Form field name for each uploaded file.
const FILES_FIELD: &str = "files";

// Note: the `SetsResponse` impl for
// `FillBlankSetListResponse` is defined once in
// `crate::api::fill_blanks` to avoid a duplicate-impl
// conflict during the in-flight migration.

/// Fetch every fill-blank set for the current user.
pub async fn get_sets() -> Result<Vec<FillBlankSet>, String>
{
    helpers::get_sets::<FillBlankSetListResponse>(
        endpoints::FILL_BLANKS,
    )
    .await
}

/// Generate a fill-blank set from a multipart upload.
pub async fn create(
    form: &web_sys::FormData,
) -> Result<CreateFillBlankResponse, String> {
    helpers::post_multipart(
        endpoints::FILL_BLANKS,
        form,
    )
    .await
}

/// Build the multipart `FormData` posted to the backend.
pub fn build_form(
    data: &GenerateFormData,
) -> web_sys::FormData {
    let form = web_sys::FormData::new().unwrap();
    let blob = build_metadata_blob(data);
    let _ = form.append_with_blob_and_filename(
        METADATA_FIELD,
        &blob,
        METADATA_NAME,
    );
    append_files(&form, &data.files);
    form
}

/// Serialize the form fields into a JSON blob.
fn build_metadata_blob(
    data: &GenerateFormData,
) -> web_sys::Blob {
    let meta = serde_json::json!({
        "name": data.name,
        "description": data.description,
        "instructions": data.instructions,
        "language": data.language,
        "level": data.level,
        "num_questions": data.num_questions,
        "subjects": data.subjects,
        "output_game": OUTPUT_GAME,
    });
    let json_str: wasm_bindgen::JsValue =
        meta.to_string().into();
    let arr = js_sys::Array::of1(&json_str);
    web_sys::Blob::new_with_str_sequence(&arr).unwrap()
}

/// Append each selected file name to the form.
fn append_files(
    form: &web_sys::FormData,
    files: &[String],
) {
    for name in files {
        let _ = form.append_with_str(FILES_FIELD, name);
    }
}
