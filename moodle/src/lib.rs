use crate::data::modules::assignment::Assignment;
use crate::data::modules::bigbluebutton::Bigbluebuttonbn;
use crate::data::modules::chat::Chat;
use crate::data::modules::choice::Choice;
use crate::data::modules::feedback::Feedback;
use crate::data::modules::folder::Folder;
use crate::data::modules::forum::Forum;
use crate::data::modules::glossary::Glossary;
use crate::data::modules::label::Label;
use crate::data::modules::page::Page;
use crate::data::modules::quiz::Quiz;
use crate::data::modules::resource::Resource;
use crate::data::modules::url::Url;
use crate::data::modules::{
    assignment, bigbluebutton, chat, choice, feedback, folder, forum, glossary, label, page, quiz,
    resource, url,
};
use crate::data::other_content::courses::Data;
use crate::data::other_content::section::Course;
use anyhow::{anyhow, Result};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use reqwest::Client;

pub mod data;

#[derive(Debug, Deserialize)]
struct Token {
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Moodle {
    base: String,
    token: String,
}

impl Moodle {
    pub fn new_with_token(base: String, token: String) -> Moodle {
        Moodle { base, token }
    }

    pub async fn new_with_login(base: String, username: &str, password: &str) -> Result<Moodle> {
        let url = format!(
            "{base}/login/token.php?username={username}&password={password}&service=MOODLE_MOBILE_APP"
        );
        let response = reqwest::get(url).await?;
        let token = response.json::<Token>().await?.token;
        Ok(Moodle { base, token })
    }

    pub async fn download_file(&self, link: String, max_size: u64) -> Result<Vec<u8>> {
        let url = if link.contains("?") {
            format!("{}&token={}", link, self.token)
        } else {
            format!("{}?token={}", link, self.token)
        };
        let response = reqwest::get(url).await?;
        if let Some(length) = response.content_length() {
            if length > max_size {
                return Err(anyhow!("Fail"));
            }
        }
        Ok(response.bytes().await?.to_vec())
    }

    pub async fn get_courses(&self, classification: Option<&str>) -> Result<Data> {
        let params = ParameterBuilder::new().add("classification", classification.unwrap_or("all"));
        self.request(
            "core_course_get_enrolled_courses_by_timeline_classification",
            params,
        )
        .await
    }

    pub async fn get_course_contents(&self, course_id: i64) -> Result<Course> {
        let params = ParameterBuilder::new().add("courseid", course_id.to_string().as_str());
        self.request("core_course_get_contents", params).await
    }

    pub async fn get_assignments_for_course(&self, course_id: i64) -> Result<Vec<Assignment>> {
        let mut res: assignment::Root = self
            .module_request("mod_assign_get_assignments", course_id)
            .await?;
        match res.courses.pop() {
            Some(course) => Ok(course.assignments),
            None => Err(anyhow!("Didnt receive course as answer")),
        }
    }

    pub async fn get_bbbs_for_course(&self, course_id: i64) -> Result<Vec<Bigbluebuttonbn>> {
        let res: bigbluebutton::Root = self
            .module_request(
                "mod_bigbluebuttonbn_get_bigbluebuttonbns_by_courses",
                course_id,
            )
            .await?;
        Ok(res.bigbluebuttonbns)
    }

    pub async fn get_chats_for_course(&self, course_id: i64) -> Result<Vec<Chat>> {
        let res: chat::Root = self
            .module_request("mod_chat_get_chats_by_courses", course_id)
            .await?;
        Ok(res.chats)
    }

    pub async fn get_choices_for_course(&self, course_id: i64) -> Result<Vec<Choice>> {
        let res: choice::Root = self
            .module_request("mod_choice_get_choices_by_courses", course_id)
            .await?;
        Ok(res.choices)
    }

    pub async fn get_feedback_for_course(&self, course_id: i64) -> Result<Vec<Feedback>> {
        let res: feedback::Root = self
            .module_request("mod_feedback_get_feedbacks_by_courses", course_id)
            .await?;
        Ok(res.feedbacks)
    }

    pub async fn get_folders_for_course(&self, course_id: i64) -> Result<Vec<Folder>> {
        let res: folder::Root = self
            .module_request("mod_folder_get_folders_by_courses", course_id)
            .await?;
        Ok(res.folders)
    }

    pub async fn get_forums_for_course(&self, course_id: i64) -> Result<Vec<Forum>> {
        let res: forum::Root = self
            .module_request("mod_forum_get_forums_by_courses", course_id)
            .await?;
        Ok(res)
    }

    pub async fn get_glossaries_for_course(&self, course_id: i64) -> Result<Vec<Glossary>> {
        let res: glossary::Root = self
            .module_request("mod_glossary_get_glossaries_by_courses", course_id)
            .await?;
        Ok(res.glossaries)
    }

    pub async fn get_labels_for_course(&self, course_id: i64) -> Result<Vec<Label>> {
        let res: label::Root = self
            .module_request("mod_label_get_labels_by_courses", course_id)
            .await?;
        Ok(res.labels)
    }

    pub async fn get_pages_for_course(&self, course_id: i64) -> Result<Vec<Page>> {
        let res: page::Root = self
            .module_request("mod_page_get_pages_by_courses", course_id)
            .await?;
        Ok(res.pages)
    }

    pub async fn get_quizzes_for_course(&self, course_id: i64) -> Result<Vec<Quiz>> {
        let res: quiz::Root = self
            .module_request("mod_quiz_get_quizzes_by_courses", course_id)
            .await?;
        Ok(res.quizzes)
    }

    pub async fn get_resources_for_course(&self, course_id: i64) -> Result<Vec<Resource>> {
        let res: resource::Root = self
            .module_request("mod_resource_get_resources_by_courses", course_id)
            .await?;
        Ok(res.resources)
    }

    pub async fn get_urls_for_course(&self, course_id: i64) -> Result<Vec<Url>> {
        let res: url::Root = self
            .module_request("mod_url_get_urls_by_courses", course_id)
            .await?;
        Ok(res.urls)
    }

    async fn module_request<T: DeserializeOwned>(
        &self,
        function: &str,
        course_id: i64,
    ) -> Result<T> {
        let params = ParameterBuilder::new().add("courseids[0]", &course_id.to_string());
        self.request(function, params).await
    }

    async fn request<T: DeserializeOwned>(
        &self,
        function: &str,
        params: ParameterBuilder,
    ) -> Result<T> {
        let params = params
            .add("moodlewsrestformat", "json")
            .add("wsfunction", function)
            .add("wstoken", &self.token);

        let response = Client::new()
            .post(format!("{}/webservice/rest/server.php", self.base))
            .form(&params.map)
            .send().await?;

        Ok(response.json::<T>().await?)
    }
}

struct ParameterBuilder {
    map: HashMap<String, String>,
}

impl ParameterBuilder {
    fn new() -> ParameterBuilder {
        let map = HashMap::new();
        ParameterBuilder { map }
    }

    fn add(mut self, key: &str, value: &str) -> ParameterBuilder {
        self.map.insert(String::from(key), String::from(value));
        self
    }
}
