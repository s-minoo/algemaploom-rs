use std::collections::{HashMap, HashSet};

use meamer_rs::functions::constant::Constant;
use meamer_rs::functions::iri::IRI;
use meamer_rs::functions::uriencode::URIEncode;
use meamer_rs::functions::FunctionChain;
use meamer_rs::operators::OperatorChain;
use meamer_rs::operators::extend::{ExtendOp, ExtendTuple};
use meamer_rs::operators::project::Projection;
use meamer_rs::operators::serializers::rdf_serializer::RDFSerializer;
use meamer_rs::operators::sources::file::csv::CSVFileSource;
use operator::Source;

pub fn main() {
    let serializer_op = RDFSerializer{
        template:  " ?tm0_sm ?tm0_p0-0 ?tm0_o0-0.\n ?tm0_sm ?tm0_p1-0 ?tm0_o1-0.\n ?tm0_sm ?tm0_p2-0 ?tm0_o2-0.\n".to_string(),
    };

    let csv_source = CSVFileSource {
        config: Source {
            config: HashMap::from([(
                "path".to_string(),
                "./Airport.csv".to_string(),
            )]),
            source_type:   operator::IOType::File,
            data_format:   operator::formats::DataFormat::CSV,
        },
    };

    let extend_func1 = ExtendTuple {
        new_attribute: "?tm0_p0-0".to_string(),
        function:      Constant {
            constant: "http://vocab.org/transit/terms/route".to_string(),
            next:     URIEncode {
                next: IRI { next: None }.into_boxed_opt(),
            }
            .into_boxed_opt(),
        }
        .into_boxed(),
    };


    let extend_func2 = ExtendTuple{
        new_attribute: "?tm0_sm0".to_string(),
        function: todo!(),
    }; 

    let extend_op = ExtendOp {
        extend_tuples: vec![],
        next:          None,
    }.into_boxed_opt();

    let projection_op = Projection {
        select_attributes: HashSet::from([
            "id".into(),
            "stop".into(),
            "latitude".into(),
            "longitude".into(),
        ]),
        next:              None,
    };
}
