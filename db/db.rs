//use rocksdb::{DB, Options};
// NB: db is automatically closed at end of lifetime

use rocksdb::{DB};

//Input value into the database with given key 
//Key and Value should be serialized 
pub fn put(key: String, value: String){
    let path = "database";
    let db = DB::open_default(path).unwrap();
    db.put(key,value).unwrap();   
}

//delete value from the database with given key 
pub fn delete(key: String){
    let path = "database";
    let db = DB::open_default(path).unwrap();
    db.delete(key).unwrap();   
}

//get value from the database with given key 
pub fn get(key: String) -> String {
    let path = "database";
    let db = DB::open_default(path).unwrap();
    match db.get(key) {
        Ok(Some(value)) => String::from_utf8(value).unwrap(),
        Ok(None) => String::from(""),
        Err(_) => String::from("Err"),
    }
}