use crate::rml_model::term_map::SubjectMap;

use super::Extractor;


impl Extractor<SubjectMap> for SubjectMap{
    fn extract(subject: &super::TermShared, graph: &sophia_inmem::graph::FastGraph) -> super::ExtractorResult<SubjectMap> {


        todo!()

    }
}

