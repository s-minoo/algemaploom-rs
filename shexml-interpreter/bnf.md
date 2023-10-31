<shexml> ::= (<prefix> "\n")+ (<source> "\n")+

<prefix> ::= "PREFIX" <whitespace> <prefix_alias> <whitespace> <iri>
<prefix_alias> ::= [a-z]* ":"

<source> ::= "SOURCE" <whitespace> <ident> <whitespace> <iri>

<iterator> ::= "ITERATOR" <whitespace> <ident> <iterator_query>
<iterator_query> ::= "<" <query_type> ">"

<query_type> ::= <xpath> | <jsonpath> | <csv>
<xpath>  ::= "xpath:" <whitespace> <xpath_iri>
<xpath_iri> ::= ("\/" | [a-z]+ | [A-Z]* | [0-9]* )+

<jsonpath> ::= "jsonpath:"  <whitespace>  <jsonquery>
<jsonquery> ::= "$" ([a-z]* | "." | [A-Z]* | [0-9]* | "[" | "]")+


<ident> ::= ([a-z]+ | "*" )+
<iri> ::= "<" ([a-z] | [0-9] | ":" | "\/" | "." | "\*")+ ">"
<whitespace> ::= " "+
