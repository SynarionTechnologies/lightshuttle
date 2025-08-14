use std::{error::Error, fs, path::Path};

use lightshuttle_core::openapi::ApiDoc;
use utoipa::OpenApi;

fn main() -> Result<(), Box<dyn Error>> {
    let doc = ApiDoc::openapi();
    let yaml = doc.to_yaml()?;
    let out_dir = Path::new("openapi");
    fs::create_dir_all(out_dir)?;
    fs::write(out_dir.join("openapi.yaml"), yaml)?;
    Ok(())
}
