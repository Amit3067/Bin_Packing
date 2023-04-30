#[derive(Debug)]
pub struct Item{
    weight: u32
}

impl Item {

    ///create a new instance of item
    pub fn new(weight: u32) -> Self {
        Self {weight}
    }

    ///get the weight value for an item
    pub fn get_weight(&self) -> u32 {
        self.weight
    }
}