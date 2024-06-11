use std::{convert::Infallible, ops::IndexMut};

use crate::{eval::Eval, op::Operator};

use super::Search;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct FirstDescentBitClimb;

impl FirstDescentBitClimb {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    fn descend_first<S, P, E>(&mut self, len: usize, solution: &mut S, problem: &P, eval: &mut E)
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

impl<P, E> Operator<Vec<bool>, P, E> for FirstDescentBitClimb
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
        self.descend_first(solution.len(), solution, problem, eval);
        Ok(())
    }
}

impl<P, E, const N: usize> Operator<[bool; N], P, E> for FirstDescentBitClimb
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
        self.descend_first(solution.len(), solution, problem, eval);
        Ok(())
    }
}

impl<S, P, E> Search<S, P, E> for FirstDescentBitClimb
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
pub struct SteepestDescentBitClimb;

impl SteepestDescentBitClimb {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    fn descend_steepest<S, P, E>(&mut self, len: usize, solution: &mut S, problem: &P, eval: &mut E)
    where
        S: IndexMut<usize, Output = bool>,
        E: Eval<S, P>,
    {
        // NOTE: This could theoretically be done using a `min_by_key`, except `E::Objective` does not necessarily
        //       impl `Ord` (because we support `f32` and `f64` as objective types). Also see the note above (in
        //       first descent) regarding iteration over indices.
        let mut best = None;
        for idx in 0..len {
            let bit = solution[idx];
            solution[idx] = !bit;
            let objective = eval.eval(solution, problem);
            match &best {
                None => best = Some((objective, idx)),
                Some((best_objective, _)) if &objective > best_objective => {
                    best = Some((objective, idx));
                },
                Some(_) => {},
            }
            solution[idx] = bit;
        }

        if let Some((_, best_idx)) = best {
            solution[best_idx] = !solution[best_idx];
        }
    }
}

impl<P, E> Operator<Vec<bool>, P, E> for SteepestDescentBitClimb
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
        self.descend_steepest(solution.len(), solution, problem, eval);
        Ok(())
    }
}

impl<P, E, const N: usize> Operator<[bool; N], P, E> for SteepestDescentBitClimb
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
        self.descend_steepest(solution.len(), solution, problem, eval);
        Ok(())
    }
}

impl<S, P, E> Search<S, P, E> for SteepestDescentBitClimb
where
    Self: Operator<S, P, E>,
{
    #[inline]
    fn search(&mut self, solution: &mut S, problem: &P, eval: &mut E) -> Result<(), Self::Error> {
        self.apply(solution, problem, eval, ())?;
        Ok(())
    }
}
