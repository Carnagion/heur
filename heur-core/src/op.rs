//! Composable heuristic operators.

use core::{convert::Infallible, error::Error, marker::PhantomData};

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

use cond::{accept::Accept, stop::Stop};

use crate::{Problem, solution::Solution};

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

mod passthrough;
pub use passthrough::Passthrough;

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
/// The core trait implemented by heuristic operators.
///
/// Operators for a given [problem](Problem) `P` can be applied to a solution `S` of [individuals](Problem::Individual)
/// along with an [evaluation function](Problem::Eval) and some input of type `In`. Applying an operator typically mutates
/// the solution, and produces either an [`Output`](Operator::Output) or an [`Error`](Operator::Error).
///
/// When chaining operators together (such as via [`then`](Operator::then), [`pipe`](Operator::pipe), [`repeat`](Operator::repeat),
/// or one of the many other available combinators), the output of one operator is passed as input to the next. If an operator does
/// not specify what input type it accepts, `In` defaults to `()`, i.e. no input.
///
/// # Composition
///
/// Much like Rust's [iterators](Iterator), operators can be composed or modified via various adapters (a.k.a. combinators) to
/// create operators capable of more complex behaviour. [`Operator`]'s full definition includes a number of combinators built
/// upon [`apply`](Operator::apply), and so you get them for free.
///
/// Common use cases of combinators include chaining two operators together ([`then`](Operator::then) or [`pipe`](Operator::pipe)),
/// applying an operator multiple times ([`repeat`](Operator::repeat) and [`repeat_until`](Operator::repeat_until)), changing the
/// output or error type of an operator ([`map`](Operator::map) or [`map_err`](Operator::map_err)), undoing changes to a solution
/// upon failure ([`accept_if`](Operator::accept_if)), and more. Refer to their documentation for more details.
///
/// # Laziness
///
/// Like [iterators](Iterator), operators are also *lazy* --- they do nothing until you call [`apply`](Operator::apply).
/// This means that, for example, chaining two operators with [`then`](Operator::then) does not actually apply both operators on the
/// spot, but creates a *new* operator that applies them both when its own implementation of [`apply`](Operator::apply) is called.
pub trait Operator<P, S, In = ()>
where
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    type Output;

    type Error: Error;

    fn apply(
        &mut self,
        solution: &mut S,
        eval: &mut P::Eval,
        problem: &P,
        input: In,
    ) -> Result<Self::Output, Self::Error>;

    fn then<U>(self, op: U) -> Then<Self, U>
    where
        Self: Operator<P, S, Output = ()> + Sized,
        U: Operator<P, S, Output = (), Error = <Self as Operator<P, S>>::Error>,
    {
        Then {
            first: self,
            second: op,
        }
    }

    fn pipe<U>(self, op: U) -> Pipe<Self, U>
    where
        Self: Sized,
        U: Operator<P, S, Self::Output, Error = Self::Error>,
    {
        Pipe { from: self, to: op }
    }

    fn ignore(self) -> Ignore<Self>
    where
        Self: Sized,
    {
        Ignore(self)
    }

    fn passthrough(self) -> Passthrough<Self>
    where
        Self: Sized,
    {
        Passthrough(self)
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
        F: Accept<P, S>,
        S: Clone,
    {
        AcceptIf { op: self, cond }
    }

    fn repeat(self, times: usize) -> Repeat<Self>
    where
        Self: Operator<P, S, In, Output = In> + Sized,
    {
        Repeat { op: self, times }
    }

    fn repeat_until<F>(self, cond: F) -> RepeatUntil<Self, F>
    where
        Self: Operator<P, S, In, Output = In> + Sized,
        F: Stop<P, S>,
    {
        RepeatUntil { op: self, cond }
    }

    fn flatten(self) -> Flatten<Self>
    where
        Self: Sized,
        Self::Output: Operator<P, S, Error = Self::Error>,
    {
        Flatten(self)
    }

    fn flat_map<U, F>(self, f: F) -> FlatMap<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Output) -> U,
        U: Operator<P, S, Error = Self::Error>,
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
    fn boxed<'a>(
        self,
    ) -> Box<dyn Operator<P, S, In, Output = Self::Output, Error = Self::Error> + 'a>
    where
        Self: Sized + 'a,
    {
        Box::new(self)
    }
}

impl<T, P, S, In> Operator<P, S, In> for &mut T
where
    T: Operator<P, S, In> + ?Sized,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    type Output = T::Output;

    type Error = T::Error;

    fn apply(
        &mut self,
        solution: &mut S,
        eval: &mut P::Eval,
        problem: &P,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        T::apply(self, solution, eval, problem, input)
    }
}

#[cfg(feature = "alloc")]
impl<T, P, S, In> Operator<P, S, In> for Box<T>
where
    T: Operator<P, S, In> + ?Sized,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    type Output = T::Output;

    type Error = T::Error;

    fn apply(
        &mut self,
        solution: &mut S,
        eval: &mut P::Eval,
        problem: &P,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        T::apply(self, solution, eval, problem, input)
    }
}

#[cfg(feature = "either")]
impl<L, R, P, S, In> Operator<P, S, In> for either::Either<L, R>
where
    L: Operator<P, S, In>,
    R: Operator<P, S, In, Output = L::Output, Error = L::Error>,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    type Output = L::Output;

    type Error = L::Error;

    fn apply(
        &mut self,
        solution: &mut S,
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

impl<P, S> Operator<P, S> for ()
where
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    type Output = ();

    type Error = Infallible;

    fn apply(
        &mut self,
        _: &mut S,
        _: &mut P::Eval,
        _: &P,
        (): (),
    ) -> Result<Self::Output, Self::Error> {
        Ok(())
    }
}

impl<T, P, S, In> Operator<P, S, In> for Option<T>
where
    T: Operator<P, S, In>,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    type Output = Option<T::Output>;

    type Error = T::Error;

    fn apply(
        &mut self,
        solution: &mut S,
        eval: &mut P::Eval,
        problem: &P,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        self.as_mut()
            .map(|op| op.apply(solution, eval, problem, input))
            .transpose()
    }
}

pub fn from_fn<P, S, In, Out, Err, F>(f: F) -> FromFn<P, S, In, Out, Err, F>
where
    F: FnMut(&mut S, &mut P::Eval, &P, In) -> Result<Out, Err>,
    P: Problem,
    S: Solution<Individual = P::Individual>,
    Err: Error,
{
    FromFn {
        f,
        marker: PhantomData,
    }
}

pub fn hint<T, P, S, In, Out, Err>(op: T) -> Hint<T, P, S, In, Out, Err>
where
    T: Operator<P, S, In, Output = Out, Error = Err>,
    P: Problem,
    S: Solution<Individual = P::Individual>,
    Err: Error,
{
    Hint {
        op,
        marker: PhantomData,
    }
}

pub fn todo<P, S, In, Out, Err>() -> Todo<P, S, In, Out, Err>
where
    P: Problem,
    S: Solution<Individual = P::Individual>,
    Err: Error,
{
    Todo(PhantomData)
}
