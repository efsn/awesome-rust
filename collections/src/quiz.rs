use rand::Rng;
use regex::Regex;
use std::collections::{linked_list, HashMap};

#[derive(Debug)]
enum Dept {
    Engineering(Vec<String>),
    Sales(Vec<String>),
}

impl Dept {
    fn add_new_employer(&mut self, name: String) {
        println!("Add {} to {:?}", &name, &self);
        match self {
            Dept::Engineering(list) => list.push(name),
            Dept::Sales(list) => list.push(name),
        }
    }

    fn show(&self, name: &str) {}
}

fn calc_average(ls: &Vec<i32>) -> i32 {
    let mut sum = 0;

    for i in ls {
        sum += i;
    }

    sum / ls.len() as i32
}

fn calc_median(ls: &Vec<i32>) -> i32 {
    let mut cln = ls.clone();
    cln.sort();

    cln[ls.len() / 2]
}

fn calc_mode(vec: &Vec<i32>) -> i32 {
    let mut mp = HashMap::new();

    for &i in vec {
        let count = mp.entry(i).or_insert(0);
        *count += 1;
    }

    println!("The map of vec is {:?}", &mp);

    let max_key = mp
        .iter()
        .max_by_key(|(_, &v)| v)
        .map(|(&k, _)| k)
        .unwrap_or_else(|| 0);

    println!("max key is {}", &max_key);

    mp[&max_key]
}

fn pig_latin(word: &str) -> String {
    if word.len() < 2 {
        return word.to_string();
    }

    let reg = Regex::new(r"^[aeiou]").unwrap();

    if reg.is_match(word) {
        return format!("{}-hay", word);
    } else {
        return format!("{}-{}ay", &word[1..], &word[0..1]);
    }
}

fn gen_range(len: i32) -> Vec<i32> {
    (0..len)
        .map(|_| rand::thread_rng().gen_range(1..10))
        .collect()
}

fn main() {
    let rdm_avg = gen_range(10);
    println!("The avg of {:?} is {:}", &rdm_avg, calc_average(&rdm_avg));

    let first = "first";
    println!("The pig latin of {} is {}", first, pig_latin(first));

    let apple = "apple";
    println!("The pig latin of {} is {}", apple, pig_latin(apple));

    let rdm_mode = gen_range(10);
    println!("The mode of {:?} is {:}", &rdm_mode, calc_mode(&rdm_mode));

    let rdm_median = gen_range(10);
    println!(
        "The median of {:?} is {:}",
        &rdm_median,
        calc_median(&rdm_median)
    );

    let sally = "Sally";
    let amir = "Amir";
    let mut engineering = Dept::Engineering(Vec::new());
    let mut sales = Dept::Sales(Vec::new());

    engineering.add_new_employer(sally.to_string());
    sales.add_new_employer(amir.to_string());
}

/*
使用哈希 map 和 vector，创建一个文本接口来允许用户向公司的部门中增加员工的名字。
例如，“Add Sally to Engineering” 或 “Add Amir to Sales”。
接着让用户获取一个部门的所有员工的列表，或者公司每个部门的所有员工按照字典序排列的列表。
*/
