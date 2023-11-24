use serde::*;

use crate::ForceExpand;

#[derive(Debug)]
pub struct Map {
    pub head: MapHead,
    pub objects: Vec<Object>,
}

#[derive(Clone, Debug)]
pub struct MapHead {
    pub name: String,
    pub version: u16,
    pub tileset: u16,
    pub tileset2: u16,
    pub bg: u16,
    pub spikes: u16,
    pub spikes2: u16,
    pub width: u16,
    pub height: u16,
    pub colors: String,
    pub scroll_mode: u16,
    pub music: u16,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Object {
    #[serde(rename = "@type")]
    pub type_id: u16,
    #[serde(rename = "@x")]
    pub x: u32,
    #[serde(rename = "@y")]
    pub y: u32,
    #[serde(default)]
    #[serde(rename = "@slot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot: Option<u16>,
    #[serde(rename = "@sprite_angle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<Rotation>,
    #[serde(rename = "event")]
    #[serde(default)]
    pub events: Vec<Event>,
    #[serde(rename = "param")]
    pub params: Vec<Param>,
    #[serde(rename = "obj")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nested_object: Option<Box<Object>>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Event {
    #[serde(rename = "@eventIndex")]
    pub id: u16,
    #[serde(rename = "param")]
    #[serde(default)]
    pub params: Vec<Param>,
    #[serde(rename = "event")]
    #[serde(default)]
    pub nested_events: Vec<Event>,
}

impl Serialize for Event {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct EventSerializable<'a> {
            #[serde(rename = "@eventIndex")]
            id: u16,
            #[serde(rename = "param")]
            params: &'a [Param],
            #[serde(rename = "event")]
            nested_events: &'a [Event],
            #[serde(rename = "$text")]
            _expand: ForceExpand,
        }

        let event = EventSerializable {
            id: self.id,
            params: &self.params,
            nested_events: &self.nested_events,
            _expand: ForceExpand,
        };
        Ok(event.serialize(serializer)?)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rotation {
    #[serde(rename = "0")]
    Rotate0,
    #[serde(rename = "90")]
    Rotate90,
    #[serde(rename = "180")]
    Rotate180,
    #[serde(rename = "270")]
    Rotate270,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Param {
    #[serde(rename = "@key")]
    pub key: String,
    #[serde(rename = "@val")]
    pub value: String,
}

impl Param {
    pub fn new<K, V>(key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct SerializableMap {
    head: SerializableMapHead,
    objects: ObjectsSerializable,
}

#[derive(Serialize, Deserialize)]
struct ObjectsSerializable {
    #[serde(rename = "object")]
    #[serde(default)]
    objects: Vec<Object>,
}

impl Serialize for Map {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let map = SerializableMap {
            head: SerializableMapHead {
                name: self.head.name.clone(),
                version: self.head.version,
                tileset: self.head.tileset,
                tileset2: self.head.tileset2,
                bg: self.head.bg,
                spikes: self.head.spikes,
                spikes2: self.head.spikes2,
                width: self.head.width,
                height: self.head.height,
                colors: self.head.colors.clone(),
                scroll_mode: self.head.scroll_mode,
                music: self.head.music,
                num_objects: self.objects.len() as u32,
            },
            objects: ObjectsSerializable {
                objects: self.objects.clone(),
            },
        };

        Ok(map.serialize(serializer)?)
    }
}

impl<'de> Deserialize<'de> for Map {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map_head = SerializableMap::deserialize(deserializer)?;

        Ok(Map {
            head: MapHead {
                name: map_head.head.name,
                version: map_head.head.version,
                tileset: map_head.head.tileset,
                tileset2: map_head.head.tileset2,
                bg: map_head.head.bg,
                spikes: map_head.head.spikes,
                spikes2: map_head.head.spikes2,
                width: map_head.head.width,
                height: map_head.head.height,
                colors: map_head.head.colors,
                scroll_mode: map_head.head.scroll_mode,
                music: map_head.head.music,
            },
            objects: map_head.objects.objects,
        })
    }
}

#[derive(Serialize, Deserialize)]
struct SerializableMapHead {
    name: String,
    version: u16,
    tileset: u16,
    tileset2: u16,
    bg: u16,
    spikes: u16,
    spikes2: u16,
    width: u16,
    height: u16,
    colors: String,
    scroll_mode: u16,
    music: u16,
    num_objects: u32,
}
