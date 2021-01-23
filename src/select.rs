use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Extension trait so you can call select() on a tuple of futures.
///
/// This is only implemented on tuples of arity 12 and less.
pub trait Select {
    type Future: Future<Output=Self::Output>;
    type Output;

    fn select(self) -> Self::Future;
}

#[doc(hidden)]
pub struct Selecter<T> {
    tuples: T,
}

macro_rules! selecter {
    ($num:expr, $($F:ident, $N:tt),*) => {
        impl<$($F),*, O> Future for Selecter<($($F,)*)>
        where
            $($F: Future<Output=O>),*,
        {
            type Output = O;
    
            fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {

                // choose a random future to start with.
                let start = $crate::select::random(..=$num);
                let mut cur = start;
                // Safety: `Selecter` is !Unpin if any of its members are.
                let this = unsafe { self.get_unchecked_mut() };
                loop {
                    match cur {
                        $(
                            $N => {
                                // Safety: `Selecter` is !Unpin if any of its members are.
                                let fut = unsafe { Pin::new_unchecked(&mut this.tuples.$N) };
                                match fut.poll(cx) {
                                    r @ Poll::Ready(_) => return r,
                                    Poll::Pending => {},
                                }
                            },
                        )*
                        _ => unreachable!(),
                    }
                    cur = (cur + 1) % ($num + 1);
                    if cur == start {
                        break;
                    }
                }
                Poll::Pending
            }
        }

        impl<$($F),*, O> Select for ($($F,)*)
        where
            $($F: Future<Output=O> ),*,
        {
            type Future = Selecter<($($F,)*)>;
            type Output = O;
    
            fn select(self) -> Self::Future {
                Selecter {
                    tuples: ($(self.$N,)*),
                }
            }
        }

        impl<$($F),*> Unpin for Selecter<($($F,)*)>
        where
            $($F: Future + Unpin),*,
        {}
    }
}

#[doc(hidden)]
pub fn random(bounds: impl std::ops::RangeBounds<u32>) -> u32 {
    use std::cell::RefCell;
    use std::time::SystemTime;

    thread_local!(static RNG: RefCell<u64> = {
        // get seed from clock.
        let d = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
        RefCell::new(d.map(|d| d.as_micros() as u64).unwrap_or(1611135316))
    });

    fn bound(b: std::ops::Bound<&u32>, u: u32) -> u32 {
        match b {
            std::ops::Bound::Included(&x) => x,
            std::ops::Bound::Excluded(&x) => x - 1,
            std::ops::Bound::Unbounded => u,
        }
    }
    let start = bound(bounds.start_bound(), 0);
    let end = bound(bounds.end_bound(), u32::MAX);
    let range = end - start + 1;

    RNG.with(|state| {
        // Knuth MMIX RNG.
        const A: u64 = 6364136223846793005;
        const C: u64 = 1442695040888963407;
        let mut state = state.borrow_mut();
        *state = ((*state).wrapping_add(C)).wrapping_mul(A);
        // map bits 47..0 to a 32 bit number.
        start + (((*state & (1 << 48) - 1)) / ((1 << 48) / (range as u64))) as u32
    })
}

selecter!(0, F0, 0);
selecter!(1, F0, 0, F1, 1);
selecter!(2, F0, 0, F1, 1, F2, 2);
selecter!(3, F0, 0, F1, 1, F2, 2, F3, 3);
selecter!(4, F0, 0, F1, 1, F2, 2, F3, 3, F4, 4);
selecter!(5, F0, 0, F1, 1, F2, 2, F3, 3, F4, 4, F5, 5);
selecter!(6, F0, 0, F1, 1, F2, 2, F3, 3, F4, 4, F5, 5, F6, 6);
selecter!(7, F0, 0, F1, 1, F2, 2, F3, 3, F4, 4, F5, 5, F6, 6, F7, 7);
selecter!(7, F0, 0, F1, 1, F2, 2, F3, 3, F4, 4, F5, 5, F6, 6, F7, 7, F8, 8);
selecter!(7, F0, 0, F1, 1, F2, 2, F3, 3, F4, 4, F5, 5, F6, 6, F7, 7, F8, 8, F9, 9);
selecter!(7, F0, 0, F1, 1, F2, 2, F3, 3, F4, 4, F5, 5, F6, 6, F7, 7, F8, 8, F9, 9, F10, 10);
selecter!(7, F0, 0, F1, 1, F2, 2, F3, 3, F4, 4, F5, 5, F6, 6, F7, 7, F8, 8, F9, 9, F10, 10, F11, 11);
