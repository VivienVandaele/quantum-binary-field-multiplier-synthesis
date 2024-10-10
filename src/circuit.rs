use std::io::Write;
use std::fs::File;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Circuit {
    pub gates: Vec<(String, Vec<usize>)>,
    pub nb_qubits: usize,
}

impl Circuit {
    pub fn new(nb_qubits: usize) -> Self {
        Circuit { 
            gates: Vec::new(),
            nb_qubits: nb_qubits,
        }
    }

    pub fn push(&mut self, gate: String, qubits: Vec<usize>) {
        for &q in &qubits {
            if q == usize::MAX { return; }
        }
        self.gates.push((gate, qubits));
    }

    pub fn append(&mut self, mut circ: Circuit) {
        self.gates.append(&mut circ.gates);
    }

    pub fn to_qc(&self, filename: &str) {
        let mut file = File::create(filename).unwrap();
        let mut map = HashMap::new();
        let mut k = 0;
        write!(file, ".v").unwrap();
        for i in vec!["a", "b", "c", "d"] {
            for j in 0..self.nb_qubits/4 {
                write!(file, " {}{}", i, j).unwrap();
                map.insert(k, format!("{}{}", i, j));
                k += 1;
            }
        }
        write!(file, "\n.i").unwrap();
        for i in vec!["a", "b"] {
            for j in 0..self.nb_qubits/4 {
                write!(file, " {}{}", i, j).unwrap();
            }
        }
        write!(file, "\nBEGIN\n").unwrap();
        for (gate, q) in &self.gates {
            match &gate[..] {
                "h" => write!(file, "H {}\n", map.get(&q[0]).unwrap()).unwrap(),
                "cx" => write!(file, "tof {} {}\n", map.get(&q[0]).unwrap(), map.get(&q[1]).unwrap()).unwrap(),
                "ccz" => write!(file, "Z {} {} {}\n", map.get(&q[0]).unwrap(), map.get(&q[1]).unwrap(), map.get(&q[2]).unwrap()).unwrap(),
                _ => {println!("Operator not implemented: {}", gate); std::process::exit(1)},
            }
        }
        write!(file, "END").unwrap();
    }
}
