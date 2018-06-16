use std::ops::Deref;
use vector::*;
use constants::*;
use std::marker::PhantomData;
use intrinsics::spirv_discard;
#[spirv(PerFragment)]
pub struct Fragment {
    pub frag_coord: Vec4<f32>,
}

impl Fragment {
    #[spirv(discard)]
    pub fn discard() {
        unsafe {
            spirv_discard();
        }
    }
}

#[spirv(PerVertex)]
pub struct Vertex {
    pub position: Vec4<f32>,
    pub point_size: f32,
}

#[spirv(Compute)]
pub struct Compute {
    pub local_invocation_index: u32,
}

#[spirv(Input)]
pub struct Input<Location: Sized, T> {
    pub data: T,
    pub _location: PhantomData<Location>,
}

// impl<LInput, LOutput, T> From<Input<LInput, T>> for Output<LOutput, T> {
//     fn from(input: Input<LInput, T>) -> Output<LOutput, T> {
//         Output::new(data: input.data)
//     }
// }
impl<Location, T> Deref for Input<Location, T> {
    type Target = T;
    #[inline(always)]
    fn deref(&self) -> &T {
        &self.data
    }
}

#[spirv(Output)]
pub struct Output<Location: Sized, T> {
    pub data: T,
    pub _location: PhantomData<Location>,
}

impl<Location, T> Deref for Output<Location, T> {
    type Target = T;
    #[inline(always)]
    fn deref(&self) -> &T {
        &self.data
    }
}

impl<Location, T> Output<Location, T> {
    pub fn new(data: T) -> Output<Location, T> {
        Output {
            _location: PhantomData,
            data,
        }
    }
}
#[spirv(RuntimeArray)]
pub struct RuntimeArray<T> {
    pub _m: PhantomData<T>,
}

#[spirv(Descriptor)]
pub struct Uniform<Binding, Set, T>
where
    Binding: Constant,
    Set: Constant,
{
    pub data: T,
    pub _location: PhantomData<Binding>,
    pub _binding: PhantomData<Set>,
}
impl<Binding, Set, T> Deref for Uniform<Binding, Set, T>
where
    Binding: Constant,
    Set: Constant,
{
    type Target = T;
    #[inline(always)]
    fn deref(&self) -> &T {
        &self.data
    }
}

// #[spirv(Buffer)]
// pub struct Buffer<Binding: Sized, Set: Sized, T> {
//     data: T
// }
