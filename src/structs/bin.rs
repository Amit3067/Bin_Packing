use super::item::Item;

#[derive(Debug)]
pub struct Bin{
    //capacity of the bin
    capacity: u32
}

impl Bin{

    ///create a new instance of bin
    pub fn new(capacity: u32) -> Self {
        Self {capacity}
    }

    ///check if the bin can hold the given items
    pub fn can_hold(&self, items: &Vec<&Item>) -> bool{
        let mut sum = 0;
        for i in items{
            sum += i.get_weight();
        }
        return sum <= self.capacity;
    }

    ///get the cost of using the bin
    pub fn cost(&self) -> u32{
        return 1;
    }

    ///get the utilization of bin given the items
    pub fn utilization(&self, items: &Vec<&Item>) -> f32 {
        let mut sum : u32 = 0;
        for item in items.iter() {
            sum += item.get_weight();
        }
        return sum as f32 / self.capacity as f32;
    }

}