pub trait UnwrapDbg<T> {
    fn unwrap_dbg(self) -> T;
}

impl<T> UnwrapDbg<T> for std::option::Option<T> {
    #[track_caller]
    #[inline]
    fn unwrap_dbg(self) -> T {
        #[cfg(debug_assertions)]
        {
            self.unwrap()
        }
        #[cfg(not(debug_assertions))]
        unsafe {
            self.unwrap_unchecked()
        }
    }
}

impl<T, E> UnwrapDbg<T> for std::result::Result<T, E>
where
    E: std::fmt::Debug,
{
    #[track_caller]
    #[inline]
    fn unwrap_dbg(self) -> T {
        #[cfg(debug_assertions)]
        {
            self.unwrap()
        }
        #[cfg(not(debug_assertions))]
        unsafe {
            self.unwrap_unchecked()
        }
    }
}
