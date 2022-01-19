use good_lp::{default_solver, Expression, ProblemVariables, Solution, SolverModel, Variable, variable};

use crate::amount::AmountF64;
use crate::bag::HashBag;
use crate::bom::Bom;
use crate::error::Result;
use crate::factory::Factory;
use crate::model::book::Book;
use crate::model::recipe::Recipe;
use crate::problem_input::ProblemInput;

pub fn solve(input: &ProblemInput, book: &dyn Book) -> Result<Bom<AmountF64>> {
    let mut variables = ProblemVariables::new();


    let nb_recipes = book.number_of_recipes();
    let recipe_amount: Vec<Variable> = variables.add_vector(variable().min(0), nb_recipes);
    let production = Factory::compute_production(book, input, &recipe_amount)?;


    let objective: Expression = production.objective();
    let constraints = production.compute_constraints();

    let variables = variables.maximise(Expression::from_other_affine(&objective)).using(default_solver);

    let variables = constraints.into_iter().fold(variables, |variables, c| variables.with(c));


    let solution = variables.solve()?;

    let used_recipes = recipe_amount
        .into_iter()
        .enumerate()
        .map(|(i, variable)| Ok((book.get_recipe(i)?.clone(), solution.value(variable))))
        .collect::<Result<HashBag<Recipe, AmountF64>>>()?;



    let production = production.evaluate(&solution);

    // let targets = evaluate(production.targets(), &solution,1f64);
    // let available:Result<HashBag<Item,AmountF64>> = input.available_items.iter()
    //     .map(|(k,v) | Ok((book.get_item_by_id(k)?.clone(),AmountF64::from(*v))))
    //     .collect();


    Ok(Bom::create(used_recipes, production))
}


