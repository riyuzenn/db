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


use std::collections::HashMap;

use super::id::LemonId;

type Data = HashMap<String, Vec<u8>>;

#[derive(Debug, Clone)]
pub struct LemonDocument {
    pub id: String,
}

impl LemonDocument {

    pub fn new() -> LemonDocument
    {
        LemonDocument {
            id: LemonId::new("id").gen()
        }
    }

    pub fn set_data(&mut self, key: &str, d: Vec<u8>) -> Result<Data, String> 
    {
        let mut map: Data = HashMap::new();
        map.insert(key.to_string(), d);

        Ok(map)
    }
}
