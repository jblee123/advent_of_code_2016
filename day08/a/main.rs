use std::fs;
use std::str;
use std::str::FromStr;

const SCREEN_WIDTH: u32 = 50;
const SCREEN_HEIGHT: u32 = 6;
type Screen = [[bool; SCREEN_HEIGHT as usize]; SCREEN_WIDTH as usize];

#[derive(Debug)]
enum Instr {
    Rect { w: u32, h: u32 },
    RotRow { row: u32, shift: u32 },
    RotCol { col: u32, shift: u32 },
}

fn parse_rect(params: &[&str]) -> Instr {
    let parts = params[0].split("x").collect::<Vec<&str>>();
    Instr::Rect {
        w: u32::from_str(parts[0]).unwrap(),
        h: u32::from_str(parts[1]).unwrap(),
    }
}

fn parse_rotate(params: &[&str]) -> Instr {
    let row_or_col_str = str::from_utf8(&params[1].as_bytes()[2..]).unwrap();
    let row_or_col = u32::from_str(row_or_col_str).unwrap();
    let shift = u32::from_str(params[3]).unwrap();
    match params[0] {
        "row" => Instr::RotRow {
            row: row_or_col,
            shift: shift
        },
        "column" => Instr::RotCol {
            col: row_or_col,
            shift: shift
        },
        _ => panic!("bad rotate param: {}", params[0]),
    }
}

fn parse_line(line: &str) -> Instr {
    let parts = line.split(" ").collect::<Vec<&str>>();
    match parts[0] {
        "rect" => parse_rect(&parts[1..]),
        "rotate" => parse_rotate(&parts[1..]),
        _ => panic!("bad instruction: {}", line),
    }
}

fn create_screen() -> Screen {
    [[false; SCREEN_HEIGHT as usize]; SCREEN_WIDTH as usize]
}

fn apply_rect(screen: &mut Screen, w: u32, h: u32) {
    for x in 0..w {
        for y in 0..h {
            screen[x as usize][y as usize] = true;
        }
    }
}

fn apply_rot_row(screen: &mut Screen, row: u32, shift: u32) {
    let mut new_row = [false; SCREEN_WIDTH as usize];
    for x in 0..SCREEN_WIDTH {
        let target_x = (x + shift) % SCREEN_WIDTH;
        new_row[target_x as usize] = screen[x as usize][row as usize];
    }
    for i in 0..SCREEN_WIDTH {
        screen[i as usize][row as usize] = new_row[i as usize];
    }
}

fn apply_rot_col(screen: &mut Screen, col: u32, shift: u32) {
    let mut new_col = [false; SCREEN_HEIGHT as usize];
    for y in 0..SCREEN_HEIGHT {
        let target_y = (y + shift) % SCREEN_HEIGHT;
        new_col[target_y as usize] = screen[col as usize][y as usize];
    }
    for i in 0..SCREEN_HEIGHT {
        screen[col as usize][i as usize] = new_col[i as usize];
    }
}

fn apply_instr(screen: &mut Screen, instr: &Instr) {
    match instr {
        Instr::Rect { w, h } => apply_rect(screen, *w, *h),
        Instr::RotRow { row, shift } => apply_rot_row(screen, *row, *shift),
        Instr::RotCol { col, shift } => apply_rot_col(screen, *col, *shift),
    };
}

fn count_pix(screen: &Screen) -> u32 {
    let mut cnt = 0;
    for pix in screen.iter().flatten() {
        if *pix {
            cnt += 1;
        }
    }

    cnt 
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines = input.lines();

    let mut screen = create_screen();

    for line in lines {
        let instr = parse_line(line);
        apply_instr(&mut screen, &instr);
    }

    println!("num pix: {}", count_pix(&screen));
}
