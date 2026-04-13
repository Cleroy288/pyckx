//! FormData builder for game generation requests

use crate::components::game_shared::generate_form::GenerateFormData;

/// Builds web_sys::FormData from GenerateFormData
///
/// The backend expects a multipart form with:
/// - "metadata": JSON blob with all form fields
/// - "files": actual file uploads (optional)
pub fn build_generate_form(
    data: &GenerateFormData,
) -> web_sys::FormData {
    let form = web_sys::FormData::new().unwrap();
    let blob = build_metadata_blob(data);
    let _ = form.append_with_blob_and_filename(
        "metadata", &blob, "metadata.json",
    );
    append_files(&form, &data.files);
    form
}

/// Serialize form data into a JSON blob
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
        "output_game": "qcm",
    });
    let json_str: wasm_bindgen::JsValue =
        meta.to_string().into();
    let arr = js_sys::Array::of1(&json_str);
    web_sys::Blob::new_with_str_sequence(&arr)
        .unwrap()
}

/// Append file names to FormData
fn append_files(
    form: &web_sys::FormData,
    files: &[String],
) {
    for name in files {
        let _ =
            form.append_with_str("files", name);
    }
}
