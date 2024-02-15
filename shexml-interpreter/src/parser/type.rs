use std::collections::{HashMap, HashSet};

use serde::ser::{SerializeSeq, SerializeStruct};
use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShExMLDocument {
    pub prefixes:         Vec<Prefix>,
    pub sources:          Vec<Source>,
    pub iterators:        Vec<Box<Iterator>>,
    pub expression_stmts: Vec<ExpressionStmt>,
    pub auto_increments:  Vec<AutoIncrement>,
    pub functions:        Vec<Function>,
    pub matchers:         Vec<Matcher>,
    pub graph_shapes:     Vec<GraphShapes>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Prefix {
    pub prefix: PrefixNameSpace,
    pub uri:    String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FieldType {
    Normal,
    Push,
    Pop,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Field {
    pub field_type: FieldType,
    pub ident:      String,
    pub query:      String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceType {
    File,
    JDBC(String),
    HTTP,
    HTTPS,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Source {
    pub ident:       String,
    pub source_type: SourceType,
    pub uri:         String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Iterator {
    pub ident:           String,
    pub query:           String,
    pub iter_type:       String,
    pub fields:          Vec<Field>,
    pub nested_iterator: Option<Box<Iterator>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExpressionEnum {
    ExpressionStmt(ExpressionStmt),
    MatcherExp(Matcher),
    AutoIncrementExp(AutoIncrement),
    FunctionExp(Function),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpressionReferenceIdent {
    pub source_ident:   String,
    pub iterator_ident: String,
    pub field:          Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum ExpressionStmtEnum {
    Join(Box<ExpressionStmtEnum>, Box<ExpressionStmtEnum>),

    Union(Box<ExpressionStmtEnum>, Box<ExpressionStmtEnum>),
    ConcatenateString {
        left_reference:  ExpressionReferenceIdent,
        concate_string:  String,
        right_reference: ExpressionReferenceIdent,
    },

    Basic {
        reference: ExpressionReferenceIdent,
    },
}

impl Serialize for ExpressionStmtEnum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ExpressionStmtEnum::Join(left, right) => {
                let mut struct_serde =
                    serializer.serialize_struct("Join", 3)?;

                struct_serde.serialize_field("type", "Join")?;
                struct_serde.serialize_field("left", left)?;
                struct_serde.serialize_field("right", right)?;

                struct_serde.end()
            }
            ExpressionStmtEnum::Union(left, right) => {
                let mut struct_serde =
                    serializer.serialize_struct("Union", 3)?;

                struct_serde.serialize_field("type", "Union")?;
                struct_serde.serialize_field("left", left)?;
                struct_serde.serialize_field("right", right)?;

                struct_serde.end()
            }
            ExpressionStmtEnum::ConcatenateString {
                left_reference: left_path,
                concate_string,
                right_reference: right_path,
            } => {
                let mut struct_serde =
                    serializer.serialize_struct("ConcatenateString", 4)?;

                struct_serde.serialize_field("type", "ConcatenateString")?;
                struct_serde.serialize_field("left_path", left_path)?;
                struct_serde
                    .serialize_field("concate_string", concate_string)?;
                struct_serde.serialize_field("right_path", right_path)?;

                struct_serde.end()
            }
            ExpressionStmtEnum::Basic { reference: path } => {
                let mut struct_serde =
                    serializer.serialize_struct("Basic", 2)?;

                struct_serde.serialize_field("type", "Basic")?;
                struct_serde.serialize_field("path", path)?;

                struct_serde.end()
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpressionStmt {
    pub ident:     String,
    pub expr_enum: ExpressionStmtEnum,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Matcher {
    pub ident:      String,
    pub rename_map: HashMap<String, HashSet<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutoIncrement {
    pub ident:  String,
    pub start:  u32,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub end:    Option<u32>,
    pub step:   Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Function {
    pub ident:     String,
    pub lang_type: String,
    pub uri:       String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShapeIdent {
    pub prefix: PrefixNameSpace,
    pub local:  String,
}

impl ShapeIdent {
    pub fn base() -> ShapeIdent {
        ShapeIdent {
            prefix: PrefixNameSpace::BasePrefix,
            local:  "".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphShapes {
    pub ident:  ShapeIdent,
    pub shapes: Vec<Shape>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Shape {
    pub ident:   ShapeIdent,
    pub subject: Subject,

    #[serde(serialize_with = "pred_obj_ser")]
    pub pred_obj_pairs: HashMap<Predicate, Object>,
}

fn pred_obj_ser<S>(
    pred_obj_pairs: &HashMap<Predicate, Object>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let tuples: Vec<(&Predicate, &Object)> = pred_obj_pairs.iter().collect();

    let mut seq = s.serialize_seq(Some(tuples.len()))?;

    for tup in tuples {
        seq.serialize_element(&tup)?;
    }

    seq.end()
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShapeReference {
    pub expr_ident: String,
    pub field:      Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ShapeExpression {
    #[serde(serialize_with = "shape_expr_ref_serialize")]
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

fn shape_expr_ref_serialize<S>(
    shape_ref: &ShapeReference,
    s: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut struct_serde = s.serialize_struct("Reference", 1)?;

    struct_serde.serialize_field("shape_reference", shape_ref)?;
    struct_serde.end()
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Object {
    pub prefix:     Option<PrefixNameSpace>,
    pub expression: ShapeExpression,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Predicate {
    pub prefix: PrefixNameSpace,
    pub local:  String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Subject {
    pub prefix:     PrefixNameSpace,
    pub expression: ShapeExpression,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PrefixNameSpace {
    #[serde(serialize_with = "ns_serialize")]
    NamedPrefix(String),
    BasePrefix,
}

fn ns_serialize<S>(ns: &String, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut serde_struct = s.serialize_struct("NamedPrefix", 1)?;

    serde_struct.serialize_field("namespace", ns)?;
    serde_struct.end()
}
