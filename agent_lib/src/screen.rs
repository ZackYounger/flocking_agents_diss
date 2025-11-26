use crate::agent::Agent;
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

    pub fn draw_agents(&self, agents: &[Box<dyn Agent>]) {
        clear_background(BLACK);
        
        // Setup 3D camera - orbiting horizontally around the cube
        let time = get_time() as f32;
        let orbit_speed = 0.3;
        let angle = time * orbit_speed;
        let distance = (self.width.max(self.height).max(self.depth) * 1.5).max(800.0);
        
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

        // Draw agents as 3D spheres
        for agent in agents {
            let state = agent.state();
            draw_sphere(vec3(state.pos.x, state.pos.y, state.pos.z), 5.0, None, BLUE);
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

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }
}

