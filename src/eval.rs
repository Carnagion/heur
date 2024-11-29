mod cached;
pub use cached::Cached;

mod from_fn;
pub use from_fn::FromFn;

// NOTE: We could have written `S: Solution` and accepted a `&S::Individual` in `eval`, but this would needlessly worsen
//       type inference. It also makes more sense to tie `Eval<P, S>` to the solution (individual) being evaluated directly,
//       rather than the container type (`Individual<T>` or some population type). Furthermore, that would require us to impl
//       `Eval` repeatedly for each different population type, which is just stupid.
// TODO: Add `#[diagnostic::on_unimplemented]` and more combinators
pub trait Eval<P, S> {
    // NOTE: In theory, having `Objective: PartialOrd` would be "enough" and allow using types like `f32` or `f64` as
    //       objective values. However, many operators rely on a total order existing in the objective value type. Plus, it
    //       is often incorrect to use `PartialOrd` in this manner - eg. it would be nonsensical to have an objective value
    //       of `NaN`. As such, we restrict `Objective` to `Ord` types only.
    type Objective: Ord;

    #[must_use]
    fn eval(&mut self, solution: &S, problem: &P) -> Self::Objective;

    fn cached(self) -> Cached<Self>
    where
        Self: Sized,
        Self::Objective: Copy,
    {
        Cached(self)
    }
}

impl<T, P, S> Eval<P, S> for &mut T
where
    T: Eval<P, S> + ?Sized,
{
    type Objective = T::Objective;

    fn eval(&mut self, solution: &S, problem: &P) -> Self::Objective {
        T::eval(self, solution, problem)
    }
}

impl<T, P, S> Eval<P, S> for Box<T>
where
    T: Eval<P, S> + ?Sized,
{
    type Objective = T::Objective;

    fn eval(&mut self, solution: &S, problem: &P) -> Self::Objective {
        T::eval(self, solution, problem)
    }
}

#[cfg(feature = "either")]
impl<L, R, P, S> Eval<P, S> for either::Either<L, R>
where
    L: Eval<P, S>,
    R: Eval<P, S, Objective = L::Objective>,
{
    type Objective = L::Objective;

    fn eval(&mut self, solution: &S, problem: &P) -> Self::Objective {
        match self {
            Self::Left(left) => left.eval(solution, problem),
            Self::Right(right) => right.eval(solution, problem),
        }
    }
}

pub fn from_fn<F, P, S, O>(f: F) -> FromFn<F>
where
    F: FnMut(&S, &P) -> O,
    O: Ord,
{
    FromFn(f)
}
