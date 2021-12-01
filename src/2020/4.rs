use regex::Regex;
use std::fs::read_to_string;

#[derive(Debug, PartialEq)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn validate(&self) -> bool {
        fn strint(s: &String) -> u32 {
            s.parse::<u32>().unwrap()
        }

        match &self.byr {
            Some(x) if strint(&x) >= 1920 && strint(&x) <= 2002 => true,
            _ => return false,
        };

                match &self.iyr {
                    Some(x) if strint(&x) >= 2010 && strint(&x) <= 2020 => true,
                    _ => return false,
                };

                match &self.eyr {
                    Some(x) if strint(&x) >= 2020 && strint(&x) <= 2030 => true,
                    _ => return false,
                };


                match &self.ecl {
                    Some(x) => match x.as_str() {
                        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                        _ => return false,
                    },
                    _ => return false,
                };

                match &self.hcl {
                    Some(x) => {
                        if x.chars().count() != 7 {
return false;
                        }
                        if x.chars().nth(0) != Some('#') {
                            return false;
                        }
                        for i in x[1..].chars() {
                            if (i < '0' || i > 'f') || (i < 'a' && i > '9') {
                                return false;
                            }
                        }
                        true
                    }
                    _ => return false,
                };

                match &self.pid {
                    Some(x) => {
                        if x.chars().count() != 9 {
                            return false;
                        }
                        for i in x.chars() {
                            if i < '0' || i > '9' {
                                return false;
                            }
                        }
                        true
                    }
                    _ => return false,
                };

                match &self.hgt {
                    Some(x) => {
                        let l = x.len();
                        match &x[(l - 2)..l] {
                            "cm" => {
                                let h = strint(&x[..(l - 2)].to_string());
                                if h < 150 || h > 193 {
                                    return false;
                                }
                                true
                            }
                            "in" => {
                                let h = strint(&x[..(l - 2)].to_string());
                                if h < 59 || h > 76 {
                                    return false;
                                }
                                true
                            }
                            _ => return false,
                        }
                    }
                    _ => return false,
                };

//end

        true
    }

    fn new(s: &String) -> Passport {
        let mut byr: Option<String> = None;
        let mut iyr: Option<String> = None;
        let mut eyr: Option<String> = None;
        let mut hgt: Option<String> = None;
        let mut hcl: Option<String> = None;
        let mut ecl: Option<String> = None;
        let mut pid: Option<String> = None;
        let mut cid: Option<String> = None;
        let r = Regex::new(r"([[:alpha:]]{3}):(\S*)").unwrap();
        for cap in r.captures_iter(s) {
            match &cap[1] {
                "byr" => byr = Some(cap[2].to_string()),
                "eyr" => eyr = Some(cap[2].to_string()),
                "iyr" => iyr = Some(cap[2].to_string()),
                "hgt" => hgt = Some(cap[2].to_string()),
                "hcl" => hcl = Some(cap[2].to_string()),
                "ecl" => ecl = Some(cap[2].to_string()),
                "cid" => cid = Some(cap[2].to_string()),
                "pid" => pid = Some(cap[2].to_string()),
                _ => (),
            }
        }
        Passport {
            byr,
            iyr,
            eyr,
            hgt,
            hcl,
            ecl,
            pid,
            cid,
        }
    }
}

fn main() {
    let mut passports = Vec::new();
    let f = read_to_string("input.txt").unwrap();
    let f2 = f.split("\n\n");
    for i in f2 {
        passports.push(Passport::new(&i.to_string()));
    }
    println!("{}", passports.iter().filter(|n| n.validate()).count());
}
