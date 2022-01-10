use std::path::{Path, PathBuf};
use std::io;
use std::collections::HashMap;
use std::env;

pub struct SceneValue {
    bulb: String,
    brightness: u16,
    kelvin: u16,
}

pub struct Scene {
    name: String,
    command: String,
    values: Vec<SceneValue>,
}

pub struct Config {
    bulbs: HashMap<String, String>,
    scenes: Vec<Scene>
}

fn get_config_dir() -> PathBuf {
    if cfg!(windows) {
        let path = Path::new(env::var("APPDATA").expect("APPDATA should be set on Windows"));
        path.into()
    } else {
        let path = Path::new(env::var("XDG_CONFIG_HOME").or_else("~/.config"));
        path.into()
    }
}

pub fn get_config() -> Result<Config, io::Error> {
    Ok(println!(get_config_dir()))
    unimplemented!()
}
