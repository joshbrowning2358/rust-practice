pub trait Solver {
    fn part_1(&self, input: &str) -> String;
    fn part_2(&self, input: &str) -> String;
    fn expected(&self) -> (&'static str, &'static str);
    fn name(&self) -> &'static str;
}
