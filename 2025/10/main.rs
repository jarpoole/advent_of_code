// https://adventofcode.com/2025/day/10

use good_lp::{Expression, ProblemVariables, Solution, SolverModel, solvers::scip, variable};

#[path = "../../helpers.rs"]
mod helpers;

#[cfg(test)]
mod tests;

#[derive(Debug)]
enum IndicatorState {
    On,
    Off,
}

impl From<&IndicatorState> for i32 {
    fn from(indicator: &IndicatorState) -> i32 {
        match indicator {
            IndicatorState::Off => 0,
            IndicatorState::On => 1,
        }
    }
}

#[derive(Debug)]
struct Machine {
    number: usize,
    target_indicator_light_configuration: Vec<IndicatorState>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<u32>,
}

impl Machine {
    fn parse(number: usize, input: &str) -> Self {
        let mut iter = input.split_whitespace();
        Machine {
            number,
            target_indicator_light_configuration: Vec::from_iter(
                iter.next()
                    .expect("every line should include the target indicator state first")
                    .chars()
                    .take_while(|&char| char != ']')
                    .filter_map(|char| match char {
                        '[' => None,
                        '.' => Some(IndicatorState::Off),
                        '#' => Some(IndicatorState::On),
                        c => panic!("unexpected character in indicator configuration '{}'", c),
                    }),
            ),
            joltages: Vec::from_iter(
                iter.next_back()
                    .map(|s| &s[1..s.len() - 1])
                    .expect("every line should include joltage requirements at the end")
                    .split(",")
                    .map(|joltage| {
                        joltage
                            .parse::<u32>()
                            .expect(&format!("failed to parse joltage {joltage}"))
                    }),
            ),
            buttons: Vec::from_iter(iter.map(|s| {
                s[1..s.len() - 1]
                    .split(",")
                    .map(|indicator| {
                        indicator
                            .parse::<usize>()
                            .expect(&format!("failed to parse indicator {indicator}"))
                    })
                    .collect::<Vec<_>>()
            })),
        }
    }
}

// Part 1:
// Model the optimization problem as a system of linear diophantine equations
// because only integer solutions are valid (cannot push a button 0.5 times). =
// To model the fact that the buttons toggle the indicator states, we can use
// modular arithmetic (mod 2).
// https://en.wikipedia.org/wiki/Diophantine_equation#System_of_linear_Diophantine_equations
fn minimum_button_presses_to_enable(machine: &Machine) -> u32 {
    let mut problem_variables = ProblemVariables::new();

    // Add a non-negative integer variable for each button denoting how many times
    // it needs to be pressed
    let variables: Vec<_> = machine
        .buttons
        .iter()
        .map(|_| problem_variables.add(variable().integer().min(0).max(i32::MAX)))
        .collect();

    // We are looking for the smallest number of total button presses so if the
    // button press variables are b1, b2 ... bn, then the objective function we
    // should minimize is b1 + b2 + ... + bn
    let objective = variables
        .iter()
        .fold(Expression::from(0), |expr, var| expr + var);

    // We will need one constraint for each indicator light so we can
    // preallocate
    let mut constraints = Vec::with_capacity(machine.target_indicator_light_configuration.len());

    // instantiate the constraints for each indicator
    // Ex: if an indicator should finish in the "on" state and is
    // toggled by buttons b2, b3, and b5 then the constraint should be:
    //
    //     b2 + b3 + b5 = 1 (mod 2)
    //
    // But because linear programming does not support modular
    // arithmetic, add another variable, k, which must be a
    // multiple of two instead. Therefore the constraint becomes:
    //
    //     b2 + b3 + b5 + 2*k = 1
    for (indicator, target_state) in machine
        .target_indicator_light_configuration
        .iter()
        .enumerate()
    {
        // if a button toggles the indicator, add the button variable to the constraint
        let mut constraint_expr = Expression::from(0);
        for (button, wiring) in machine.buttons.iter().enumerate() {
            if wiring.contains(&indicator) {
                constraint_expr += variables[button];
            }
        }

        // Add modulo variable
        let modulo = problem_variables.add(variable().integer().min(i32::MIN).max(i32::MAX));
        constraint_expr += 2 * modulo;

        // set the RHS of the constraint based on the desired state of the
        // indicator. Specifically, if the indicator should end in the "on"
        // state then set the RHS = 1, and for "off", set RHS = 0
        let constraint = constraint_expr.eq::<i32>(target_state.into());
        constraints.push(constraint);
    }

    let solution = problem_variables
        .minimise(objective)
        .using(scip::scip)
        .with_all(constraints)
        .solve()
        .expect(&format!(
            "failed to find solution for machine {}",
            machine.number
        ));

    let minimum_button_presses = variables
        .iter()
        .map(|&v| solution.value(v).round() as u32)
        .sum();
    return minimum_button_presses;
}

// see "minimum_button_presses_to_enable"
fn minimum_button_presses_to_power(machine: &Machine) -> u32 {
    let mut problem_variables = ProblemVariables::new();
    let variables: Vec<_> = machine
        .buttons
        .iter()
        .map(|_| problem_variables.add(variable().integer().min(0).max(i32::MAX)))
        .collect();
    let objective = variables
        .iter()
        .fold(Expression::from(0), |expr, var| expr + var);
    let mut constraints = Vec::with_capacity(machine.target_indicator_light_configuration.len());
    for (indicator, joltage) in machine.joltages.iter().enumerate() {
        let mut constraint_expr = Expression::from(0);
        for (button, wiring) in machine.buttons.iter().enumerate() {
            if wiring.contains(&indicator) {
                constraint_expr += variables[button];
            }
        }
        constraints.push(constraint_expr.eq(*joltage));
    }

    let solution = problem_variables
        .minimise(objective)
        .using(scip::scip)
        .with_all(constraints)
        .solve()
        .expect(&format!(
            "failed to find solution for machine {}",
            machine.number
        ));

    let minimum_button_presses = variables
        .iter()
        .map(|&v| solution.value(v).round() as u32)
        .sum();
    return minimum_button_presses;
}

fn parse_machines(input: &str) -> impl Iterator<Item = Machine> + Clone {
    input
        .split("\n")
        .filter_map(|line| (!line.is_empty()).then_some(line.trim()))
        .enumerate()
        .map(|(number, line)| Machine::parse(number, line))
}

fn enable_all_machines(machines: impl Iterator<Item = Machine>) -> u32 {
    machines
        .map(|machine| minimum_button_presses_to_enable(&machine))
        .sum()
}

fn power_all_machines(machines: impl Iterator<Item = Machine>) -> u32 {
    machines
        .map(|machine| minimum_button_presses_to_power(&machine))
        .sum()
}

fn main() {
    let input = helpers::get_input(2025, 10).unwrap();
    let machines = parse_machines(&input);
    println!(
        "The minimum number of button presses to enable all the machines is: {}",
        enable_all_machines(machines.clone())
    );
    println!(
        "The minimum number of button presses to power all the machines is: {}",
        power_all_machines(machines)
    );
}
