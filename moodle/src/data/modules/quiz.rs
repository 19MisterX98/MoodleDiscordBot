use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::data::course_contents::{Generate, Id};
use crate::data::gen_module::GenModuleBuilder;
use crate::data::modules::unknown::CourseModule;
use crate::Moodle;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub quizzes: Vec<Quiz>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quiz {
    //pub id: i64,
    //pub course: i64,
    pub coursemodule: i64,
    pub name: String,
    pub intro: String,
    //pub introformat: i64,
    //pub introfiles: Vec<Value>,
    pub timeopen: i64,
    pub timeclose: i64,
    pub timelimit: i64,
    //pub preferredbehaviour: String,
    pub attempts: i64, //0 -> unlimited
    //pub grademethod: i64,
    //pub decimalpoints: i64,
    //pub questiondecimalpoints: i64,
    pub sumgrades: i64, //questions
    //pub grade: i64,
    //pub hasfeedback: i64,
    //pub section: i64,
    //pub visible: i64,
    //pub groupmode: i64,
    //pub groupingid: i64,
    //#[serde(flatten)]
    //pub additional_data: Option<AdditionalData>,
}

//some additional info, too lazy to parse it and teachers generally use the defaults
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AdditionalData {
    //pub overduehandling: String,
    //pub graceperiod: i64,
    //pub canredoquestions: i64,
    //pub attemptonlast: i64,
    //pub reviewattempt: i64,
    //pub reviewcorrectness: i64,
    //pub reviewmarks: i64,
    //pub reviewspecificfeedback: i64,
    //pub reviewgeneralfeedback: i64,
    //pub reviewrightanswer: i64,
    //pub reviewoverallfeedback: i64,
    //pub questionsperpage: i64,
    //pub navmethod: String,
    //pub browsersecurity: String,
    //pub delay1: i64,
    //pub delay2: i64,
    //pub showuserpicture: i64,
    //pub showblocks: i64,
    //pub completionattemptsexhausted: i64,
    //pub completionpass: i64,
    //pub allowofflineattempts: i64,
    //pub autosaveperiod: i64,
    //pub hasquestions: i64,
}

impl Id for Quiz {
    fn get_id(&self) -> i64 {
        self.coursemodule
    }
}

#[async_trait]
impl Generate for Quiz {
    async fn request(client: &Moodle, id: i64) -> anyhow::Result<Vec<Self>> {
        client.get_quizzes_for_course(id).await
    }

    fn gen(self, builder: &mut GenModuleBuilder, _: CourseModule) -> &mut GenModuleBuilder {
        builder
            .string("Bescheibung", self.intro)
            .date("Öffnungsdatum", self.timeopen)
            .date("Schlussdatum", self.timeclose)
            .num("Zeitspanne in Sekunden", self.timelimit)
            .num("Versuche", self.attempts)
            .num("Fragenanzahl", self.sumgrades)
    }
}
