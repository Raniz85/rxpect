use std::borrow::Borrow;

pub enum BorrowedOrOwned<'a, T> {
    Borrowed(&'a T),
    Owned(T),
}

impl<'a, T> Borrow<T> for BorrowedOrOwned<'a, T> {
    fn borrow(&self) -> &T {
        match self {
            BorrowedOrOwned::Borrowed(reference) => reference,
            BorrowedOrOwned::Owned(owned) => owned,
        }
    }
}
