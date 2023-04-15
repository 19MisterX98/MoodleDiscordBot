use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::data::course_contents::{Generate, Id};
use crate::data::gen_module::GenModuleBuilder;
use crate::data::modules::unknown::CourseModule;
use crate::Moodle;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub feedbacks: Vec<Feedback>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Feedback {
    //pub id: i64,
    //pub course: i64,
    pub name: String,
    pub intro: String,
    //pub introformat: i64,
    //pub anonymous: i64,
    //#[serde(rename = "multiple_submit")]
    //pub multiple_submit: bool,
    //pub autonumbering: bool,
    //#[serde(rename = "page_after_submitformat")]
    //pub page_after_submitformat: i64,
    //#[serde(rename = "publish_stats")]
    //pub publish_stats: bool,
    //pub completionsubmit: bool,
    pub coursemodule: i64,
    //pub introfiles: Vec<Value>,
}

impl Id for Feedback {
    fn get_id(&self) -> i64 {
        self.coursemodule
    }
}

#[async_trait]
impl Generate for Feedback {
    async fn request(client: &Moodle, id: i64) -> anyhow::Result<Vec<Self>> {
        client.get_feedback_for_course(id).await
    }

    fn gen(self, builder: &mut GenModuleBuilder, _: CourseModule) -> &mut GenModuleBuilder {
        builder.string("Bescheibung", self.intro)
    }
}
