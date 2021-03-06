use std::collections::HashSet;
use crate::model::book::{Book, FilterableBook};
use crate::error::{Error, Result};
use crate::model::full_book::FullBook;
use crate::model::item::Item;
use crate::Recipe;
use crate::recipe_filter::RecipeFilter;

pub struct FilteredBook<'a> {
    full_book: &'a FullBook,
    filtered_recipe_indices: Vec<usize>,
}

impl<'a> FilteredBook<'a> {
    pub fn new(full_book: &'a FullBook, filtered_recipe_indices: Vec<usize>) -> Self {
        FilteredBook { full_book, filtered_recipe_indices }
    }
}

impl FilterableBook for FilteredBook<'_> {
    fn filter(&self, predicate: &RecipeFilter) -> Result<FilteredBook> {
        let mut new_recipes = Vec::<usize>::new();

        for index in &self.filtered_recipe_indices {
            if self.recipe_matches(*index, predicate)? {
                new_recipes.push(*index)
            }
        }

        Ok(FilteredBook { full_book: self.full_book, filtered_recipe_indices: new_recipes })
    }
}

impl Book for FilteredBook<'_> {
    fn number_of_recipes(&self) -> usize {
        self.filtered_recipe_indices.len()
    }

    fn get_recipe(&self, recipe_index: usize) -> Result<&Recipe> {
        self.filtered_recipe_indices
            .get(recipe_index)
            .ok_or(Error::InvalidRecipeIndex(recipe_index))
            .and_then(|i| self.full_book.get_recipe(*i))
    }

    fn get_involved_items(&self) -> crate::error::Result<HashSet<Item>> {
        let mut result = HashSet::<Item>::new();

        for i in &self.filtered_recipe_indices {
            let recipe = self.full_book.get_recipe(*i)?;
            for item in recipe.get_involved_items() {
                result.insert(item.clone());
            }
        }

        Ok(result)
    }

    fn get_item_by_id(&self, item_id: &str) -> Result<&Item> {
        self.full_book.get_item_by_id(item_id)
    }
}

impl FilteredBook<'_> {
    fn recipe_matches(&self, recipe_index: usize, predicate: &RecipeFilter) -> Result<bool> {
        self.get_recipe(recipe_index).map(|r| predicate.matches(r))
    }
}