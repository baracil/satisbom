use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use crate::model::building::Building;
use crate::model::item::Item;
use crate::model::reactant::Reactant;


#[derive(Clone)]
#[allow(dead_code)]
pub struct Recipe {
    id: String,
    duration: u32,
    building: Building,
    alternate: bool,
    inputs: Vec<Reactant>,
    outputs: Vec<Reactant>,
}

impl Recipe {
    pub(crate) fn uses_item(&self, item_id: &str) -> bool {
        self.inputs.iter().any(|r| r.item_id().eq(item_id))
    }
}

impl Recipe {
    pub(crate) fn uses_a_refinery(&self) -> bool {
        self.building.id().eq("refinery")
    }
}

impl Recipe {
    pub(crate) fn uses_a_blender(&self) -> bool {
        self.building.id().eq("blender")
    }
}

impl Recipe {
    pub(crate) fn input_reactant(&self, item: &Item) -> Option<&Reactant> {
        self.inputs.iter().find(|r| r.item().eq(item))
    }
    pub(crate) fn output_reactant(&self, item: &Item) -> Option<&Reactant> {
        self.outputs.iter().find(|r| r.item().eq(item))
    }
}

impl Recipe {
    pub(crate) fn uses_manual_resources(&self) -> bool {
        self.inputs.iter().any(|r| r.item().is_resource_picked_manually())
    }
}

impl Eq for Recipe {}

impl Hash for Recipe {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl PartialEq for Recipe {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl Recipe {
    pub fn id(&self) -> &str {
        &self.id
    }


    pub fn duration(&self) -> u32 {
        self.duration
    }

    pub fn nb_per_minute(&self) -> f64 {
        60f64/(self.duration as f64)
    }


}

impl Recipe {
    pub fn new(id: String, duration: u32, building: Building, alternate: bool, inputs: Vec<Reactant>, outputs: Vec<Reactant>) -> Self {
        Recipe { id, duration, building, alternate, inputs, outputs }
    }


}

impl Display for Recipe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.format(f,1f64)
    }
}


impl Recipe {

    pub fn format(&self, f:&mut Formatter<'_>, amount:f64) -> std::fmt::Result{
        for (i,reactant) in self.inputs.iter().enumerate() {
            if i != 0 {
                f.write_str(" + ")?;
            }
            reactant.format(f, amount)?;
        };

        f.write_str(" -> ")?;

        for (i,reactant) in self.outputs.iter().enumerate() {
            if i != 0 {
                f.write_str(" + ")?;
            }
            reactant.format(f, amount)?
        };
        Ok(())
    }
}


impl Recipe {
    pub fn building(&self) -> &Building {
        &self.building
    }

    pub fn alternate(&self) -> bool {
        self.alternate
    }
    pub fn inputs(&self) -> &[Reactant] {
        &self.inputs
    }
    pub fn outputs(&self) -> &[Reactant]{
        &self.outputs
    }
}

impl Recipe {



    //IMPROVE find a find to factorize the three methods below
    /// Retrieve the indices in the referenceBook of the items
    /// involved in this recipe
    pub fn get_involved_items<'a>(&'a self) -> impl Iterator<Item=&Item>  + 'a {
        self.get_input_items().chain(self.get_output_items())
    }


    pub fn get_input_items<'a>(&'a self) -> impl Iterator<Item=&Item>  + 'a {
        self.inputs
            .iter()
            .map(|i| i.item())
    }

    pub fn get_output_items<'a>(&'a self) -> impl Iterator<Item=&Item>  + 'a {
        self.outputs
            .iter()
            .map(|i| i.item())
    }

}