//! Keywords API surface — re-exports the HTTP client and
//! provides a multipart form builder for AI generation.
//!
//! The HTTP client lives in `crate::api::keywords` so the
//! API layer stays organised by domain. The form builder
//! is local because the shared one is wired against a
//! different `GenerateFormData` type.

use crate::game_engine::GenerateFormData;

pub use crate::api::keywords::{
    create_keywords, get_keyword_sets,
};

const META_FIELD: &str = "metadata";
const META_FILE: &str = "metadata.json";
const FILES_FIELD: &str = "files";

/// Build the multipart `web_sys::FormData` payload for
/// the keywords AI generation endpoint.
pub fn build_form_data(
    data: &GenerateFormData,
) -> web_sys::FormData {
    let form = web_sys::FormData::new().unwrap();
    let blob = build_metadata_blob(data);
    let _ = form.append_with_blob_and_filename(
        META_FIELD, &blob, META_FILE,
    );
    append_file_names(&form, &data.files);
    form
}

/// Serialize the user-provided form fields as a JSON
/// metadata blob.
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
        "output_game": "keywords",
    });
    let json: wasm_bindgen::JsValue =
        meta.to_string().into();
    let arr = js_sys::Array::of1(&json);
    web_sys::Blob::new_with_str_sequence(&arr).unwrap()
}

/// Append every selected file name to the form.
fn append_file_names(
    form: &web_sys::FormData,
    files: &[String],
) {
    for name in files {
        let _ = form.append_with_str(FILES_FIELD, name);
    }
}
