use std::collections::BTreeMap;

use opaquekeys::{CourseKey, UsageKey};

use crate::{Aggregator, BlockCompletion, User};
use crate::xblock::{get_xblock_modes, CompletionMode, XBlock};

pub struct Course {
    coursekey: CourseKey,
    root: CourseNode,
}

impl Course {
    pub fn from_structure(structure: &BTreeMap<UsageKey, Vec<UsageKey>>) -> Course {
        let mut rootblock = None;
        for usagekey in structure.keys() {
            if usagekey.blocktype() == "course" {
                // This will break for course subgraphs, which won't have a course.
                rootblock = Some(usagekey);
            }
        }
        let rootblock = rootblock.unwrap(); //CRASH!!!
        let xblock_modes = get_xblock_modes();
        Course {
            coursekey: rootblock.course_key().clone(),
            root: CourseNode::new(rootblock.clone(), structure, &xblock_modes),
        }
    }

    pub fn aggregate(
        &self,
        user: &User,
        completions: &BTreeMap<(User, UsageKey), BlockCompletion>,
    ) -> Vec<Aggregator> {
        self.root.aggregate(user, completions).0
    }
}

#[derive(Debug)]
struct CourseNode {
    xblock: XBlock,
    blockkey: UsageKey,
    children: Vec<CourseNode>,
}

impl CourseNode {
    fn new(
        blockkey: UsageKey,
        structure: &BTreeMap<UsageKey, Vec<UsageKey>>,
        xblock_modes: &BTreeMap<String, CompletionMode>,
    ) -> CourseNode {
        let name = blockkey.blocktype().to_owned();
        let mode = *xblock_modes
            .get(&name)
            .unwrap_or(&CompletionMode::Completable);
        let xblock = XBlock {
            name,
            mode,
            block_key: blockkey.clone(),
        };
        let children = match structure.get(&blockkey) {
            Some(children) => children.clone(),
            None => Vec::new(),
        };
        let children = children
            .iter()
            .map(|key| CourseNode::new(key.clone(), structure, xblock_modes))
            .collect();
        CourseNode {
            xblock,
            blockkey,
            children,
        }
    }

    fn aggregate(
        &self,
        user: &User,
        completions: &BTreeMap<(User, UsageKey), BlockCompletion>,
    ) -> (Vec<Aggregator>, (f64, f64)) {
        match self.xblock.mode {
            CompletionMode::Excluded => (vec![], (0.0, 0.0)),
            CompletionMode::Completable => {
                // I want to be able to use a borrowed key, but I can only borrow the tuple, but not the elements inside it.
                (
                    vec![],
                    (
                        completions
                            .get(&(user.clone(), self.blockkey.clone()))
                            .map(|bc| bc.completion)
                            .unwrap_or(0.0),
                        1.0,
                    ),
                )
            }
            CompletionMode::Aggregator => {
                println!("Aggregating block {:?}", self);
                let mut combined_aggs = vec![];
                let mut earned = 0.0;
                let mut possible = 0.0;

                for child in &self.children {
                    let (mut aggs, (child_earned, child_possible)) =
                        child.aggregate(user, completions);
                    earned += child_earned;
                    possible += child_possible;
                    combined_aggs.append(&mut aggs);
                }
                combined_aggs.push(Aggregator {
                    block_key: self.blockkey.clone(),
                    user: user.clone(),
                    earned,
                    possible,
                });
                (combined_aggs, (earned, possible))
            }
        }
    }
}
