use crate::{
    eval::Eval,
    op::{mutate::Mutate, search::Search, Operator},
    solution::{Individual, Population},
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[must_use]
pub struct ForEach<T>(pub(super) T);

impl<T, P, S, E, In> Operator<P, S, E, In> for ForEach<T>
where
    T: Operator<P, Individual<S::Individual>, E, In, Output = In>,
    S: Population + ?Sized,
    // NOTE: This bound is currently satisfied by most population types except for `im::Vector<T>`. This seems to be an
    //       oversight on `im`'s part and can be easily fixed.
    for<'a> &'a mut S: IntoIterator<Item = &'a mut S::Individual>,
    E: Eval<P, S::Individual>,
{
    type Output = In;

    type Error = T::Error;

    #[inline]
    fn apply(
        &mut self,
        problem: &P,
        population: &mut S,
        eval: &mut E,
        mut input: In,
    ) -> Result<Self::Output, Self::Error> {
        for solution in population {
            let solution = Individual::from_mut(solution);
            input = self.0.apply(problem, solution, eval, input)?;
        }
        Ok(input)
    }
}

impl<T, P, S, E> Mutate<P, S, E> for ForEach<T>
where
    T: Mutate<P, Individual<S::Individual>, E, Output = ()>,
    S: Population + ?Sized,
    for<'a> &'a mut S: IntoIterator<Item = &'a mut S::Individual>,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn mutate(&mut self, problem: &P, population: &mut S, eval: &mut E) -> Result<(), Self::Error> {
        for solution in population {
            let solution = Individual::from_mut(solution);
            self.0.mutate(problem, solution, eval)?;
        }
        Ok(())
    }
}

impl<T, P, S, E> Search<P, S, E> for ForEach<T>
where
    T: Search<P, Individual<S::Individual>, E, Output = ()>,
    S: Population + ?Sized,
    for<'a> &'a mut S: IntoIterator<Item = &'a mut S::Individual>,
    E: Eval<P, S::Individual>,
{
    #[inline]
    fn search(&mut self, problem: &P, population: &mut S, eval: &mut E) -> Result<(), Self::Error> {
        for solution in population {
            let solution = Individual::from_mut(solution);
            self.0.search(problem, solution, eval)?;
        }
        Ok(())
    }
}
