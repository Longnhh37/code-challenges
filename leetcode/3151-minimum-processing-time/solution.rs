impl Solution {
    pub fn min_processing_time(mut processor_time: Vec<i32>, mut tasks: Vec<i32>) -> i32 {
        processor_time.sort_unstable();
        tasks.sort_unstable_by(|a, b| b.cmp(a));

        processor_time
            .iter()
            .enumerate()
            .map(|(i, &t)| t + tasks[4 * i])
            .max()
            .unwrap()
    }
}
