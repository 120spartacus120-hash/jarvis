// vosk speech recognition model

use std::sync::Arc;
use vosk::Model;

use crate::models::registry::ModelRegistry;

pub struct VoskModel {
    pub model: Model,
}

unsafe impl Send for VoskModel {}
unsafe impl Sync for VoskModel {}

// load a vosk model by path through the registry.
// vosk models aren't in the catalog (they use their own directory structure),
// so we pass the path directly and use model_id for dedup.
// @ToDo: Consider moving to catalog
pub fn load(registry: &ModelRegistry, model_id: &str, model_path: &str) -> Result<Arc<VoskModel>, String> {
    // check if already loaded
    if let Some(existing) = registry.get::<VoskModel>(model_id) {
        info!("Vosk model '{}' already loaded, reusing", model_id);
        return Ok(existing);
    }

    info!("Loading Vosk model from: {}", model_path);

    let native_path = crate::path_util::native_library_path(std::path::Path::new(model_path));
    let load_path = native_path.to_str().unwrap_or(model_path);

    let model = Model::new(load_path)
        .ok_or_else(|| format!("Failed to load Vosk model from: {}", load_path))?;

    info!("Vosk model loaded: {}", model_id);
    Ok(registry.insert(model_id, VoskModel { model }))
}
