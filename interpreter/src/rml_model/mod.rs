pub mod term_map;
pub mod source_target;
pub mod join;


use term_map::TriplesMap; 

pub struct Document{
    pub triples_maps : Vec<TriplesMap>,

}
