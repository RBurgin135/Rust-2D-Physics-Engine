use macroquad::prelude::*;

pub struct VerletObject{
    pub current_position: Vec2,
    pub previous_position: Vec2,
    pub acceleration: Vec2,
    colour: Color,
    pub radius: f32
} impl VerletObject{

    pub fn new(pos: Vec2) -> VerletObject {
        VerletObject{
            current_position: pos,
            previous_position: pos,
            acceleration: Vec2::ZERO,
            colour: RED,
            radius: 10.0
        }
    }

    pub fn draw(&self){
        draw_circle(
            self.current_position.x, 
            self.current_position.y, 
            self.radius, 
            self.colour);
    }

    pub fn update_position(&mut self, dt: f32){
        let vel = self.current_position - self.previous_position;
        self.previous_position = self.current_position;
        
        self.current_position += vel + self.acceleration * (dt * dt);
        self.acceleration = Vec2::ZERO;
    }

    pub fn accelerate(&mut self, acc: Vec2){
        self.acceleration += acc;
    }
}