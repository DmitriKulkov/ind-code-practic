use std::io;
use std::vec::Vec;

pub fn input_size() -> (usize, usize) {
  loop {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");

    let result = s
      .split_whitespace()
      .map(|x| x.parse::<usize>())
      .collect::<Result<Vec<usize>, _>>()
      .unwrap();
    if result[0] <= 0 || result[1] <= 0 {
      // println!("Вы что-то не то ввели!");
      continue;
    }
    return (result[0], result[1]);
  }
}

pub enum Fill {
  Manually,
  Auto,
}

pub fn select_way() -> Fill {
  loop {
    println!("Выберите способ заполнения матриц");
    println!("1 - Вручную \n2 - Случайным образом");
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    if s == "1\n" {
      return Fill::Manually;
    } else if s == "2\n" {
      return Fill::Auto;
    }
  }
}
