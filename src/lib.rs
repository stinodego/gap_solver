pub mod config;
use std::fmt::Debug;

pub fn solve<A, T, C>(config: config::SolverConfig<A, T, C>)
where
    A: Debug,
    T: Debug,
    C: Debug,
{
    println!("{:#?}", config)
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn it_works() {
    //     assert_eq!(2 + 2, 4);
    // }
}
