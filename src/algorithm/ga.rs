use super::super::structs::bin::Bin;
use super::super::structs::item::Item;
use super::super::structs::solution::Solution;

use crate::structs::config::Config;
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};
use std::fmt::Debug;

///Genetic Algorithm for Bin Packing Problem 
#[derive(Debug)]
pub struct Algorithm{
    config: Config,
    bins: Vec<Bin>,
    items: Vec<Item>,
    population: Vec<Solution>,
    u_b: usize
}

impl Algorithm{

///create a new instance of Genetic Algorithm
    pub fn new(config: Config) -> Result<Self, &'static str>{
        
        Ok(Algorithm {
            config,
            items: Vec::<Item>::new(),
            bins: Vec::<Bin>::new(),
            population: Vec::<Solution>::new(),
            u_b: 0
        })

    }

    ///add new instances of items to the algorithm
    pub fn add_item(&mut self, item : Item) -> () {
        self.items.push(item);
    }

    ///add new instances of bins to the algorithm
    pub fn add_bin(&mut self, bin : Bin) -> () {
        self.bins.push(bin);
    }

    ///start execution
    pub fn run(&mut self) -> Solution {
        //initialize the upper bound on the number of bins
        self.u_b = self.bins.len()-1 as usize;

        //initialize the population
        self.init();

        //variable to store the current iteration number
        let mut cur_iter = 0;

        //loop to iterate till the maximum number of iterations have been reached
        while cur_iter < self.config.get_max_iter() {

            //creating a mating pool
            let mut mating_pool = self.reproduce();

            //performing crossover between solutions of mating pool
            self.crossover(&mut mating_pool);

            //performing mutation over the offspring population
            self.mutate(&mut mating_pool);

            //combine off-springs and parents for fitness-based (mu+lambda) survivor slection
            mating_pool.append(&mut self.population);

            //perform fitness based survivor selection
            mating_pool = self.get_survivors(&mating_pool, self.config.get_p_size() as usize);

            //update the current population
            self.population = mating_pool;

            //track the minimum bin count obtained so far
            let mut min_bin_cnt = self.u_b;

            for sol in &mut self.population {

                //update the solutions to occupy consecutive bins
                sol.adapt(&self.items, &self.bins);

                //reassign infeasible solutions using best-fit-decreasing algorithm
                sol.best_fit(&self.items, &self.bins);

                //update the minimum bin count
                if sol.fitness(&self.items, &self.bins) > 0.0{
                    min_bin_cnt = std::cmp::min(min_bin_cnt, sol.get_bin_cnt() as usize);
                }
            }

            //update the upper bound for the number of bins
            self.u_b = min_bin_cnt;

            //increment the number of iterations
            cur_iter += 1;
        }

        //obtain the best solution of the population
        let res = self.get_best();

        //Output the result of the genetic algorithm
        println!("Fitness = {:?}, Bins = {:?}", res.fitness(&self.items, &self.bins), res.get_bin_cnt());
        return res;
    }

    ///initialize the population for ga
    fn init(&mut self) -> () {
        self.population.clear();
        self.population = (0..self.config.get_p_size()).map(|_| Solution::new(&self.items, &self.bins)).collect();
    }

    ///perform reproduction operation
    fn reproduce(&mut self) -> Vec<Solution> {

        //calculate population fitness
        let fitness = self.population_fitness(&self.population);

        //enumerate population fitness
        let mut ind : Vec<(usize, f32)> = fitness.iter().enumerate().map(|tup| (tup.0 as usize, *tup.1)).collect();

        //sort by decreasing fitness
        ind.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        //generate selection probabilities based on ranking selection
        let sel_pr : Vec<f32> = Algorithm::generate_sel_prob(self.population.len() as u32);

        //mating pool
        let mut pool = Vec::<Solution>::new();

        while pool.len() < ind.len(){
            let rnd : f32 = thread_rng().gen_range(0.0,1.0);

            let elem_ind = match sel_pr.binary_search_by(|prob| prob.partial_cmp(&rnd).unwrap()) {
                Ok(i) => i,
                Err(e) => e
            };

            pool.push(self.population[ind[elem_ind].0].clone());
        }

        return pool;
    }

    ///generate selection probabilities for a population size of mu
    fn generate_sel_prob(mu: u32) -> Vec<f32>{
        let s = 1.5;

        let mu_ = mu as f32;

        let mut sel_pr : Vec<f32> = (0..mu as u32).map(|i| {
            (2.0-s) / mu_ + 2.0 * i as f32 * (s-1.0) / (mu_ * (mu_ - 1.0))
        }).collect();

        for i in 1..sel_pr.len() {
            sel_pr[i] = sel_pr[i-1] + sel_pr[i];
        }

        return sel_pr;
    }

    ///calculate fitness of all the solutions in the population
    fn population_fitness(&self, population: &Vec<Solution>) -> Vec<f32>{
        population.iter().map(|s| s.fitness(&self.items, &self.bins)).collect()
    }

    ///perform mutation operation on a pool of solutions
    fn mutate(&mut self, pool: &mut Vec<Solution>) {

        for individual in pool {
            individual.mutate(&self.items, &self.bins, self.config.get_pr_m(), self.u_b);
        }
    }

    ///perform crossover operation on a pool of solutions
    fn crossover(&mut self, pool: &mut Vec<Solution>) {
        let mut ind : Vec<usize> = (0..pool.len()).collect();

        ind.shuffle(&mut thread_rng());

        let mut i = 0;

        while i+1  < pool.len() {
            //first parent
            let mut p1 = pool[i].clone();

            //second parent
            let mut p2 = pool[i+1].clone();

            //random number for recombination probability
            let rnd : f32 = thread_rng().gen_range(0.0,1.0);

            if self.config.get_pr_c() < rnd {
                p1.crossover(&mut p2);
            }

            pool[i] = p2;
            pool[i+1] = p1;
            i += 2;
        }
    }

    //find the best solution of the current population
    fn get_best(&mut self) -> Solution {
        let mut best = self.population[0].clone();
        let mut fitness  = best.fitness(&self.items, &self.bins);

        for individual in &mut self.population{
            let cur_fitness = individual.fitness(&self.items, &self.bins);
            if cur_fitness > fitness {
                fitness = cur_fitness;
                best = individual.clone();
            }
        }
        return best;
    }

    //perform survival selection to select mu elements from population\
    fn get_survivors(&self, population: &Vec<Solution>, mu: usize) -> Vec<Solution>{
        
        // println!("mu {} population {}",mu,population.len());
        //calculate population fitness
        let fitness = self.population_fitness(&population);

        //enumerate population fitness
        let mut ind : Vec<(usize, f32)> = fitness.iter().enumerate().map(|tup| (tup.0 as usize, *tup.1)).collect();

        //sort by decreasing fitness
        ind.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        //mating pool
        let mut pool = Vec::<Solution>::new();

        //select top mu solutions as survivors
        while pool.len() < mu{
            pool.push(population[ind[pool.len()].0].clone());
        }
        
        return pool;
    }
}