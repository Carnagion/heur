use std::array;

use heur::{
    eval::{self, Eval},
    genetic::{
        combine::{on_combined, UniformCrossover},
        insert::ElitistInserter,
        select::ElitistSelector,
    },
    op::{self, init, population, stop::Iterations, Operator},
    solution::{Individual, Solve},
};

use rand::{distributions::Bernoulli, Rng};

fn main() {
    // Create an N-queens problem instance. In this case, N = 8.
    let problem = Problem { n_queens: 8 };

    // Solve the problem instance using a genetic algorithm.
    ga(&problem);
}

// This represents the problem data we are given while solving.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Problem {
    n_queens: usize,
}

// Each queen is on a different spot on the chessboard. The x component represents its file (column) while the y
// component represents its rank (row).
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

// We will use a vector of positions to encode individual solutions to the N-queens problem. Each element in the vector
// represents the position of a different queen.
type Solution = Vec<Pos>;

// An objective function that calculates the cost, aka objective value, of a given solution (`Vec<Pos>`) to an N-queens problem
// instance (`Problem`).
fn cost(positions: &Solution, _problem: &Problem) -> isize {
    let collisions = positions
        .iter()
        .enumerate()
        .flat_map(|lhs| positions.iter().enumerate().map(move |rhs| (lhs, rhs)))
        .filter_map(|((idx_lhs, lhs), (idx_rhs, rhs))| (idx_lhs != idx_rhs).then_some((lhs, rhs)))
        .filter(|(lhs, rhs)| {
            lhs.x == rhs.x
                || lhs.y == rhs.y
                || lhs.x + lhs.y == rhs.x + rhs.y
                || lhs.x - lhs.y == rhs.x - rhs.y
        })
        .count();

    // Since `heur` assumes that the objective is being maximised, we negate the number of collisions. This way, the
    // highest possible objective value is 0, with lower objective values indicating worse solutions.
    -(collisions as isize)
}

// A helper function to generate a random (potentially infeasible) solution to an N-queens problem instance. This is
// used later when creating multiple individuals for the genetic algorithm to modify.
fn init_random<R>(problem: &Problem, rng: &mut R) -> Solution
where
    R: Rng,
{
    let n_rows = problem.n_queens;
    let n_cols = problem.n_queens;
    (0..n_rows)
        .map(|y| Pos {
            x: rng.gen_range(0..n_cols),
            y,
        })
        .collect()
}

fn ga(problem: &Problem) {
    // Create an objective function that can be given to the metaheuristic. This example uses `eval::from_fn` to wrap up
    // the objective function above, but you could create a custom type and impl `Eval` for it manually like so:
    //
    // ```rs
    // struct Cost;
    //
    // impl Eval<Problem, Vec<Pos>> for Cost {
    //     type Objective = isize;
    //
    //     fn eval(&mut self, solution: &Vec<Pos>, problem: &Problem) -> isize { ... }
    // }
    // ```
    let mut eval = eval::from_fn(cost);

    let mut rng = rand::thread_rng();

    // Define the various operators we will be using for the genetic algorithm. We initialise a population of 100 solutions,
    // with each individual being initialised randomly by `init_random`.
    //
    // We use elitist selection with a selection size of 50, uniform crossover with a 0.5 (50%) probability of swapping each
    // element (position) between pairs of selected individuals, and elitist insertion to replace the worst individuals in the
    // old population with the newly combined and mutated individuals. Mutation is done by randomly modifying all x-positions
    // of each element in the individuals produced by crossover.
    //
    // We stop the algorithm when we get to 10000 iterations.
    let population: [Solution; 100] = array::from_fn(|_| init_random(problem, &mut rng));
    let init = init::from_population(population);
    let select = ElitistSelector::new(50);
    let combine = UniformCrossover::new(Bernoulli::new(0.5).unwrap(), rng.clone());
    let mutate = op::from_fn(
        |solution: &mut Individual<Vec<Pos>>, problem: &Problem, _eval, _input| {
            for pos in &mut **solution {
                pos.x = rng.gen_range(0..problem.n_queens);
            }
            Ok(())
        },
    );
    let insert = ElitistInserter::new();
    let stop = Iterations::new(10000);

    // Note that we could have also handwritten the metaheuristic like so, which is equivalent to the combinator-based version:
    //
    // ```rs
    // let mut solution = init.init(problem, &mut eval).unwrap();
    //
    // while !stop.stop(&solution, problem, &mut eval) {
    //     let selected = select.select(&solution, problem, &mut eval).unwrap();
    //     let mut combined = combine.combine(&solution, problem, &mut eval, selected).unwrap();
    //     for individual in &mut combined {
    //         let individual = Individual::from_mut(individual);
    //         mutate.mutate(individual, problem, &mut eval).unwrap();
    //     }
    //     insert.insert(&mut solution, problem, &mut eval, combined).unwrap();
    // }
    // ```
    //
    // Constructing metaheuristics via combinators will generally produce the same code, as the compiler is able to easily
    // inline and optimise away the various layers of wrappers.
    let mut ga = op::hint(init).then(
        op::hint(select)
            .unwrapped()
            .pipe(op::hint(combine).unwrapped())
            .pipe(on_combined(population::for_each(mutate)))
            .pipe(insert)
            .repeat_until(stop),
    );

    // These combined operators now impl `Solve`, so we can pass it a problem instance and an objective function (anything that
    // impls `Eval<P, S>` where `S` is the solution type and `P` is the problem type), and we get back a solution (or an error
    // if something went wrong during solving).
    //
    // In this case, all operators chosen above have an error type of `Infallible`, so the error type of their combination is
    // also `Infallible` and we can safely unwrap the result. For more complex operators, they may return errors, which you
    // would want to handle properly.
    //
    // Since we started with a population of individuals (see `init::from_population`), we get back a population as well.
    let population: [Solution; 100] = ga.solve(problem, &mut eval).unwrap();

    // Evaluate the best individual from the population. Note that it is not guaranteed that we will get an optimal
    // solution - but it is very likely that we get one, or at least a near-optimal solution.
    let best_objective = population
        .iter()
        .map(|solution| eval.eval(solution, problem))
        .max()
        .unwrap();
    println!("found solution with objective value of {}", best_objective);
}
