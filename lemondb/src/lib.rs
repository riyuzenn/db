///! LemonDb
///! =======
///! 
///! LemonDB is a lightweight no-sql document-oriented key-value storage database focusing in
///! performance and easability. It is heavily inspired with [lemondb](https://github.com/riyuzenn/lemondb) with added features
///!
///! The logic is simple. All data were act as document and stored in a tables.
///! Let's say you have a table of users with corresponding documents and value 
///! (key: name, value: John Doe). The diagram shows how it stores the data
///!
///! 
///! |-----------------------------------------------|
///! | Table: User                                   |
///! |-----------------------------------------------|
///! | Document                                      |
///! |   |                                           |
///! |   |-- id                                      |
///! |   |-- Data                                    |
///! |       |                                       |
///! |       |-- key (name)                          |
///! |       |   |-- &str                            |
///! |       |                                       |
///! |       |-- value (John Doe)                    |
///! |           |-- Any serializable type (serde)   |
///! |                                               |
///! |-----------------------------------------------|
///!  \----------------------------------------------\
///!

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


pub use crate::db::{
    LemonDb,
    LemonOption,
    LemonDumpRule
};

pub use crate::serializer::Serializer;

pub(crate) use crate::serializer::LemonSerializer;

pub mod db;
pub mod utils;
pub mod id;
pub mod serializer;

mod document;
mod storage;
mod query;
