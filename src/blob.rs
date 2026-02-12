use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
    reflect::TypePath,
};
use thiserror::Error;

pub(super) fn plugin(app: &mut App) {
    app.init_asset::<Blob>()
        .init_asset_loader::<BlobAssetLoader>();
}

#[derive(Asset, TypePath, Debug)]
pub struct Blob {
    bytes: Vec<u8>,
}

#[derive(Default, TypePath)]
struct BlobAssetLoader;

/// Possible errors that can be produced by [`BlobAssetLoader`]
#[non_exhaustive]
#[derive(Debug, Error)]
enum BlobAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load file: {0}")]
    Io(#[from] std::io::Error),
}

impl AssetLoader for BlobAssetLoader {
    type Asset = Blob;
    type Settings = ();
    type Error = BlobAssetLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        ctx: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        info!("Loading Blob...");
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        #[cfg(target_arch = "wasm32")]
        if let Some(name) = ctx.path().path().file_stem() {
            trigger_download(bytes.clone(), name.to_str().unwrap());
        };

        Ok(Blob { bytes })
    }

    fn extensions(&self) -> &[&str] {
        &["blob"]
    }
}

//#[cfg(target_arch = "wasm32")]
fn trigger_download(data: Vec<u8>, filename: &str) {
    use js_sys::{Array, Uint8Array};
    use wasm_bindgen::JsCast;
    use web_sys::{Blob, BlobPropertyBag, HtmlAnchorElement, Url};

    let uint8_array = Uint8Array::from(data.as_slice());
    let parts = Array::new();
    parts.push(&uint8_array);

    let opts = BlobPropertyBag::new();
    opts.set_type("application/pdf");
    let blob = Blob::new_with_u8_array_sequence_and_options(&parts, &opts).unwrap();

    let url = Url::create_object_url_with_blob(&blob).unwrap();

    let document = web_sys::window().unwrap().document().unwrap();
    let a = document
        .create_element("a")
        .unwrap()
        .dyn_into::<HtmlAnchorElement>()
        .unwrap();
    a.set_href(&url);
    a.set_download(filename);
    a.click();

    // clean up
    Url::revoke_object_url(&url).unwrap();
}
