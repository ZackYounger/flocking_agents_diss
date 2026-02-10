use crate::vec::Vec2;
use crate::spatial_hash::AccelerationStructureTrait;

// Axis-Aligned Bounding Box for 3D space
#[derive(Debug, Clone, Copy)]
pub struct AABB {
    pub center: Vec2,
    pub half_size: Vec2,
}

impl AABB {
    pub fn new(center: Vec2, half_size: Vec2) -> Self {
        Self { center, half_size }
    }

    // Check if a point is inside this bounding box
    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.center.x - self.half_size.x
            && point.x <= self.center.x + self.half_size.x
            && point.y >= self.center.y - self.half_size.y
            && point.y <= self.center.y + self.half_size.y
            && point.z >= self.center.z - self.half_size.z
            && point.z <= self.center.z + self.half_size.z
    }

    // Check if this AABB intersects with a sphere (point + radius)
    pub fn intersects_sphere(&self, center: Vec2, radius: f32) -> bool {
        // Find the closest point on the AABB to the sphere center
        let closest_x = center.x.clamp(
            self.center.x - self.half_size.x,
            self.center.x + self.half_size.x,
        );
        let closest_y = center.y.clamp(
            self.center.y - self.half_size.y,
            self.center.y + self.half_size.y,
        );
        let closest_z = center.z.clamp(
            self.center.z - self.half_size.z,
            self.center.z + self.half_size.z,
        );

        // Calculate distance from closest point to sphere center
        let dx = center.x - closest_x;
        let dy = center.y - closest_y;
        let dz = center.z - closest_z;
        let distance_sq = dx * dx + dy * dy + dz * dz;

        distance_sq <= radius * radius
    }
}

// A single node in the octree
// The octree recursively subdivides 3D space into 8 octants

// Each octant is numbered 0-7 based on which side of each axis the point falls
// Bit 0 (1): x >= center.x
// Bit 1 (2): y >= center.y  
// Bit 2 (4): z >= center.z
#[derive(Debug)]
pub struct OctreeNode {
    pub boundary: AABB,
    pub points: Vec<(usize, Vec2)>, // (index, position)
    pub children: Option<Box<[OctreeNode; 8]>>, //None if this is a leaf
    pub capacity: usize,
    pub max_depth: usize,
    pub depth: usize,
}

impl OctreeNode {
    pub fn new(boundary: AABB, capacity: usize, max_depth: usize, depth: usize) -> Self {
        Self {
            boundary,
            points: Vec::with_capacity(capacity),
            children: None,
            capacity,
            max_depth,
            depth,
        }
    }

    // Insert a point into the octree
    // Returns true if the point was inserted successfully
    pub fn insert(&mut self, index: usize, point: Vec2) -> bool {
        // Check if point is within this node's boundary
        if !self.boundary.contains(point) {
            return false;
        }

        // If we have children, put it there
        if let Some(ref mut children) = self.children {
            let octant = OctreeNode::get_octant_static(point, self.boundary.center);
            return children[octant].insert(index, point);
        }

        // If we have room or can't subdivide further, add the point here
        if self.points.len() < self.capacity || self.depth >= self.max_depth {
            self.points.push((index, point));
            return true;
        }

        // Subdivide and redistribute points
        self.subdivide();
        
        // Move existing points to the new children
        let old_points: Vec<_> = self.points.drain(..).collect();
        for (idx, pos) in old_points {
            let octant = OctreeNode::get_octant_static(pos, self.boundary.center);
            if let Some(ref mut children) = self.children {
                children[octant].insert(idx, pos);
            }
        }

        // Insert the new point in the correct child
        let octant = OctreeNode::get_octant_static(point, self.boundary.center);
        if let Some(ref mut children) = self.children {
            children[octant].insert(index, point)
        } else {
            false
        }
    }

    // Determine which octant a point belongs to, relative to a given center
    // Returns 0-7
    fn get_octant_static(point: Vec2, center: Vec2) -> usize {
        let mut octant = 0;
        if point.x >= center.x {
            octant |= 1;
        }
        if point.y >= center.y {
            octant |= 2;
        }
        if point.z >= center.z {
            octant |= 4;
        }
        octant
    }

    /// Subdivide this node into 8 children
    fn subdivide(&mut self) {
        let half = self.boundary.half_size;
        let quarter = Vec2::new_3d(half.x / 2.0, half.y / 2.0, half.z / 2.0);
        let center = self.boundary.center;

        // Create 8 children, one for each octant
        // Octant numbering:
        //   0: -x, -y, -z    
        //   1: +x, -y, -z    
        //   2: -x, +y, -z    
        //   3: +x, +y, -z    
        //   4: -x, -y, +z
        //   5: +x, -y, +z
        //   6: -x, +y, +z
        //   7: +x, +y, +z
        let offsets = [
            Vec2::new_3d(-quarter.x, -quarter.y, -quarter.z), // 0
            Vec2::new_3d( quarter.x, -quarter.y, -quarter.z), // 1
            Vec2::new_3d(-quarter.x,  quarter.y, -quarter.z), // 2
            Vec2::new_3d( quarter.x,  quarter.y, -quarter.z), // 3
            Vec2::new_3d(-quarter.x, -quarter.y,  quarter.z), // 4
            Vec2::new_3d( quarter.x, -quarter.y,  quarter.z), // 5
            Vec2::new_3d(-quarter.x,  quarter.y,  quarter.z), // 6
            Vec2::new_3d( quarter.x,  quarter.y,  quarter.z), // 7
        ];

        let children: [OctreeNode; 8] = std::array::from_fn(|i| {
            let child_center = center + offsets[i];
            let child_boundary = AABB::new(child_center, quarter);
            OctreeNode::new(child_boundary, self.capacity, self.max_depth, self.depth + 1)
        });

        self.children = Some(Box::new(children));
    }

    // Query all points within a spherical radius of a given position
    pub fn query_radius(&self, center: Vec2, radius: f32, results: &mut Vec<usize>) {

        if !self.boundary.intersects_sphere(center, radius) {
            return;
        }

        // Check points in this node
        let radius_sq = radius * radius;
        for &(index, pos) in &self.points {
            let diff = pos - center;
            if diff.length_sq() <= radius_sq {
                results.push(index);
            }
        }

        // Recursively query children
        if let Some(ref children) = self.children {
            for child in children.iter() {
                child.query_radius(center, radius, results);
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct OctreeStats {
    pub total_nodes: usize,
    pub leaf_nodes: usize,
    pub max_depth: usize,
    pub total_points: usize,
}


pub struct Octree {
    root: OctreeNode,
}

impl Octree {

    pub fn new(width: f32, height: f32, depth: f32, capacity: usize, max_depth: usize) -> Self {
        let half_size = Vec2::new_3d(width / 2.0, height / 2.0, depth / 2.0);
        let center = Vec2::new_3d(width / 2.0, height / 2.0, 0.0);
        let boundary = AABB::new(center, half_size);
        
        Self {
            root: OctreeNode::new(boundary, capacity, max_depth, 0),
        }
    }

    // Build the octree from a slice of positions
    // This clears existing data
    pub fn build(&mut self, positions: &[Vec2]) {
        let boundary = self.root.boundary;
        let capacity = self.root.capacity;
        let max_depth = self.root.max_depth;
        self.root = OctreeNode::new(boundary, capacity, max_depth, 0);

        // Insert all positions
        for (i, &pos) in positions.iter().enumerate() {
            self.root.insert(i, pos);
        }
    }

    // Query all point indices within a radius of the given position
    pub fn query(&self, pos: Vec2, radius: f32) -> Vec<usize> {
        let mut results = Vec::new();
        self.root.query_radius(pos, radius, &mut results);
        results
    }
}

impl AccelerationStructureTrait for Octree {
    fn build(&mut self, positions: &[Vec2]) {
        Octree::build(self, positions);
    }
    fn query(&self, pos: Vec2, radius: f32) -> Vec<usize> {
        Octree::query(self, pos, radius)
    }
}