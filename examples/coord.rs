fn coord_to_linear(x_raw: i32, y_raw: i32, rows: i32) -> i32 {
  let mut x = x_raw;
  if x_raw >= rows {
    x = x_raw % rows;
  }
  if x_raw < 0 {
    x = rows + x_raw;
  }

  let mut y = y_raw;
  if y_raw >= rows {
    y = y_raw % rows;
  }
  if y_raw < 0 {
    y = rows + y_raw;
  }

  println!("  x_raw: {}, x: {}, y_raw: {}, y: {}", x_raw, x, y_raw, y);

  x * rows + y
}

pub fn main() {
    println!("{}", coord_to_linear(0, 0, 10));
    println!("{}", coord_to_linear(1, 1, 10));
    println!("{}", coord_to_linear(-1, -1, 10));
    println!("{}", coord_to_linear(9, 9, 10));
    println!("{}", coord_to_linear(10, 10, 10));
}
