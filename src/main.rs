pub mod algorithm;
pub mod structs;

use structs::config::Config;
use algorithm::ga::Algorithm;
use structs::item::Item;
use structs::bin::Bin;

fn algorithm_test_120() {
    let config = Config::new(2, 0.2, 1000,0.5);

    let mut my_algo = Algorithm::new(config).unwrap();

    let weights = [97, 57, 81, 62, 75, 81, 23, 43, 50, 38, 60, 58, 70, 88, 36, 90, 37, 45, 45, 39, 44, 53, 70, 24, 82, 81, 47, 97, 35, 65, 74, 68, 49, 55, 52, 94, 95, 29, 99, 20, 22, 25, 49, 46, 98, 59, 98, 60, 23, 72, 33, 98, 80, 95, 78, 57, 67, 53, 47, 53, 36, 38, 92, 30, 80, 32, 97, 39, 80, 72, 55, 41, 60, 67, 53, 65, 95, 20, 66, 78, 98, 47, 100, 85, 53, 53, 67, 27, 22, 61, 43, 52, 76,64, 61, 29, 30, 46, 79, 66, 27, 79, 98, 90, 22, 75, 57, 67, 36, 70, 99, 48, 43, 45, 71, 100, 88, 48, 27, 39];

    for weight in weights {
        my_algo.add_item(Item::new(weight));
    }

    for _i in 0..120 {
        my_algo.add_bin(Bin::new(150));
    }

    // println!("{:?}",my_algo);

    my_algo.run();
    
}

fn algorithm_test_50() {
    let config = Config::new(2, 0.2, 1000,0.5);

    let mut my_algo = Algorithm::new(config).unwrap();

    let weights = [12, 45, 90, 92, 30, 40, 42, 51, 17, 18, 12, 25, 32, 47, 41, 63, 60, 51, 81, 73, 51, 40, 15, 20, 11, 12, 9, 13, 91, 92, 75, 44, 71, 32, 35, 16, 29, 44, 39, 58, 16, 23, 9, 91, 26, 42, 84, 98, 50, 62];

    for weight in weights {
        my_algo.add_item(Item::new(weight));
    }

    for _i in 0..50 {
        my_algo.add_bin(Bin::new(100));
    }

    my_algo.run();
    
}

fn algorithm_test_40() {
    let config = Config::new(2, 0.2, 1000,0.5);

    let mut my_algo = Algorithm::new(config).unwrap();

    let weights = [12, 13, 11, 40, 22, 60, 61, 63, 11, 10, 19, 31, 32, 37, 25, 14, 21, 38, 51, 59, 40, 45, 54, 62, 59, 40, 13, 31, 17, 20, 26, 36, 15, 12, 9,
    10, 27, 31, 55, 40];

    for weight in weights {
        my_algo.add_item(Item::new(weight));
    }

    for _i in 0..40 {
        my_algo.add_bin(Bin::new(70));
    }

    // println!("{:?}",my_algo);

    my_algo.run();
    
}

fn algorithm_test_20() {
    let config = Config::new(2, 0.2, 1000,0.5);

    let mut my_algo = Algorithm::new(config).unwrap();

    let weights = [17, 19, 12, 11, 17, 18, 17, 4, 5, 21, 10, 23, 37, 32, 29, 40, 41, 30, 21, 11];

    for weight in weights {
        my_algo.add_item(Item::new(weight));
    }

    for _i in 0..20 {
        my_algo.add_bin(Bin::new(45));
    }

    // println!("{:?}",my_algo);

    my_algo.run();
    
}

fn algorithm_test_10() {
    let config = Config::new(2, 0.2, 1000,0.5);

    let mut my_algo = Algorithm::new(config).unwrap();

    let weights = [14, 15, 12, 2, 4, 8, 13, 19, 20, 7];

    for weight in weights {
        my_algo.add_item(Item::new(weight));
    }

    for _i in 0..10 {
        my_algo.add_bin(Bin::new(20));
    }

    // println!("{:?}",my_algo);

    my_algo.run();
    
}

fn algorithm_test_9() {
    let config = Config::new(2, 0.2, 1000,0.5);

    let mut my_algo = Algorithm::new(config).unwrap();

    let weights = [5, 7, 3, 5, 12, 11, 10, 11, 9];

    for weight in weights {
        my_algo.add_item(Item::new(weight));
    }

    for _i in 0..9 {
        my_algo.add_bin(Bin::new(14));
    }

    // println!("{:?}",my_algo);

    my_algo.run();
    
}

fn algorithm_test_4() {
    let config = Config::new(2, 0.2, 1000,0.5);

    let mut my_algo = Algorithm::new(config).unwrap();

    let weights = [2, 2, 4, 4];

    for weight in weights {
        my_algo.add_item(Item::new(weight));
    }

    for _i in 0..4 {
        my_algo.add_bin(Bin::new(6));
    }

    // println!("{:?}",my_algo);

    my_algo.run();
}

fn main() {
    self::algorithm_test_4();
    self::algorithm_test_9();
    self::algorithm_test_10();
    self::algorithm_test_20();
    self::algorithm_test_40();
    self::algorithm_test_50();
    self::algorithm_test_120();
}