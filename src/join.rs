use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Extension trait so you can call join() on a tuple of futures.
///
/// This is only implemented on tuples of arity 12 or less.
pub trait Join {
    type Future: Future;

    /// (fut1, fut2, fut3).join().await
    fn join(self) -> Self::Future;
}

/// Holds the future, or the value it generated.
pub struct Holder<F: Future>(FutStatus<F, F::Output>);

enum FutStatus<F, O> {
    Future(F),
    Output(O),
    Empty,
}

/// Future returned by the `join()` method.
pub struct JoinFuture<T> {
    tuples: T,
}

macro_rules! dopoll {
    ($tuple:expr, $cx:expr) => {
        match $tuple {
            FutStatus::Future(ref mut fut) => {
                // Safety: `JoinFuture` is !Unpin if any of its members are.
                let f = unsafe { Pin::new_unchecked(fut) };
                match f.poll($cx) {
                    Poll::Ready(r) => {
                        $tuple = FutStatus::Output(r);
                        true
                    }
                    Poll::Pending => false,
                }
            },
            FutStatus::Output(_) => true,
            FutStatus::Empty => unreachable!(),
        }
    }
}

macro_rules! output {
    ($s: expr) => {
        match std::mem::replace(&mut $s, FutStatus::Empty) {
            FutStatus::Output(o) => o,
            _ => unreachable!(),
        }
    }
}

macro_rules! join_impl {
    ($($F:ident, $N:tt),*) => {
        impl<$($F),*> Future for JoinFuture<($(Holder<$F>,)*)>
        where
            $($F: Future,)*
        {
            type Output = ($($F::Output,)*);
    
            fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                let mut done = true;
                // Safety: `JoinFuture` is !Unpin if any of its members are.
                let this = unsafe { self.get_unchecked_mut() };
                $(
                    done &= dopoll!(this.tuples.$N.0, cx);
                )*
                if done {
                    Poll::Ready(($(output!(this.tuples.$N.0),)*)) 
                } else {
                    Poll::Pending
                }
            }
        }

        impl<$($F),*> Join for ($($F,)*)
        where
            $($F: Future,)*
        {
            type Future = JoinFuture<($(Holder<$F>,)*)>;
    
            fn join(self) -> Self::Future {
                JoinFuture {
                    tuples: ($(Holder(FutStatus::Future(self.$N)), )*)
                }
            }
        }

        impl<$($F),*> Unpin for JoinFuture<($(Holder<$F>,)*)>
        where
        $(
            $F: Future + Unpin,
        )*
        {}
    }
}

join_impl!(F0, 0);
join_impl!(F0, 0, F1, 1);
join_impl!(F0, 0, F1, 1, F2, 2);
join_impl!(F0, 0, F1, 1, F2, 2, F3, 3);
join_impl!(F0, 0, F1, 1, F2, 2, F3, 3, F4, 4);
join_impl!(F0, 0, F1, 1, F2, 2, F3, 3, F4, 4, F5, 5);
join_impl!(F0, 0, F1, 1, F2, 2, F3, 3, F4, 4, F5, 5, F6, 6);
join_impl!(F0, 0, F1, 1, F2, 2, F3, 3, F4, 4, F5, 5, F6, 6, F7, 7);
join_impl!(F0, 0, F1, 1, F2, 2, F3, 3, F4, 4, F5, 5, F6, 6, F7, 7, F8, 8);
join_impl!(F0, 0, F1, 1, F2, 2, F3, 3, F4, 4, F5, 5, F6, 6, F7, 7, F8, 8, F9, 9);
join_impl!(F0, 0, F1, 1, F2, 2, F3, 3, F4, 4, F5, 5, F6, 6, F7, 7, F8, 8, F9, 9, F10, 10);
join_impl!(F0, 0, F1, 1, F2, 2, F3, 3, F4, 4, F5, 5, F6, 6, F7, 7, F8, 8, F9, 9, F10, 10, F11, 11);
