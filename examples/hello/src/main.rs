use lemondb::{
    LemonDb,
    LemonOption,
    LemonDumpRule,
    Serializer
};

use serde::{
    Serialize,
    Deserialize
};

#[derive(Serialize, Deserialize)]
pub struct User {
    name: String,
    surname: String,
}

fn main() {
    let u = User {
        name: "John".to_string(),
        surname: "Doe".to_string()
    };

    let mut db = LemonDb::new(
        "db",
        LemonOption {
           serializer: Serializer::YAML,
           dump_rule: LemonDumpRule::AUTO,
           table_name: None
        }
    );


    db.insert::<User>("user", &u).unwrap();
}
