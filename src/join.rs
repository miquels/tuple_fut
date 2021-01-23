use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Extension trait so you can call join() on a tuple of futures.
///
/// This is only implemented on tuples of arity 12 or less.
pub trait Join {
    type Future: Future<Output=Self::Output>;
    type Output;

    fn join(self) -> Self::Future;
}

#[doc(hidden)]
pub enum FutStatus<F, O> {
    Future(F),
    Output(O),
    Empty,
}

#[doc(hidden)]
pub struct Joiner<T> {
    tuples: T,
}

macro_rules! dopoll {
    ($tuple:expr, $cx:expr) => {
        match $tuple {
            FutStatus::Future(fut) => {
                // Safety: `Joiner` is !Unpin if any of its members are.
                let f = unsafe { Pin::new_unchecked(fut) };
                match f.poll($cx) {
                    Poll::Ready(r) => {
                        *$tuple = FutStatus::Output(r);
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

macro_rules! joiner {
    ($($F:ident, $O:ident, $N:tt),*) => {
        impl<$($F, $O),*> Future for Joiner<($(FutStatus<$F, $O>,)*)>
        where
            $($F: Future<Output=$O>,)*
        {
            type Output = ($($O,)*);
    
            fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                let mut done = true;
                // Safety: `Joiner` is !Unpin if any of its members are.
                let this = unsafe { self.get_unchecked_mut() };
                $(
                    done &= dopoll!(&mut this.tuples.$N, cx);
                )*
                if done {
                    Poll::Ready(($(output!(this.tuples.$N),)*)) 
                } else {
                    Poll::Pending
                }
            }
        }

        impl<$($F, $O),*> Join for ($($F,)*)
        where
            $($F: Future<Output=$O>,)*
        {
            type Future = Joiner<($(FutStatus<$F, $O>,)*)>;
            type Output = ($($O,)*);
    
            fn join(self) -> Self::Future {
                Joiner {
                    tuples: ($(FutStatus::Future(self.$N), )*)
                }
            }
        }

        impl<$($F, $O),*> Unpin for Joiner<($(FutStatus<$F, $O>,)*)>
        where
        $(
            $F: Future<Output=$O> + Unpin,
            $O: Unpin,
        )*
        {}
    }
}

joiner!(F0, O0, 0);
joiner!(F0, O0, 0, F1, O1, 1);
joiner!(F0, O0, 0, F1, O1, 1, F2, O2, 2);
joiner!(F0, O0, 0, F1, O1, 1, F2, O2, 2, F3, O3, 3);
joiner!(F0, O0, 0, F1, O1, 1, F2, O2, 2, F3, O3, 3, F4, O4, 4);
joiner!(F0, O0, 0, F1, O1, 1, F2, O2, 2, F3, O3, 3, F4, O4, 4, F5, O5, 5);
joiner!(F0, O0, 0, F1, O1, 1, F2, O2, 2, F3, O3, 3, F4, O4, 4, F5, O5, 5, F6, O6, 6);
joiner!(F0, O0, 0, F1, O1, 1, F2, O2, 2, F3, O3, 3, F4, O4, 4, F5, O5, 5, F6, O6, 6, F7, O7, 7);
joiner!(F0, O0, 0, F1, O1, 1, F2, O2, 2, F3, O3, 3, F4, O4, 4, F5, O5, 5, F6, O6, 6, F7, O7, 7, F8, O8, 8);
joiner!(F0, O0, 0, F1, O1, 1, F2, O2, 2, F3, O3, 3, F4, O4, 4, F5, O5, 5, F6, O6, 6, F7, O7, 7, F8, O8, 8, F9, O9, 9);
joiner!(F0, O0, 0, F1, O1, 1, F2, O2, 2, F3, O3, 3, F4, O4, 4, F5, O5, 5, F6, O6, 6, F7, O7, 7, F8, O8, 8, F9, O9, 9, F10, I10, 10);
joiner!(F0, O0, 0, F1, O1, 1, F2, O2, 2, F3, O3, 3, F4, O4, 4, F5, O5, 5, F6, O6, 6, F7, O7, 7, F8, O8, 8, F9, O9, 9, F10, I10, 10, F11, O11, 11);
