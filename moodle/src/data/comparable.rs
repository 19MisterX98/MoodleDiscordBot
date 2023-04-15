use crate::data::comparable::ModuleEntry::{Added, Changed, Removed};
use crate::data::course_contents::Id;
use indexmap::IndexMap;

pub struct Comparison<A, B> {
    pub a: Vec<A>,
    pub b: Vec<B>,
    pub common: Vec<(A, B)>,
}

pub fn compare<A: Id, B: Id>(a: Vec<A>, mut b: Vec<B>) -> Comparison<A, B> {
    let mut comp = Comparison {
        a: vec![],
        b: vec![],
        common: vec![],
    };
    for el_a in a {
        let id_1 = el_a.get_id();
        let el_b_pos = b.iter().position(|el_b| id_1 == el_b.get_id());
        match el_b_pos {
            None => comp.a.push(el_a),
            Some(el_b_pos) => {
                let el_b = b.swap_remove(el_b_pos);
                comp.common.push((el_a, el_b));
            }
        }
    }
    comp.b = b;
    comp
}

pub enum ModuleEntry {
    Added(String),
    Removed(String),
    Changed(String, String), // old, new
}

impl ModuleEntry {
    pub fn format(&self, key: String) -> String {
        match self {
            Added(val) => format!("🟢 **{key}:** {val}"),
            Removed(val) => format!("🔴 **{key}:** {val}"),
            Changed(old, new) => format!("🔵**{key}**\n__From:__ {old}\n__To:__ {new}"),
        }
    }
}

pub fn diff_module_entries(
    mut old: IndexMap<String, String>,
    new: IndexMap<String, String>,
) -> IndexMap<String, ModuleEntry> {
    let mut mapped = IndexMap::new();
    for (new_key, new_val) in new {
        let old_val = old.remove(&new_key);
        match old_val {
            Some(old_val) if old_val != new_val => {
                mapped.insert(new_key, Changed(old_val, new_val))
            }
            None => mapped.insert(new_key, Added(new_val)),
            _ => None,
        };
    }
    for (old_key, old_val) in old {
        mapped.insert(old_key, Removed(old_val));
    }
    mapped
}
