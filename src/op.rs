use std::{convert::Infallible, error::Error, marker::PhantomData};

use crate::{eval::Eval, solution::Solution};

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

// TODO: Add `#[diagnostic::on_unimplemented]` and more combinators
pub trait Operator<P, S, E, In = ()>
where
    // NOTE: The solution type `S` does not necessarily have to be `Sized`, since we could apply operators in isolation to
    //       already-initialised solutions. This allows eg. applying an operator to `[T]` as a solution type, where the
    //       `&mut [T]` already exists and does not have to be initialised by an initialisation operator.
    S: Solution + ?Sized,
    E: Eval<P, S::Individual>,
{
    // TODO: Should this be set to a default `()` once defaults for associated types lands?
    //       See https://github.com/rust-lang/rust/issues/29661.
    type Output;

    // TODO: Do we really need to bound on `Error`?
    type Error: Error;

    fn apply(
        &mut self,
        problem: &P,
        solution: &mut S,
        eval: &mut E,
        input: In,
    ) -> Result<Self::Output, Self::Error>;

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
    S: Solution + ?Sized,
    E: Eval<P, S::Individual>,
{
    type Output = T::Output;

    type Error = T::Error;

    #[inline]
    fn apply(
        &mut self,
        problem: &P,
        solution: &mut S,
        eval: &mut E,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        T::apply(self, problem, solution, eval, input)
    }
}

impl<T, P, S, E, In> Operator<P, S, E, In> for Box<T>
where
    T: Operator<P, S, E, In> + ?Sized,
    S: Solution + ?Sized,
    E: Eval<P, S::Individual>,
{
    type Output = T::Output;

    type Error = T::Error;

    #[inline]
    fn apply(
        &mut self,
        problem: &P,
        solution: &mut S,
        eval: &mut E,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        T::apply(self, problem, solution, eval, input)
    }
}

#[cfg(feature = "either")]
impl<L, R, P, S, E, In> Operator<P, S, E, In> for either::Either<L, R>
where
    L: Operator<P, S, E, In>,
    R: Operator<P, S, E, In, Output = L::Output, Error = L::Error>,
    S: Solution + ?Sized,
    E: Eval<P, S::Individual>,
{
    type Output = L::Output;

    type Error = L::Error;

    #[inline]
    fn apply(
        &mut self,
        problem: &P,
        solution: &mut S,
        eval: &mut E,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        match self {
            Self::Left(left) => left.apply(problem, solution, eval, input),
            Self::Right(right) => right.apply(problem, solution, eval, input),
        }
    }
}

impl<P, S, E> Operator<P, S, E> for ()
where
    S: Solution + ?Sized,
    E: Eval<P, S::Individual>,
{
    type Output = ();

    type Error = Infallible;

    #[inline]
    fn apply(
        &mut self,
        _problem: &P,
        _solution: &mut S,
        _eval: &mut E,
        _input: (),
    ) -> Result<Self::Output, Self::Error> {
        Ok(())
    }
}

impl<T, P, S, E, In> Operator<P, S, E, In> for Option<T>
where
    T: Operator<P, S, E, In>,
    S: Solution + ?Sized,
    E: Eval<P, S::Individual>,
{
    type Output = Option<T::Output>;

    type Error = T::Error;

    #[inline]
    fn apply(
        &mut self,
        problem: &P,
        solution: &mut S,
        eval: &mut E,
        input: In,
    ) -> Result<Self::Output, Self::Error> {
        self.as_mut()
            .map(|op| op.apply(problem, solution, eval, input))
            .transpose()
    }
}

#[inline]
pub fn from_fn<P, S, E, In, Out, Err, F>(f: F) -> FromFn<F>
where
    F: FnMut(&P, &mut S, &mut E, In) -> Result<Out, Err>,
    S: Solution + ?Sized,
    E: Eval<P, S::Individual>,
    Err: Error,
{
    FromFn(f)
}

#[inline]
pub fn hint<P, S, E, In, T>(op: T) -> Hint<T, P, S, E, In>
where
    T: Operator<P, S, E, In>,
    S: Solution + ?Sized,
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
    S: Solution + ?Sized,
    E: Eval<P, S::Individual>,
    Err: Error,
{
    Todo(PhantomData)
}
