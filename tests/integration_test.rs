use gap_solver::{solve, GapSpec};

#[test]
fn default_spec() {
    let agents = vec!['a', 'b'];
    let tasks = vec!['1', '2'];
    let spec: GapSpec<char, char, u32> = GapSpec::new(agents, tasks);

    solve(&spec);
}
