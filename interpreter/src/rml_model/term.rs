use uuid::Uuid;


#[derive(Debug, Clone)]
pub enum Term {
    Literal(Literal),
    Resource(Resource),
}

#[derive(Debug, Clone)]
pub enum Resource{
   IRI(IRI), 
   BlankNode(BlankNode)
}

#[derive(Debug, Clone)]
pub struct IRI {
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct Literal {
    pub value:     String,
    pub language:  Option<String>,
    pub data_type: Option<DataType>,
}

#[derive(Debug, Clone)]
pub struct BlankNode {
    pub value: String,
}

impl Default for BlankNode {
    fn default() -> Self {
        Self {
            value: Uuid::new_v4().to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DataType {
    Int,
    Bool,
    Float,
    String,
    Double,
    Long,
}
