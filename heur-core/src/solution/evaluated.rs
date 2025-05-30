use core::{
    cell::Cell,
    fmt::{self, Debug, Formatter},
    ops::{Deref, DerefMut},
};

#[must_use]
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

impl<S, O> Eq for Evaluated<S, O>
where
    S: Eq,
    O: Eq + Copy,
{
}

impl<S, O> PartialEq for Evaluated<S, O>
where
    S: PartialEq,
    O: PartialEq + Copy,
{
    fn eq(&self, other: &Self) -> bool {
        self.solution == other.solution && self.objective == other.objective
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

impl<S, O> AsRef<S> for Evaluated<S, O> {
    fn as_ref(&self) -> &S {
        self
    }
}

impl<S, O> AsMut<S> for Evaluated<S, O> {
    fn as_mut(&mut self) -> &mut S {
        self
    }
}

impl<S, O> IntoIterator for Evaluated<S, O>
where
    S: IntoIterator,
{
    type Item = S::Item;

    type IntoIter = S::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        Self::into_inner(self).into_iter()
    }
}

impl<'a, S, O> IntoIterator for &'a Evaluated<S, O>
where
    &'a S: IntoIterator,
{
    type Item = <&'a S as IntoIterator>::Item;

    type IntoIter = <&'a S as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        // NOTE: While we don't need to deref here, it looks consistent with the impl for `&mut Evaluated<S, O>`.
        let solution = &**self;
        solution.into_iter()
    }
}

impl<'a, S, O> IntoIterator for &'a mut Evaluated<S, O>
where
    &'a mut S: IntoIterator,
{
    type Item = <&'a mut S as IntoIterator>::Item;

    type IntoIter = <&'a mut S as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        // NOTE: We need to deref here so that the cached objective value can be reset to `None`.
        let solution = &mut **self;
        solution.into_iter()
    }
}
