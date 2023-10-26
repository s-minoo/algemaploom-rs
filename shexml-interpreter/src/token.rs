#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShExMLToken {
    //prologue
    Prefix,
    Source,
    Iterator,
    Matcher,
    Expression,

    //interim
    As,
    Matching,
    Field,
    PushField,
    PopField,
    Union, 
    Join, 
    StringSep(String),
    If, 


    // "jsonpath:" "csvperrow" "xmlpath:" etc..
    IteratorType(String), 
    IteratorQuery(String),

    // Baseprefix
    BasePrefix,
    // prefix namespace
    PrefixNS(String),
    // prefix local name
    PrefixLN(String),
    
    URI(String),

    // Identifier used by source, matcher, function, expression
    Ident(String),
    // Field query
    FieldQuery(String),



    // :
    PrefixSep,

    // <
    AngleStart,
    // >
    AngleEnd,

    //{
    CurlStart,
    //}
    CurlEnd,

    //;
    PredicateSplit,
    //,
    ObjectSplit,

    //,
    Comma,

    //[
    BrackStart,
    //]
    BrackEnd,

    //@
    ShapeLinkStart,

    ShapeName(String)
}
