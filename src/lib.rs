pub mod map;

use std::error::Error;

use map::{Map, MapHead};
use serde::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "sfm_maps")]
pub struct Level {
    #[serde(rename = "maps_head")]
    pub head: LevelHead,
    #[serde(rename = "sfm_map")]
    pub maps: Vec<Map>,
}

#[derive(Debug)]
pub struct LevelHead {
    pub name: String,
    pub version: u16,
    pub screenshot_submap: u16,
    pub last_submap: u16,
    pub submap_order: Vec<u16>,
}

#[derive(Deserialize, Debug, Default)]
pub(crate) struct ForceExpand; // Workaround due to a bug in quick-xml not expanding empty elements when containing an @ / $text field

impl Serialize for ForceExpand {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Ok(serializer.serialize_str("\0")?)
    }
}

mod level_head_serde {
    use super::*;
    #[derive(Serialize, Deserialize)]
    struct InternalMapHead {
        maps_name: String,
        maps_version: u16,
        screenshot_submap: u16,
        last_submap: u16,
        submap_order: SubmapOrder,
    }

    #[derive(Serialize, Deserialize)]
    struct SubmapOrder {
        #[serde(rename = "map")]
        map: Vec<MapId>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct MapId {
        #[serde(rename = "@id")]
        id: u16,
        #[serde(rename = "$text")]
        #[serde(skip_deserializing)]
        _force_expand: ForceExpand,
    }

    impl Serialize for LevelHead {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let submap_order = SubmapOrder {
                map: self
                    .submap_order
                    .iter()
                    .map(|id| MapId {
                        id: *id,
                        _force_expand: ForceExpand,
                    })
                    .collect(),
            };

            let map_head = InternalMapHead {
                maps_name: self.name.clone(),
                maps_version: self.version,
                screenshot_submap: self.screenshot_submap,
                last_submap: self.last_submap,
                submap_order,
            };
            Ok(map_head.serialize(serializer)?)
        }
    }

    impl<'de> Deserialize<'de> for LevelHead {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let map_head = InternalMapHead::deserialize(deserializer)?;

            Ok(LevelHead {
                name: map_head.maps_name,
                version: map_head.maps_version,
                screenshot_submap: map_head.screenshot_submap,
                last_submap: map_head.last_submap,
                submap_order: map_head
                    .submap_order
                    .map
                    .into_iter()
                    .map(|id| id.id)
                    .collect(),
            })
        }
    }
}

pub fn deserialize_level(level_xml: &str) -> Result<Level, Box<dyn Error>> {
    Ok(quick_xml::de::from_str(&level_xml)?)
}

pub fn serialize_level(level: &Level) -> Result<String, Box<dyn Error>> {
    let mut result = String::new();
    let mut serializer = quick_xml::se::Serializer::new(&mut result);
    serializer.expand_empty_elements(true); // Bugged: doesn't expand empty elements when has an @id / $text field
    level.serialize(serializer).unwrap();
    Ok(result.replace("\0", ""))
}

pub fn generic_level(name: impl Into<String>) -> Level {
    let name = name.into();
    Level {
        head: LevelHead {
            name: name.clone(),
            version: 103,
            screenshot_submap: 0,
            last_submap: 0,
            submap_order: vec![0],
        },
        maps: vec![Map {
            head: MapHead {
                name,
                version: 103,
                tileset: 20,
                tileset2: 20,
                bg: 1,
                spikes: 4,
                spikes2: 40,
                width: 1600,
                height: 608,
                colors: "5A0200000600000005000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".into(),
                scroll_mode: 0,
                music: 60,
            },
            objects: vec![],
        }],
    }
}
