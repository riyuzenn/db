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


use serde::{Serialize, de::DeserializeOwned};

#[derive(Debug, Clone)]
pub struct YamlSerializer {}

impl YamlSerializer {
    
    pub(super) fn new() -> YamlSerializer {
        YamlSerializer{}
    }

    pub fn deserialize<V>(&self, data: &[u8]) -> Option<V>
    where 
        V: DeserializeOwned,
    {
        match serde_yaml::from_str(std::str::from_utf8(data).unwrap()) {
            Ok(val) => Some(val),
            Err(_) => None,
        }

    }

    pub fn serialize<V>(&self, data: &V) -> Result<Vec<u8>, String>
    where
        V: Serialize,
    {
        match serde_yaml::to_string(data) {
            Ok(data) => Ok(data.into_bytes()),
            Err(err) => Err(err.to_string())
        }
    }
}
