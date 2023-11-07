use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Default)]
pub struct ShExMLDocument {
    pub prefix_nspaces:   Vec<PrefixNameSpace>,
    pub sources:          Vec<Source>,
    pub iterators:        Vec<Iterator>,
    pub expression_stmts: Vec<ExpressionStatement>,
    pub matchers:         Vec<Matcher>,
    pub shapes:           Vec<Shape>,
}

#[derive(Debug, Clone)]
pub struct PrefixNameSpace {
    pub prefix: String,
    pub local:  String,
}

impl<'a> From<PrefixNameSpace> for String {
    fn from(value: PrefixNameSpace) -> Self {
        format!("{}:{}", value.prefix, value.local)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FieldType {
    Normal,
    Push,
    Pop,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub field_type: FieldType,
    pub name:       String,
    pub query:      String,
}

#[derive(Debug, Clone)]
pub struct Source {
    pub name: String,
    pub uri:  String,
}

#[derive(Debug, Clone)]
pub struct Iterator {
    pub ident:           String,
    pub query:           String,
    pub iter_type:       String,
    pub fields:          Vec<Field>,
    pub nested_iterator: Option<Box<Iterator>>,
}

#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    pub name:       String,
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
    pub ident:      String,
    pub rename_map: HashMap<String, HashSet<String>>,
}

#[derive(Debug, Clone)]
pub struct AutoIncrement {
    pub ident:  String,
    pub start:  u32,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub end:    Option<u32>,
    pub step:   Option<u32>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub ident:     String,
    pub lang_type: String,
    pub uri:       String,
}

#[derive(Debug, Clone)]
pub struct Shape {
    pub name:              PrefixNameSpace,
    pub pred_object_pairs: HashMap<PrefixNameSpace, String>,
    pub pred_shape_paris:  HashMap<PrefixNameSpace, Box<Shape>>,
}
