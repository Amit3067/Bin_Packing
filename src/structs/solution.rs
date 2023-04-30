use rand::{thread_rng, Rng};
use super::item::Item;
use super::bin::Bin;
use std::{collections::{HashMap, HashSet}, ops::Not};

///Solution for the GA
#[derive(Debug)]
pub struct Solution{
    gene: Vec<usize>
}

///Clone trait implementation for Solution
impl Clone for Solution{

    ///Create copy of the solution
    fn clone(&self) -> Self {
        Self { 
            gene: self.gene.iter().map(|v| *v).collect()
        }
    }
}


impl Solution{

    ///Perform mutation on the solution
    pub fn mutate(&mut self, _items: &Vec<Item>, bins: &Vec<Bin>, p_m: f32, u_b: usize) -> (){

        for allele in &mut self.gene {

            //probability for mutation of allele
            let pr = thread_rng().gen_bool(p_m as f64);

            if pr{
                *allele = thread_rng().gen_range(0, u_b+1);
            }
        }
    }

    ///Calculate fitness of the solution
    pub fn fitness(&self, items: &Vec<Item>, bins: &Vec<Bin>) -> f32{
        
        let mut mp = HashMap::<usize, Vec<&Item>>::new();

        let mut cur = 0;

        while cur < self.gene.len() {
            mp.entry(self.gene[cur]).or_insert(Vec::<&Item>::new()).push(&items[cur]);
            cur += 1;
        }

        let mut utilization: f32 = 0.0;
        for k in mp.keys(){
            if bins[*k].can_hold(&mp[k]).not() {
                return 0.0;
            }
            utilization += bins[0].utilization(&mp[k]).powi(4);
        }
        
        // return 1.0 / mp.len() as f32;

        return utilization / mp.len() as f32;
    }

    ///Create a new instance of the Solution
    pub fn new(items: &Vec<Item>, bins: &Vec<Bin>, u_b: usize) -> Self{

        //total item count
        let item_cnt = items.len();

        //total bin count
        let bin_cnt = bins.len();

        //
        let mut res = Solution {
            gene: (0..item_cnt).map(|_v| thread_rng().gen_range(0, bin_cnt)).collect()
        };
       
        //update the solutions to occupy consecutive bins
        res.adapt(items, bins);
        
        //reassign infeasible solutions using best-fit-decreasing algorithm
        res.best_fit(items, bins, u_b);
        
        return res;
    }

    ///Get the used bins count
    pub fn get_bin_cnt(&self) -> u32 {
        let mut st = HashSet::<usize>::new();
    
        for allele in &self.gene {
            st.insert(*allele);
        }
    
        return st.len() as u32;
    }

    ///Perform crossover between the current and the other solution
    pub fn crossover(&mut self, other : &mut Self) {

        //variable to store the size of the gene
        let sz = self.gene.len();

        //variable to store the index of current allele
        let mut ind : usize = 0;

        //generate first random crossover point
        let r1 = thread_rng().gen_range(0, sz);

        //generate second random crossover point
        let mut r2 = thread_rng().gen_range(0, sz);

        //ensure the two crossover points are unique
        while r2 == r1 {
            r2 = thread_rng().gen_range(0, sz);
        }

        //first crossover point
        let ind1 = std::cmp::min(r1, r2);

        //second crossover point
        let ind2 = std::cmp::max(r1, r2);

        //make a copy of first parent
        let p1 = self.clone();

        //make a copy of second parent
        let p2 = other.clone();

        //perform crossover till first crossover point
        while ind < ind1 {
            self.gene[ind] = p1.gene[ind];
            other.gene[ind] = p2.gene[ind];
            ind += 1;
        }

        //perform crossover from first crossover point to second crossover point
        while(ind < ind2) {
            self.gene[ind] = p2.gene[ind];
            other.gene[ind] = p1.gene[ind];
            ind += 1;
        }

        //perform crossover after second crossover point
        while(ind < sz){
            self.gene[ind] = p1.gene[ind];
            other.gene[ind] = p2.gene[ind];
            ind += 1;
        }
        
    }

    //Update the solution to occupy consecutive bins
    pub fn adapt(&mut self, items: &Vec<Item>, bins: &Vec<Bin>) {

        //variable to store the current index of bin
        let mut cnt = 0;

        //variable to store current_bin_number -> new_bin_number mappings
        let mut mp = HashMap::<usize,usize>::new();

        let mut ind = 0;

        //loop over the entire gene
        while ind < self.gene.len() {
            let bin_ind = mp.get(&self.gene[ind]);

            //obtain the new_bin_index corresponding to current_bin_number
            let cur_bin = match bin_ind {
                Some(T) => *T,
                None => {
                    mp.insert(self.gene[ind], cnt);
                    cnt += 1;
                    (cnt - 1)
                }
            };

            //update the current_bin_number with new_bin_number
            self.gene[ind] = cur_bin;

            //increment the index
            ind += 1;
        }
    }

    //perform best_fit on the current solution
    pub fn best_fit(&mut self, items: &Vec<Item>, bins: &Vec<Bin>, u_b: usize) {
        
        //store bin -> allele_list mapping
        let mut bin_allele: HashMap<usize, Vec<usize>> = HashMap::<usize, Vec<usize>>::new();

        //store current_index of bin
        let mut cur = 0;

        //map items to bins
        while cur < self.gene.len() {
            bin_allele.entry(self.gene[cur]).or_insert(Vec::<usize>::new()).push(cur);
            cur += 1;
        }

        //vector to store the overflowing items
        let mut extra = Vec::<usize>::new();

        //variable to store current_index of bin used
        let mut bin_index = 0;

        //store new bin -> allele_list mapping
        let mut new_bin_allele: HashMap<usize, Vec<usize>> = HashMap::<usize, Vec<usize>>::new();

        //iterate over all bins to find overflowing bins
        for k in bin_allele.values_mut(){

            //generate the list of items contained in the bins
            let item_lst:Vec<&Item> = k.iter().map(|v| &items[*v]).collect();

            //check if the bin is not overflowing
            if bins[0].can_hold(&item_lst) == true{

                //assign all the items to the bin_index
                for i in k {
                    self.gene[*i] = bin_index;
                    new_bin_allele.entry(bin_index).or_insert(Vec::<usize>::new()).push(*i);
                }
                bin_index += 1;
                continue;
            }

            //if the bin is overflowing append its items to the extra's list
            extra.append(k);
        }

        extra.sort_by(|a,b| {
            items[*b].get_weight().cmp(&items[*a].get_weight())
        });

        //fit all the extra items into bins using BFD
        for i in extra {

            let mut fittest_bin = 200;
            let mut best_rem : f32 = 1.0;
            for j in 0..bin_index {
                let mut item_lst:Vec<&Item> = new_bin_allele[&j].iter().map(|v| &items[*v]).collect();
                item_lst.push(&items[i]);

                if bins[j].can_hold(&item_lst) == true{
                    let cur_rem = 1.0 - bins[0].utilization(&item_lst);
                    if cur_rem < best_rem {
                        best_rem = cur_rem;
                        fittest_bin = j;
                    }
                }
            }

            if fittest_bin == 200{
                self.gene[i] = bin_index;
                new_bin_allele.entry(bin_index).or_insert(Vec::<usize>::new()).push(i);
                bin_index += 1;
                continue;
            }
            self.gene[i] = fittest_bin;
            new_bin_allele.entry(fittest_bin).or_insert(Vec::<usize>::new()).push(i);
        }
    }
}