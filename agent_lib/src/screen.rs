use crate::boid::Boid;
use macroquad::prelude::*;

pub struct Screen {
    width: f32,
    height: f32,
    depth: f32,
}

impl Screen {
    pub fn new(width: f32, height: f32, depth: f32) -> Self {
        Self { width, height, depth }
    }

    pub fn draw_agents(&self, agents: &[Boid]) {
        clear_background(BLACK);
        
        // Setup 3D camera - orbiting horizontally around the cube
        let time = get_time() as f32;
        let orbit_speed = 0.3;
        let angle = time * orbit_speed;
        let distance = (self.width.max(self.height).max(self.depth) * 1.5).max(400.0);  // Halved from 800 to maintain visual size
        
        let center_x = self.width / 2.0;
        let center_y = self.height / 2.0;
        let center_z = 0.0;
        
        let camera_x = center_x + angle.cos() * distance * 0.9;
        let camera_z = center_z + angle.sin() * distance * 0.9;
        let camera_y = self.height * 1.3;
        
        set_camera(&Camera3D {
            position: vec3(camera_x, camera_y, camera_z),
            target: vec3(center_x, center_y, center_z),
            up: vec3(0.0, 1.0, 0.0),
            fovy: 45.0,
            projection: Projection::Perspective,
            ..Default::default()
        });

        // Draw simulation space bounding box
        self.draw_bounding_box();

        // Draw agents as cones pointing in their direction
        for agent in agents {
            self.draw_cone_for_agent(agent);
        }

        // Reset to default 2D camera
        set_default_camera();
    }

    fn draw_bounding_box(&self) {
        // Define 8 corners of the bounding box
        let corners = [
            vec3(0.0, 0.0, -self.depth / 2.0),              // 0: front-bottom-left
            vec3(self.width, 0.0, -self.depth / 2.0),       // 1: front-bottom-right
            vec3(self.width, self.height, -self.depth / 2.0), // 2: front-top-right
            vec3(0.0, self.height, -self.depth / 2.0),      // 3: front-top-left
            vec3(0.0, 0.0, self.depth / 2.0),               // 4: back-bottom-left
            vec3(self.width, 0.0, self.depth / 2.0),        // 5: back-bottom-right
            vec3(self.width, self.height, self.depth / 2.0),  // 6: back-top-right
            vec3(0.0, self.height, self.depth / 2.0),       // 7: back-top-left
        ];
        
        // Draw front face
        draw_line_3d(corners[0], corners[1], WHITE);
        draw_line_3d(corners[1], corners[2], WHITE);
        draw_line_3d(corners[2], corners[3], WHITE);
        draw_line_3d(corners[3], corners[0], WHITE);
        
        // Draw back face
        draw_line_3d(corners[4], corners[5], WHITE);
        draw_line_3d(corners[5], corners[6], WHITE);
        draw_line_3d(corners[6], corners[7], WHITE);
        draw_line_3d(corners[7], corners[4], WHITE);
        
        // Draw connecting edges
        draw_line_3d(corners[0], corners[4], WHITE);
        draw_line_3d(corners[1], corners[5], WHITE);
        draw_line_3d(corners[2], corners[6], WHITE);
        draw_line_3d(corners[3], corners[7], WHITE);
    }

    fn draw_cone_for_agent(&self, agent: &Boid) {
        let pos = &agent.agent.pos;
        let vel = &agent.agent.vel;
        
        // Cone dimensions
        let cone_length = 8.0;
        let cone_radius = 2.5;
        let num_segments = 8;
        
        // Normalize velocity to get direction
        let speed_sq = vel.x * vel.x + vel.y * vel.y + vel.z * vel.z;
        if speed_sq < 0.0001 {
            // If not moving, just draw a small sphere
            draw_sphere(vec3(pos.x, pos.y, pos.z), 2.0, None, BLUE);
            return;
        }
        
        let speed = speed_sq.sqrt();
        let dir_x = vel.x / speed;
        let dir_y = vel.y / speed;
        let dir_z = vel.z / speed;
        
        // Cone tip (pointing in direction of movement)
        let tip = vec3(
            pos.x + dir_x * cone_length,
            pos.y + dir_y * cone_length,
            pos.z + dir_z * cone_length,
        );
        
        // Create a perpendicular vector for the base circle
        let (perp_x, perp_y, perp_z) = if dir_y.abs() < 0.9 {
            // Cross product with Y axis
            let px = dir_z;
            let py = 0.0;
            let pz = -dir_x;
            let plen = (px * px + pz * pz).sqrt();
            (px / plen, py, pz / plen)
        } else {
            // Cross product with X axis
            let px = 0.0;
            let py = -dir_z;
            let pz = dir_y;
            let plen = (py * py + pz * pz).sqrt();
            (px, py / plen, pz / plen)
        };
        
        // Second perpendicular (cross product of direction and first perp)
        let perp2_x = dir_y * perp_z - dir_z * perp_y;
        let perp2_y = dir_z * perp_x - dir_x * perp_z;
        let perp2_z = dir_x * perp_y - dir_y * perp_x;
        
        // Draw cone base circle and lines to tip
        for i in 0..num_segments {
            let angle1 = (i as f32) * std::f32::consts::TAU / (num_segments as f32);
            let angle2 = ((i + 1) as f32) * std::f32::consts::TAU / (num_segments as f32);
            
            let cos1 = angle1.cos();
            let sin1 = angle1.sin();
            let cos2 = angle2.cos();
            let sin2 = angle2.sin();
            
            // Points on base circle
            let base1 = vec3(
                pos.x + (perp_x * cos1 + perp2_x * sin1) * cone_radius,
                pos.y + (perp_y * cos1 + perp2_y * sin1) * cone_radius,
                pos.z + (perp_z * cos1 + perp2_z * sin1) * cone_radius,
            );
            
            let base2 = vec3(
                pos.x + (perp_x * cos2 + perp2_x * sin2) * cone_radius,
                pos.y + (perp_y * cos2 + perp2_y * sin2) * cone_radius,
                pos.z + (perp_z * cos2 + perp2_z * sin2) * cone_radius,
            );
            
            // Draw base circle edge
            draw_line_3d(base1, base2, BLUE);
            
            // Draw line from base to tip
            draw_line_3d(base1, tip, BLUE);
        }
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }
}

