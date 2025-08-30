use std::str::FromStr;

pub struct FileData{
    name: String,
    path: String,
    last_changes: String,
}

impl FileData {
    pub fn begin() -> String{
        return String::from_str("|Name file|Path|Last changes|")
                        .ok()
                        .unwrap();
    }

    pub fn new(name: String, path: String, last_changes: String,) -> Self{
        Self { name: name, path: path, last_changes: last_changes }
    }

    pub fn to_string(&self) -> String {
        let data = format!("|{}|{}|{}|",self.name,self.path,self.last_changes);
        return data;
    }
}