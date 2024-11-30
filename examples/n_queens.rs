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
    let problem = Problem { n_queens: 8 };
    ga(&problem);
}

struct Problem {
    n_queens: usize,
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

type Solution = Vec<Pos>;

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

    -(collisions as isize)
}

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
    let mut rng = rand::thread_rng();

    let population: [_; 100] = array::from_fn(|_| init_random(problem, &mut rng));

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

    let mut ga = op::hint(init).then(
        op::hint(select)
            .unwrapped()
            .pipe(op::hint(combine).unwrapped())
            .pipe(on_combined(population::for_each(mutate)))
            .pipe(insert)
            .repeat_until(stop),
    );

    let mut eval = eval::from_fn(cost);

    let solution = ga.solve(problem, &mut eval).unwrap();
    let best_objective = solution
        .iter()
        .map(|solution| eval.eval(solution, problem))
        .max()
        .unwrap();

    println!("found solution with objective value of {}", best_objective);
}
