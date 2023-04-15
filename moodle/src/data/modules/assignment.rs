use crate::data::course_contents::{Generate, Id};
use crate::data::gen_module::GenModuleBuilder;
use crate::data::modules::unknown::CourseModule;
use crate::data::other_content::file::FileInfo;
use crate::Moodle;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub courses: Vec<Course>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Course {
    //pub id: i64,
    //pub fullname: String,
    //pub shortname: String,
    //pub timemodified: i64,
    pub assignments: Vec<Assignment>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Assignment {
    //pub id: i64,
    pub cmid: i64,
    //pub course: i64,
    pub name: String,
    //pub nosubmissions: i64,
    //pub submissiondrafts: i64,
    //pub sendnotifications: i64,
    //pub sendlatenotifications: i64,
    //pub sendstudentnotifications: i64,
    pub duedate: i64,
    pub allowsubmissionsfromdate: i64,
    //pub grade: i64,
    pub timemodified: i64,
    //pub completionsubmit: i64,
    pub cutoffdate: i64,
    pub gradingduedate: i64,
    //pub teamsubmission: i64,
    //pub requireallteammemberssubmit: i64,
    //pub teamsubmissiongroupingid: i64,
    //pub blindmarking: i64,
    //pub hidegrader: i64,
    //pub revealidentities: i64,
    //pub attemptreopenmethod: String,
    //pub maxattempts: i64,
    //pub markingworkflow: i64,
    //pub markingallocation: i64,
    //pub requiresubmissionstatement: i64,
    //pub preventsubmissionnotingroup: i64,
    //pub configs: Vec<Config>,
    pub intro: String,
    //pub introformat: i64,
    //pub introfiles: Vec<Value>,
    pub introattachments: Vec<FileInfo>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub plugin: String,
    pub subtype: String,
    pub name: String,
    pub value: String,
}

impl Id for Assignment {
    fn get_id(&self) -> i64 {
        self.cmid
    }
}

#[async_trait]
impl Generate for Assignment {
    async fn request(client: &Moodle, id: i64) -> anyhow::Result<Vec<Self>> {
        client.get_assignments_for_course(id).await
    }

    fn gen(self, builder: &mut GenModuleBuilder, _: CourseModule) -> &mut GenModuleBuilder {
        builder
            .string("Bescheibung", self.intro)
            .date("Abgabebeginn", self.allowsubmissionsfromdate)
            .date("Abgabedatum", self.duedate)
            .date("Letzte Abgabemöglichkeit", self.cutoffdate)
            .date("Bewertungstermin", self.gradingduedate)
            .date("Änderungsdatum", self.timemodified)
            .files(self.introattachments)
    }
}
