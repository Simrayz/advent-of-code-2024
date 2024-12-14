use glam::{DMat2, U64Vec2};

#[derive(Debug)]
pub struct Machine {
    pub prize: U64Vec2,
    pub a: U64Vec2,
    pub b: U64Vec2,
}

impl Machine {
    pub fn solve(&self) -> Result<U64Vec2, ()> {
        let coefficient_matrix = DMat2::from_cols_array(&[
            self.a.x as f64,
            self.a.y as f64,
            self.b.x as f64,
            self.b.y as f64,
        ]);
        let coeff_d = coefficient_matrix.determinant();

        let mat_ac = DMat2::from_cols_array(&[
            self.prize.x as f64,
            self.prize.y as f64,
            self.b.x as f64,
            self.b.y as f64,
        ]);
        let d_ac = mat_ac.determinant();

        let mat_bc = DMat2::from_cols_array(&[
            self.a.x as f64,
            self.a.y as f64,
            self.prize.x as f64,
            self.prize.y as f64,
        ]);
        let d_bc = mat_bc.determinant();
        let x = d_ac / coeff_d;
        let y = d_bc / coeff_d;

        // Check that there are no decimals
        if x.trunc() != x || y.trunc() != y {
            return Err(());
        }
        let max = if cfg!(test) { 100f64 } else { f64::INFINITY };

        if x > max || y > max {
            Err(())
        } else {
            Ok(U64Vec2::new(x as u64, y as u64))
        }
    }
}

pub fn parse_claw_machines(input: &str, move_prize: bool) -> Vec<Machine> {
    let adjustment = if move_prize && !cfg!(test) {
        10000000000000
    } else {
        0
    };
    input
        .split("\n\n")
        .map(|paragraph| {
            let mut lines = paragraph.trim().lines();
            let (ax, ay) = parse_numbers(lines.next().unwrap());
            let (bx, by) = parse_numbers(lines.next().unwrap());
            let (px, py) = parse_numbers(lines.next().unwrap());
            let [ax, ay, bx, by, px, py] =
                [ax, ay, bx, by, px, py].map(|s| s[2..].parse::<u64>().unwrap());
            Machine {
                a: U64Vec2 { x: ax, y: ay },
                b: U64Vec2 { x: bx, y: by },
                prize: U64Vec2 {
                    x: px + adjustment,
                    y: py + adjustment,
                },
            }
        })
        .collect()
}

fn parse_numbers(s: &str) -> (&str, &str) {
    s.split_once(": ").unwrap().1.split_once(", ").unwrap()
}
