use crate::data::course_contents::{Generate, Id};
use crate::data::gen_module::{GenModule, GenModuleBuilder};
use crate::Moodle;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseModule {
    pub id: i64,             //id
    pub url: Option<String>, //points to segment
    pub name: String,        //name
    pub modicon: String,
    pub modname: String, //label, resource, feedback, forum, assign, folder, publication, questionaire, url...
    //pub customdata: String, //customcompletionrules and some other stuff
    pub dates: Vec<Date>, //for assignments and publications
    #[serde(default)]
    pub contents: Vec<Content>,
    pub description: Option<String>, //html description
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Date {
    pub label: String,
    pub timestamp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    #[serde(rename = "type")]
    pub type_field: String, //file or url
    pub filename: String,         //self explanatory
    pub filesize: i64,            // filesize in bytes
    pub fileurl: String,          //link to file
    pub timecreated: Option<i64>, //is always present?
    pub timemodified: i64,        //this too
    pub mimetype: Option<String>, //pdf etc
    pub author: Option<String>,   //creator
}

impl Id for CourseModule {
    fn get_id(&self) -> i64 {
        self.id
    }
}

#[async_trait]
impl Generate for CourseModule {
    async fn request(_: &Moodle, _: i64) -> anyhow::Result<Vec<Self>> {
        unreachable!()
    }

    async fn process(course_modules: Vec<CourseModule>, _: &Moodle, _: i64) -> anyhow::Result<Vec<GenModule>> {
        let res = course_modules
            .into_iter()
            .map(|module| {
                let mut builder =
                    GenModuleBuilder::new(module.id, module.modicon, module.name, module.url);
                builder
                    .string("Type", module.modname)
                    .contents(module.contents);
                builder.build()
            })
            .collect();
        Ok(res)
    }

    fn gen(self, _: &mut GenModuleBuilder, _: CourseModule) -> &mut GenModuleBuilder {
        unreachable!()
    }
}
