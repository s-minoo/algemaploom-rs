use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Default)]
pub struct ShExMLDocument<'a> {
    pub prefix_names:     Vec<PrefixName<'a>>,
    pub sources:          Vec<Source>,
    pub iterators:        Vec<Iterator<'a>>,
    pub expression_stmts: Vec<ExpressionStatement<'a>>,
    pub matchers:         Vec<Matcher>,
    pub shapes:           Vec<Shape<'a>>,
}

#[derive(Debug, Clone)]
pub struct PrefixName<'a> {
    pub prefix: &'a str,
    pub local:  &'a str,
}

impl<'a> From<PrefixName<'a>> for String {
    fn from(value: PrefixName) -> Self {
        format!("{}:{}", value.prefix, value.local)
    }
}

#[derive(Debug, Clone)]
pub struct Field<'a> {
    pub name:  &'a str,
    pub query: &'a str,
}

#[derive(Debug, Clone)]
pub struct Source {}

#[derive(Debug, Clone)]
pub struct Iterator<'a> {
    pub fields:          Vec<Field<'a>>,
    pub push_fields:     Vec<Field<'a>>,
    pub pop_fields:      Vec<Field<'a>>,
    pub nested_iterator: Option<Box<Iterator<'a>>>,
}

#[derive(Debug, Clone)]
pub struct ExpressionStatement<'a> {
    pub name:       &'a str,
    pub expression: Expression,
}

#[derive(Debug, Clone)]
pub enum Expression {
    ConcateString {
        left_path:      String,
        concate_string: String,
        right_path:     String,
    },
    Join(Box<Expression>, Box<Expression>),
    Union(Box<Expression>, Box<Expression>),
    Basic {
        path: String,
    },
}

#[derive(Debug, Clone)]
pub struct Matcher {
    pub rename_map: HashMap<String, HashSet<String>>,
}

#[derive(Debug, Clone)]
pub struct Shape<'a> {
    pub name:              PrefixName<'a>,
    pub pred_object_pairs: HashMap<PrefixName<'a>, String>,
    pub pred_shape_paris:  HashMap<PrefixName<'a>, Box<Shape<'a>>>,
}
