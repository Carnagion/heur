#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![deny(rust_2018_idioms)]
// #![warn(missing_docs)] // TODO: Enable once finished
#![deny(rustdoc::broken_intra_doc_links)]
#![no_std]

extern crate alloc;

use alloc::vec::Vec;

use combine::{Combine, on_combined};

use heur_core::{
    Optimize,
    eval::Eval,
    op::{Operator, init::Init, stop::Stop},
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

impl<P, S, E, Ini, Sel, Com, Mut, Ins, Sto> Optimize<P, S, E>
    for GeneticAlgorithm<Ini, Sel, Com, Mut, Ins, Sto>
where
    S: Population,
    E: Eval<P, S::Individual>,
    Ini: Init<P, S, E, Output = ()>,
    Sel: Select<P, S, E, Error = Ini::Error>,
    Com: Combine<P, S, E, Error = Ini::Error>,
    Mut: Operator<P, Vec<S::Individual>, E, Output = (), Error = Ini::Error>,
    Ins: Insert<P, S, E, Output = (), Error = Ini::Error>,
    Sto: Stop<P, S, E>,
{
    type Error = Ini::Error;

    fn optimize(&mut self, problem: &P, eval: &mut E) -> Result<S, Self::Error> {
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

        ga.optimize(problem, eval)
    }
}
