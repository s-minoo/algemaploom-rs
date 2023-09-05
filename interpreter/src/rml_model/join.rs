#[derive(Debug, Clone)]
pub struct JoinCondition {
    pub parent_attributes: Vec<String>,
    pub child_attributes:  Vec<String>,
}

impl Default for JoinCondition {
    fn default() -> Self {
        Self {
            parent_attributes: Default::default(),
            child_attributes:  Default::default(),
        }
    }
}
