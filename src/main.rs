use rayon::prelude::*;

const THRESHOLD: usize = 200_000;

pub trait Compute: Sized {
    type Output;
    type Subtasks;
    type Subtask;
    type ComputeSubtaskOutput;
    type DistributeOutput;

    fn complexity(&self) -> usize;
    fn compute_single_threaded(self) -> Self::Output;
    fn split(self) -> Self::Subtasks;
    fn distribute(subtasks: Self::Subtasks) -> Self::DistributeOutput;
    fn compute_subtask(subtask: Self::Subtask) -> Self::ComputeSubtaskOutput;
    fn subtask_on_finish(subtask: Self::Subtask, subtask_output: Self::ComputeSubtaskOutput);
    fn join(distributed_output: Self::DistributeOutput) -> Self::Output;
    fn compute_multi_threaded(self) -> Self::Output {
        let tasks = self.split();
        let distribute_output = Self::distribute(tasks);
        Self::join(distribute_output)
    }
    fn compute(self) -> Self::Output {
        if self.complexity() < THRESHOLD {
            self.compute_single_threaded()
        } else {
            self.compute_multi_threaded()
        }
    }
}

fn copying_xor(inp: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(inp.len());

    let f = |i: &u8| *i ^ 0xFF;

    if inp.len() < THRESHOLD {
        out.extend(inp.iter().map(f))
    } else {
        out.par_extend(inp.par_iter().map(f));
    }

    out
}

fn main() {
    let inp: Vec<u8> = (0..255).collect();
    let v = copying_xor(&inp);
    assert!(v.windows(2).all(|w| w[0] > w[1]));
    println!("{v:?}");
}
