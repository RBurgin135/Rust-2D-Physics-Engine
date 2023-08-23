use macroquad::prelude::*;
use crate::solver::Solver;

pub mod solver;
pub mod verlet;


#[macroquad::main("Physics2D")]
async fn main() {

    let mut solver = Solver::new();
    let mut t = 0;

    loop {
        clear_background(BLACK);
        if t %5 == 0{
            solver.generate_verlet();
        }
        solver.update(1);
        t+=1;
        next_frame().await
    }
}
