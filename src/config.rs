use std::collections::HashMap;
use std::convert::TryInto;
use std::fs;
use std::path::Path;

use serde_derive::Deserialize;

use crate::messages::{Command, MacAddress};

#[derive(Deserialize, Debug, Clone)]
pub struct SceneValue {
    pub bulb: String,
    pub brightness: u16,
    pub kelvin: u16,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Scene {
    pub name: String,
    pub command: String,
    pub values: Vec<SceneValue>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub bulbs: HashMap<String, String>,
    pub scenes: Vec<Scene>,
}

impl Config {
    pub fn from_file(path: &Path) -> Config {
        let contents = fs::read_to_string(path).expect(&format!(
            "Something went wrong reading the config file at: {}\nAre you sure it exists?",
            path.to_str().expect("Config file path is not valid UTF-8")
        ));
        toml::from_str(&contents).expect("The config file is not respecting the correct schema")
    }

    pub fn from_default_file() -> Config {
        let mut filepath = dirs::config_dir().expect("Unable to retrieve config folder");
        filepath.push("sunrs");
        filepath.push("config.toml");
        Config::from_file(&filepath)
    }

    pub fn bulbs_addresses(&self) -> Vec<MacAddress> {
        self.bulbs
            .iter()
            .map(|(_k, v)| -> MacAddress { str_to_address(v) })
            .collect()
    }

    pub fn scene_by_command(&self, command: &str) -> Option<Vec<Command>> {
        for scene in self.scenes.iter() {
            if &scene.command == command {
                let commands = self.commands_from_scene(scene);
                return Some(commands);
            }
        }
        None
    }

    pub fn scene_by_name(&self, name: &str) -> Option<Vec<Command>> {
        for scene in self.scenes.iter() {
            if &scene.name == name {
                let commands = self.commands_from_scene(scene);
                return Some(commands);
            }
        }
        None
    }

    pub fn scene_by_index(&self, index: usize) -> Option<Vec<Command>> {
        let scene = self.scenes.get(index)?;
        let commands = self.commands_from_scene(scene);
        return Some(commands);
    }

    fn commands_from_scene(&self, scene: &Scene) -> Vec<Command> {
        scene
            .values
            .iter()
            .map(|sv: &SceneValue| {
                let mac_address = self
                    .mac_from_bulb_name(&sv.bulb)
                    .expect("Bulb in scene value does not exists in bulbs list");
                Command {
                    mac_address,
                    brightness: sv.brightness,
                    kelvin: sv.kelvin,
                }
            })
            .collect()
    }

    fn mac_from_bulb_name(&self, bulb: &str) -> Option<MacAddress> {
        let bulb_address = self.bulbs.get(bulb)?;
        Some(str_to_address(&bulb_address))
    }
}

fn str_to_address(mac_address: &str) -> MacAddress {
    let parsed_mac = hex::decode(format!("{}0000", mac_address.replace(":", ""))).expect(&format!(
        "Expected a 6 bytes mac address but got '{}'",
        mac_address
    ));

    let mac_array: Box<MacAddress> = parsed_mac.into_boxed_slice().try_into().expect(&format!(
        "Expected a 6 bytes mac address but got '{}'",
        mac_address
    ));
    *mac_array
}
