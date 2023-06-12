use crate::data::comparable::ModuleEntry::{Added, Changed, Removed};
use crate::data::course_contents::Id;
use indexmap::IndexMap;

pub struct Comparison<A, B> {
    pub a: Vec<A>,
    pub b: Vec<B>,
    pub common: Vec<(A, B)>,
}

/// matches common entries of two vecs by their ID. If an element is not in both vecs then it wont
/// get moved to the common vec
pub fn compare<A: Id, B: Id>(a: Vec<A>, mut b: Vec<B>) -> Comparison<A, B> {
    let mut comp = Comparison {
        a: vec![],
        b: vec![],
        common: vec![],
    };
    for element_a in a {
        let id_a = element_a.get_id();
        // gets the position of a potential matching id in the b vector
        let el_b_pos = b.iter().position(|element_b| id_a == element_b.get_id());
        match el_b_pos {
            // if the position does not exist then element_a has no matching element in b
            None => comp.a.push(element_a),
            // if the the position exists then remove it and combine it with our element
            Some(el_b_pos) => {
                let el_b = b.swap_remove(el_b_pos);
                comp.common.push((element_a, el_b));
            }
        }
    }
    // all elements in b dont have a counterpart in a
    comp.b = b;
    comp
}

pub enum ModuleEntry {
    Added(String),
    Removed(String),
    Changed(String, String), // old, new
}

pub fn diff_module_entries(
    mut old: IndexMap<String, String>,
    new: IndexMap<String, String>,
) -> IndexMap<String, ModuleEntry> {
    let mut mapped = IndexMap::new();

    for (new_key, new_val) in new {
        let old_val = old.remove(&new_key);
        match old_val {
            // Something changed between the old and the new value
            Some(old_val) if old_val != new_val => {
                mapped.insert(new_key, Changed(old_val, new_val))
            }
            // There is no old entry corresponding to the key
            None => mapped.insert(new_key, Added(new_val)),
            // The entry is identical in both maps
            _ => None,
        };
    }
    // All entries that dont exist in the new map were removed
    for (old_key, old_val) in old {
        mapped.insert(old_key, Removed(old_val));
    }
    mapped
}
