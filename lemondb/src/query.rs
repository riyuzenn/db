use std::collections::HashMap;



type Data = HashMap<String, Vec<u8>>;

pub struct Cursor {
    data: Document,
    id: String,

}

