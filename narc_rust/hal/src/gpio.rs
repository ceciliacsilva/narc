//! General Purpose Input Output

// It should provide the same interface as the program in C.
// TODO: Assert that

use core::marker::PhantomData;

pub trait GpioExt {
    type Parts;

    pub fn split(self) -> Self::Parts;
}

/// Input Mode.
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Input Mode types. 
pub struct Floating;
pub struct PullDown;
pub struct PullUp;

pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

/// Output Mode types.
pub struct PushPull;
pub struct OpenDrain;

/// Alternative function output.
pub struct Alternative<MODE> {
    _mode: PhantomData<MODE>,
}


