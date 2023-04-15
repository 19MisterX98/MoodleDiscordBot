use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Data {
    pub courses: Vec<Course>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Course {
    pub id: i64,
    pub fullname: String,
    pub shortname: String,
    pub summary: String,
    pub startdate: i64,
    pub enddate: i64,
    pub viewurl: String,
    pub courseimage: String,
    pub coursecategory: String,
}
