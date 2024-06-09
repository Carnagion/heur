mod from_fn;
pub use from_fn::FromFn;

// NOTE: We need this extra trait because without it, a plain old bound of `PartialOrd` on `Eval::Objective`
//       results in `Eval` losing its object safety. However, putting a bound of `Objective` is perfectly OK
//       somehow. There is a potential fix for this behaviour - see https://github.com/rust-lang/rust/pull/122804.
//       However, until that fix lands, we are stuck with this workaround.
pub trait Objective: PartialOrd {}

impl<T> Objective for T where T: PartialOrd {}

// TODO: Add `#[diagnostic::on_unimplemented]` and more combinators
pub trait Eval<S, P> {
    // NOTE: See the note on `Objective` above.
    type Objective: Objective;

    fn eval(&mut self, solution: &S, problem: &P) -> Self::Objective;
}

impl<T, S, P> Eval<S, P> for &mut T
where
    T: Eval<S, P> + ?Sized,
{
    type Objective = T::Objective;

    #[inline]
    fn eval(&mut self, solution: &S, problem: &P) -> Self::Objective {
        T::eval(self, solution, problem)
    }
}

impl<T, S, P> Eval<S, P> for Box<T>
where
    T: Eval<S, P> + ?Sized,
{
    type Objective = T::Objective;

    #[inline]
    fn eval(&mut self, solution: &S, problem: &P) -> Self::Objective {
        T::eval(self, solution, problem)
    }
}

#[cfg(feature = "either")]
impl<L, R, S, P> Eval<S, P> for either::Either<L, R>
where
    L: Eval<S, P>,
    R: Eval<S, P, Objective = L::Objective>,
{
    type Objective = L::Objective;

    #[inline]
    fn eval(&mut self, solution: &S, problem: &P) -> Self::Objective {
        match self {
            Self::Left(left) => left.eval(solution, problem),
            Self::Right(right) => right.eval(solution, problem),
        }
    }
}

#[inline]
#[must_use]
pub fn from_fn<F, S, P, O>(f: F) -> FromFn<F>
where
    F: FnMut(&S, &P) -> O,
    O: Objective,
{
    FromFn(f)
}
