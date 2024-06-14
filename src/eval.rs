mod from_fn;
pub use from_fn::FromFn;

// TODO: Add `#[diagnostic::on_unimplemented]` and more combinators
pub trait Eval<S, P> {
    type Objective: Ord;

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
    O: Ord,
{
    FromFn(f)
}
