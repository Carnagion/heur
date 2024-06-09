use std::error::Error;

// NOTE: We don't bound `E: Eval<S, P>` for a couple of reasons:
//       1. Some operators don't use the evaluation function.
//       2. Population-based operators use `Population<T>` as their solution, and having `E: Eval<S, P>` would mean that
//          it's an evaluation function over `Population<T>`. This doesn't really make sense, as we want to evaluate
//          individuals, not populations. We may get eg. the average or best individual in a population and use that for
//          decisions (eg. move acceptance), but that's not the same as evaluating a population as a whole.
// TODO: Add `#[diagnostic::on_unimplemented]` and more combinators
pub trait Operator<S, P, E, In = ()> {
    // TODO: Should this be set to a default `()` once defaults for associated types lands?
    //       See https://github.com/rust-lang/rust/issues/29661.
    type Output;

    // TODO: Do we really need to bound on `Error`?
    type Error: Error;

    fn apply(
        &mut self,
        solution: &mut S,
        problem: &P,
        eval: &mut E,
        input: In,
    ) -> Result<Self::Output, Self::Error>;
}
