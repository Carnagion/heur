use std::convert::Infallible;

use heur_core::{
    eval::Eval,
    op::{search::Search, Operator},
    solution::Individual,
};

use crate::Bits;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct FirstAscentBitClimb;

impl FirstAscentBitClimb {
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl<P, B, E> Operator<P, Individual<B>, E> for FirstAscentBitClimb
where
    B: Bits,
    E: Eval<P, B>,
{
    type Output = ();

    type Error = Infallible;

    fn apply(
        &mut self,
        solution: &mut Individual<B>,
        problem: &P,
        eval: &mut E,
        _input: (),
    ) -> Result<Self::Output, Self::Error> {
        self.search(solution, problem, eval)
    }
}

impl<P, B, E> Search<P, Individual<B>, E> for FirstAscentBitClimb
where
    B: Bits,
    E: Eval<P, B>,
{
    fn search(
        &mut self,
        solution: &mut Individual<B>,
        problem: &P,
        eval: &mut E,
    ) -> Result<(), Self::Error> {
        let solution = &mut **solution;

        let objective = eval.eval(solution, problem);

        // NOTE: We iterate over indices here because it's not possible to iterate over the solution mutably
        //       while also passing it immutably to `eval` for evaluation. Also because `Bits` doesn't expose
        //       an API for mutably iterating over bits.
        let next = (0..solution.len()).find_map(|idx| {
            let bit = solution.flip(idx).unwrap(); // PANICS: We know that the index is valid
            let improved = (eval.eval(solution, problem) > objective).then_some(idx);
            solution.set(idx, bit).unwrap(); // PANICS: Same as above
            improved
        });

        if let Some(idx) = next {
            solution.flip(idx).unwrap(); // PANICS: We know that the index is valid from above
        }

        Ok(())
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct SteepestAscentBitClimb;

impl SteepestAscentBitClimb {
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl<P, B, E> Operator<P, Individual<B>, E> for SteepestAscentBitClimb
where
    B: Bits,
    E: Eval<P, B>,
{
    type Output = ();

    type Error = Infallible;

    fn apply(
        &mut self,
        solution: &mut Individual<B>,
        problem: &P,
        eval: &mut E,
        _input: (),
    ) -> Result<Self::Output, Self::Error> {
        self.search(solution, problem, eval)
    }
}

impl<P, B, E> Search<P, Individual<B>, E> for SteepestAscentBitClimb
where
    B: Bits,
    E: Eval<P, B>,
{
    fn search(
        &mut self,
        solution: &mut Individual<B>,
        problem: &P,
        eval: &mut E,
    ) -> Result<(), Self::Error> {
        let solution = &mut **solution;

        // NOTE: See the note above regarding iteration over indices.
        let best = (0..solution.len()).max_by_key(|&idx| {
            let bit = solution.flip(idx).unwrap(); // PANICS: We know that the index is valid from above
            let objective = eval.eval(solution, problem);
            solution.set(idx, bit).unwrap(); // PANICS: Same as above
            objective
        });

        if let Some(idx) = best {
            solution.flip(idx).unwrap(); // PANICS: We know that the index is valid from above
        }

        Ok(())
    }
}
