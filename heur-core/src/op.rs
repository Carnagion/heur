use core::{convert::Infallible, error::Error, marker::PhantomData};

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

use cond::{accept::Accept, stop::Stop};

use crate::Problem;

mod then;
pub use then::Then;

mod pipe;
pub use pipe::Pipe;

mod ignore;
pub use ignore::Ignore;

mod map;
pub use map::{Map, MapErr, TryMap};

mod once;
pub use once::Once;

mod accept_if;
pub use accept_if::AcceptIf;

mod repeat;
pub use repeat::{Repeat, RepeatUntil};

mod flatten;
pub use flatten::{FlatMap, Flatten};

mod unwrapped;
pub use unwrapped::Unwrapped;

mod from_fn;
pub use from_fn::FromFn;

mod hint;
pub use hint::Hint;

mod todo;
pub use todo::Todo;

pub mod init;

pub mod population;

pub mod cond;

// TODO: Add #[diagnostic::on_unimplemented]
pub trait Operator<P: Problem, In = ()> {
    type Output;

    type Error: Error;

    fn apply(
        &mut self,
        solution: &mut P::Solution,
        eval: &mut P::Eval,
        problem: &P,
        input: In,
    ) -> Result<Self::Output, Self::Error>;

    fn then<U>(self, op: U) -> Then<Self, U>
    where
        Self: Operator<P, Output = ()> + Sized,
        U: Operator<P, Output = (), Error = <Self as Operator<P>>::Error>,
    {
        Then {
            first: self,
            second: op,
        }
    }

    fn pipe<U>(self, op: U) -> Pipe<Self, U>
    where
        Self: Sized,
        U: Operator<P, Self::Output, Error = Self::Error>,
    {
        Pipe { from: self, to: op }
    }

    fn ignore(self) -> Ignore<Self>
    where
        Self: Sized,
    {
        Ignore(self)
    }

    fn map<Out, F>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Output) -> Out,
    {
        Map { op: self, f }
    }

    fn map_err<Err, F>(self, f: F) -> MapErr<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Error) -> Err,
        Err: Error,
    {
        MapErr { op: self, f }
    }

    fn try_map<Out, F>(self, f: F) -> TryMap<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Output) -> Result<Out, Self::Error>,
    {
        TryMap { op: self, f }
    }

    fn once(self) -> Once<Self>
    where
        Self: Sized,
    {
        Once(Some(self))
    }

    fn accept_if<F>(self, cond: F) -> AcceptIf<Self, F>
    where
        Self: Sized,
        F: Accept<P>,
        P::Solution: Clone,
    {
        AcceptIf { op: self, cond }
    }

    fn repeat(self, times: usize) -> Repeat<Self>
    where
        Self: Operator<P, In, Output = In> + Sized,
    {
        Repeat { op: self, times }
    }

    fn repeat_until<F>(self, cond: F) -> RepeatUntil<Self, F>
    where
        Self: Operator<P, In, Output = In> + Sized,
        F: Stop<P>,
    {
        RepeatUntil { op: self, cond }
    }

    fn flatten(self) -> Flatten<Self>
    where
        Self: Sized,
        Self::Output: Operator<P, Error = Self::Error>,
    {
        Flatten(self)
    }

    fn flat_map<U, F>(self, f: F) -> FlatMap<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Output) -> U,
        U: Operator<P, Error = Self::Error>,
    {
        FlatMap { op: self, f }
    }

    fn unwrapped(self) -> Unwrapped<Self>
    where
        Self: Sized,
    {
        Unwrapped(self)
    }

    #[must_use]
    fn by_ref(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self
    }

    #[cfg(feature = "alloc")]
    #[must_use]
    fn boxed<'a>(self) -> Box<dyn Operator<P, In, Output = Self::Output, Error = Self::Error> + 'a>
    where
        Self: Sized + 'a,
    {
        Box::new(self)
    }
}

impl<T, P, In> Operator<P, In> for &mut T
where
    T: Operator<P, In> + ?Sized,
    P: Problem,
{
    type Output = T::Output;

    type Error = T::Error;

    fn apply(
        &mut self,
        solution: &mut P::Solution,
        eval: &mut P::Eval,
        problem: &P,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        T::apply(self, solution, eval, problem, input)
    }
}

#[cfg(feature = "alloc")]
impl<T, P, In> Operator<P, In> for Box<T>
where
    T: Operator<P, In> + ?Sized,
    P: Problem,
{
    type Output = T::Output;

    type Error = T::Error;

    fn apply(
        &mut self,
        solution: &mut P::Solution,
        eval: &mut P::Eval,
        problem: &P,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        T::apply(self, solution, eval, problem, input)
    }
}

#[cfg(feature = "either")]
impl<L, R, P, In> Operator<P, In> for either::Either<L, R>
where
    L: Operator<P, In>,
    R: Operator<P, In, Output = L::Output, Error = L::Error>,
    P: Problem,
{
    type Output = L::Output;

    type Error = L::Error;

    fn apply(
        &mut self,
        solution: &mut P::Solution,
        eval: &mut P::Eval,
        problem: &P,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        match self {
            Self::Left(left) => left.apply(solution, eval, problem, input),
            Self::Right(right) => right.apply(solution, eval, problem, input),
        }
    }
}

impl<P> Operator<P> for ()
where
    P: Problem,
{
    type Output = ();

    type Error = Infallible;

    fn apply(
        &mut self,
        _: &mut P::Solution,
        _: &mut P::Eval,
        _: &P,
        (): (),
    ) -> Result<Self::Output, Self::Error> {
        Ok(())
    }
}

impl<T, P, In> Operator<P, In> for Option<T>
where
    T: Operator<P, In>,
    P: Problem,
{
    type Output = Option<T::Output>;

    type Error = T::Error;

    fn apply(
        &mut self,
        solution: &mut P::Solution,
        eval: &mut P::Eval,
        problem: &P,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        self.as_mut()
            .map(|op| op.apply(solution, eval, problem, input))
            .transpose()
    }
}

pub fn from_fn<P, In, Out, Err, F>(f: F) -> FromFn<P, In, Out, Err, F>
where
    F: FnMut(&mut P::Solution, &mut P::Eval, &P, In) -> Result<Out, Err>,
    P: Problem,
    Err: Error,
{
    FromFn {
        f,
        marker: PhantomData,
    }
}

pub fn hint<T, P, In, Out, Err>(op: T) -> Hint<T, P, In, Out, Err>
where
    T: Operator<P, In, Output = Out, Error = Err>,
    P: Problem,
    Err: Error,
{
    Hint {
        op,
        marker: PhantomData,
    }
}

pub fn todo<P, In, Out, Err>() -> Todo<P, In, Out, Err>
where
    P: Problem,
    Err: Error,
{
    Todo(PhantomData)
}
