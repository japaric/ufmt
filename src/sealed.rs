use crate::{uWrite, Formatter};

pub trait DoAsFormatter {
    type Writer: uWrite;

    fn do_as_formatter(
        self,
        f: impl FnOnce(&mut Formatter<'_, Self::Writer>) -> Result<(), <Self::Writer as uWrite>::Error>,
    ) -> Result<(), <Self::Writer as uWrite>::Error>;
}

impl<W> DoAsFormatter for &'_ mut W
where
    W: uWrite,
{
    type Writer = W;

    fn do_as_formatter(
        self,
        f: impl FnOnce(&mut Formatter<'_, W>) -> Result<(), W::Error>,
    ) -> Result<(), W::Error> {
        f(&mut Formatter::new(self))
    }
}

impl<W> DoAsFormatter for &'_ mut Formatter<'_, W>
where
    W: uWrite,
{
    type Writer = W;

    fn do_as_formatter(
        self,
        f: impl FnOnce(&mut Formatter<'_, W>) -> Result<(), W::Error>,
    ) -> Result<(), W::Error> {
        f(self)
    }
}
