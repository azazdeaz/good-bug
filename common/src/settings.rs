use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::{env, path::Path};

#[derive(Debug, Deserialize)]
pub struct Slam {
    pub openvslam_config: String,
    pub vocab: String,
    pub video: Option<String>,
    pub enable_auto_slace_estimation: bool,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub grpc_port: u16,
    pub rover_address: String,
    pub slam: Slam,
    pub detecor_model: String,
    // TODO add slam options
}

fn absolute_path(path: &str) -> String {
    let root = env::var("CONFIG_ROOT").unwrap_or_else(|_| "./config".into());
    let root = Path::new(&root);
    let path = root.join(path);
    path.canonicalize()
        .expect(&format!(
            "can't find {:?} from {:?}",
            path,
            Path::new(".").canonicalize()
        ))
        .to_str()
        .unwrap()
        .into()
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::default();
        s.merge(File::with_name(&absolute_path("default.toml")))?;
        s.merge(File::with_name(&absolute_path("local.toml")).required(false))?;
        let mut settings: Settings = s.try_into()?;

        // TODO find a better way to do this
        settings.slam.openvslam_config = absolute_path(&settings.slam.openvslam_config);
        settings.slam.vocab = absolute_path(&settings.slam.vocab);
        settings.slam.video = if let Some(video) = settings.slam.video {
            Some(absolute_path(&video))
        } else {
            None
        };
        settings.detecor_model = absolute_path(&settings.detecor_model);

        Ok(settings)
    }
}
