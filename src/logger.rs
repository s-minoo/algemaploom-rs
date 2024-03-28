use std::env;

pub fn init_logger() -> anyhow::Result<()> {
    let mut pwd = env::current_dir()?;
    pwd.push("log4rs.yaml");
    log4rs::init_file(pwd, Default::default())?;
    Ok(())
}
