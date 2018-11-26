use std::collections::HashMap;

use opaquekeys::{CourseKey, UsageKey};

use crate::{BlockCompletion, User};
use crate::xblock::{CompletionMode, XBlock};


#[derive(Clone, Debug, PartialEq)]
pub struct Aggregator {
    user: User,
    block_key: UsageKey,
    earned: f64,
    possible: f64,
}


pub struct Course {  // A tree structure
    course_key: CourseKey,
    root: CourseNode
}

impl Course {
    pub fn aggregate(&self, user: &User, completions: &HashMap<UsageKey, BlockCompletion>) -> Vec<Aggregator> {
        self.root.aggregate(user, completions).0
    }

}

struct CourseNode {
    xblock: XBlock,
    block_key: UsageKey,
    children: Vec<CourseNode>
}

impl CourseNode {
    fn aggregate(&self, user: &User, completions: &HashMap<UsageKey, BlockCompletion>) -> (Vec<Aggregator>, (f64, f64)) {
        match self.xblock.mode {
            CompletionMode::Excluded => {
                (vec![], (0.0, 0.0))
            }
            CompletionMode::Completable => {
                (vec![], (completions.get(&self.block_key).map(|bc| bc.completion).unwrap_or(0.0), 1.0))
            }
            CompletionMode::Aggregator => {
                let mut combined_aggs = vec![];
                let mut earned = 0.0;
                let mut possible = 0.0;

                for child in &self.children {
                    let (mut aggs, (child_earned, child_possible)) = child.aggregate(user, completions);
                    earned += child_earned;
                    possible += child_possible;
                    combined_aggs.append(&mut aggs);
                }
                combined_aggs.push(Aggregator {
                    block_key: self.block_key.clone(),
                    user: user.clone(),
                    earned,
                    possible,
                });
                (combined_aggs, (earned, possible))
            }
        }
    }
}

