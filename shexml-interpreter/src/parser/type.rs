use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Default)]
pub struct ShExMLDocument {
    pub prefixes:         Vec<Prefix>,
    pub sources:          Vec<Source>,
    pub iterators:        Vec<Iterator>,
    pub expression_stmts: Vec<ExpressionStatement>,
    pub matchers:         Vec<Matcher>,
    pub graph_shapes:     Vec<GraphShapes>,
}

#[derive(Debug, Clone)]
pub struct Prefix {
    pub prefix: String,
    pub uri:    String,
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
    pub ident:      String,
    pub query:      String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Source {
    pub ident: String,
    pub uri:   String,
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
    pub ident:      String,
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

#[derive(Debug, Clone, PartialEq, Eq)]
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
pub struct GraphShapes {
    pub ident:  String,
    pub shapes: Vec<Shape>,
}

#[derive(Debug, Clone)]
pub struct Shape {
    pub ident:          String,
    pub subject:        Subject,
    pub pred_obj_pairs: HashMap<Predicate, Object>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShapeReference {
    pub expr_ident: String,
    pub field:      Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShapeExpression {
    Reference(ShapeReference),

    Matching {
        expr_ident:    String,
        matcher_ident: String,
    },

    Conditional {
        reference:        ShapeReference,
        conditional_expr: Box<ShapeExpression>,
    },

    Function {
        fun_method_ident: ShapeReference,
        params_idents:    Vec<ShapeReference>,
    },
}

#[derive(Debug, Clone)]
pub struct Object {
    pub prefix:     PrefixNameSpace,
    pub expression: ShapeExpression,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Predicate {
    pub prefix: PrefixNameSpace,
    pub name:   String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subject {
    pub prefix:     PrefixNameSpace,
    pub expression: ShapeExpression,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PrefixNameSpace {
    NamedPrefix(String),
    BasePrefix,
}
