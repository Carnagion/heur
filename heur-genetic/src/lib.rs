#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![deny(rust_2018_idioms)]
// #![warn(missing_docs)] // TODO: Enable once finished
#![deny(rustdoc::broken_intra_doc_links)]
#![allow(clippy::uninlined_format_args)] // TODO: Remove if this lint is pushed back into pedantic
#![no_std]

extern crate alloc;

use alloc::vec::Vec;

use combine::{Combine, on_combined};

use heur_core::{
    Optimize,
    Problem,
    op::{Operator, cond::stop::Stop, init::Init},
    solution::Population,
};

use insert::Insert;

use select::Select;

pub mod select;

pub mod combine;

pub mod insert;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct GeneticAlgorithm<Ini, Sel, Com, Mut, Ins, Sto> {
    pub init: Ini,
    pub select: Sel,
    pub combine: Com,
    pub mutate: Mut,
    pub insert: Ins,
    pub stop: Sto,
}

impl<P, S, Ini, Sel, Com, Mut, Ins, Sto> Optimize<P, S>
    for GeneticAlgorithm<Ini, Sel, Com, Mut, Ins, Sto>
where
    P: Problem,
    S: Population<Individual = P::Individual>,
    Ini: Init<P, S>,
    Sel: Select<P, S, Error = Ini::Error>,
    Com: Combine<P, S, Error = Ini::Error>,
    Mut: Operator<P, Vec<P::Individual>, Output = (), Error = Ini::Error>,
    Ins: Insert<P, S, Error = Ini::Error>,
    Sto: Stop<P, S>,
{
    type Error = Ini::Error;

    fn optimize(&mut self, eval: &mut P::Eval, problem: &P) -> Result<S, Self::Error> {
        let init = self.init.by_ref();
        let select = self.select.by_ref();
        let combine = self.combine.by_ref();
        let mutate = self.mutate.by_ref();
        let insert = self.insert.by_ref();

        let mut ga = init.then(
            select
                .pipe(combine)
                .pipe(on_combined(mutate))
                .pipe(insert)
                .repeat_until(&mut self.stop),
        );

        ga.optimize(eval, problem)
    }
}
