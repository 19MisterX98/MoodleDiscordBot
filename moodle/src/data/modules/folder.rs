use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::data::course_contents::{Generate, Id};
use crate::data::gen_module::GenModuleBuilder;
use crate::data::modules::unknown::CourseModule;
use crate::Moodle;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub folders: Vec<Folder>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Folder {
    //pub id: i64,
    pub coursemodule: i64,
    //pub course: i64,
    pub name: String,
    pub intro: String,
    //pub introformat: i64,
    //pub introfiles: Vec<Value>,
    pub revision: i64,
    pub timemodified: i64,
    //pub display: i64,
    //pub showexpanded: i64,
    //pub showdownloadfolder: i64,
    //pub forcedownload: i64,
    //pub section: i64,
    //pub visible: i64,
    //pub groupmode: i64,
    //pub groupingid: i64,
}

impl Id for Folder {
    fn get_id(&self) -> i64 {
        self.coursemodule
    }
}

#[async_trait]
impl Generate for Folder {
    async fn request(client: &Moodle, id: i64) -> anyhow::Result<Vec<Self>> {
        client.get_folders_for_course(id).await
    }

    fn gen(
        self,
        builder: &mut GenModuleBuilder,
        course_module: CourseModule,
    ) -> &mut GenModuleBuilder {
        builder
            .string("Bescheibung", self.intro)
            .num("Revision", self.revision)
            .date("Änderungsdatum", self.timemodified)
            .contents(course_module.contents)
    }
}
