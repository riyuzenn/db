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

use std::{
    path::{
        PathBuf, 
        Path
    }, 
    time::{Duration, Instant}, 
    collections::HashMap
};
use serde::Serialize;
use anyhow::{Result, Context};

use crate::{
    storage::LemonStorage, 
    document::LemonDocument, 
    Serializer, 
    LemonSerializer,
};

type Data = HashMap<String, Vec<u8>>;
type Document = HashMap<String, Data>;
type Table = HashMap<String, Document>;


/// The lemon dump rule for dumping the database.
/// AUTO - All changes were automatically dumped for persistency.
/// NEVER - Never dump any changes, keep it in the memory
/// PERIODIC(Duration) - Dump the databse on the given duration.
#[derive(Debug, Clone)]
pub enum LemonDumpRule {
    AUTO,
    NEVER,
    PERIODIC(Duration)
}

#[derive(Debug, Clone)]
pub struct LemonDb {
    // Set the database path and table name
    // accessible by the public.
    pub db_path: PathBuf,
    pub table: String,

    map: Vec<
        HashMap<String, Document>
    >,
    storage: LemonStorage,
    dump_rule: LemonDumpRule,
    serializer: LemonSerializer,
    last_dump: Instant,
}

#[derive(Debug, Clone)]
pub struct LemonOption {
    pub table_name: Option<&'static str>,
    pub dump_rule: LemonDumpRule,
    pub serializer: Serializer,
}

impl LemonDb {

    /// Create a ned LemonDb instance
    ///
    ///
    /// LemonDb is a lightweight no-sql document-oriented key-value storage database focusing in
    /// performance and easability.
    ///
    /// The logic is simple. All data were act as document and stored in a tables.
    /// Let's say you have a table foo and a document of bar, baz as well as a table of
    /// users with corresponding documents. The diagram shows
    /// how it stores the data
    ///
    /// |-----------------------------------------------|
    /// | Table: User                                   |
    /// |-----------------------------------------------|
    /// | Document                                      |
    /// |   |                                           |
    /// |   |-- id                                      |
    /// |   |-- Data                                    |
    /// |       |                                       |
    /// |       |-- key (name)                          |
    /// |       |   |-- &str                            |
    /// |       |                                       |
    /// |       |-- value (John Doe)                    |
    /// |           |-- Any serializable type (serde)   |
    /// |-----------------------------------------------|
    ///  \----------------------------------------------\
    ///
    /// 
    /// # Arguments
    ///
    /// * `db_path` - The path of the database
    /// * `option`  - Init option for the database
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use lemondb::{LemonDb, LemonOption,
    ///   LemonDumpRule,
    ///   Serializer
    /// };
    ///
    /// // Initialize databse
    /// let db = LemonDb::new(
    ///   "test.db",
    ///   LemonOption {
    ///     table_name: None,
    ///     dump_rule: LemonDumpRule::AUTO,
    ///     serializer: Serializer::JSON,
    ///   }
    /// );
    /// ...
    ///
    /// ```
    pub fn new<P: AsRef<Path>>(
        db_path: P,
        option: LemonOption
    ) -> LemonDb {
        
        let db_path_buf = PathBuf::new().join(db_path); 

        let s = LemonStorage::new(db_path_buf.clone(), option.serializer.clone());
        let table_name = option.table_name.or(Some("_table")).unwrap();

        let empty_map: Document = HashMap::new();
        let mut map: HashMap<String, Document> = HashMap::new();
        map.insert(table_name.to_string(), empty_map);


        LemonDb {
            db_path: db_path_buf,
            // Set the default table name to _table
            table: table_name.to_string(),
            map: vec![map],
            storage: s,
            serializer: LemonSerializer::new(option.serializer),
            dump_rule: option.dump_rule,
            last_dump: Instant::now(),
        }
    }
    
    /// Load the DB from the file
    ///
    /// This function open and load the file & deserialize it then return a structured data 
    /// of the database. The function return a Result of LemonDb instance.
    ///
    /// # Arguments
    ///
    /// * `dp_path` - The path of the database to be loaded
    /// * `option`  - The init option of the databse `LemonOption`
    ///
    /// # Examples
    ///
    /// ```
    /// let db = LemonDb::open(
    ///   "test.db",
    ///   LemonOption {
    ///     table_name: None,
    ///     dump_rule: LemonDumpRule::AUTO,
    ///     serializer: Serializer::JSON
    ///   }
    /// ).unwrap()
    /// ...
    /// ```
    ///
    pub fn open<P: AsRef<Path>>(
        db_path: P,
        option: LemonOption,
    ) -> Result<LemonDb> 
    {
        let db_path_buf = PathBuf::new().join(db_path.as_ref());
        let s = LemonStorage::new(&db_path, option.serializer.clone());
        let content = s.read()
            .context("Failed to read the database. It's either doenst exist or not a database object")?;
        
        let table_name = option.table_name.or(Some("_table")).unwrap();
        Ok(
            LemonDb {
                db_path: db_path_buf,
                table: table_name.to_string(),
                map: content,
                storage: s,
                serializer: LemonSerializer::new(option.serializer),
                dump_rule: option.dump_rule,
                last_dump: Instant::now(),
            }
        )

    }

    /// ### table `fn`
    ///
    /// Return a copy of the database given by the table name
    ///
    /// # Arguments
    ///
    /// * `name` - The table name to be use
    ///
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// let mut db = LemonDb(...);
    /// 
    /// // Insert the data in the default table: `_table`
    /// db.insert::<String>("hello", &String::from("world")).unwrap();
    ///
    /// 
    /// let mut user = db.table("user");
    ///
    /// // Insert the data to the user table
    /// user.insert::<String>("name"d, &"John Doe".to_string()).unwrap();
    ///
    /// ```
    ///
    pub fn table(&mut self, name: &str) -> Self {
        
        // Check if the table name already exist. If not then
        // create a empty table
        for i in self.map.iter_mut() {
            if !i.contains_key(name) {
                let temp: Document = HashMap::new();
                i.insert(name.to_string(), temp);
            }
        }

        Self {
            db_path: self.db_path.clone(),
            table: name.to_string(),
            map: self.map.clone(),
            storage: self.storage.clone(),
            serializer: self.serializer.clone(),
            dump_rule: self.dump_rule.clone(),
            last_dump: self.last_dump.clone(),
        }

    }

    /// ### insert `fn`
    ///
    /// Insert an item to the datbase. 
    ///
    /// The key has to be string but the value can be any type defined by the user
    /// that is serializable. That includes primitive types, tuples, vectors, structs and more
    ///
    /// The function return a result which can be wrapped.
    ///
    /// # Arguments
    ///
    /// * `key` - A string of key
    /// * `value` - Any value that can be serializable
    ///
    /// # Examples
    ///
    /// ```
    /// 
    /// let mut db = LemonDb::new("db", Serializer::JSON, LemonDumpRule::AUTO);
    /// db.insert("hello", &"world");
    /// 
    /// ```
    ///  
    /// ---
    ///
    ///
    /// You can also insert any serde serializable object including struct and enums
    ///
    /// **Example with struct object**
    /// ```
    /// # use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Serialize, Deserialize)]
    /// struct User {
    ///     name: String,
    ///     surname: String
    /// }
    ///
    /// impl User {
    ///     fn new(name: &str, surname: &str) -> User {
    ///         User {
    ///             name: name,
    ///             surname: surname
    ///         }
    ///     }
    ///     fn fullname(&self) -> String {
    ///         format!("{} {}", self.name, self.surname)
    ///     }
    /// }
    ///
    /// let mut db = LemonDb(...);
    /// db.insert::<User>("user1", &User::new("John", "Doe")).unwrap();
    ///
    /// ```
    pub fn insert<V>(&mut self, key: &str, value: &V) -> Result<()>
    where
        V: Serialize,
    {
 
        let mut document = LemonDocument::new();
        let raw = self.serializer.serialize(value).unwrap();
        let data = document.set_data(key, raw).unwrap();
        self.insert_data_in_table(&document.id, data).unwrap();

        self.dump().unwrap();
        Ok(())
    }
    
    /// An alias for insert `fn`
    /// 
    /// # Example
    ///
    /// ```rust
    /// 
    /// let mut db = LemonDb(...);
    /// db.set::<String>("hello", &String::from("world")).unwrap();
    /// 
    /// ```
    pub fn set<V>(&mut self, k: &str, v: &V) -> Result<()> 
    where
        V: Serialize,
    {
        self.insert(k, &v).with_context(|| "Failed to insert data")?;
        Ok(())

    }


    /// Dump the data to the file. The rule were set with
    /// `LemonDumpRule`
    ///
    pub fn dump(&mut self) -> Result<()> {
    
        let data = self.serializer.serialize::<Vec<Table>>(&self.map).unwrap();
        let write = || -> u64 {
            self.storage.write(Some(data)).unwrap()
        };

        match self.dump_rule {

            LemonDumpRule::AUTO => {
                write();
                self.last_dump = Instant::now();
            },
            LemonDumpRule::PERIODIC(duration) => {
                let now = Instant::now();
                if now.duration_since(self.last_dump) > duration {
                    write();
                };
            },
            LemonDumpRule::NEVER => (),

        }

        Ok(())

    }

    
    fn insert_data_in_table(&mut self, key: &str, data: Data) -> Result<Vec<Table>> {

        for table in self.map.iter_mut() {

            if table.contains_key(&self.table) {
                table.get_mut(&self.table).unwrap()
                    .insert(key.to_string(), data.clone());
            }
            
        }

        Ok(self.map.clone())
    }
    
}
