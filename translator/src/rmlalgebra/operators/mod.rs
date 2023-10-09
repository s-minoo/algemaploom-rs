use operator::Operator;

pub mod extend;
pub mod serializer;

pub trait RMLTranslator {
    fn translate(self) -> Operator;
}
