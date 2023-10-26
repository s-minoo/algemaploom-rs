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

    // prefix namespace
    PrefixNS(String),
    // prefix local name
    PrefixLN(String),

    // Identifier used by source, matcher, function, expression
    Ident(String),
    // Value
    Value(String),



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
