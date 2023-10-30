#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShExMLToken {
    //prologue
    Prefix,
    Source,
    Iterator,
    Matcher,
    Expression,
    AutoIncrement,

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


    //prefix autoinc
    AutoIncPrefix(String),
    AutoIncStart(u32),
    AutoIncEnd(u32),
    AutoIncStep(u32), 
    AutoIncSuffix(String),



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
    
    // values used in matcher
    Value(String),
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

    //.
    Dot,

    //[
    BrackStart,
    //]
    BrackEnd,

    //&
    MatcherSplit, 


    //@
    ShapeLinkStart,

    ShapeName(String),
}



