use std::{convert::Infallible, error::Error, marker::PhantomData};

use crate::{eval::Eval, solution::Solution};

use accept::Accept;

use stop::Stop;

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

pub mod mutate;

pub mod search;

pub mod population;

pub mod genetic;

pub mod accept;

pub mod stop;

// TODO: Add `#[diagnostic::on_unimplemented]` and more combinators
pub trait Operator<P, S, E, In = ()>
where
    S: Solution,
    E: Eval<P, S::Individual>,
{
    // TODO: Should this be set to a default `()` once defaults for associated types lands?
    //       See https://github.com/rust-lang/rust/issues/29661.
    type Output;

    // TODO: Do we really need to bound on `Error`?
    type Error: Error;

    fn apply(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
        input: In,
    ) -> Result<Self::Output, Self::Error>;

    #[inline]
    fn then<U>(self, op: U) -> Then<Self, U>
    where
        Self: Operator<P, S, E, Output = ()> + Sized,
        U: Operator<P, S, E, Output = (), Error = <Self as Operator<P, S, E>>::Error>,
    {
        Then {
            first: self,
            second: op,
        }
    }

    #[inline]
    fn pipe<U>(self, to: U) -> Pipe<Self, U>
    where
        Self: Sized,
        U: Operator<P, S, E, Self::Output, Error = Self::Error>,
    {
        Pipe { from: self, to }
    }

    #[inline]
    fn ignore(self) -> Ignore<Self>
    where
        Self: Sized,
    {
        Ignore(self)
    }

    #[inline]
    fn map<Out, F>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Output) -> Out,
    {
        Map { op: self, f }
    }

    #[inline]
    fn map_err<Err, F>(self, f: F) -> MapErr<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Error) -> Err,
        Err: Error,
    {
        MapErr { op: self, f }
    }

    #[inline]
    fn try_map<Out, F>(self, f: F) -> TryMap<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Output) -> Result<Out, Self::Error>,
    {
        TryMap { op: self, f }
    }

    #[inline]
    fn once(self) -> Once<Self>
    where
        Self: Sized,
    {
        Once(Some(self))
    }

    #[inline]
    fn accept_if<F>(self, cond: F) -> AcceptIf<Self, F>
    where
        Self: Sized,
        F: Accept<P, S, E>,
        S: Clone,
    {
        AcceptIf { op: self, cond }
    }

    #[inline]
    fn repeat(self, times: usize) -> Repeat<Self>
    where
        Self: Operator<P, S, E, In, Output = In> + Sized,
    {
        Repeat { op: self, times }
    }

    #[inline]
    fn repeat_until<F>(self, cond: F) -> RepeatUntil<Self, F>
    where
        Self: Operator<P, S, E, In, Output = In> + Sized,
        F: Stop<P, S, E>,
    {
        RepeatUntil { op: self, cond }
    }

    #[inline]
    fn flatten(self) -> Flatten<Self>
    where
        Self: Sized,
        Self::Output: Operator<P, S, E, Error = Self::Error>,
    {
        Flatten(self)
    }

    #[inline]
    fn flat_map<U, F>(self, f: F) -> FlatMap<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Output) -> U,
        U: Operator<P, S, E, Error = Self::Error>,
    {
        FlatMap { op: self, f }
    }

    #[inline]
    fn unwrapped(self) -> Unwrapped<Self>
    where
        Self: Sized,
    {
        Unwrapped(self)
    }

    #[inline]
    #[must_use]
    fn by_ref(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self
    }

    #[inline]
    #[must_use]
    fn boxed<'a>(
        self,
    ) -> Box<dyn Operator<P, S, E, In, Output = Self::Output, Error = Self::Error> + 'a>
    where
        Self: Sized + 'a,
    {
        Box::new(self)
    }
}

impl<T, P, S, E, In> Operator<P, S, E, In> for &mut T
where
    T: Operator<P, S, E, In> + ?Sized,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    type Output = T::Output;

    type Error = T::Error;

    #[inline]
    fn apply(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        T::apply(self, solution, problem, eval, input)
    }
}

impl<T, P, S, E, In> Operator<P, S, E, In> for Box<T>
where
    T: Operator<P, S, E, In> + ?Sized,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    type Output = T::Output;

    type Error = T::Error;

    #[inline]
    fn apply(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        T::apply(self, solution, problem, eval, input)
    }
}

#[cfg(feature = "either")]
impl<L, R, P, S, E, In> Operator<P, S, E, In> for either::Either<L, R>
where
    L: Operator<P, S, E, In>,
    R: Operator<P, S, E, In, Output = L::Output, Error = L::Error>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    type Output = L::Output;

    type Error = L::Error;

    #[inline]
    fn apply(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        match self {
            Self::Left(left) => left.apply(solution, problem, eval, input),
            Self::Right(right) => right.apply(solution, problem, eval, input),
        }
    }
}

impl<P, S, E> Operator<P, S, E> for ()
where
    S: Solution,
    E: Eval<P, S::Individual>,
{
    type Output = ();

    type Error = Infallible;

    #[inline]
    fn apply(
        &mut self,
        _solution: &mut S,
        _problem: &P,
        _eval: &mut E,
        _input: (),
    ) -> Result<Self::Output, Self::Error> {
        Ok(())
    }
}

impl<T, P, S, E, In> Operator<P, S, E, In> for Option<T>
where
    T: Operator<P, S, E, In>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    type Output = Option<T::Output>;

    type Error = T::Error;

    #[inline]
    fn apply(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        self.as_mut()
            .map(|op| op.apply(solution, problem, eval, input))
            .transpose()
    }
}

#[inline]
pub fn from_fn<P, S, E, In, Out, Err, F>(f: F) -> FromFn<F>
where
    F: FnMut(&mut S, &P, &mut E, In) -> Result<Out, Err>,
    S: Solution,
    E: Eval<P, S::Individual>,
    Err: Error,
{
    FromFn(f)
}

#[inline]
pub fn hint<P, S, E, In, T>(op: T) -> Hint<T, P, S, E, In>
where
    T: Operator<P, S, E, In>,
    S: Solution,
    E: Eval<P, S::Individual>,
{
    Hint {
        op,
        _marker: PhantomData,
    }
}

#[inline]
pub fn todo<P, S, E, In, Out, Err>() -> Todo<P, S, E, In, Out, Err>
where
    S: Solution,
    E: Eval<P, S::Individual>,
    Err: Error,
{
    Todo(PhantomData)
}
