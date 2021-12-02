pub mod prelude {
    pub use color_eyre::{eyre::eyre as err, eyre::WrapErr, Result};
    pub use itertools::Itertools;
}

pub fn setup() -> color_eyre::Result<()> {
    color_eyre::install()?;
    Ok(())
}
