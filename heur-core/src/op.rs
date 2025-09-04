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

    /// Apply one operator after another.
    ///
    /// Both operators must take no input and produce no output, i.e. `In` = `()` and [`Output`](Operator::Output) = `()`.
    /// Additionally, both operators must have the same [`Error`](Operator::Error) type. In other words, applying `a.then(b)`
    /// to a solution is equivalent to applying `a` and then applying `b` like so:
    /// ```
    /// # use heur_core::{op::Operator, solution::Solution, Problem};
    /// #
    /// # fn test<T, U, P, S>(mut a: T, mut b: U, solution: &mut S, eval: &mut P::Eval, problem: &P) -> Result<(), T::Error>
    /// # where
    /// #     T: Operator<P, S, Output = ()>,
    /// #     U: Operator<P, S, Output = (), Error = T::Error>,
    /// #     P: Problem,
    /// #     S: Solution<Individual = P::Individual>,
    /// # {
    /// a.apply(solution, eval, problem, ())?;
    /// b.apply(solution, eval, problem, ())?;
    /// Ok(())
    /// # }
    /// ```
    ///
    /// See [`pipe`](Operator::pipe) for a version of this combinator where the first operator's output is passed to the
    /// second operator as its input.
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

    /// Apply one operator after another, using the first operator's output as the input to the second.
    ///
    /// Both operators must have the same [`Error`](Operator::Error) type. Put simply, applying `a.pipe(b)` to a solution
    /// is equivalent to applying `a` and then applying `b`, passing the output of `a` to `b`:
    /// ```
    /// # use heur_core::{op::Operator, solution::Solution, Problem};
    /// #
    /// # fn test<T, U, P, S, In>(
    /// #     mut a: T,
    /// #     mut b: U,
    /// #     solution: &mut S,
    /// #     eval: &mut P::Eval,
    /// #     problem: &P,
    /// #     input: In,
    /// # ) -> Result<U::Output, T::Error>
    /// # where
    /// #     T: Operator<P, S, In>,
    /// #     U: Operator<P, S, T::Output, Error = T::Error>,
    /// #     P: Problem,
    /// #     S: Solution<Individual = P::Individual>,
    /// # {
    /// let intermediate = a.apply(solution, eval, problem, input)?;
    /// let output = b.apply(solution, eval, problem, intermediate)?;
    /// Ok(output)
    /// # }
    /// ```
    ///
    /// See [`then`](Operator::then) for a version of this combinator where both operators take no input and produce no output.
    fn pipe<U>(self, op: U) -> Pipe<Self, U>
    where
        Self: Sized,
        U: Operator<P, S, Self::Output, Error = Self::Error>,
    {
        Pipe { from: self, to: op }
    }

    /// Ignore the output produced by an operator, producing `()` instead.
    ///
    /// This is primarily useful to ignore the outputs of combinators such as [`accept_if`](Operator::accept_if) or
    /// [`once`](Operator::once), which may produce output types carrying no useful information, like `Option<()>`.
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

    /// Transforms the output of the operator to another value via a function.
    ///
    /// The [`Output`](Operator::Output) type of the resulting combinator will be the return type of the mapping function
    /// `f`; i.e. applying `a.map(f)` is equivalent to applying the operator `a` and then calling `f` on its output:
    /// ```
    /// # use heur_core::{op::Operator, solution::Solution, Problem};
    /// #
    /// # fn test<T, F, P, S, In, Out>(
    /// #     mut a: T,
    /// #     mut f: F,
    /// #     solution: &mut S,
    /// #     eval: &mut P::Eval,
    /// #     problem: &P,
    /// #     input: In,
    /// # ) -> Result<Out, T::Error>
    /// # where
    /// #     T: Operator<P, S, In>,
    /// #     F: FnMut(T::Output) -> Out,
    /// #     P: Problem,
    /// #     S: Solution<Individual = P::Individual>,
    /// # {
    /// let output = a.apply(solution, eval, problem, input)?;
    /// let mapped = f(output);
    /// Ok(mapped)
    /// # }
    /// ```
    ///
    /// See [`map_err`](Operator::map_err) for a version of this combinator that maps operator errors instead of outputs, and
    /// [`try_map`](Operator::try_map) for a version that supports a fallible mapping function.
    fn map<Out, F>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Output) -> Out,
    {
        Map { op: self, f }
    }

    /// Transforms any errors produced by the operator to error type via a function.
    ///
    /// The [`Error`](Operator::Error) type of the resulting combinator will be the return type of the mapping function
    /// `f`; i.e. applying `a.map_err(f)` is equivalent to applying the operator `a`, calling `f` on any error produced:
    /// ```
    /// # use core::error::Error;
    /// #
    /// # use heur_core::{op::Operator, solution::Solution, Problem};
    /// #
    /// # fn test<T, F, P, S, In, Err>(
    /// #     mut a: T,
    /// #     mut f: F,
    /// #     solution: &mut S,
    /// #     eval: &mut P::Eval,
    /// #     problem: &P,
    /// #     input: In,
    /// # ) -> Result<T::Output, Err>
    /// # where
    /// #     T: Operator<P, S, In>,
    /// #     F: FnMut(T::Error) -> Err,
    /// #     P: Problem,
    /// #     S: Solution<Individual = P::Individual>,
    /// #     Err: Error,
    /// # {
    /// match a.apply(solution, eval, problem, input) {
    ///     Ok(output) => Ok(output),
    ///     Err(err) => Err(f(err)),
    /// }
    /// # }
    /// ```
    ///
    /// See [`map`](Operator::map) for a version of this combinator that maps operator outputs instead of errors, and
    /// [`try_map`](Operator::try_map) for a version that supports a fallible output mapping function.
    fn map_err<Err, F>(self, f: F) -> MapErr<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Error) -> Err,
        Err: Error,
    {
        MapErr { op: self, f }
    }

    /// Transforms the output of the operator to another value via a fallible function.
    ///
    /// Like [`map`](Operator::map), the [`Output`](Operator::Output) type of the resulting combinator is decided by
    /// the mapping function `f`. However, here `f` is fallible --- it returns a [`Result`], where the value in [`Ok`]
    /// becomes the operator's new output, and any [`Err`] causes an early return.
    ///
    /// That is, applying `a.try_map(f)` is equivalent to applying the operator `a` and then calling `f` on its output
    /// like so:
    /// ```
    /// # use heur_core::{op::Operator, solution::Solution, Problem};
    /// #
    /// # fn test<T, F, P, S, In, Out>(
    /// #     mut a: T,
    /// #     mut f: F,
    /// #     solution: &mut S,
    /// #     eval: &mut P::Eval,
    /// #     problem: &P,
    /// #     input: In,
    /// # ) -> Result<Out, T::Error>
    /// # where
    /// #     T: Operator<P, S, In>,
    /// #     F: FnMut(T::Output) -> Result<Out, T::Error>,
    /// #     P: Problem,
    /// #     S: Solution<Individual = P::Individual>,
    /// # {
    /// let output = a.apply(solution, eval, problem, input)?;
    /// let mapped = f(output)?;
    /// Ok(mapped)
    /// # }
    /// ```
    ///
    /// See [`map`](Operator::map) for a version of this combinator that takes an infallible mapping function, and
    /// [`map_err`](Operator::map_err) for a version that maps operator errors instead of outputs.
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

    /// Repeatedly apply an operator for a given number of times.
    ///
    /// Each iteration's output is passed to the next iteration as its input, which requires that the operator's
    /// input and output types are the same (i.e. `In` = [`Output`](Operator::Output)). In other words, applying
    /// the operator `a.repeat(n)` is equivalent to applying `a` in a loop like so:
    /// ```
    /// # use heur_core::{op::Operator, solution::Solution, Problem};
    /// #
    /// # fn test<T, P, S, In>(
    /// #     mut a: T,
    /// #     n: usize,
    /// #     solution: &mut S,
    /// #     eval: &mut P::Eval,
    /// #     problem: &P,
    /// #     input: In,
    /// # ) -> Result<T::Output, T::Error>
    /// # where
    /// #     T: Operator<P, S, In, Output = In>,
    /// #     P: Problem,
    /// #     S: Solution<Individual = P::Individual>,
    /// # {
    /// let mut state = input;
    /// for _ in 0..n {
    ///     state = a.apply(solution, eval, problem, state)?;
    /// }
    /// Ok(state)
    /// # }
    /// ```
    ///
    /// See [`repeat_until`](Operator::repeat_until) for a version of this combinator that supports arbitrary
    /// stopping conditions other than the number of iterations.
    fn repeat(self, times: usize) -> Repeat<Self>
    where
        Self: Operator<P, S, In, Output = In> + Sized,
    {
        Repeat { op: self, times }
    }

    /// Repeatedly apply an operator until a stopping condition is fulfilled.
    ///
    /// Each iteration's output is passed to the next iteration as its input, which requires that the operator's
    /// input and output types are the same (i.e. `In` = [`Output`](Operator::Output)). In other words, applying
    /// `a.repeat_until(f)` is equivalent to applying `a` in a loop like so:
    /// ```
    /// # use heur_core::{op::{cond::stop::Stop, Operator}, solution::Solution, Problem};
    /// #
    /// # fn test<T, F, P, S, In>(
    /// #     mut a: T,
    /// #     mut f: F,
    /// #     solution: &mut S,
    /// #     eval: &mut P::Eval,
    /// #     problem: &P,
    /// #     input: In,
    /// # ) -> Result<T::Output, T::Error>
    /// # where
    /// #     T: Operator<P, S, In, Output = In>,
    /// #     F: Stop<P, S>,
    /// #     P: Problem,
    /// #     S: Solution<Individual = P::Individual>,
    /// # {
    /// let mut state = input;
    /// while !f.stop(solution, eval, problem) {
    ///     state = a.apply(solution, eval, problem, state)?;
    /// }
    /// Ok(state)
    /// # }
    /// ```
    ///
    /// See [`repeat`](Operator::repeat) for a simpler version of this combinator that only supports an exact
    /// number of iterations as a stopping condition.
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
