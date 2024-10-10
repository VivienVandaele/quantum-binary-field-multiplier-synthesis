mod circuit;
mod gf_mult;
use crate::gf_mult::gf_mult_synth;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let indices: Vec<usize> = args.iter().map(|arg| arg.parse::<usize>().expect("Please provide valid indices")).collect();
    let mut p = Vec::new();
    if let Some(&max_index) = indices.iter().max() {
        p.resize(max_index, 0);
        for &index in &indices {
            if index < max_index {
                p[index] = 1;
            }
        }
    }
    else {
        println!("No indices provided.");
    }
    let n = p.len();
    let circ = gf_mult_synth(p);
    circ.to_qc(&format!("circuits/gf2^{}_mult.qc", n));
}
