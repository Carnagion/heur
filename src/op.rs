use std::{convert::Infallible, error::Error, marker::PhantomData};

use accept::Accept;

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

mod hint;
pub use hint::Hint;

mod todo;
pub use todo::Todo;

mod accept_if;
pub use accept_if::AcceptIf;

pub mod accept;

pub mod stop;

pub mod init;

pub mod mutate;

pub mod search;

// NOTE: We don't bound `E: Eval<S, P>` for a couple of reasons:
//       1. Some operators don't use the evaluation function.
//       2. Population-based operators use `Population<T>` as their solution, and having `E: Eval<S, P>` would mean that
//          it's an evaluation function over `Population<T>`. This doesn't really make sense, as we want to evaluate
//          individuals, not populations. We may get eg. the average or best individual in a population and use that for
//          decisions (eg. move acceptance), but that's not the same as evaluating a population as a whole.
// TODO: Add `#[diagnostic::on_unimplemented]` and more combinators
pub trait Operator<S, P, E, In = ()> {
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
    #[must_use]
    fn then<U>(self, op: U) -> Then<Self, U>
    where
        Self: Operator<S, P, E> + Sized,
        U: Operator<S, P, E, Error = <Self as Operator<S, P, E>>::Error>,
    {
        Then {
            first: self,
            second: op,
        }
    }

    #[inline]
    #[must_use]
    fn pipe<U>(self, to: U) -> Pipe<Self, U>
    where
        Self: Sized,
        U: Operator<S, P, E, Self::Output, Error = Self::Error>,
    {
        Pipe { from: self, to }
    }

    #[inline]
    #[must_use]
    fn ignore(self) -> Ignore<Self>
    where
        Self: Sized,
    {
        Ignore(self)
    }

    #[inline]
    #[must_use]
    fn map<Out, F>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Output) -> Out,
    {
        Map { op: self, f }
    }

    #[inline]
    #[must_use]
    fn map_err<Err, F>(self, f: F) -> MapErr<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Error) -> Err,
        Err: Error,
    {
        MapErr { op: self, f }
    }

    #[inline]
    #[must_use]
    fn try_map<Out, F>(self, f: F) -> TryMap<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Output) -> Result<Out, Self::Error>,
    {
        TryMap { op: self, f }
    }

    #[inline]
    #[must_use]
    fn once(self) -> Once<Self>
    where
        Self: Sized,
    {
        Once(Some(self))
    }

    #[inline]
    #[must_use]
    fn accept_if<F>(self, cond: F) -> AcceptIf<Self, F>
    where
        Self: Sized,
        F: Accept<S, P, E>,
        S: Clone,
    {
        AcceptIf { op: self, cond }
    }
}

impl<T, S, P, E, In> Operator<S, P, E, In> for &mut T
where
    T: Operator<S, P, E, In> + ?Sized,
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

impl<T, S, P, E, In> Operator<S, P, E, In> for Box<T>
where
    T: Operator<S, P, E, In> + ?Sized,
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
impl<L, R, S, P, E, In> Operator<S, P, E, In> for either::Either<L, R>
where
    L: Operator<S, P, E, In>,
    R: Operator<S, P, E, In, Output = L::Output, Error = L::Error>,
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

impl<S, P, E> Operator<S, P, E> for () {
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

impl<T, S, P, E, In> Operator<S, P, E, In> for Option<T>
where
    T: Operator<S, P, E, In>,
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
#[must_use]
pub fn hint<S, P, E, In, T>(op: T) -> Hint<T, S, P, E, In>
where
    T: Operator<S, P, E, In>,
{
    Hint {
        op,
        _marker: PhantomData,
    }
}

#[inline]
#[must_use]
pub fn todo<S, P, E, In, Out, Err>() -> Todo<S, P, E, In, Out, Err>
where
    Err: Error,
{
    Todo(PhantomData)
}
