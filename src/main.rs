use std::{fs, time::Instant};

#[derive(Clone)]
struct LensBox {
    lenses: Vec<Lens>,
}
impl LensBox {
    fn new() -> LensBox {
        LensBox { lenses: vec![] }
    }
    fn try_remove_lens(&mut self, label: &str) {
        if let Some(pos) = self.lenses.iter().position(|l| &l.label[..] == label) {
            self.lenses.remove(pos);
        }
    }
    fn add_lens(&mut self, label: &str, focal_length: i64) {
        match self.lenses.iter_mut().find(|l| &l.label == label) {
            Some(old_l) => old_l.replace(focal_length),
            None => self.lenses.push(Lens::new(label, focal_length)),
        }
    }
    fn get_focusing_power(&self) -> i64 {
        let mut x = 0;
        for (i, lens) in self.lenses.iter().enumerate() {
            x += (i as i64 + 1) * lens.focal_length;
        }
        x
    }
}

#[derive(Clone)]
struct Lens {
    label: String,
    focal_length: i64,
}
impl Lens {
    fn new(l: &str, f: i64) -> Self {
        Lens {
            label: l.into(),
            focal_length: f,
        }
    }
    fn replace(&mut self, new_f: i64) {
        self.focal_length = new_f;
    }
}
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let commands = parse_1(input);
    let now = Instant::now();
    let ans_1 = commands.iter().fold(0, |acc, x| acc + part_1(x));
    let elapsed_1 = now.elapsed().as_millis();
    let now = Instant::now();
    let ans_2 = part_2(&commands);
    let elapsed_2 = now.elapsed().as_millis();
    println!("1: {ans_1} in {elapsed_1} ms\n\n2: {ans_2} in {elapsed_2} ms");
}

fn part_1(command: &String) -> i64 {
    let mut val = 0;
    let b_command = command.as_bytes();
    for c in b_command {
        val += *c as i64;
        val *= 17;
        val = val % 256;
    }
    val
}
fn part_2(commands: &Vec<String>) -> i64 {
    let mut boxes = vec![LensBox::new(); 256];
    for command in commands {
        let data = command
            .split(|c| c == '=' || c == '-')
            .collect::<Vec<&str>>();
        let box_idx = part_1(&String::from(data[0]));
        let bocks: &mut LensBox = &mut boxes[box_idx as usize];
        if command.contains("=") {
            bocks.add_lens(data[0], data[1].parse().unwrap());
        } else {
            bocks.try_remove_lens(data[0]);
        }
    }
    let mut ans = 0;
    for (i, bokes) in boxes.iter().enumerate() {
        ans += (i as i64 + 1) * bokes.get_focusing_power()
    }
    ans
}

fn parse_1(input: String) -> Vec<String> {
    input.split(",").map(|s| s.to_owned()).collect()
}
