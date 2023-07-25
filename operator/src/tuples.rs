use std::collections::HashMap;

use crate::value::Value;

pub type SolutionMapping = HashMap<String, Value>;
pub type SolutionSequence = Vec<SolutionMapping>;
pub type MappingTuple = HashMap<String, SolutionSequence>;
