use operator::Operator;

pub fn translate_logical_plans(logical_plans: Vec<Operator>) {
    for plan in logical_plans {
        translate_logical_plan(plan)
    }
}

pub fn translate_logical_plan(logical_plan: Operator) {
    match logical_plan {
        Operator::SourceOp { config } => todo!(),
        Operator::JoinOp { config } => todo!(),
        Operator::ProjectOp { config } => todo!(),
        Operator::ExtendOp { config } => todo!(),
        Operator::RenameOp { config } => todo!(),
        Operator::SerializerOp { config } => todo!(),
        Operator::TargetOp { config } => todo!(),
    }
}
