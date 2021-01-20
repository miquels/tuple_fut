use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Extension trait so you can call join() on a tuple of futures.
///
/// This is implemented on tuples from 1 to 8 values.
pub trait Join {
    type F: Future<Output=Self::O> + Unpin;
    type O: Unpin;

    fn join(self) -> Self::F;
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
    ($fut: expr, $cx: expr) => {
        match &mut $fut {
            FutStatus::Future(fut) => {
                match Pin::new(fut).poll($cx) {
                    Poll::Ready(r) => {
                        $fut = FutStatus::Output(r);
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
            $($F: Future<Output=$O> + Unpin,)*
            $($O: Unpin,)*
        {
            type Output = ($($O,)*);
    
            fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                let mut done = true;
                $(
                    done &= dopoll!(self.as_mut().tuples.$N, cx);
                )*
                if done {
                    Poll::Ready(($(output!(self.as_mut().tuples.$N),)*)) 
                } else {
                    Poll::Pending
                }
            }
        }

        impl<$($F, $O),*> Join for ($($F,)*)
        where
            $($F: Future<Output=$O> + Unpin,)*
            $($O: Unpin,)*
        {
            type F = Joiner<($(FutStatus<$F, $O>,)*)>;
            type O = ($($O,)*);
    
            fn join(self) -> Self::F {
                Joiner {
                    tuples: ($(FutStatus::Future(self.$N), )*)
                }
            }
        }
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
