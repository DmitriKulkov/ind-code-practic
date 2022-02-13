use rand::Rng;
use std::io;

pub fn matrix(row: usize, col: usize) -> Vec<Vec<i32>> {
  return vec![vec![0i32; col]; row];
}

pub fn auto_fill_matrix(mtrx: &mut Vec<Vec<i32>>) {
  for i in 0..mtrx.len() {
    for j in 0..mtrx[i].len() {
      mtrx[i][j] = rand::thread_rng().gen_range(0..10);
    }
  }
}

pub fn man_fill_matrix(mtrx: &mut Vec<Vec<i32>>) {
  for i in 0..mtrx.len() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");

    let result = s
      .split_whitespace()
      .map(|x| x.parse::<i32>())
      .collect::<Result<Vec<i32>, _>>()
      .unwrap();
    for j in 0..mtrx[i].len() {
      mtrx[i][j] = result[j];
    }
  }
}

pub fn print_matrix(mtrx: &mut Vec<Vec<i32>>) {
  for row in mtrx {
    for &mut elem in row {
      print!("{} ", elem)
    }
    print!("\n");
  }
  println!();
}

pub fn clear_matrix(mtrx: &mut Vec<Vec<i32>>) {
  for i in 0..mtrx.len() {
    mtrx[i].clear();
  }
  mtrx.clear();
}

pub fn copy_values(mat1: &mut Vec<Vec<i32>>, mat2: &mut Vec<Vec<i32>>) {
  let rows = if mat1.len() > mat2.len() {
    mat2.len()
  } else {
    mat1.len()
  };
  let cols = if mat1[0].len() > mat2[0].len() {
    mat2[0].len()
  } else {
    mat1[0].len()
  };

  for i in 0..rows {
    for j in 0..cols {
      mat2[i][j] = mat1[i][j];
    }
  }
}

pub fn sub_matrices(
  mat1: &mut Vec<Vec<i32>>,
  mat2: &mut Vec<Vec<i32>>,
  aug_size: usize,
) -> Vec<Vec<Vec<i32>>> {
  let mut ang_mtrx = vec![matrix(aug_size, aug_size); 8];
  for i in 0..aug_size {
    for j in 0..aug_size {
      ang_mtrx[0][i][j] = mat1[i][j];
      ang_mtrx[4][i][j] = mat2[i][j];
      //
      ang_mtrx[1][i][j] = mat1[i][j + aug_size];
      ang_mtrx[5][i][j] = mat2[i][j + aug_size];
      //
      ang_mtrx[2][i][j] = mat1[i + aug_size][j];
      ang_mtrx[6][i][j] = mat2[i + aug_size][j];
      //
      ang_mtrx[3][i][j] = mat1[i + aug_size][j + aug_size];
      ang_mtrx[7][i][j] = mat2[i + aug_size][j + aug_size];
    }
  }
  return ang_mtrx;
}

pub fn declare_intermediate_matrices(aug_size: usize) -> Vec<Vec<Vec<i32>>> {
  return vec![matrix(aug_size, aug_size); 7];
}

pub fn calc_interm(mat: &mut Vec<Vec<Vec<i32>>>, interm: &mut Vec<Vec<Vec<i32>>>, aug_size: usize) {
  for i in 0..aug_size {
    for j in 0..aug_size {
      for z in 0..aug_size {
        interm[z][i][j] = 0;
      }
      for z in 0..aug_size {
        interm[0][i][j] += (mat[0][i][z] + mat[3][i][z]) * (mat[4][z][j] + mat[7][z][j]);
        interm[1][i][j] += (mat[2][i][z] + mat[3][i][z]) * mat[4][z][j];
        interm[2][i][j] += mat[0][i][z] * (mat[5][z][j] - mat[7][z][j]);
        interm[3][i][j] += mat[3][i][z] * (mat[6][z][j] - mat[4][z][j]);
        interm[4][i][j] += (mat[0][i][z] + mat[1][i][z]) * mat[7][z][j];
        interm[5][i][j] += (mat[2][i][z] - mat[0][i][z]) * (mat[4][z][j] + mat[5][z][j]);
        interm[6][i][j] += (mat[1][i][z] - mat[3][i][z]) * (mat[6][z][j] + mat[7][z][j]);
      }
    }
  }
}

pub fn calc_helpers(helpers: &mut Vec<Vec<Vec<i32>>>, interm: &mut Vec<Vec<Vec<i32>>>, aug_size: usize) {
  for i in 0..aug_size {
    for j in 0..aug_size {
      helpers[0][i][j] = interm[0][i][j] + interm[3][i][j] - interm[4][i][j] + interm[6][i][j];
      helpers[1][i][j] = interm[2][i][j] + interm[4][i][j];
      helpers[2][i][j] = interm[1][i][j] + interm[3][i][j];
      helpers[3][i][j] = interm[0][i][j] - interm[1][i][j] + interm[2][i][j] + interm[5][i][j];
    }
  }
}

pub fn copy_helpers_to_result(
  helpers: &mut Vec<Vec<Vec<i32>>>,
  result: &mut Vec<Vec<i32>>,
  aug_size: usize,
) {
  for i in 0..aug_size {
    for j in 0..aug_size {
      result[i][j] = helpers[0][i][j];
      result[i][j + aug_size] = helpers[1][i][j];
      result[i + aug_size][j] = helpers[2][i][j];
      result[i + aug_size][j + aug_size] = helpers[3][i][j];
    }
  }
}

pub fn matrix_bounds(mat: &mut Vec<Vec<i32>>, aug_size: usize) -> (usize, usize) {
  let mut num_rows;
  let mut num_cols;
  let (mut bot_border, mut right_border) = (aug_size, aug_size);

  for i in 0..aug_size {
    num_rows = 0;
    num_cols = 0;
    for j in 0..aug_size {
      if mat[i][j] != 0 {
        num_rows += 1;
        bot_border = 100;
      }
      if mat[j][i] != 0 {
        num_cols += 1;
        right_border = 100;
      }
    }
    if num_rows == 0 && i < bot_border {
      bot_border = i;
    }
    if num_cols == 0 && i < right_border {
      right_border = i;
    }
  }
  return (bot_border, right_border);
}
