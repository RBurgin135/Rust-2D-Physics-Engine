use macroquad::{prelude::*, rand::RandomRange};
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

    pub fn update(&mut self, dt: i32){
        //calculate sub steps
        const SUB_STEPS: i16 = 2;
        let sub_dt: f32 = (dt as f32) / (SUB_STEPS as f32);
        for _ in 0..SUB_STEPS{

            //calculate physics
            self.apply_gravity();
            self.apply_circular_constraint();
            self.solve_collisions();
            self.update_positions(sub_dt);
            self.render();
        }
    }

    pub fn generate_verlet(&mut self){
        //calculate starting details
        let pos = Vec2::new(
            self.border_position.x, 
            self.border_position.y-20.0
        );
        let acc = Vec2::new(
            RandomRange::gen_range(-10,10) as f32, 
            RandomRange::gen_range(-10,10) as f32
        );
        let rad = RandomRange::gen_range(
            5,
            12
        ) as f32;
        let colour = Color::new(
            RandomRange::gen_range(0,255) as f32/255.0, 
            RandomRange::gen_range(0,255) as f32/255.0, 
            RandomRange::gen_range(0,255) as f32/255.0, 
            1.0);


        //create new verlet
        let new_verlet = VerletObject::new(pos,rad,acc,colour);

        //add new verlet
        self.add_verlet(new_verlet);
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

    fn update_positions(&mut self, dt: f32){
        for v in &mut self.verlets {
            v.update_position(dt);
        }
    }

    fn solve_collisions(&mut self) {
        let len = self.verlets.len();
    
        //iterate over collection
        for x in 0..len {
            let (first_seg, second_seg) = self.verlets.split_at_mut(x+1);

            //define verlets
            let cv = &mut first_seg[x];
            for tv in second_seg{

                //collision
                let collision_axis = cv.cur_pos - tv.cur_pos;
                let dist = collision_axis.length();

                let min_dist = cv.radius + tv.radius;
                if dist < min_dist {
                    let n = collision_axis / dist;
                    let d = min_dist - dist;
                    
                    //enact collision
                    cv.cur_pos += 0.5 * d * n;
                    tv.cur_pos -= 0.5 * d * n;
                }
            }
        }
    }
    

    
    fn apply_circular_constraint(&mut self) {
        for v in &mut self.verlets {
            //find displacement and distance
            let displacement = v.cur_pos - self.border_position;
            let distance = displacement.length();

            //if verlet is outside border
            if distance > self.border_radius - v.radius {
                let n = displacement / distance;
                v.cur_pos = self.border_position + n * (self.border_radius - v.radius);
            }
            
        }
    }


    fn render(&mut self){
        //draw border
        draw_circle(
            self.border_position.x, 
            self.border_position.y, 
            self.border_radius,
            DARKGRAY);

        //draw verlets
        for v in &mut self.verlets{
            v.draw();
        }
    }



}