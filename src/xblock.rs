use opaquekeys::UsageKey;
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CompletionMode {
    Aggregator,
    Completable,
    Excluded,
}

pub struct XBlock {
    pub name: String,
    pub block_key: UsageKey,
    pub mode: CompletionMode,
}

/// Return all available xblock types with their modes as a BTreeMap
/// This may move to a port/adapter in the future.
pub fn get_xblock_modes() -> BTreeMap<String, CompletionMode> {
    use self::CompletionMode::*;
    let mut map = BTreeMap::new();
    map.insert("course".to_owned(), Aggregator);
    map.insert("chapter".to_owned(), Aggregator);
    map.insert("sequential".to_owned(), Aggregator);
    map.insert("vertical".to_owned(), Aggregator);
    map.insert("html".to_owned(), Completable);
    map.insert("ooyala".to_owned(), Completable);
    map.insert("poll".to_owned(), Completable);
    map.insert("survey".to_owned(), Completable);
    map.insert("image-explorer".to_owned(), Completable);
    map.insert("problem-builder".to_owned(), Completable);
    map.insert("group-project".to_owned(), Excluded);
    map.insert("discussion".to_owned(), Excluded);
    map
}
