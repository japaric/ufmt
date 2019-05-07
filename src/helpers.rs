use crate::{uDebug, uWrite, Formatter};

impl<'w, W> Formatter<'w, W>
where
    W: uWrite,
{
    /// Creates a `DebugList` builder designed to assist with creation of `fmt::Debug`
    /// implementations for list-like structures.
    pub fn debug_list(&mut self) -> Result<DebugList<'_, 'w, W>, W::Error> {
        self.write("[")?;

        if self.pretty {
            self.indentation += 1;
        }

        Ok(DebugList {
            first: true,
            formatter: self,
        })
    }

    /// Creates a `DebugMap` builder designed to assist with creation of `fmt::Debug`
    /// implementations for map-like structures.
    pub fn debug_map(&mut self) -> Result<DebugMap<'_, 'w, W>, W::Error> {
        self.write("{")?;

        if self.pretty {
            self.indentation += 1;
        }

        Ok(DebugMap {
            first: true,
            formatter: self,
        })
    }

    /// Creates a `DebugSet` builder designed to assist with creation of `fmt::Debug`
    /// implementations for set-like structures.
    pub fn debug_set(&mut self) -> Result<DebugSet<'_, 'w, W>, W::Error> {
        self.write("{")?;

        if self.pretty {
            self.indentation += 1;
        }

        Ok(DebugSet {
            first: true,
            formatter: self,
        })
    }

    /// Creates a `DebugStruct` builder designed to assist with creation of `fmt::Debug`
    /// implementations for structs.
    pub fn debug_struct(&mut self, name: &str) -> Result<DebugStruct<'_, 'w, W>, W::Error> {
        self.write(name)?;

        if self.pretty {
            self.indentation += 1;
        }

        Ok(DebugStruct {
            first: true,
            formatter: self,
        })
    }

    /// Creates a `DebugTuple` builder designed to assist with creation of `fmt::Debug`
    /// implementations for tuple structs.
    pub fn debug_tuple(&mut self, name: &str) -> Result<DebugTuple<'_, 'w, W>, W::Error> {
        self.write(name)?;

        if self.pretty {
            self.indentation += 1;
        }

        Ok(DebugTuple {
            fields: 0,
            first: true,
            formatter: self,
            unnamed: name.is_empty(),
        })
    }
}

/// A struct to help with `fmt::Debug` implementations.
///
/// This is useful when you wish to output a formatted list of items as a part of your `Debug::fmt`
/// implementation.
pub struct DebugList<'f, 'w, W>
where
    W: uWrite,
{
    first: bool,
    formatter: &'f mut Formatter<'w, W>,
}

impl<W> DebugList<'_, '_, W>
where
    W: uWrite,
{
    /// Adds a new entry to the list output.
    pub fn entry(&mut self, entry: &impl uDebug) -> Result<&mut Self, W::Error> {
        if self.first {
            self.first = false;

            if self.formatter.pretty {
                self.formatter.write("\n")?;
            }
        } else if !self.formatter.pretty {
            self.formatter.write(", ")?;
        }

        if self.formatter.pretty {
            self.formatter.indent()?;
        }

        entry.fmt(self.formatter)?;

        if self.formatter.pretty {
            self.formatter.write(",\n")?;
        }

        Ok(self)
    }

    /// Adds the contents of an iterator of entries to the list output.
    pub fn entries(
        &mut self,
        entries: impl IntoIterator<Item = impl uDebug>,
    ) -> Result<&mut Self, W::Error> {
        for entry in entries {
            self.entry(&entry)?;
        }

        Ok(self)
    }

    /// Finishes output and returns any error encountered.
    pub fn finish(&mut self) -> Result<(), W::Error> {
        if self.formatter.pretty {
            self.formatter.indentation -= 1;
            self.formatter.indent()?;
        }

        self.formatter.write("]")
    }
}

/// A struct to help with `fmt::Debug` implementations.
///
/// This is useful when you wish to output a formatted map as a part of your `Debug::fmt`
/// implementation.
pub struct DebugMap<'f, 'w, W>
where
    W: uWrite,
{
    first: bool,
    formatter: &'f mut Formatter<'w, W>,
}

impl<W> DebugMap<'_, '_, W>
where
    W: uWrite,
{
    /// Adds a new entry to the map output.
    pub fn entry(&mut self, key: &impl uDebug, value: &impl uDebug) -> Result<&mut Self, W::Error> {
        if self.first {
            self.first = false;

            if self.formatter.pretty {
                self.formatter.write("\n")?;
            }
        } else if !self.formatter.pretty {
            self.formatter.write(", ")?;
        }

        if self.formatter.pretty {
            self.formatter.indent()?;
        }

        key.fmt(self.formatter)?;
        self.formatter.write(": ")?;
        value.fmt(self.formatter)?;

        if self.formatter.pretty {
            self.formatter.write(",\n")?;
        }

        Ok(self)
    }

    /// Adds the contents of an iterator of entries to the map output.
    pub fn entries(
        &mut self,
        entries: impl IntoIterator<Item = (impl uDebug, impl uDebug)>,
    ) -> Result<&mut Self, W::Error> {
        for (k, v) in entries.into_iter() {
            self.entry(&k, &v)?;
        }

        Ok(self)
    }

    /// Finishes output and returns any error encountered.
    pub fn finish(&mut self) -> Result<(), W::Error> {
        self.formatter.write("}")
    }
}

/// A struct to help with `fmt::Debug` implementations.
pub struct DebugSet<'f, 'w, W>
where
    W: uWrite,
{
    first: bool,
    formatter: &'f mut Formatter<'w, W>,
}

impl<W> DebugSet<'_, '_, W>
where
    W: uWrite,
{
    /// Adds a new entry to the set output.
    pub fn entry(&mut self, entry: &impl uDebug) -> Result<&mut Self, W::Error> {
        if self.first {
            self.first = false;

            if self.formatter.pretty {
                self.formatter.write("\n")?;
            }
        } else if !self.formatter.pretty {
            self.formatter.write(", ")?;
        }

        if self.formatter.pretty {
            self.formatter.indent()?;
        }

        entry.fmt(self.formatter)?;

        if self.formatter.pretty {
            self.formatter.write(",\n")?;
        }

        Ok(self)
    }

    /// Adds the contents of an iterator of entries to the set output.
    pub fn entries(
        &mut self,
        entries: impl IntoIterator<Item = impl uDebug>,
    ) -> Result<&mut Self, W::Error> {
        for entry in entries {
            self.entry(&entry)?;
        }

        Ok(self)
    }

    /// Finishes output and returns any error encountered.
    pub fn finish(&mut self) -> Result<(), W::Error> {
        self.formatter.write("}")
    }
}

/// A struct to help with `fmt::Debug` implementations.
pub struct DebugStruct<'f, 'w, W>
where
    W: uWrite,
{
    first: bool,
    formatter: &'f mut Formatter<'w, W>,
}

impl<W> DebugStruct<'_, '_, W>
where
    W: uWrite,
{
    /// Adds a new field to the generated struct output.
    pub fn field(&mut self, name: &str, value: &impl uDebug) -> Result<&mut Self, W::Error> {
        if self.first {
            self.first = false;

            self.formatter.write(" {")?;

            if self.formatter.pretty {
                self.formatter.write("\n")?;
            } else {
                self.formatter.write(" ")?;
            }
        } else if !self.formatter.pretty {
            self.formatter.write(", ")?;
        }

        if self.formatter.pretty {
            self.formatter.indent()?;
        }

        self.formatter.write(name)?;
        self.formatter.write(": ")?;
        value.fmt(self.formatter)?;

        if self.formatter.pretty {
            self.formatter.write(",\n")?;
        }

        Ok(self)
    }

    /// Finishes output and returns any error encountered.
    pub fn finish(&mut self) -> Result<(), W::Error> {
        if self.formatter.pretty {
            self.formatter.indentation -= 1;
        }

        if !self.first {
            if self.formatter.pretty {
                self.formatter.indent()?;
            } else {
                self.formatter.write(" ")?;
            }

            self.formatter.write("}")?;
        }

        Ok(())
    }
}

/// A struct to help with `fmt::Debug` implementations.
pub struct DebugTuple<'f, 'w, W>
where
    W: uWrite,
{
    fields: u8,
    first: bool,
    formatter: &'f mut Formatter<'w, W>,
    unnamed: bool,
}

impl<W> DebugTuple<'_, '_, W>
where
    W: uWrite,
{
    /// Adds a new field to the generated tuple struct output.
    pub fn field(&mut self, value: &impl uDebug) -> Result<&mut Self, W::Error> {
        self.fields += 1;

        if self.first {
            self.first = false;

            self.formatter.write("(")?;

            if self.formatter.pretty {
                self.formatter.write("\n")?;
            }
        } else if !self.formatter.pretty {
            self.formatter.write(", ")?;
        }

        if self.formatter.pretty {
            self.formatter.indent()?;
        }

        value.fmt(self.formatter)?;

        if self.formatter.pretty {
            self.formatter.write(",\n")?;
        }

        Ok(self)
    }

    /// Finishes output and returns any error encountered.
    pub fn finish(&mut self) -> Result<(), W::Error> {
        if self.formatter.pretty {
            self.formatter.indentation -= 1;
        }

        if !self.first {
            if self.formatter.pretty {
                self.formatter.indent()?;
            } else if self.unnamed && self.fields == 1 {
                // this is a one-element tuple so we need a trailing comma
                self.formatter.write(",")?;
            }

            self.formatter.write(")")?;
        }

        Ok(())
    }
}
