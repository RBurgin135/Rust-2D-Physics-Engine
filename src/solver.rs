use macroquad::prelude::*;
use crate::verlet::VerletObject;

pub struct Solver{
    verlets: Vec<VerletObject>,
    gravity: Vec2,

    //border
    border_position: Vec2,
    border_radius: f32,

} impl Solver {

    pub fn new() -> Solver{
        Solver{
            //verlets
            verlets : Vec::new(),

            //gravity
            gravity : Vec2::new(0.0, 0.1),

            //border
            border_position : Vec2::new(screen_width()/2.0, screen_height()/2.0),
            border_radius : 300.0,
        }
    }

    pub fn update(&mut self){
        self.apply_gravity();
        self.apply_circular_constraint();
        self.update_positions();
        self.render();
    }

    pub fn add_verlet(&mut self, new_verlet: VerletObject){
        self.verlets.push(new_verlet);
    }

    fn apply_gravity(&mut self){
        for v in &mut self.verlets {
            //apply gravity
            v.accelerate(self.gravity);
        }
    }

    fn update_positions(&mut self){
        for v in &mut self.verlets {
            //update position, dt=1.0
            v.update_position(1.0);
        }
    }

    
    fn apply_circular_constraint(&mut self) {
        for v in &mut self.verlets {
            //find displacement and distance
            let displacement = v.current_position - self.border_position;
            let distance = displacement.length();

            //if verlet is outside border
            if distance > self.border_radius - v.radius {
                let n = displacement / distance;
                v.current_position = self.border_position + n * (distance - v.radius);
            }
            
        }
    }


    fn render(&mut self){
        //draw border
        draw_circle(
            self.border_position.x, 
            self.border_position.y, 
            self.border_radius,
            WHITE);

        //draw verlets
        for v in &mut self.verlets{
            v.draw();
        }
    }



}