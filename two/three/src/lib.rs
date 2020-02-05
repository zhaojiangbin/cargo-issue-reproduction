//!
use std::io::{Write, Result};

pub trait Three {
    fn write(&self, output: &mut dyn Write) -> Result<()>;
}

impl Three for &[u8] {
    fn write(&self, _: &mut dyn Write) -> Result<()> {
        Ok(())
    }
}
