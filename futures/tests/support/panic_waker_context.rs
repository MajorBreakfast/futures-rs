use std::sync::Arc;

use futures::task::Wake;
use futures::prelude::*;

use super::panic_executor::PanicExecutor;

pub fn with_panic_waker_context<F, R>(f: F) -> R
    where F: FnOnce(&mut task::Context) -> R
{
    struct PanicWake;

    impl Wake for PanicWake {
        fn wake(_: &Arc<Self>) {
            panic!("should not be woken");
        }
    }

    let panic_waker = unsafe { task::local_waker(Arc::new(PanicWake)) };
    let exec = &mut PanicExecutor;

    let cx = &mut task::Context::new(&panic_waker, exec);
    f(cx)
}
