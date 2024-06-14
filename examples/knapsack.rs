use heur::{
    eval::{self, Eval},
    op::{
        self,
        accept::NonWorsening,
        init,
        mutate::FlipAllBits,
        search::SteepestAscentBitClimb,
        stop::Iterations,
        Operator,
    },
    solve::Solve,
};

use ordered_float::NotNan;

fn main() {
    // Load and parse the knapsack problem instance. The problem instance at `../instances/knapsack.in` is taken from
    // a collection of hard 0-1 knapsack problem instances here: https://github.com/JorikJooken/knapsackProblemInstances.
    let instance = include_str!("../instances/knapsack.in");
    let knapsack = parse_knapsack(instance);

    // Solve the problem instance using iterated local search.
    ils(&knapsack);
}

// This represents the problem data we are given while solving. A knapsack problem instance is comprised of a list of items,
// each having a weight and a value, and a maximum capacity - i.e. the maximum weight the knapsack can hold.
struct Knapsack {
    items: Vec<Item>,
    capacity: f64,
}

// An item has a weight and a value. For the purposes of this example, we assume that these are always positive (the objective
// function below breaks otherwise).
struct Item {
    weight: f64,
    value: f64,
}

// Since this is a 0-1 knapsack problem example, will use a bitstring as a solution encoding. If an item is included in the
// knapsack, its bit is set to `true` (1). Otherwise, its bit is set to `false` (0).
type Solution = Vec<bool>;

// An objective function that calculates the cost, or objective value, of a given solution (`Vec<bool>`) to a knapsack problem
// instance (`Knapsack`).
fn cost(solution: &Solution, knapsack: &Knapsack) -> NotNan<f64> {
    // Calculate the total weight and value of the items in the knapsack by summing them up together. Only the items
    // that are included (i.e. whose bits in the solution are `true`) are counted.
    let (value, weight) = solution
        .iter()
        .copied()
        .zip(&knapsack.items)
        .filter_map(|(included, item)| included.then_some(item))
        .fold((0.0, 0.0), |(value, weight), item| {
            (value + item.value, weight + item.weight)
        });

    // If the total weight of the included items is greater than the maximum capacity of the knapsack, we have an infeasible
    // solution. The cost for infeasible solutions is represented as the negative of the total weight. This way, an infeasible
    // solution is always going to be worse than a feasible one, since feasible solutions will only have positive values.
    // Infeasible solutions can also be ordered - one infeasible solution is "less bad" than another if it has a lower included
    // weight.
    let cost = if weight > knapsack.capacity {
        -weight
    } else {
        value
    };

    NotNan::new(cost).unwrap()
}

fn ils(knapsack: &Knapsack) {
    // Create an objective function that can be given to the metaheuristic. This example uses `eval::from_fn` to wrap up
    // the objective function above, but you could create a custom type and impl `Eval` for it manually like so:
    // ```rs
    // struct Cost;
    //
    // impl Eval<Vec<bool>, Knapsack> for Cost {
    //     type Objective = f64;
    //
    //     fn eval(&mut self, solution: &Vec<bool>, knapsack: &Knapsack) -> f64 { ... }
    // }
    // ```
    let mut eval = eval::from_fn(cost);

    // Define the various operators we will be using for the iterated local search metaheuristic. We initialise the solution
    // using an all-zeros bitstring (i.e. no items are included at the start).
    //
    // We use a bit-flipping operator as the mutation (aka perturbation) operator, with a 0.002 (0.2%) probability of flipping
    // each bit, and a steepest ascent hill climb as the local search operator.
    //
    // Any changes made by the mutation and local search operators are only accepted if they are non-worsening (i.e. produce an
    // objective value that is no worse than the previous known value), and we stop when we get to 1000 iterations.
    let init = init::from_value(vec![false; knapsack.items.len()]);
    let mutate = FlipAllBits::new(0.002, rand::thread_rng()).unwrap();
    let local_search = SteepestAscentBitClimb::new();
    let accept = NonWorsening::new();
    let stop = Iterations::new(1000);

    // Construct the metaheuristic by combining the above operators. We first initialise the solution, then perform mutation
    // and local search, accepting the new solution based on our acceptance criterion above. We then ignore whether the solution
    // was accepted or not and repeat these steps (excluding initialisation) until our termination criterion returns `true`.
    //
    // You may also notice `op::hint` - this is necessary to guide type inference, otherwise it would produce an error about
    // ambiguous types/impls.
    //
    // Note that we could have also handwritten the metaheuristic like so, which is equivalent to the combinator-based version:
    // ```rs
    // let mut solution = init.init(knapsack, &mut eval).unwrap();
    //
    // while !stop.stop(&solution, knapsack, &mut eval) {
    //     let prev_solution = solution.clone();
    //
    //     mutate.mutate(&mut solution, knapsack, &mut eval).unwrap();
    //     local_search.search(&mut solution, knapsack, &mut eval).unwrap();
    //
    //     if !accept.accept(&solution, &prev_solution, knapsack, &mut eval) {
    //         solution = prev_solution;
    //     }
    // }
    // ```
    //
    // Constructing metaheuristics via combinators will generally produce the same code, as the compiler is able to easily
    // inline and optimise away the various layers of wrappers.
    let mut ils = op::hint(init).then(
        op::hint(mutate)
            .then(local_search)
            .accept_if(accept)
            .ignore()
            .repeat_until(stop),
    );

    // These combined operators now impl `Solve`, so we can pass it a problem instance and an objective function (anything that
    // impls `Eval<S, P>` where `S` is the solution type and `P` is the problem type), and we get back a solution (or an error
    // if something went wrong during solving).
    //
    // In this case, all operators chosen above have an error type of `Infallible`, so the error type of their combination is
    // also `Infallible` and we can safely unwrap the result. For more complex operators, they may return errors, which you
    // would want to handle properly.
    let solution = ils.solve(knapsack, &mut eval).unwrap();

    // Evaluate the solution and print it. Note that since we only ran the metaheuristic for 1000 iterations (see the `stop`
    // operator above), we will likely not get an optimal objective value - but it is very likely that we get a near-optimal
    // value around ~1% away from the global optimum.
    let objective = eval.eval(&solution, knapsack);
    println!("found solution with objective value of {}", objective);
}

// This code parses a knapsack problem instance from a string - an example instance file is at `../instances/knapsack.in`.
fn parse_knapsack(instance: &str) -> Knapsack {
    let mut lines = instance.lines();

    let len = lines.next().unwrap().parse().unwrap();
    let capacity = lines.next_back().unwrap().parse().unwrap();

    let mut items = Vec::with_capacity(len);
    items.extend(lines.map(|line| {
        let (_id, item) = line.split_once(' ').unwrap();
        let (value, weight) = item.split_once(' ').unwrap();
        let value = value.parse().unwrap();
        let weight = weight.parse().unwrap();
        Item { value, weight }
    }));

    Knapsack { items, capacity }
}
