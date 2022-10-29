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
use std::fs;
use std::path::{PathBuf, Path};
use anyhow::{Result, Context};

use crate::utils::now_timestamp;
use crate::{
    serializer::LemonSerializer,
    serializer::Serializer
};


#[derive(Debug, Clone)]
pub struct LemonStorage {
    db_path: PathBuf,
    serializer: LemonSerializer
}

type Data = HashMap<String, Vec<u8>>;
type Document = HashMap<String, Data>;
type Table = HashMap<String, Document>;

impl LemonStorage {

     pub fn new<P: AsRef<Path>>(
        db: P,
        s: Serializer,
    ) -> LemonStorage {

         let mut db_path_buf = PathBuf::new();
         db_path_buf.push(db);
         LemonStorage {
             db_path: db_path_buf,
             serializer: LemonSerializer::new(s)
         }
    }

    pub(crate) fn read(&self) ->  Result<Vec<Table>> {
        let raw = fs::read(&self.db_path).expect("Failed to open the file");
        let data = self.serializer
            .deserialize::<Vec<Table>>(&raw)
            .context("Failed to deserialize the file")?;

        Ok(data)

    }

    pub(crate) fn write(&self, data: Option<Vec<u8>>) -> Result<u64, String> {
        if let Some(data) = data {
            fs::write(&self.db_path, data).expect("Failed to write data");
        }
        Ok(now_timestamp())
    }

}
