use crate::circuit::Circuit;

fn reduction_cnot_circuit(p: Vec<u8>) -> Circuit {
    let mut circ = Circuit::new(p.len());
    let offset = 3 * p.len();
    for i in 1..(p.len()-1) {
        for j in (p.len() - i)..p.len() {
            if p[j] == 1 {
                circ.push("cx".to_string(), vec![offset + j - p.len() + i, offset + i]);
            }
        }
    }
    for i in (0..p.len()-1).rev() {
        for j in 1..p.len()-i {
            if p[j] == 1 {
                circ.push("cx".to_string(), vec![offset + i + j, offset + i]);
            }
        }
    }
    circ
}

fn gf_mult_synth_rec(circ: &mut Circuit, mut a: Vec<usize>, mut b: Vec<usize>, mut c: Vec<usize>, mut d: Vec<usize>) {
    if a.len() == 1 {
        circ.push("ccz".to_string(), vec![a[0], b[0], c[0]]);
        return;
    }

    if a.len() % 2 == 1 {
        a.push(usize::MAX);
        b.push(usize::MAX);
        c.push(d.remove(0));
        d.push(usize::MAX);
        d.push(usize::MAX);
    }

    let mid = a.len() / 2;
    let (a_l, a_r) = a.split_at(mid);
    let (b_l, b_r) = b.split_at(mid);
    let (c_l, c_r) = c.split_at(mid);
    let (d_l, d_r) = d.split_at(mid);


    for i in 0..mid {
        circ.push("cx".to_string(), vec![a_r[i], a_l[i]]);
        circ.push("cx".to_string(), vec![b_r[i], b_l[i]]);
    }
    gf_mult_synth_rec(circ, a_l.to_vec(), b_l.to_vec(), c_r.to_vec(), d_l.to_vec());

    for i in 0..mid {
        circ.push("cx".to_string(), vec![b_r[i], b_l[i]]);
        circ.push("cx".to_string(), vec![a_r[i], a_l[i]]);
    }
    for i in 0..mid {
        circ.push("cx".to_string(), vec![c_r[i], c_l[i]]);
        circ.push("cx".to_string(), vec![d_l[i], c_r[i]]);
        circ.push("cx".to_string(), vec![d_r[i], d_l[i]]);
    }

    gf_mult_synth_rec(circ, a_r.to_vec(), b_r.to_vec(), c_r.to_vec(), d_l.to_vec());
    gf_mult_synth_rec(circ, a_l.to_vec(), b_l.to_vec(), c_l.to_vec(), c_r.to_vec());

    for i in 0..mid {
        circ.push("cx".to_string(), vec![d_r[i], d_l[i]]);
        circ.push("cx".to_string(), vec![d_l[i], c_r[i]]);
        circ.push("cx".to_string(), vec![c_r[i], c_l[i]]);
    }
}


pub fn gf_mult_synth(p: Vec<u8>) -> Circuit{
    let n = p.len();
    let mut circ = Circuit::new(4 * p.len());
    for i in 2*n..3*n {
        circ.push("h".to_string(), vec![i]);
    }
    for i in 2*n..3*n {
        circ.push("cx".to_string(), vec![i, i+n]);
    }
    let mut cnot_circ = reduction_cnot_circuit(p.clone());
    cnot_circ.gates.reverse();
    circ.append(cnot_circ.clone());
    cnot_circ.gates.reverse();
    gf_mult_synth_rec(&mut circ, (0..n).collect(), (n..2 * n).collect(),(2 * n..3 * n).collect(), (3 * n..4 * n).collect());
    circ.append(cnot_circ);
    for i in (2*n..3*n).rev() {
        circ.push("cx".to_string(), vec![i, i+n]);
    }
    for i in 2*n..3*n {
        circ.push("h".to_string(), vec![i]);
    }
    circ
}
