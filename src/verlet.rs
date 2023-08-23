use macroquad::prelude::*;

pub struct VerletObject{
    pub cur_pos: Vec2,
    pub prev_pos: Vec2,
    pub acc: Vec2,
    colour: Color,
    pub radius: f32
} impl VerletObject{

    pub fn new(pos: Vec2, rad: f32, acc:Vec2, colour: Color) -> VerletObject {
        VerletObject{
            cur_pos: pos,
            prev_pos: pos,
            acc: acc,
            colour: colour,
            radius: rad
        }
    }

    pub fn draw(&self){
        draw_circle(
            self.cur_pos.x, 
            self.cur_pos.y, 
            self.radius, 
            self.colour);
    }

    pub fn update_position(&mut self, dt: f32){
        let vel = self.cur_pos - self.prev_pos;
        self.prev_pos = self.cur_pos;
        
        self.cur_pos += vel + self.acc * (dt * dt);
        self.acc = Vec2::ZERO;
    }

    pub fn accelerate(&mut self, acc: Vec2){
        self.acc += acc;
    }
}