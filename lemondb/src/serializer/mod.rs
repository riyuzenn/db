/*
 *
 * Copyright (c) 2022 riyuzenn
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * at your option) any later version.
 *
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
*/


use std::fmt;

use serde::{Serialize, de::DeserializeOwned};

use crate::serializer::{
    json::JsonSerializer,
    yaml::YamlSerializer,
};

pub mod json;
pub mod yaml;


#[derive(Debug, Clone)]
pub enum Serializer {
    JSON,
    YAML
}

impl From<i32> for Serializer {
    fn from(item: i32) -> Self {
        match item {
            0 => Serializer::JSON,
            1 => Serializer::YAML,
            _ => Serializer::JSON,
        }
    }
}

impl fmt::Display for Serializer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// A wrapper for the avaialble Serializers such as
/// `Json`, `Yaml`, etc.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LemonSerializer {
    serializer: Serializer,
    json: JsonSerializer,
    yaml: YamlSerializer
}

impl LemonSerializer {

    /// Create a new LemonSerializer object.
    /// 
    /// ### Parameters
    /// `m` -> Serializer: The serializer method (JSON, YAML)
    pub fn new(m: Serializer) -> LemonSerializer {
        LemonSerializer {
            serializer: m,
            json: JsonSerializer::new(),
            yaml: YamlSerializer::new()
        }
    }

    pub fn serialize<V>(&self, data: &V) -> Result<Vec<u8>, String> 
    where 
        V: Serialize,
    {
        #[allow(unreachable_patterns)]
        match self.serializer {
            Serializer::JSON => self.json.serialize(data),
            Serializer::YAML => self.yaml.serialize(data),

            // Default Serializer: JSON
            _ => self.json.serialize(data),
        }
    }

    pub fn deserialize<V>(&self, data: &[u8]) -> Option<V>
    where
        V: DeserializeOwned,
    {
        #[allow(unreachable_patterns)]
        match self.serializer {
            Serializer::JSON => self.json.deserialize(data),
            Serializer::YAML => self.json.deserialize(data),

            // Default Serializer: JSON
            _ => self.json.deserialize(data)
        }
    }
} 

