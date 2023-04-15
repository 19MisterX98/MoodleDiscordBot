use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::data::course_contents::{Generate, Id};
use crate::data::gen_module::GenModuleBuilder;
use crate::data::modules::unknown::CourseModule;
use crate::Moodle;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub glossaries: Vec<Glossary>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Glossary {
    //pub id: i64,
    pub coursemodule: i64,
    //pub course: i64,
    pub name: String,
    pub intro: String,
    //pub introformat: i64,
    //pub introfiles: Vec<Value>,
    //pub allowduplicatedentries: i64,
    pub displayformat: String,
    //pub mainglossary: i64,
    //pub showspecial: i64,
    //pub showalphabet: i64,
    //pub showall: i64,
    //pub allowcomments: i64,
    //pub allowprintview: i64,
    //pub usedynalink: i64,
    //pub defaultapproval: i64,
    //pub approvaldisplayformat: String,
    //pub globalglossary: i64,
    //pub entbypage: i64,
    //pub editalways: i64,
    //pub rsstype: i64,
    //pub rssarticles: i64,
    //pub assessed: i64,
    //pub assesstimestart: i64,
    //pub assesstimefinish: i64,
    //pub scale: i64,
    pub timecreated: i64,
    pub timemodified: i64,
    //pub completionentries: i64,
    //pub section: i64,
    //pub visible: i64,
    //pub groupmode: i64,
    //pub groupingid: i64,
    //pub browsemodes: Vec<String>,
    //pub canaddentry: i64,
}

impl Id for Glossary {
    fn get_id(&self) -> i64 {
        self.coursemodule
    }
}

#[async_trait]
impl Generate for Glossary {
    async fn request(client: &Moodle, id: i64) -> anyhow::Result<Vec<Self>> {
        client.get_glossaries_for_course(id).await
    }

    fn gen(self, builder: &mut GenModuleBuilder, _: CourseModule) -> &mut GenModuleBuilder {
        builder
            .string("Bescheibung", self.intro)
            .string("Format", self.displayformat)
            .date("Änderungsdatum", self.timemodified)
    }
}
