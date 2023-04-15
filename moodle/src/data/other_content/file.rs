use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileInfo {
    pub filename: String,
    //pub filepath: String,
    //pub filesize: i64,
    pub fileurl: String,
    //pub timemodified: i64,
    //pub mimetype: String,
    //pub isexternalfile: bool,
}
