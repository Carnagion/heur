use super::Operator;

// NOTE: We don't bound `E: Eval<S, P>` for the same reasons as described in `Operator`.
// TODO: Add `#[diagnostic::on_unimplemented]`
pub trait Init<S, P, E>: Operator<S, P, E> {
    fn init(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error>;

    #[inline]
    fn init_into(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
    ) -> Result<(), Self::Error> {
        *solution = self.init(problem, eval)?;
        Ok(())
    }
}

impl<T, S, P, E> Init<S, P, E> for &mut T
where
    T: Init<S, P, E> + ?Sized,
{
    #[inline]
    fn init(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error> {
        T::init(self, problem, eval)
    }

    #[inline]
    fn init_into(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
    ) -> Result<(), Self::Error> {
        T::init_into(self, solution, problem, eval)
    }
}

impl<T, S, P, E> Init<S, P, E> for Box<T>
where
    T: Init<S, P, E> + ?Sized,
{
    #[inline]
    fn init(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error> {
        T::init(self, problem, eval)
    }

    #[inline]
    fn init_into(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
    ) -> Result<(), Self::Error> {
        T::init_into(self, solution, problem, eval)
    }
}

#[cfg(feature = "either")]
impl<L, R, S, P, E> Init<S, P, E> for either::Either<L, R>
where
    L: Init<S, P, E>,
    R: Init<S, P, E, Output = L::Output, Error = L::Error>,
{
    #[inline]
    fn init(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error> {
        match self {
            Self::Left(left) => left.init(problem, eval),
            Self::Right(right) => right.init(problem, eval),
        }
    }

    #[inline]
    fn init_into(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
    ) -> Result<(), Self::Error> {
        match self {
            Self::Left(left) => left.init_into(solution, problem, eval),
            Self::Right(right) => right.init_into(solution, problem, eval),
        }
    }
}
