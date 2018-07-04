use std::sync::Arc;

use futures::task::Wake;
use futures::prelude::*;

use super::panic_executor::PanicExecutor;

pub fn with_noop_waker_context<F, R>(f: F) -> R
    where F: FnOnce(&mut task::Context) -> R
{
    struct NoopWake;

    impl Wake for NoopWake {
        fn wake(_: &Arc<Self>) {}
    }

    let noop_waker = unsafe { task::local_waker(Arc::new(NoopWake)) };
    let exec = &mut PanicExecutor;

    let cx = &mut task::Context::new(&noop_waker, exec);
    f(cx)
}
