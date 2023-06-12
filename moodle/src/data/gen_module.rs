use crate::data::course_contents::Id;
use crate::data::modules::unknown::Content;
use crate::data::other_content::file::FileInfo;
use indexmap::IndexMap;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::cmp::max;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenModule {
    pub entries: IndexMap<String, String>,
    pub files: IndexMap<String, String>,
    pub mod_icon_url: String,
    pub name: String,
    pub link: Option<String>,
    id: i64,
}

impl Id for GenModule {
    fn get_id(&self) -> i64 {
        self.id
    }
}

/// Serialize module entries
pub struct GenModuleBuilder(GenModule);

impl GenModuleBuilder {
    pub fn new(
        id: i64,
        mod_icon_url: String,
        name: String,
        link: Option<String>,
    ) -> GenModuleBuilder {
        let module = GenModule {
            entries: IndexMap::new(),
            files: IndexMap::new(),
            mod_icon_url,
            link,
            name,
            id,
        };
        GenModuleBuilder(module)
    }

    pub fn build(self) -> GenModule {
        self.0
    }

    pub fn date(&mut self, name: &str, date: i64) -> &mut Self {
        if date != 0 {
            self.0
                .entries
                .insert(name.to_string(), format!("<t:{:?}:F>", date));
        }
        self
    }

    pub fn date_option(&mut self, name: &str, date: Option<i64>) -> &mut Self {
        if let Some(date) = date {
            self.date(name, date);
        }
        self
    }

    pub fn num(&mut self, name: &str, num: i64) -> &mut Self {
        self.0.entries.insert(name.to_string(), num.to_string());
        self
    }

    pub fn megabytes(&mut self, name: &str, bytes: i64) -> &mut Self {
        let mb = bytes as f32 / (1 << 20) as f32;
        let s = format!("{:.2}MB", mb);
        self.0.entries.insert(name.to_string(), s);
        self
    }

    pub fn megabytes_option(&mut self, name: &str, bytes: Option<i64>) -> &mut Self {
        if let Some(bytes) = bytes {
            self.megabytes(name, bytes);
        }
        self
    }

    pub fn string(&mut self, name: &str, string: String) -> &mut Self {
        if !string.is_empty() {
            self.0
                .entries
                .insert(name.to_string(), remove_html(&string));
        }
        self
    }

    pub fn string_option(&mut self, name: &str, string: Option<String>) -> &mut Self {
        if let Some(string) = string {
            self.string(name, string);
        }
        self
    }

    pub fn bool(&mut self, name: &str, bool: bool) -> &mut Self {
        if bool {
            self.0.entries.insert(name.to_string(), "Ja".to_string());
        } else {
            self.0.entries.insert(name.to_string(), "Nein".to_string());
        }
        self
    }

    pub fn files(&mut self, files: Vec<FileInfo>) -> &mut Self {
        files
            .into_iter()
            .fold(self, |builder, file| builder.file(file))
    }

    pub fn file(&mut self, file: FileInfo) -> &mut Self {
        let file_url = remove_revision(&file.fileurl);
        self.0.files.insert(file.filename, file_url);
        self
    }

    pub fn contents(&mut self, contents: Vec<Content>) -> &mut Self {
        contents
            .into_iter()
            .fold(self, |builder, content| builder.content(content))
    }

    pub fn content(&mut self, content: Content) -> &mut Self {
        if content.type_field == "file" {
            let file_url = remove_revision(&content.fileurl);
            self.0.files.insert(content.filename, file_url);
        } else {
            self.0.entries.insert(content.filename, content.fileurl);
        }
        self
    }
}

// descriptions often contain html tags for presentation on the moodle website, we want plaintext
fn remove_html(string: &str) -> String {
    let mut chars = string.chars();
    let mut open_brackets = 0;
    let mut new_str = String::new();
    while let Some(char) = chars.next() {
        if char == '<' {
            open_brackets += 1;
            new_str.push(' ');
        } else if open_brackets == 0 {
            new_str.push(char);
        } else if char == '>' {
            open_brackets -= 1;
            open_brackets = max(0, open_brackets);
        }
    }
    new_str.replace("&nbsp;", " ")
}

// moodle files often contain the revision number in the url.
// That kinda sucks cause the revision number is sometimes changing even if the file stays identical
fn remove_revision(url: &str) -> String {
    let regex = Regex::new(r"/content/(\d+)/").unwrap();
    regex.replace(url, "/content/0/").into()
}
