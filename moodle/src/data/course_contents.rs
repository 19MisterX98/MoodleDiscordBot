use crate::data::comparable::compare;
use crate::data::gen_module::{GenModule, GenModuleBuilder};
use crate::data::modules::unknown::CourseModule;
use crate::Moodle;
use anyhow::Result;
use async_trait::async_trait;

pub trait Id {
    fn get_id(&self) -> i64;
}

/// Defines how moodle modules should be captured
#[async_trait]
pub trait Generate: Id + Sized {
    /// send a request to fetch all modules of a certain type in a moodle course
    async fn request(client: &Moodle, id: i64) -> Result<Vec<Self>>;

    /// Generate Modules from the corresponding course-module and a more specific request for the exact module type
    async fn process(
        course_modules: Vec<CourseModule>,
        client: &Moodle,
        course_id: i64,
    ) -> Result<Vec<GenModule>> {
        let mapped_modules = Self::request(client, course_id).await?;
        let res = merge(course_modules, mapped_modules) //think about non 1:1 cases
            .into_iter()
            .map(|(course_module, mapped_module)| {
                let mut builder = GenModuleBuilder::new(
                    course_module.id,
                    course_module.modicon.clone(),
                    course_module.name.clone(),
                    course_module.url.clone(),
                );
                mapped_module.gen(&mut builder, course_module);
                builder.build()
            })
            .collect();
        Ok(res)
    }

    /// serialize the module entries
    fn gen(
        self,
        builder: &mut GenModuleBuilder,
        course_module: CourseModule,
    ) -> &mut GenModuleBuilder;
}

/// find all pairs of elements in a and b with the same id
pub fn merge<A: Id, B: Id>(a: Vec<A>, b: Vec<B>) -> Vec<(A, B)> {
    compare(a, b).common
}
