/// Converts a futures 0.3 [`TryFuture`](futures_core::future::TryFuture) or
/// [`TryStream`](futures_core::stream::TryStream) into a futures 0.1
/// [`Future`](futures01::future::Future) or
/// [`Stream`](futures01::stream::Stream) and vice versa.
#[derive(Debug)]
#[must_use = "futures do nothing unless polled"]
pub struct Compat<T, Sp> {
    crate inner: T,
    crate spawn: Option<Sp>,
}

impl<T, Sp> Compat<T, Sp> {
    /// Returns the inner item.
    pub fn into_inner(self) -> T {
        self.inner
    }

    /// Creates a new [`Compat`].
    crate fn new(inner: T, spawn: Option<Sp>) -> Compat<T, Sp> {
        Compat { inner, spawn }
    }
}
