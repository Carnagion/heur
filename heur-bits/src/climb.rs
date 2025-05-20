use core::convert::Infallible;

use heur_core::{Problem, eval::Eval, op::Operator, solution::Individual};

use crate::Bits;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct FirstAscentBitClimb;

impl<P, S> Operator<P> for FirstAscentBitClimb
where
    P: Problem<Solution = Individual<S>>,
    S: Bits,
{
    type Output = ();

    type Error = Infallible;

    fn apply(
        &mut self,
        solution: &mut P::Solution,
        eval: &mut P::Eval,
        problem: &P,
        _input: (),
    ) -> Result<Self::Output, Self::Error> {
        let solution = &mut **solution;

        let objective = eval.eval(solution, problem);

        // NOTE: We iterate over indices here because it's not possible to iterate over the solution mutably
        //       while also passing it immutably to `eval` for evaluation. Also because `Bits` cannot expose
        //       an API for mutably iterating over bits.
        let next = (0..solution.len()).find_map(|idx| {
            let bit = solution.get(idx).unwrap(); // PANICS: We know that the index is valid
            solution.set(idx, !bit);

            let improved = (eval.eval(solution, problem) > objective).then_some(idx);
            solution.set(idx, bit).unwrap(); // PANICS: Same as above
            improved
        });

        if let Some(idx) = next {
            let bit = solution.get(idx).unwrap(); // PANICS: We know that the index is valid
            solution.set(idx, !bit);
        }

        Ok(())
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct SteepestAscentBitClimb;

impl<P, S> Operator<P> for SteepestAscentBitClimb
where
    P: Problem<Solution = Individual<S>>,
    S: Bits,
    P::Eval: Eval<P, Objective: Ord>,
{
    type Output = ();

    type Error = Infallible;

    fn apply(
        &mut self,
        solution: &mut P::Solution,
        eval: &mut P::Eval,
        problem: &P,
        _input: (),
    ) -> Result<Self::Output, Self::Error> {
        let solution = &mut **solution;

        // NOTE: See the note above regarding iteration over indices.
        let best = (0..solution.len()).max_by_key(|&idx| {
            let bit = solution.get(idx).unwrap(); // PANICS: We know that the index is valid
            solution.set(idx, !bit);

            let objective = eval.eval(solution, problem);
            solution.set(idx, bit).unwrap(); // PANICS: Same as above
            objective
        });

        if let Some(idx) = best {
            let bit = solution.get(idx).unwrap(); // PANICS: We know that the index is valid
            solution.set(idx, !bit);
        }

        Ok(())
    }
}
