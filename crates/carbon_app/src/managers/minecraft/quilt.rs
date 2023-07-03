use daedalus::modded::{LoaderVersion, Manifest, PartialVersionInfo};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum QuiltManifestError {
    #[error("Could not fetch quilt manifest from launchermeta: {0}")]
    NetworkError(#[from] reqwest::Error),
}

pub async fn get_manifest(
    reqwest_client: &reqwest_middleware::ClientWithMiddleware,
    meta_base_url: &reqwest::Url,
) -> anyhow::Result<Manifest> {
    let server_url = meta_base_url.join("quilt/v0/manifest.json")?;
    let new_manifest = reqwest_client
        .get(server_url)
        .send()
        .await?
        .json::<Manifest>()
        .await
        .map_err(QuiltManifestError::from)?;

    Ok(new_manifest)
}

pub async fn get_version(
    reqwest_client: &reqwest_middleware::ClientWithMiddleware,
    manifest_version_meta: LoaderVersion,
) -> anyhow::Result<PartialVersionInfo> {
    let server_url = reqwest::Url::parse(&manifest_version_meta.url)?;
    let new_manifest = reqwest_client
        .get(server_url)
        .send()
        .await?
        .json::<PartialVersionInfo>()
        .await?;

    Ok(new_manifest)
}

pub fn replace_template(
    template_info: &PartialVersionInfo,
    game_version: &str,
    template: &str,
) -> PartialVersionInfo {
    let mut version_info = template_info.clone();
    version_info.id = version_info.id.replace(template, game_version);
    version_info.inherits_from = version_info.inherits_from.replace(template, game_version);
    for library in version_info.libraries.iter_mut() {
        library.name.version = library.name.version.replace(template, game_version);
    }

    version_info
}