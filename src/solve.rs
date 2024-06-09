use std::error::Error;

// NOTE: We don't bound `E: Eval<S, P>` for the same reasons as described in `Operator`.
// TODO: Add `#[diagnostic::on_unimplemented]`
pub trait Solve<S, P, E> {
    // TODO: Do we really need to bound on `Error`?
    type Error: Error;

    fn solve(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error>;
}
