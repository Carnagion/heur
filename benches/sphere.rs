use std::{array, iter};

use ordered_float::NotNan;

use rand::{Rng, distr::Bernoulli};

fn main() {
    divan::main();
}

struct Sphere {
    dim: usize,
}

type Solution = Vec<f64>;

fn cost(solution: &Solution, _sphere: &Sphere) -> NotNan<f64> {
    let objective = solution.iter().map(|x| x.powi(2)).sum::<f64>();
    objective.try_into().unwrap()
}

fn init_random<R>(sphere: &Sphere, rng: &mut R) -> Solution
where
    R: Rng,
{
    iter::from_fn(|| Some(rng.random_range(-1.0..=1.0)))
        .take(sphere.dim)
        .collect()
}

const DIMS: &[usize] = &[2, 3, 4, 5, 10];

const N: usize = 10;

const TOUR: usize = 5;

const ITERS: usize = 10000;

const PC: f64 = 0.5;

use heur::{
    eval::{self, Eval},
    genetic::{
        combine::{UniformCrossover, on_combined},
        insert::ElitistInserter,
        select::TournamentSelector,
    },
    op::{self, Operator, init, population, stop::Iterations},
    solution::{Individual, Solve},
};

#[divan::bench(args = DIMS)]
fn heur_ga(dim: usize) -> f64 {
    let sphere = Sphere { dim };

    let mut eval = eval::from_fn(cost);

    let mut rng = rand::rng();

    let population: [_; N] = array::from_fn(|_| init_random(&sphere, &mut rng));
    let init = init::from_population(population);
    let select = TournamentSelector::new(TOUR, N, rng.clone());
    let combine = UniformCrossover::new(Bernoulli::new(PC).unwrap(), rng.clone());
    let mutate = op::from_fn(
        |solution: &mut Individual<Solution>, _sphere, _eval, _input| {
            for x in &mut **solution {
                let min = (*x - 0.1).max(-1.0);
                let max = (*x + 0.1).min(1.0);
                *x = rng.random_range(min..=max);
            }
            Ok(())
        },
    );
    let insert = ElitistInserter::new();
    let stop = Iterations::new(ITERS);

    let mut ga = op::hint(init).then(
        op::hint(select)
            .unwrapped()
            .pipe(op::hint(combine).unwrapped())
            .pipe(on_combined(population::for_each(mutate)))
            .pipe(insert)
            .ignore()
            .repeat_until(stop),
    );

    let population: [_; N] = ga.solve(&sphere, &mut eval).unwrap();

    let best_objective = population
        .iter()
        .map(|solution| eval.eval(solution, &sphere))
        .max()
        .unwrap();
    best_objective.into()
}

use std::ops::Range;

use mahf::{
    Problem,
    SingleObjective,
    conditions::LessThanN,
    heuristics::ga::{self, RealProblemParameters},
    problems::{LimitedVectorProblem, Sequential, VectorProblem, evaluate::ObjectiveFunction},
};

impl Problem for Sphere {
    type Encoding = Solution;

    type Objective = SingleObjective;

    fn name(&self) -> &str {
        "sphere"
    }
}

impl VectorProblem for Sphere {
    type Element = f64;

    fn dimension(&self) -> usize {
        self.dim
    }
}

impl LimitedVectorProblem for Sphere {
    fn domain(&self) -> Vec<Range<Self::Element>> {
        iter::repeat(-1.0..1.0).take(self.dim).collect()
    }
}

impl ObjectiveFunction for Sphere {
    fn objective(&self, solution: &Self::Encoding) -> Self::Objective {
        let objective = f64::from(cost(solution, self));
        objective.try_into().unwrap()
    }
}

#[divan::bench(args = DIMS)]
fn mahf_ga(dim: usize) -> f64 {
    let sphere = Sphere { dim };

    let ga = ga::real_ga(
        RealProblemParameters {
            population_size: N as u32,
            tournament_size: TOUR as u32,
            pm: 1.0,
            deviation: 0.1,
            pc: PC,
        },
        LessThanN::iterations(ITERS as u32),
    )
    .unwrap();

    let state = ga.optimize(&sphere, Sequential::new()).unwrap();
    let populations = state.populations();
    let population = populations.current();

    let best_objective = population
        .iter()
        .map(mahf::Individual::objective)
        .max()
        .unwrap();
    best_objective.value()
}
