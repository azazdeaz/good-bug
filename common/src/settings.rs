use crate::types::{Map, NavGoal, Point3};
use config::{Config, ConfigError, File};
use eyre::{Result, WrapErr};
use serde::{Deserialize, Serialize};
use std::{
    env,
    io::Write,
    path::{Path, PathBuf},
};

impl Map {
    pub fn get_abs_db_path(&self) -> String {
        absolute_path(&self.db_path).unwrap()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Slam {
    pub openvslam_config: String,
    pub vocab: String,
    pub gstreamer_pipeline: Option<String>,
    pub video: Option<String>,
    pub mask: Option<String>,
    pub current_map_name: Option<String>,
    pub maps: Option<Vec<Map>>,
    pub enable_auto_slace_estimation: bool,
}

impl Slam {
    pub fn get_current_map(&self) -> Option<&Map> {
        if let (Some(maps), Some(current_map_name)) = (&self.maps, &self.current_map_name) {
            maps.into_iter().find(|map| &map.name == current_map_name)
        }
        else {
            None
        }
    }
    pub fn get_current_map_mut(&mut self) -> Option<&mut Map> {
        if let (Some(maps), Some(current_map_name)) = (&mut self.maps, &self.current_map_name) {
            maps.into_iter().find(|map| &map.name == current_map_name)
        }
        else {
            None
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Navigation {
    pub travel_thrust: f64,
    pub turn_right_thrust: (f64, f64),
    pub xy_goal_tolerance: f64,
    pub yaw_goal_tolerance: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    pub grpc_port: u16,
    pub rover_address: String,
    pub slam: Slam,
    pub navigation: Navigation,
    pub detecor_model: Option<String>,
    // TODO add slam options
}

fn path_in_config_folder(path: &str) -> PathBuf {
    let root = env::var("CONFIG_ROOT").unwrap_or_else(|_| "./config".into());
    let root = Path::new(&root);
    root.join(path)
}

fn absolute_path(path: &str) -> Result<String> {
    let path = path_in_config_folder(path);
    Ok(path
        .canonicalize()
        .wrap_err_with(|| {
            format!(
                "can't find {:?} from {:?}",
                path,
                Path::new(".").canonicalize()
            )
        })?
        .to_str()
        .unwrap()
        .into())
}

impl Settings {
    pub fn new() -> Result<Self> {
        let mut s = Config::default();
        s.merge(File::with_name(&absolute_path("default.toml")?))?;
        if let Ok(path) = absolute_path("local.toml") {
            s.merge(File::with_name(&path).required(false))?;
        }
        if let Ok(path) = absolute_path("generated.toml") {
            s.merge(File::with_name(&path).required(false))?;
        }
        let mut settings: Settings = s.try_into()?;

        // TODO find a better way to do this
        settings.slam.openvslam_config = absolute_path(&settings.slam.openvslam_config)?;
        settings.slam.vocab = absolute_path(&settings.slam.vocab)?;
        settings.slam.video = if let Some(video) = settings.slam.video {
            Some(absolute_path(&video)?)
        } else {
            None
        };
        settings.slam.mask = if let Some(mask) = settings.slam.mask {
            Some(absolute_path(&mask)?)
        } else {
            None
        };

        settings.detecor_model = if let Some(detecor_model) = settings.detecor_model {
            Some(absolute_path(&detecor_model)?)
        } else {
            None
        };

        Ok(settings)
    }

    pub fn add_map(&mut self, map_name: String, db_path: String) {
        if self.slam.maps.is_none() {
            self.slam.maps = Some(Vec::new());
        }

        let maps = &mut self.slam.maps;
        let mut map_updated = false;

        if let Some(maps) = maps {
            for map in maps {
                if map.name == map_name {
                    map.db_path = db_path.clone();
                    map_updated = true;
                }
            }
        }

        if let Some(maps) = maps {
            if !map_updated {
                maps.push(Map {
                    name: map_name,
                    db_path,
                    waypoints: Vec::new(),
                });
            }
        }

        self.save();
    }

    pub fn set_current_map_name(&mut self, map_name: Option<String>) {
        self.slam.current_map_name = map_name;
        self.save();
    }

    pub fn set_waypoints(&mut self, waypoints: Vec<NavGoal>) {
        if let Some(map) = &mut self.slam.get_current_map_mut() {
            map.waypoints = waypoints;
            self.save();
        }
    }

    pub fn save(&self) {
        let mut file = std::fs::File::create(path_in_config_folder("generated.toml"))
            .expect("Failed to create config file");
        file.write_all(toml::Value::try_from(self).unwrap().to_string().as_bytes())
            .expect("Failed to write config file");
    }
}
