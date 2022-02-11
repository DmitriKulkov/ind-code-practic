use rand::Rng;
use std::io;

pub fn matrix(row: usize, col: usize) -> Vec<Vec<i32>> {
  return vec![vec![0i32; col]; row];
}

pub fn auto_fill_matrix(v: &mut Vec<Vec<i32>>) {
  for i in 0..v.len() {
    for j in 0..v[i].len() {
      v[i][j] = rand::thread_rng().gen_range(0..10);
    }
  }
}

pub fn man_fill_matrix(v: &mut Vec<Vec<i32>>) {
  for i in 0..v.len() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");

    let result = s
      .split_whitespace()
      .map(|x| x.parse::<i32>())
      .collect::<Result<Vec<i32>, _>>()
      .unwrap();
    for j in 0..v[i].len() {
      v[i][j] = result[j];
    }
  }
}

pub fn print_matrix(v: &mut Vec<Vec<i32>>) {
  for row in v {
    for &mut elem in row {
      print!("{} ", elem)
    }
    print!("\n");
  }
  println!();
}

pub fn clear_matrix(v: &mut Vec<Vec<i32>>) {
  for i in 0..v.len() {
    v[i].clear();
  }
  v.clear();
}

pub fn copy_values(mat1: &mut Vec<Vec<i32>>, mat2: &mut Vec<Vec<i32>>) {
  let len = if mat1.len() > mat2.len() {
    mat2.len()
  } else {
    mat1.len()
  };

  for i in 0..len {
    for j in 0..len {
      mat2[i][j] = mat1[i][j];
    }
  }
}

pub fn sub_matrices(
  mat1: &mut Vec<Vec<i32>>,
  mat2: &mut Vec<Vec<i32>>,
  l: usize,
) -> Vec<Vec<Vec<i32>>> {
  let mut ve = vec![matrix(l, l); 8];
  for i in 0..l {
    for j in 0..l {
      ve[0][i][j] = mat1[i][j];
      ve[4][i][j] = mat2[i][j];
      //
      ve[1][i][j] = mat1[i][j + l];
      ve[5][i][j] = mat2[i][j + l];
      //
      ve[2][i][j] = mat1[i + l][j];
      ve[6][i][j] = mat2[i + l][j];
      //
      ve[3][i][j] = mat1[i + l][j + l];
      ve[7][i][j] = mat2[i + l][j + l];
    }
  }
  return ve;
}

pub fn declare_intermediate_matrices(l: usize) -> Vec<Vec<Vec<i32>>> {
  return vec![matrix(l, l); 7];
}

pub fn calc_interm(mat: &mut Vec<Vec<Vec<i32>>>, interm: &mut Vec<Vec<Vec<i32>>>, l: usize) {
  for i in 0..l {
    for j in 0..l {
      for z in 0..l {
        interm[z][i][j] = 0;
      }
      for z in 0..l {
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

pub fn calc_helpers(helpers: &mut Vec<Vec<Vec<i32>>>, interm: &mut Vec<Vec<Vec<i32>>>, l: usize) {
  for i in 0..l {
    for j in 0..l {
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
  l: usize,
) {
  for i in 0..l {
    for j in 0..l {
      result[i][j] = helpers[0][i][j];
      result[i][j + l] = helpers[1][i][j];
      result[i + l][j] = helpers[2][i][j];
      result[i + l][j + l] = helpers[3][i][j];
    }
  }
}

pub fn matrix_bounds(mat: &mut Vec<Vec<i32>>, l: usize) -> (usize, usize) {
  let (mut x1, mut x2, mut f, mut s) = (0, 0, 100, 100);

  for i in 0..l {
    x1 = 0;
    x2 = 0;
    for j in 0..l {
      if mat[i][j] != 0 {
        x1 += 1;
        f = 100;
      }
      if mat[j][i] != 0 {
        x2 += 1;
        s = 100;
      }
    }
    if x1 == 0 && i < f {
      f = i;
    }
    if x2 == 0 && i < s {
      s = i;
    }
  }
  return (f, s);
}
