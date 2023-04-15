use crate::data::gen_module::{GenModule, GenModuleBuilder};
use crate::data::modules::unknown::CourseModule;
use serde::{Deserialize, Serialize};

pub type Course = Vec<Section>;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Section {
    #[serde(flatten)]
    pub section_info: SectionInfo,
    //pub section: i64,    // index of segment in course, starting at 0
    pub modules: Vec<CourseModule>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SectionInfo {
    pub id: i64,         //id
    pub name: String,    //name
    pub summary: String, // empty or html
}

// sections dont have module ids so we just give them a high value to make them not collide with other modules
impl SectionInfo {
    pub fn process(self) -> GenModule {
        let image_link = "https://cdn.discordapp.com/attachments/1092233307867070554/1095647451739865108/section.png".into();
        let mut builder = GenModuleBuilder::new(self.id * 10000, image_link, self.name, None);
        builder.string("Zusammenfassung", self.summary);
        builder.build()
    }
}
