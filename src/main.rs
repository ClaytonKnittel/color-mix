use std::io::{stdin, stdout, BufRead, Lines, Write};

use color_mix::color_mix::{Action, ColorMix};

fn read_action<W: BufRead>(c: &ColorMix, input: &mut Lines<W>) -> Action {
  print!("{} action: ", if c.p1_turn() { "P1" } else { "P2" });
  stdout().flush().unwrap();
  let line = input.next().unwrap().unwrap();
  match line.parse() {
    Ok(action) => action,
    Err(err) => {
      println!("{err}");
      read_action(c, input)
    }
  }
}

fn main() {
  let mut c = ColorMix::new(6);

  let stdin = stdin();
  let mut lines = stdin.lock().lines();
  while !c.finished() {
    println!("{c}");
    let action = read_action(&c, &mut lines);
    if let Err(err) = c.do_action(action) {
      println!("{err}");
    }
  }

  println!("Done!");
  println!("{} wins!", if c.p1_wins() { "P1" } else { "P2" });
}
