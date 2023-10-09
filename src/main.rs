use rayon::prelude::*;
use smallvec::SmallVec;

const THRESHOLD: usize = 200_000;

// This is a generic trait that is an overkill for many use cases.
// Ideally, it should be implemented for simpler traits.
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

struct CopyingXor<'a>(&'a [u8]);

impl<'a> Compute for CopyingXor<'a> {
    type Output = Vec<u8>;
    // TODO: find a way to make this constant better
    type Subtasks = SmallVec<[CopyingXor<'a>; 32]>;
    type Subtask = CopyingXor<'a>;
    type ComputeSubtaskOutput = Vec<u8>;
    type DistributeOutput = ();

    fn complexity(&self) -> usize {
        self.0.len()
    }

    fn compute_single_threaded(self) -> Self::Output {
        let mut out = Vec::with_capacity(self.0.len());
        out.extend(self.0.iter().map(|i| *i ^ 0xFF));
        out
    }

    fn split(self) -> Self::Subtasks {
        let Self(inp) = self;
        let cpu_count = num_cpus::get();
        let mut subtasks = SmallVec::with_capacity(cpu_count);
        // the following assumes that the threads are of equal power
        let smallest_chunk_size = inp.len() / cpu_count;
        let remainer = inp.len() - smallest_chunk_size * cpu_count;
        let subtask_iter = (0..cpu_count).map(|i| {
            let start = i * smallest_chunk_size + std::cmp::min(i, remainer);
            let end = start + smallest_chunk_size + if i < remainer { i } else { 0 };
            let Some(slice) = inp.get(start..end) else {
                unreachable!()
            };
            Self(slice)
        });
        subtasks.extend(subtask_iter);
        subtasks
    }

    fn distribute(subtasks: Self::Subtasks) -> Self::DistributeOutput {
        todo!()
    }

    fn compute_subtask(subtask: Self::Subtask) -> Self::ComputeSubtaskOutput {
        todo!()
    }

    fn subtask_on_finish(subtask: Self::Subtask, subtask_output: Self::ComputeSubtaskOutput) {
        todo!()
    }

    fn join(distributed_output: Self::DistributeOutput) -> Self::Output {
        todo!()
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
