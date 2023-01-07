use super::*;

// wxRefCounter
wxwidgets! {
    /// This class is used to manage reference-counting providing a simple interface and a counter.
    /// - [`RefCounter`] represents a C++ `wxRefCounter` class instance which your code has ownership, [`RefCounterInRust`]`<false>` represents one which don't own.
    /// - Use [`RefCounter`]'s `new()` to create an instance of this class.
    /// - See [C++ `wxRefCounter` class's documentation](https://docs.wxwidgets.org/3.2/classwx_ref_counter.html) for more details.
    #[doc(alias = "wxRefCounter")]
    #[doc(alias = "RefCounter")]
    class RefCounter
        = RefCounterInRust<true>(wxRefCounter) impl
        RefCounterMethods
}
impl<const OWNED: bool> RefCounterInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxRefCounter::wxRefCounter()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_ref_counter.html#aebcddb8241dfea7f60f8e4df6776a0e3).
    pub fn new() -> RefCounterInRust<OWNED> {
        unsafe { RefCounterInRust(ffi::wxRefCounter_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for RefCounterInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for RefCounterInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxRefCounter_delete(self.0) }
        }
    }
}
