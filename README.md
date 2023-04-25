# Meamer Rs

A modular mapping architecture built in Rust.
The goal of this project is to provide a set of components commonly found
in all, if not most, of the KG mapping engines, and to provide
heterogeneous data to heterogeneous data mapping solution.

The project is split into the following subprojects:

- [Meamer](src/):
  Contains the different mapping components which could be chained with one another
  to create a working mapping engine. Components are chained based on the
  intermediate representation as provided by the [operator subproject](operator/).
- [Operator](operator/):
  Defines the intermediate representations to describe
  mapping processes.
- [Interpreter](interpreter/)
  Interprets mapping languages, such as RML, and SPARQL-Generate, and translate
  it to the intermediate representation as defined in [operator subproject](operator/).
- [DataIO](dataio/)
  Contains APIs to read different input types and stream them to the rest of the
  Meamer components.
- [Vocab](vocab/)
  RDF vocabularies used throughout this project. 

# Installation

- TODO: <25-04-23, Min Oo> +
