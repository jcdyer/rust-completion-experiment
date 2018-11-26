use opaquekeys::UsageKey;

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
