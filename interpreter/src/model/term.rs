
#[derive(Debug, Clone)]
pub enum Term{
    IRI{
        value:String, 
        ttype:TermType
    },

    Literal{
        value:String, 
        language:String, 
        ttype:TermType, 
        data_type:DataType, 
    }, 

    BlankNode{
        value:Option<String>, 
        ttype:TermType, 
    }
}

#[derive(Debug, Clone)]
pub enum DataType{
    Int, 
    Bool,
    Float,
    String,
    Double,
    Long,
}




#[derive(Debug, Clone)]
pub enum TermType{
    Literal, 
    BlankNode, 
    IRI, 
}


