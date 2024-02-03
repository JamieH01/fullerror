#![doc = include_str!("../README.md")]

use std::{error::Error, ops::{Deref, DerefMut}, fmt::Display};
use std::fmt::Debug;

const SOURCE_DEF: &str = "from =>";

///Use a custom tab for nested sources. Defaults to "from =>".
#[macro_export]
macro_rules! set_tab {
    ($tab:tt) => {
        std::env::set_var(TAB_ENV, $tab)
    }
}

///The enviroment variable that sets the source tab. Defaults to "from =>" if this is not set. Set
///with the [`set_tab`] macro.
pub const TAB_ENV: &str = "FULLERROR_SOURCE_HEADER";

///The full error.
///
///`?` will implicitly construct this error type.
pub struct FullError<E>(E);

impl<E> Deref for FullError<E> {
    type Target = E;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<E> DerefMut for FullError<E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<E> From<E> for FullError<E> {
    fn from(value: E) -> Self {
        Self(value)
    }
}

impl<E: Debug> Debug for FullError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<E: Error> Display for FullError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.0)?;

        let mut source = self.0.source();
        while let Some(e) = source {
            writeln!(f, "{} {e}", std::env::var("FULLERROR_SOURCE_HEADER").unwrap_or(SOURCE_DEF.to_string()))?;
            source = source.and_then(Error::source);
        }

        Ok(())
    }
}

impl<E: Error> Error for FullError<E> {}


#[cfg(test)]
mod tests {
    use thiserror::Error;

    use super::*;

    #[test]
    fn it_works() {
        #[derive(Debug, Error)]
        enum ErrA {
            #[error("error variant a: {0}")]
            ErrVarA(u8),
            #[error("error variant b")]
            ErrVarB,
        }

        #[derive(Debug, Error)]
        enum ErrB {
            #[error("ErrA error")]
            ErrA(#[source] #[from] ErrA),
            #[error("error variant a")]
            ErrVarA,
        }

        
        let error: FullError<ErrB> = ErrB::from(ErrA::ErrVarA(5)).into();
        println!("{error}");
    }
}
