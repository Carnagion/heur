use std::{
    cell::Cell,
    fmt::{self, Debug, Formatter},
    ops::{Deref, DerefMut},
};

// TODO: Manually impl common traits
pub struct Evaluated<S, O> {
    solution: S,
    // NOTE: In theory, we could have a design that doesn't use `Cell<T>` and instead stores a regular `Option<O>` instead,
    //       along with a function `Evaluated<S, O>` to either use the existing objective value or update it. However, this
    //       design would prevent `Evaluated<S, O>` from being a drop-in replacement for `S`, and would also be unnecessary
    //       for nearly all cases, as objective values are typically small `Copy` types and so can be used with `Cell<T>`.
    objective: Cell<Option<O>>,
}

impl<S, O> Evaluated<S, O> {
    pub fn new(solution: S) -> Self {
        Self {
            solution,
            objective: Cell::new(None),
        }
    }

    pub fn into_inner(this: Self) -> S {
        this.solution
    }

    pub fn objective(&self) -> Option<O>
    where
        O: Copy,
    {
        self.objective.get()
    }

    pub(crate) fn objective_or_eval<F>(&self, eval: F) -> O
    where
        F: FnOnce(&S) -> O,
        O: Copy,
    {
        match self.objective.get() {
            Some(objective) => objective,
            None => {
                let objective = eval(&self.solution);
                self.objective.set(Some(objective));
                objective
            },
        }
    }
}

impl<S, O> Deref for Evaluated<S, O> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.solution
    }
}

impl<S, O> DerefMut for Evaluated<S, O> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.objective.set(None);
        &mut self.solution
    }
}

impl<S, O> Clone for Evaluated<S, O>
where
    S: Clone,
    O: Copy,
{
    fn clone(&self) -> Self {
        Self {
            solution: self.solution.clone(),
            objective: self.objective.clone(),
        }
    }
}

impl<S, O> Debug for Evaluated<S, O>
where
    S: Debug,
    O: Debug + Copy,
{
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Evaluated")
            .field("solution", &self.solution)
            .field("objective", &self.objective)
            .finish()
    }
}
