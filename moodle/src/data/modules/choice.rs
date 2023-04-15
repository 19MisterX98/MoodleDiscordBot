use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::data::course_contents::{Generate, Id};
use crate::data::gen_module::GenModuleBuilder;
use crate::data::modules::unknown::CourseModule;
use crate::Moodle;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub choices: Vec<Choice>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Choice {
    //pub id: i64,
    pub coursemodule: i64,
    //pub course: i64,
    pub name: String,
    pub intro: String,
    //pub introformat: i64,
    //pub introfiles: Vec<Value>,
    //pub publish: bool,
    //pub showresults: i64,
    //pub display: i64,
    pub allowupdate: bool,
    pub allowmultiple: bool,
    //pub showunanswered: bool,
    //pub includeinactive: bool,
    //pub limitanswers: bool,
    //pub timeopen: i64,
    //pub timeclose: i64,
    //pub showpreview: bool,
    //pub showavailable: bool,
}

impl Id for Choice {
    fn get_id(&self) -> i64 {
        self.coursemodule
    }
}

#[async_trait]
impl Generate for Choice {
    async fn request(client: &Moodle, id: i64) -> anyhow::Result<Vec<Self>> {
        client.get_choices_for_course(id).await
    }

    fn gen(self, builder: &mut GenModuleBuilder, _: CourseModule) -> &mut GenModuleBuilder {
        builder
            .string("Bescheibung", self.intro)
            .bool("Änderbar", self.allowupdate)
            .bool("Mehrfachauswahl", self.allowmultiple)
    }
}
