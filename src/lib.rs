pub mod prelude {
    pub use color_eyre::{Result, eyre::eyre as err};
    pub use itertools::Itertools;
}

pub fn setup() -> color_eyre::Result<()> {
    color_eyre::install()?;
    Ok(())
}
