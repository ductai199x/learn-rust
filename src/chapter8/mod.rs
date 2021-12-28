#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use fstrings::*;
use rand::Rng;
use std::{collections::{HashMap}, fmt, io, borrow::Cow};

#[derive(Debug, Clone)]
struct ListInt {
    pub list: Vec<u32>,
    pub length: u32,
    pub mean: f32,
    pub median: Option<f32>,
    pub mode: u32,
}

impl ListInt {
    fn new(v: Vec<u32>) -> ListInt {
        let mut l = ListInt {
            length: v.len() as u32,
            list: v,
            mean: 0.0,
            median: None,
            mode: 0,
        };
        l.mean = l.mean();
        l.median = l.median();
        l.mode = l.mode();
        return l;
    }

    fn sum(&self) -> u32 {
        let mut sum: u32 = 0;
        for el in &self.list {
            sum += el;
        }
        sum
    }

    fn mean(&self) -> f32 {
        self.sum() as f32 / self.length as f32
    }

    fn sort(&self) -> ListInt {
        let mut new_list = self.clone();
        new_list.list.sort();
        return new_list;
    }

    fn median(&self) -> Option<f32> {
        if self.length < 2 {
            return None;
        }
        let sorted_ = self.sort();
        if self.length % 2 == 1 {
            let mid_idx = (self.length / 2) as usize;
            return Some(sorted_.list[mid_idx] as f32);
        } else {
            let mid_idx = (self.length / 2 - 1) as usize;
            return Some((sorted_.list[mid_idx] + sorted_.list[mid_idx + 1]) as f32 / 2.0);
        }
    }

    fn mode(&self) -> u32 {
        let mut count_map: HashMap<u32, u32> = HashMap::new();
        for i in &self.list {
            let count = count_map.entry(*i).or_insert(0);
            *count += 1;
        }
        let mut max_count = 0;
        let mut mode_ = self.list[0];
        for (k, v) in count_map {
            if v > max_count {
                max_count = v;
                mode_ = k;
            }
        }

        return mode_;
    }
}

fn pig_latin(s: &str) -> String {
    let mut consonants = String::from("BCDFGJKLMNPQSTVXZHRWY");
    consonants += &consonants.to_lowercase();

    let first_letter = &s[0..1];
    if consonants.contains(first_letter) {
        format!("{}-{}{}", &s[1..], first_letter, "ay")
    } else {
        format!("{}-{}{}", &s[..], "h", "ay")
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Department {
    Engineering,
    Sales,
    HumanResources,
}

#[derive(Debug)]
enum AdminActions {
    Add,
    Remove,
}

#[derive(Debug, Clone)]
struct Employee {
    name: String,
    dept: Department,
    id: Option<usize>,
}

#[derive(Debug)]
struct Company {
    id_to_emp_map: HashMap<usize, Employee>,
    emp_to_id_map: HashMap<String, usize>,
    id_gen: usize,
    emp_count: usize,
}

impl fmt::Display for Department {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Department::Engineering => write!(f, "Engineering"),
            Department::Sales => write!(f, "Sales"),
            Department::HumanResources => write!(f, "HumanResources"),
        }
    }
}

impl Employee {
    fn new(name: &str, dept: Department) -> Employee {
        Employee {
            name: name.to_string(), dept: dept, id: None,
        }
    }

    fn from(employee: &Employee) -> Employee {
        employee.clone()
    }
}

impl Company {
    fn new() -> Company {
        Company {
            id_to_emp_map: HashMap::new(),
            emp_to_id_map: HashMap::new(),
            id_gen: 0,
            emp_count: 0,
        }
    }

    fn get_emp_key(&mut self, employee: &Employee) -> String {
        return format!("{}#{}", employee.name, employee.dept);
    }

    fn gen_new_id(&mut self) -> usize {
        self.id_gen += 1;
        self.id_gen
    }

    fn add(&mut self, mut employee: Employee) -> Result<u32, &'static str> {
        let emp_key = self.get_emp_key(&employee);
        if self.emp_to_id_map.contains_key(&emp_key) {
            return Err("Employee already exists!");
        } else {
            let new_emp_id = self.gen_new_id();
            self.emp_to_id_map.insert(emp_key.clone(), new_emp_id);
            employee.id = Some(new_emp_id);
            self.id_to_emp_map.insert(new_emp_id, employee);
            self.emp_count = self.id_to_emp_map.len();
            return Ok(0);
        }
    }

    fn remove_by_id(&mut self, emp_id: usize) -> Result<u32, &'static str> {
        if self.id_to_emp_map.contains_key(&emp_id) {
            let emp = self.id_to_emp_map.remove(&emp_id).unwrap();
            let emp_key = self.get_emp_key(&emp);
            self.emp_to_id_map.remove(&emp_key);
            self.emp_count -= 1;
            return Ok(0);
        } else {
            return Err("Employee does not exist!");
        }
    }

    fn remove_by_emp_obj(&mut self, employee: &Employee) -> Result<u32, &'static str> {
        let emp_key = self.get_emp_key(&employee);
        if self.emp_to_id_map.contains_key(&emp_key) {
            let emp_id = self.emp_to_id_map.get(&emp_key).unwrap();
            self.id_to_emp_map.remove(&emp_id);
            self.emp_to_id_map.remove(&emp_key);
            self.emp_count -= 1;
            return Ok(0);
        } else {
            return Err("Employee does not exist!");
        }
    }

    fn query_by_id(&self, emp_id: usize) -> Result<u32, Cow<'static, str>> {
        if self.id_to_emp_map.contains_key(&emp_id) {
            let emp = self.id_to_emp_map.get(&emp_id).unwrap();
            println!("{:?}", &emp);
            return Ok(0);
        } else {
            return Err(Cow::Owned(f!("Employee with id={emp_id} does not exist!")));
        }
    }
}

pub fn chapter8() {
    let num_el = 6;
    let mut rng = rand::thread_rng();
    let vec: Vec<u32> = (0..num_el).map(|_| rng.gen_range(0..100)).collect();
    let list_int = ListInt::new(vec);
    dbg!(&list_int);

    let sorted_list = list_int.sort();
    dbg!(sorted_list);

    println!("{}", pig_latin("first"));
    println!("{}", pig_latin("apple"));

    let mut company = Company::new();
    let example_employee = Employee::new("abc", Department::Engineering);
    match company.add(example_employee) {
        Ok(o) => (),
        Err(v) => println_f!("{v}"),
    };
    dbg!(&company);
    match company.query_by_id(1) {
        Ok(o) => (),
        Err(v) => println_f!("{v}"),
    };
    match company.remove_by_id(1) {
        Ok(o) => (),
        Err(v) => println_f!("{v}"),
    };
    // company.remove_by_emp_obj(&Employee { name: String::from("abc"), dept: Department::Engineering, id: None });
    dbg!(&company);
    match company.query_by_id(1) {
        Ok(o) => (),
        Err(v) => println_f!("{v}"),
    };
}
