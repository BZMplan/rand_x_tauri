use std::{io::{BufRead, BufReader}, fs::File};


// 获取前五名的数据
pub fn get_info(index: usize) -> (i32, usize){

    let file = File::open("data/data.randx").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut data: Vec<i32> = Vec::new();

    // 将每一行的数据存储在向量中
    for line in reader.lines() {
        if let Ok(num) = line.unwrap().parse::<i32>() {
            data.push(num);
        }
    }

    // 找到最大的五个数及其位置
    let mut max_values: Vec<(i32, usize)> = Vec::new();
    for (i, &num) in data.iter().enumerate() {
        if max_values.len() < 5 {
            max_values.push((num, i));
        } else {
            let min_index = max_values.iter().enumerate().min_by_key(|&(_, &(value, _))| value).unwrap().0;
            if num > max_values[min_index].0 {
                max_values[min_index] = (num, i);
            }
        }
    }

    max_values.sort_by_key(|&(value, _)| std::cmp::Reverse(value));
    max_values[index]

}
