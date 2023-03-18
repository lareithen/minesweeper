#![allow(unused)]

extern crate rand;
use rand::Rng;
use std::io;
use std::io::Write;

// constants
const WIDTH: i8 = 10;
const HEIGHT: i8 = 10;
const MINE_COUNT: i8 = 10;
const MINE: i8 = -1;

fn get_input() -> i8 {
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .unwrap();
    
    let fm: i8 = input
        .trim()
        .parse()
        .expect("Invalid input.");

    fm
}

fn main() {
    // get first move from user
    let mut input = String::new();
    println!("Please enter your first move.");

    // x
    print!("X: ");
    let fm_x: i8 = get_input();

    // y
    print!("Y: ");
    let fm_y: i8 = get_input();

    // create table
    let mut table: Vec<Vec<i8>> = Vec::new();
    for _ in 0..HEIGHT {
        let row: Vec<i8> = vec![0i8; WIDTH as usize];
        table.push(row);
    }

    // place mines
    for _ in 0..MINE_COUNT {
        loop {
            let mut rng = rand::thread_rng();
            let x: usize = rng.gen_range(0..WIDTH as usize);
            let y: usize = rng.gen_range(0..HEIGHT as usize);

            if x == fm_x as usize && y == fm_y as usize {
                break;
            }

            if table[y][x] == 0 {
                table[y][x] = MINE;
                break;
            }
        }
    }

    // get neighbors and count mines
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if table[y as usize][x as usize] == MINE {
                continue;
            }

            let mut count: i8 = 0;
            let arr: [i8; 3] = [-1, 0, 1];

            for dy in arr.iter() {
                for dx in arr.iter() {
                    if x as i8 + dx < 0 || x as i8 + dx >= WIDTH || y as i8 + dy < 0 || y as i8 + dy >= HEIGHT {
                        continue;
                    }

                    if table[(y as i8 + dy) as usize][(x as i8 + dx) as usize] == MINE {
                        count += 1;
                    }
                }
            }
            table[y as usize][x as usize] = count;
        }
    }

    // print table
    for i in table {
        println!("{:?}", i);
    }

} 

// end of the code and mine too :neutral_face:

// rust is good but its gay
// ~ an unsigned 8-bit integer 
