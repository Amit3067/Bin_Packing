use rand::{thread_rng, Rng};
use super::item::Item;
use super::bin::Bin;
use std::{collections::{HashMap, HashSet}, ops::Not};

#[derive(Debug)]
pub struct Solution{
    gene: Vec<usize>
}

impl Clone for Solution{
    fn clone(&self) -> Self {
        Self { 
            gene: self.gene.iter().map(|v| *v).collect()
        }
    }
}


impl Solution{

    pub fn mutate(&mut self, _items: &Vec<Item>, bins: &Vec<Bin>, p_m: f32, u_b: usize) -> (){
        for allele in &mut self.gene {
            let pr = thread_rng().gen_bool(p_m as f64);

            if pr{
                *allele = thread_rng().gen_range(0, u_b+1);
            }
        }
    }

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
            utilization += bins[0].utilization(&mp[k]).powi(2);
        }
        
        // return 1.0 / mp.len() as f32;

        return utilization / mp.len() as f32;
    }


    pub fn new(items: &Vec<Item>, bins: &Vec<Bin>) -> Self{
        let item_cnt = items.len();
        let bin_cnt = bins.len();
        let mut res = Solution {
            gene: (0..item_cnt).map(|_v| thread_rng().gen_range(0, bin_cnt)).collect()
        };
        let mut extra : Vec<usize> = (0..item_cnt).collect();

        // extra.sort_by_key(|a,b|)
        let mut cur_bin = 0;

        let mut cur_ex = 0;
        while cur_ex < extra.len() {
            let mut mx_len = cur_ex+1;
            let mut item_lst:Vec<&Item> = (cur_ex..mx_len).map(|v| &items[extra[v]]).collect();
            while bins[0].can_hold(&item_lst) == true && mx_len < extra.len(){
                mx_len += 1;
                item_lst = (cur_ex..mx_len).map(|v| &items[extra[v]]).collect();
            }

            while cur_ex < mx_len {
                res.gene[extra[cur_ex]] = cur_bin;
                cur_ex += 1;
            }
            cur_bin += 1;
        }

        // println!("{:?}", res);

        // println!("{:?}", res.fitness(items, bins));
        return res;
    }

    pub fn get_bin_cnt(&self) -> u32 {
        let mut st = HashSet::<usize>::new();
    
        for allele in &self.gene {
            st.insert(*allele);
        }
    
        return st.len() as u32;
    }

    pub fn crossover(&mut self, other : &mut Self) {
        let sz = self.gene.len();
        let mut ind : usize = 0;
        let r1 = thread_rng().gen_range(0, sz);
        let mut r2 = thread_rng().gen_range(0, sz);
        while r2 == r1 {
            r2 = thread_rng().gen_range(0, sz);
        }
        let ind1 = std::cmp::min(r1, r2);
        let ind2 = std::cmp::max(r1, r2);

        let p1 = self.clone();
        let p2 = other.clone();
        while ind < ind1 {
            self.gene[ind] = p1.gene[ind];
            other.gene[ind] = p2.gene[ind];
            ind += 1;
        }

        while(ind < ind2) {
            self.gene[ind] = p2.gene[ind];
            other.gene[ind] = p1.gene[ind];
            ind += 1;
        }

        while(ind < sz){
            self.gene[ind] = p1.gene[ind];
            other.gene[ind] = p2.gene[ind];
            ind += 1;
        }
        
    }

    pub fn adapt(&mut self, items: &Vec<Item>, bins: &Vec<Bin>) {
        let mut cnt = 0;

        let mut mp = HashMap::<usize,usize>::new();

        let mut ind = 0;

        while ind < self.gene.len() {
            let bin_ind = mp.get(&self.gene[ind]);

            let cur_bin = match bin_ind {
                Some(T) => *T,
                None => {
                    mp.insert(self.gene[ind], cnt);
                    cnt += 1;
                    (cnt - 1)
                }
            };
            self.gene[ind] = cur_bin;
            ind += 1;
        }
    }

    pub fn best_fit(&mut self, items: &Vec<Item>, bins: &Vec<Bin>) {
        
        let mut mp = HashMap::<usize, Vec<usize>>::new();

        let mut cur = 0;

        while cur < self.gene.len() {
            mp.entry(self.gene[cur]).or_insert(Vec::<usize>::new()).push(cur);
            cur += 1;
        }

        let mut extra = Vec::<usize>::new();


        for k in mp.values_mut(){
            // println!("{:?}",extra);
            while k.len() > 0 {
                let item_lst:Vec<&Item> = k.iter().map(|v| &items[*v]).collect();
                if bins[0].can_hold(&item_lst) == true{
                    break;
                }
                extra.push(*k.last().unwrap());
                k.pop();
            }
        }

        let mut cur_bin = mp.len() as usize;

        let mut cur_ex = 0;

        while cur_ex < extra.len() {
            let mut mx_len = cur_ex+1;
            let mut item_lst:Vec<&Item> = (cur_ex..mx_len).map(|v| &items[extra[v]]).collect();
            while bins[0].can_hold(&item_lst) == true && mx_len < extra.len(){
                mx_len += 1;
                item_lst = (cur_ex..mx_len).map(|v| &items[extra[v]]).collect();
            }

            while cur_ex < mx_len {
                self.gene[extra[cur_ex]] = cur_bin;
                cur_ex += 1;
            }
            cur_bin += 1;
        }
    }
}