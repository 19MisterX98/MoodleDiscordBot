use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::data::course_contents::{Generate, Id};
use crate::data::gen_module::GenModuleBuilder;
use crate::data::modules::unknown::CourseModule;
use crate::Moodle;

pub type Root = Vec<Forum>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Forum {
    //pub id: i64,
    //pub course: i64,
    //pub type_field: String,
    pub name: String,
    pub intro: String,
    //pub introformat: i64,
    //pub introfiles: Vec<Value>,
    //pub duedate: i64,
    //pub cutoffdate: i64,
    //pub assessed: i64,
    //pub assesstimestart: i64,
    //pub assesstimefinish: i64,
    //pub scale: i64,
    //#[serde(rename = "grade_forum")]
    //pub grade_forum: i64,
    //#[serde(rename = "grade_forum_notify")]
    //pub grade_forum_notify: i64,
    //pub maxbytes: i64,
    //pub maxattachments: i64,
    //pub forcesubscribe: i64,
    //pub trackingtype: i64,
    //pub rsstype: i64,
    //pub rssarticles: i64,
    pub timemodified: i64,
    //pub warnafter: i64,
    //pub blockafter: i64,
    //pub blockperiod: i64,
    //pub completiondiscussions: i64,
    //pub completionreplies: i64,
    //pub completionposts: i64,
    pub cmid: i64,
    pub numdiscussions: i64,
    //pub cancreatediscussions: bool,
    //pub lockdiscussionafter: i64,
    //pub istracked: bool,
}

impl Id for Forum {
    fn get_id(&self) -> i64 {
        self.cmid
    }
}

#[async_trait]
impl Generate for Forum {
    async fn request(client: &Moodle, id: i64) -> anyhow::Result<Vec<Self>> {
        client.get_forums_for_course(id).await
    }

    fn gen(self, builder: &mut GenModuleBuilder, _: CourseModule) -> &mut GenModuleBuilder {
        builder
            .string("Bescheibung", self.intro)
            .num("Anzahl an Disskusionen", self.numdiscussions)
            .date("Änderungsdatum", self.timemodified)
    }
}
