use core::marker::Unpin;
use core::mem::PinMut;
use futures_core::future::Future;
use futures_core::task::{self, Poll, Spawn};

/// Future for the `with_spawner` combinator, assigning a [`Spawn`]
/// to be used when spawning other futures.
///
/// This is created by the `Future::with_spawner` method.
#[derive(Debug)]
#[must_use = "futures do nothing unless polled"]
pub struct WithSpawner<Fut, Sp> where Fut: Future, Sp: Spawn {
    spawner: Sp,
    future: Fut
}

impl<Fut: Future, Sp: Spawn> WithSpawner<Fut, Sp> {
    pub(super) fn new(future: Fut, spawner: Sp) -> WithSpawner<Fut, Sp> {
        WithSpawner { spawner, future }
    }
}

impl<Fut: Future + Unpin, Sp: Spawn> Unpin for WithSpawner<Fut, Sp> {}

impl<Fut, Sp> Future for WithSpawner<Fut, Sp>
    where Fut: Future,
          Sp: Spawn,
{
    type Output = Fut::Output;

    fn poll(self: PinMut<Self>, cx: &mut task::Context) -> Poll<Fut::Output> {
        let this = unsafe { PinMut::get_mut_unchecked(self) };
        let fut = unsafe { PinMut::new_unchecked(&mut this.future) };
        let spawner = &mut this.spawner;
        fut.poll(&mut cx.with_spawner(spawner))
    }
}
