use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Extension trait so you can call select() on a tuple of futures.
///
/// This is implemented on tuples from 1 to 8 values.
pub trait Select {
    type F: Future<Output=Self::O> + Unpin;
    type O: Unpin;

    fn select(self) -> Self::F;
}

#[doc(hidden)]
pub struct Selecter<T, O> {
    tuples: T,
    output: PhantomData<O>,
}

macro_rules! dopoll {
    ($fut: expr, $cx: expr) => {
        match Pin::new(&mut $fut).poll($cx) {
            r @ Poll::Ready(_) => return r,
            Poll::Pending => {},
        }
    }
}

macro_rules! selecter {
    ($num:expr, $($F:ident, $N:tt),*) => {
        impl<$($F),*, O> Future for Selecter<($($F,)*), O>
        where
            $($F: Future<Output=O> + Unpin),*,
            O: Unpin,
        {
            type Output = O;
    
            fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {

                // choose a random future to start with.
                let start = $crate::select::random(..=$num);
                let mut cur = start;
                loop {
                    match cur {
                        $(
                            $N => dopoll!(self.as_mut().tuples.$N, cx),
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
            $($F: Future<Output=O> + Unpin),*,
            O: Unpin,
        {
            type F = Selecter<($($F,)*), O>;
            type O = O;
    
            fn select(self) -> Self::F {
                Selecter {
                    tuples: ($(self.$N,)*),
                    output: PhantomData,
                }
            }
        }
    }
}

selecter!(0, F0, 0);
selecter!(1, F0, 0, F1, 1);
selecter!(2, F0, 0, F1, 1, F2, 2);
selecter!(3, F0, 0, F1, 1, F2, 2, F3, 3);
selecter!(4, F0, 0, F1, 1, F2, 2, F3, 3, F4, 4);
selecter!(5, F0, 0, F1, 1, F2, 2, F3, 3, F4, 4, F5, 5);
selecter!(6, F0, 0, F1, 1, F2, 2, F3, 3, F4, 4, F5, 5, F6, 6);
selecter!(7, F0, 0, F1, 1, F2, 2, F3, 3, F4, 4, F5, 5, F6, 6, F7, 7);

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
