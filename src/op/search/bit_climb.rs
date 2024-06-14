use std::{convert::Infallible, ops::IndexMut};

use crate::{eval::Eval, op::Operator};

use super::Search;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct FirstAscentBitClimb;

impl FirstAscentBitClimb {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    fn ascend_first<S, P, E>(&mut self, len: usize, solution: &mut S, problem: &P, eval: &mut E)
    where
        S: IndexMut<usize, Output = bool>,
        E: Eval<S, P>,
    {
        let objective = eval.eval(solution, problem);

        // NOTE: We iterate over indices here because it's not possible to iterate over the solution mutably
        //       while also passing it immutably to `eval` for evaluation.
        let next = (0..len).find_map(|idx| {
            let bit = solution[idx];
            solution[idx] = !bit;
            let improved = (eval.eval(solution, problem) > objective).then_some(idx);
            solution[idx] = bit;
            improved
        });

        if let Some(idx) = next {
            solution[idx] = !solution[idx];
        }
    }
}

impl<P, E> Operator<Vec<bool>, P, E> for FirstAscentBitClimb
where
    E: Eval<Vec<bool>, P>,
{
    type Output = ();

    type Error = Infallible;

    #[inline]
    fn apply(
        &mut self,
        solution: &mut Vec<bool>,
        problem: &P,
        eval: &mut E,
        _input: (),
    ) -> Result<Self::Output, Self::Error> {
        self.ascend_first(solution.len(), solution, problem, eval);
        Ok(())
    }
}

impl<P, E, const N: usize> Operator<[bool; N], P, E> for FirstAscentBitClimb
where
    E: Eval<[bool; N], P>,
{
    type Output = ();

    type Error = Infallible;

    #[inline]
    fn apply(
        &mut self,
        solution: &mut [bool; N],
        problem: &P,
        eval: &mut E,
        _input: (),
    ) -> Result<Self::Output, Self::Error> {
        self.ascend_first(solution.len(), solution, problem, eval);
        Ok(())
    }
}

impl<S, P, E> Search<S, P, E> for FirstAscentBitClimb
where
    Self: Operator<S, P, E>,
{
    #[inline]
    fn search(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        self.apply(solution, problem, eval, ())?;
        Ok(())
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct SteepestAscentBitClimb;

impl SteepestAscentBitClimb {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    fn ascend_steepest<S, P, E>(&mut self, len: usize, solution: &mut S, problem: &P, eval: &mut E)
    where
        S: IndexMut<usize, Output = bool>,
        E: Eval<S, P>,
    {
        // NOTE: See the note above regarding iteration over indices.
        let best = (0..len).max_by_key(|&idx| {
            let bit = solution[idx];
            solution[idx] = !bit;
            let objective = eval.eval(solution, problem);
            solution[idx] = bit;
            objective
        });

        if let Some(idx) = best {
            solution[idx] = !solution[idx];
        }
    }
}

impl<P, E> Operator<Vec<bool>, P, E> for SteepestAscentBitClimb
where
    E: Eval<Vec<bool>, P>,
{
    type Output = ();

    type Error = Infallible;

    #[inline]
    fn apply(
        &mut self,
        solution: &mut Vec<bool>,
        problem: &P,
        eval: &mut E,
        _input: (),
    ) -> Result<Self::Output, Self::Error> {
        self.ascend_steepest(solution.len(), solution, problem, eval);
        Ok(())
    }
}

impl<P, E, const N: usize> Operator<[bool; N], P, E> for SteepestAscentBitClimb
where
    E: Eval<[bool; N], P>,
{
    type Output = ();

    type Error = Infallible;

    #[inline]
    fn apply(
        &mut self,
        solution: &mut [bool; N],
        problem: &P,
        eval: &mut E,
        _input: (),
    ) -> Result<Self::Output, Self::Error> {
        self.ascend_steepest(solution.len(), solution, problem, eval);
        Ok(())
    }
}

impl<S, P, E> Search<S, P, E> for SteepestAscentBitClimb
where
    Self: Operator<S, P, E>,
{
    #[inline]
    fn search(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        self.apply(solution, problem, eval, ())?;
        Ok(())
    }
}
