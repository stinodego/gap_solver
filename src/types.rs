use std::collections::{BTreeMap, BTreeSet};

pub type Assignment = BTreeMap<&'static str, BTreeSet<&'static str>>;
