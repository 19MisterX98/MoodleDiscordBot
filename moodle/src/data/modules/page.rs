use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::data::course_contents::{Generate, Id};
use crate::data::gen_module::GenModuleBuilder;
use crate::data::modules::unknown::CourseModule;
use crate::data::other_content::file::FileInfo;
use crate::Moodle;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub pages: Vec<Page>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    //pub id: i64,
    pub coursemodule: i64,
    //pub course: i64,
    pub name: String,
    pub intro: String,
    //pub introformat: i64,
    //pub introfiles: Vec<Value>,
    pub content: String,
    //pub contentformat: i64,
    pub contentfiles: Vec<FileInfo>,
    //pub legacyfiles: i64,
    //pub legacyfileslast: Value,
    //pub display: i64,
    //pub displayoptions: String,
    pub revision: i64,
    pub timemodified: i64,
    //pub section: i64,
    //pub visible: i64,
    //pub groupmode: i64,
    //pub groupingid: i64,
}

impl Id for Page {
    fn get_id(&self) -> i64 {
        self.coursemodule
    }
}

#[async_trait]
impl Generate for Page {
    async fn request(client: &Moodle, id: i64) -> anyhow::Result<Vec<Self>> {
        client.get_pages_for_course(id).await
    }

    fn gen(self, builder: &mut GenModuleBuilder, _: CourseModule) -> &mut GenModuleBuilder {
        builder
            .string("Beschreibung", self.intro)
            .string("Content", self.content)
            .num("Revision", self.revision)
            .date("Änderungsdatum", self.timemodified)
            .files(self.contentfiles)
    }
}
