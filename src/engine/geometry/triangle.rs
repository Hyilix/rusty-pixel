use crate::engine::geometry::point::Point;

pub struct Triangle {
    pub corner1: Point,
    pub corner2: Point,
    pub corner3: Point,
}

impl Default for Triangle {
    fn default() -> Self {
        Self::new(
            Point::pair_to_point((0, 0)),
            Point::pair_to_point((0, 1)),
            Point::pair_to_point((1, 0))
        )
    }
}

impl Triangle {
    pub fn new(corner1: Point, corner2: Point, corner3: Point) -> Self {
        Self {corner1, corner2, corner3}
    }
}

impl Triangle {
    fn sign(&self) -> i32 {
        (self.corner1.x - self.corner3.x) *
        (self.corner2.y - self.corner3.y) -
        (self.corner2.x - self.corner3.x) *
        (self.corner1.y - self.corner3.y)
    }

    pub fn contains(&self, point: Point) -> bool {
        let trig1 = Self::new(point, self.corner1, self.corner2);
        let trig2 = Self::new(point, self.corner3, self.corner1);
        let trig3 = Self::new(point, self.corner2, self.corner3);

        let s1 = trig1.sign();
        let s2 = trig2.sign();
        let s3 = trig3.sign();

        let neg = s1 < 0 || s2 < 0 || s3 < 0;
        let poz = s1 > 0 || s2 > 0 || s3 > 0;

        !(neg && poz)
    }

    // Get the area of the triangle using Heron's formula
    pub fn area(&self) -> f32 {
        let semi_perim = self.perimeter() / 2f32;
        let line12 = self.corner1.dist_to_other(&self.corner2);
        let line13 = self.corner1.dist_to_other(&self.corner3);
        let line23 = self.corner2.dist_to_other(&self.corner3);

        f32::sqrt(
            semi_perim *
            (semi_perim - line12) *
            (semi_perim - line13) *
            (semi_perim - line23)
        )
    }

    // Get the perimeter of the triangle
    pub fn perimeter(&self) -> f32 {
        let line12 = self.corner1.dist_to_other(&self.corner2);
        let line13 = self.corner1.dist_to_other(&self.corner3);
        let line23 = self.corner2.dist_to_other(&self.corner3);

        line12 + line13 + line23
    }
}

