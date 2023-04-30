#[derive(Debug)]
pub struct Config{
    ///population size
    p_size: u32,
    ///mutation probability
    pr_m: f32,
    ///maximum number of iterations
    max_iter: u32,
    ///probability of recombination
    pr_c: f32
}

impl Config{
    pub fn new(p_size: u32, pr_m: f32, max_iter:u32, pr_c: f32) -> Self{
        Config{
            p_size,
            pr_m,
            max_iter,
            pr_c
        }
    }

    ///get the population size value
    pub fn get_p_size(&self) -> u32{
        self.p_size
    }

    ///get the mutation probabilty value
    pub fn get_pr_m(&self) -> f32{
        self.pr_m
    }
    
    //get the maximum number of iterations
    pub fn get_max_iter(&self) -> u32{
        self.max_iter
    }

    ///get the crossover probabilty value
    pub fn get_pr_c(&self) -> f32{
        self.pr_c
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn config_creation() {
        let config = Config::new(10, 0.3, 10, 0.5);

        println!("Configuration {:?}",config);
    }

    #[test]
    fn test_getters() {
        let config = Config::new(10, 0.3, 10, 0.5);

        assert_eq!(config.get_p_size(), 10);
        assert_eq!(config.get_pr_m(), 0.3);
        assert_eq!(config.get_max_iter(), 10);
    }
}