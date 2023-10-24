

pub mod extend;
pub mod serializer;
pub mod fragment;

pub trait RMLTranslator<Output> {
    fn translate(self) -> Output;
}
