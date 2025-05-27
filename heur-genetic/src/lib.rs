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
    Problem,
    op::{Operator, cond::stop::Stop, init::Init},
    solution::{Population, Solution, reencode::Reencoded},
};

use insert::Insert;

use select::Select;

pub mod select;

pub mod combine;

pub mod insert;

type VecPopulation<P> = Vec<<<P as Problem>::Solution as Solution>::Individual>;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct GeneticAlgorithm<Ini, Sel, Com, Mut, Ins, Sto> {
    pub init: Ini,
    pub select: Sel,
    pub combine: Com,
    pub mutate: Mut,
    pub insert: Ins,
    pub stop: Sto,
}

impl<P, Ini, Sel, Com, Mut, Ins, Sto> Optimize<P> for GeneticAlgorithm<Ini, Sel, Com, Mut, Ins, Sto>
where
    P: Problem<Solution: Population>,
    Ini: Init<P>,
    Sel: Select<P, Error = Ini::Error>,
    Com: Combine<P, Error = Ini::Error>,
    Mut: Operator<Reencoded<P, VecPopulation<P>>, Output = (), Error = Ini::Error>,
    Ins: Insert<P, Error = Ini::Error>,
    Sto: Stop<P>,
{
    type Error = Ini::Error;

    fn optimize(&mut self, eval: &mut P::Eval, problem: &P) -> Result<P::Solution, Self::Error> {
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
