use std::convert::TryInto;
use std::fs::read_to_string;

fn main() {
    let mut oxygen = 0;
    let mut carbon = 0;
    let mut data = Vec::new();
    let f = read_to_string("input.txt").unwrap();
    let f = f.split("\n");
    for i in f {
        if i.len() == 0 {
            break;
        }
        let mut v = Vec::new();
        for j in i.chars() {
            if j != '\r' {
                v.push(j);
            }
        }
        data.push(v);
    }
    let l = data[0].len();
    let mut oxivec = data.clone();
    let mut carvec = data.clone();
    for i in 0..l {
        if oxivec.len() == 1 {
            break;
        }
        let val;
        let mut oxitruths = vec![0; l];
        let mut oxilies = vec![0; l];
        for oxyi in 0..l {
            for j in oxivec.iter() {
                if j[oxyi] == '0' {
                    oxilies[oxyi] += 1;
                }
                if j[oxyi] == '1' {
                    oxitruths[oxyi] += 1;
                }
            }
        }
        if oxitruths[i] >= oxilies[i] {
            val = '1';
        } else {
            val = '0'
        }
        oxivec.retain(|item| item[i] == val);
    }
    for i in 0..l {
        if oxivec[0][i] == '1' {
            oxygen += 2_u32.pow((l - (i + 1)).try_into().unwrap());
        }
    }
    for i in 0..l {
        if carvec.len() == 1 {
            break;
        }
        let val;
        let mut cartruths = vec![0; l];
        let mut carlies = vec![0; l];
        for cari in 0..l {
            for j in carvec.iter() {
                if j[cari] == '0' {
                    carlies[cari] += 1;
                }
                if j[cari] == '1' {
                    cartruths[cari] += 1;
                }
            }
        }
        if carlies[i] <= cartruths[i] {
            val = '0';
        } else {
            val = '1'
        }
        carvec.retain(|item| item[i] == val);
    }
    for i in 0..l {
        if carvec[0][i] == '1' {
            carbon += 2_u32.pow((l - (i + 1)).try_into().unwrap());
        }
    }
    println!(
        "Oxigen: {}. Carbon: {}. Product: {}.",
        oxygen,
        carbon,
        oxygen * carbon
    );
}
