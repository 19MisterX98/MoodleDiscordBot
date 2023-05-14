use crate::data::course_contents::Generate;
use crate::data::gen_module::GenModule;
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
use crate::data::modules::unknown::CourseModule;
use crate::data::modules::url::Url;
use crate::Moodle;
use anyhow::Result;
use std::collections::HashMap;

pub async fn get_course_info(client: &Moodle, course_id: i64) -> Result<Vec<GenModule>> {
    let course = client.get_course_contents(course_id).await?;
    let mut sections = vec![];
    let mut grouped_modules = HashMap::new();

    for section in course {
        sections.push(section.section_info);

        for module in section.modules {
            grouped_modules
                .entry(module.modname.clone())
                .or_insert(vec![])
                .push(module);
        }
    }

    let mut gen_modules = vec![];
    for (typ, module_group) in grouped_modules {
        gen_modules.extend(match_type(typ.as_str(), module_group, client, course_id).await?);
    }

    gen_modules.extend(sections.into_iter().map(|section| section.process()));
    Ok(gen_modules)
}

async fn match_type(
    name: &str,
    course_modules: Vec<CourseModule>,
    client: &Moodle,
    course_id: i64,
) -> Result<Vec<GenModule>> {
    match name {
        "assign" => Assignment::process(course_modules, client, course_id).await,
        "bigbluebuttonbn" => Bigbluebuttonbn::process(course_modules, client, course_id).await,
        "chat" => Chat::process(course_modules, client, course_id).await,
        "choice" | "questionnaire" => Choice::process(course_modules, client, course_id).await,
        "feedback" => Feedback::process(course_modules, client, course_id).await,
        "folder" => Folder::process(course_modules, client, course_id).await,
        "forum" => Forum::process(course_modules, client, course_id).await,
        "glossary" => Glossary::process(course_modules, client, course_id).await,
        "label" => Label::process(course_modules, client, course_id).await,
        "page" => Page::process(course_modules, client, course_id).await,
        "quiz" => Quiz::process(course_modules, client, course_id).await,
        "resource" => Resource::process(course_modules, client, course_id).await,
        "url" => Url::process(course_modules, client, course_id).await,
        _ => CourseModule::process(course_modules, client, course_id).await,
    }
}
