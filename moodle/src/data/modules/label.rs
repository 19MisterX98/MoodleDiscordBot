use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::data::course_contents::{Generate, Id};
use crate::data::gen_module::GenModuleBuilder;
use crate::data::modules::unknown::CourseModule;
use crate::Moodle;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub labels: Vec<Label>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Label {
    //pub id: i64,
    pub coursemodule: i64,
    //pub course: i64,
    pub name: String,
    pub intro: String,
    //pub introformat: i64,
    //pub introfiles: Vec<Introfile>,
    pub timemodified: i64,
    //pub section: i64,
    //pub visible: i64,
    //pub groupmode: i64,
    //pub groupingid: i64,
}

impl Id for Label {
    fn get_id(&self) -> i64 {
        self.coursemodule
    }
}

#[async_trait]
impl Generate for Label {
    async fn request(client: &Moodle, id: i64) -> anyhow::Result<Vec<Self>> {
        client.get_labels_for_course(id).await
    }

    fn gen(self, builder: &mut GenModuleBuilder, _: CourseModule) -> &mut GenModuleBuilder {
        builder
            .string("Bescheibung", self.intro)
            .date("Änderungsdatum", self.timemodified)
    }
}
