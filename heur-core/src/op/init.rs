use core::marker::PhantomData;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

use crate::{
    Optimize,
    Problem,
    solution::{Individual, Population, Solution},
};

use super::Operator;

mod from_value;
pub use from_value::{FromIndividual, FromPopulation};

mod from_solver;
pub use from_solver::FromSolver;

// TODO: Add `#[diagnostic::on_unimplemented]`
/// A trait for operators that can construct initial solutions to a [`Problem`].
///
/// "Initialiser" operators implementing [`Init`] differ from other "non-initialiser" operators by virtue of being able to
/// construct initial solutions from nothing. Additionally, they must take no input and produce no output, i.e. `In` = `()`
/// and [`Output`](Operator::Output) = `()`.
///
/// # Composition
///
/// When initialiser operators are chained with certain combinators (e.g. [`then`](Operator::then)), it is possible to create
/// entirely combinator-based metaheuristic solvers that implement [`Optimize`] for a given [`Problem`]. For instance, a
/// simple Iterated Local Search (ILS) metaheuristic can be constructed and used as a solver like so:
/// ```
/// # use heur_core::{
/// #     op::{self, cond::{accept::Accept, stop::Stop}, init::Init, Operator},
/// #     solution::Solution,
/// #     Optimize,
/// #     Problem,
/// # };
/// #
/// # fn test<P, S, Ini, Mut, Loc, Acc, Stp>(
/// #     init: Ini,
/// #     mutate: Mut,
/// #     local_search: Loc,
/// #     acceptance: Acc,
/// #     stop: Stp,
/// #     eval: &mut P::Eval,
/// #     problem: &P,
/// # ) -> Result<S, Ini::Error>
/// # where
/// #     P: Problem,
/// #     S: Solution<Individual = P::Individual> + Clone,
/// #     Ini: Init<P, S>,
/// #     Mut: Operator<P, S, Output = (), Error = Ini::Error>,
/// #     Loc: Operator<P, S, Output = (), Error = Ini::Error>,
/// #     Acc: Accept<P, S>,
/// #     Stp: Stop<P, S>,
/// # {
/// let mut ils = init.then(
///     mutate
///         .then(local_search)
///         .accept_if(acceptance)
///         .ignore()
///         .repeat_until(stop),
/// );
///
/// let solution = ils.optimize(eval, problem)?;
/// # Ok(solution)
/// # }
/// ```
///
/// A combinator-based metaheuristic must begin with an operator that implements [`Init`] in order to act as a solver. See the
/// documentation of [`Optimize`] for more details.
///
/// Heur provides a number of basic initialiser operators, namely [`from_individual`], [`from_population`], and
/// [`from_solver`](from_solver()), which produce initial solutions by cloning an individual (a.k.a single-point) solution, by
/// cloning a population of solutions (a.k.a. multipoint), and by running an existing solver for the problem, respectively.
///
/// Certain operator combinators also implement this trait --- for example, [`then`](Operator::then) and [`pipe`](Operator::pipe)
/// both implement [`Init`] if the first of their inner operators do. Other notable examples include [`hint`](crate::op::hint()),
/// [`ignore`](Operator::ignore), and [`unwrapped`](Operator::unwrapped), and more.
pub trait Init<P, S>: Operator<P, S, Output = ()>
where
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    /// Construct an initial solution given some problem data and an evaluation function for the [`Problem`].
    ///
    /// # Note to Implementors
    ///
    /// Although not enforced, it is strongly encouraged that the implementation of [`init`](Init::init) behaves in
    /// accordance with [`init_into`](Init::init_into) and [`Operator::apply`]. That is, for a given initialiser
    /// operator `a`, the solutions produced in the following situations should be equivalent:
    /// - `a.init(eval, problem)`
    /// - `a.init_into(solution, eval, problem)`
    /// - `a.apply(solution, eval, problem, ())`
    ///
    /// The default implementation of [`init_into`](Init::init_into) merely delegates to [`init`](Init::init) and thus already
    /// ensures this partially.
    ///
    /// Furthermore, like [`Operator::apply`], avoid having panics inside your implementation of [`init`](Init::init). Panicking
    /// versions of initialiser operators can be created using [`Operator::unwrapped`] if necessary --- the operator returned by
    /// [`unwrapped`](Operator::unwrapped) implements [`Init`] if its inner operator does.
    fn init(&mut self, eval: &mut P::Eval, problem: &P) -> Result<S, Self::Error>;

    /// Overwrite an existing solution with an initial solution produced from the given problem data and an evaluation
    /// function.
    ///
    /// This method is provided mainly for performance reasons --- when re-initialising solutions in certain cases, it may
    /// be possible to re-use resources such as a heap allocation. Implementors should override this method where possible,
    /// as the default implementation simply delegates to [`init`](Init::init).
    fn init_into(
        &mut self,
        solution: &mut S,
        eval: &mut P::Eval,
        problem: &P,
    ) -> Result<(), Self::Error> {
        *solution = self.init(eval, problem)?;
        Ok(())
    }
}

impl<T, P, S> Init<P, S> for &mut T
where
    T: Init<P, S> + ?Sized,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    fn init(&mut self, eval: &mut P::Eval, problem: &P) -> Result<S, Self::Error> {
        T::init(self, eval, problem)
    }

    fn init_into(
        &mut self,
        solution: &mut S,
        eval: &mut P::Eval,
        problem: &P,
    ) -> Result<(), Self::Error> {
        T::init_into(self, solution, eval, problem)
    }
}

#[cfg(feature = "alloc")]
impl<T, P, S> Init<P, S> for Box<T>
where
    T: Init<P, S> + ?Sized,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    fn init(&mut self, eval: &mut P::Eval, problem: &P) -> Result<S, Self::Error> {
        T::init(self, eval, problem)
    }

    fn init_into(
        &mut self,
        solution: &mut S,
        eval: &mut P::Eval,
        problem: &P,
    ) -> Result<(), Self::Error> {
        T::init_into(self, solution, eval, problem)
    }
}

#[cfg(feature = "either")]
impl<L, R, P, S> Init<P, S> for either::Either<L, R>
where
    L: Init<P, S>,
    R: Init<P, S, Error = L::Error>,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    fn init(&mut self, eval: &mut P::Eval, problem: &P) -> Result<S, Self::Error> {
        match self {
            Self::Left(left) => left.init(eval, problem),
            Self::Right(right) => right.init(eval, problem),
        }
    }

    fn init_into(
        &mut self,
        solution: &mut S,
        eval: &mut P::Eval,
        problem: &P,
    ) -> Result<(), Self::Error> {
        match self {
            Self::Left(left) => left.init_into(solution, eval, problem),
            Self::Right(right) => right.init_into(solution, eval, problem),
        }
    }
}

pub fn from_individual<P, S>(solution: S) -> FromIndividual<P, S>
where
    P: Problem<Individual = S>,
    S: Clone,
{
    FromIndividual {
        solution: Individual(solution),
        marker: PhantomData,
    }
}

pub fn from_population<P, S>(population: S) -> FromPopulation<P, S>
where
    P: Problem,
    S: Population<Individual = P::Individual> + Clone,
{
    FromPopulation {
        population,
        marker: PhantomData,
    }
}

pub fn from_solver<P, S, T>(solver: T) -> FromSolver<P, S, T>
where
    T: Optimize<P, S>,
    P: Problem,
    S: Solution<Individual = P::Individual>,
{
    FromSolver {
        solver,
        marker: PhantomData,
    }
}
