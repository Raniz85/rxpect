/// A simpler version of Cow that can only borrow values, not convert them into owned.
pub enum BorrowedOrOwned<'a, T> {
    Borrowed(&'a T),
    Owned(T),
}

impl<'a, T> BorrowedOrOwned<'a, T> {
    /// Borrows this value with a lifetime that matches self
    pub fn borrow_self(&'a self) -> &'a T {
        match self {
            BorrowedOrOwned::Borrowed(reference) => reference,
            BorrowedOrOwned::Owned(owned) => owned,
        }
    }
}
