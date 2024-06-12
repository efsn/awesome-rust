use rand::Rng;
use regex::Regex;
use std::collections::HashMap;

/*
使用哈希 map 和 vector，创建一个文本接口来允许用户向公司的部门中增加员工的名字。
例如，“Add Sally to Engineering” 或 “Add Amir to Sales”。
接着让用户获取一个部门的所有员工的列表，或者公司每个部门的所有员工按照字典序排列的列表。
*/

/// 给定一系列数字，使用 vector 并返回这个列表的平均数（mean, average）、
/// 中位数（排列数组后位于中间的值）
/// 和众数（mode，出现次数最多的值；
/// 这里哈希 map 会很有帮助）
fn calc_average(ls: &Vec<i32>) -> i32 {
    let mut sum = 0;

    for i in ls {
        sum += i;
    }

    sum / ls.len() as i32
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

/// 将字符串转换为 Pig Latin，
/// 也就是每一个单词的第一个辅音字母被移动到单词的结尾并增加 “ay”，
/// 所以 “first” 会变成 “irst-fay”。
/// 元音字母开头的单词则在结尾增加 “hay”（“apple” 会变成 “apple-hay”）。
/// 牢记 UTF-8 编码！
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

fn main() {
    let org = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("The avg is {}", calc_average(&org));

    let first = "first";
    println!("The pig latin of {} is {}", first, pig_latin(first));

    let apple = "apple";
    println!("The pig latin of {} is {}", apple, pig_latin(apple));

    let rdm_u32 = (0..10)
        .map(|_| rand::thread_rng().gen_range(1..10))
        .collect();
    println!("The mode of {:?} is {:}", &rdm_u32, calc_mode(&rdm_u32));
}
