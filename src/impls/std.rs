use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use crate::{uDebug, uDisplay, uWrite, Formatter};

impl<T> uDebug for Box<T>
where
    T: uDebug,
{
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        <T as uDebug>::fmt(self, f)
    }
}

impl<T> uDisplay for Box<T>
where
    T: uDisplay,
{
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        <T as uDisplay>::fmt(self, f)
    }
}

impl<K, V> uDebug for BTreeMap<K, V>
where
    K: uDebug,
    V: uDebug,
{
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        f.debug_map()?.entries(self)?.finish()
    }
}

impl<T> uDebug for BTreeSet<T>
where
    T: uDebug,
{
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        f.debug_set()?.entries(self)?.finish()
    }
}

impl<K, V> uDebug for HashMap<K, V>
where
    K: uDebug,
    V: uDebug,
{
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        f.debug_map()?.entries(self)?.finish()
    }
}

impl<T> uDebug for HashSet<T>
where
    T: uDebug,
{
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        f.debug_set()?.entries(self)?.finish()
    }
}

// TODO
// impl uDebug for String {
//     fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
//     where
//         W: uWrite + ?Sized,
//     {
//         <str as uDebug>::fmt(self, f)
//     }
// }

impl uDisplay for String {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        <str as uDisplay>::fmt(self, f)
    }
}

impl<T> uDebug for Vec<T>
where
    T: uDebug,
{
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        <[T] as uDebug>::fmt(self, f)
    }
}
