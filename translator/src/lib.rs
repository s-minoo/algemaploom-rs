use plangenerator::{
    error::PlanError,
    plan::{Init, Plan},
};

pub mod rmlalgebra;
pub mod shexml;
mod test_macro;

pub type TranslateResult = Result<Plan<Init>, PlanError>;

pub trait Translator<T> {
    fn translate_to_plan(model: T) -> TranslateResult;
}
