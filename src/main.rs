use macroquad::prelude::*;
use crate::solver::Solver;

pub mod solver;
pub mod verlet;


#[macroquad::main("Physics2D")]
async fn main() {

    let mut solver = Solver::new();

    for _ in 1..2{
        solver.add_verlet(
            verlet::VerletObject::new(
                Vec2::new(
                    screen_width()/2.0+200.0, 
                    screen_height()/2.0
                )
            )
        );
    }

    loop {
        clear_background(BLACK);
        solver.update();
        next_frame().await
    }
}
