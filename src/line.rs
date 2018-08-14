use lina::Vector2;

type Point = Vector2<i32>;

struct Octant(u8);

impl Octant {
    /// adapted from http://codereview.stackexchange.com/a/95551
    fn from_points(start: Point, end: Point) -> Octant {
        let mut dx = end.0 - start.0;
        let mut dy = end.1 - start.1;

        let mut octant = 0;

        if dy < 0 {
            dx = -dx;
            dy = -dy;
            octant += 4;
        }

        if dx < 0 {
            let tmp = dx;
            dx = dy;
            dy = -tmp;
            octant += 2
        }

        if dx < dy {
            octant += 1
        }

        Octant(octant)
    }

    fn to_octant0(&self, p: Point) -> Point {
        match self.0 {
            0 => Point::new(p.0, p.1),
            1 => Point::new(p.1, p.0),
            2 => Point::new(p.1, -p.0),
            3 => Point::new(-p.0, p.1),
            4 => Point::new(-p.0, -p.1),
            5 => Point::new(-p.1, -p.0),
            6 => Point::new(-p.1, p.0),
            7 => Point::new(p.0, -p.1),
            _ => unreachable!(),
        }
    }

    fn from_octant0(&self, p: Point) -> Point {
        match self.0 {
            0 => Point::new(p.0, p.1),
            1 => Point::new(p.1, p.0),
            2 => Point::new(-p.1, p.0),
            3 => Point::new(-p.0, p.1),
            4 => Point::new(-p.0, -p.1),
            5 => Point::new(-p.1, -p.0),
            6 => Point::new(p.1, -p.0),
            7 => Point::new(p.0, -p.1),
            _ => unreachable!(),
        }
    }
}

pub struct Line {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    x1: i32,
    diff: i32,
    octant: Octant,
    steps: i32,
    current: i32,
}

impl Line {
    pub fn new(start: Point, end: Point) -> Line {
        let octant = Octant::from_points(start, end);

        let start = octant.to_octant0(start);
        let end = octant.to_octant0(end);

        let dx = end.0 - start.0;
        let dy = end.1 - start.1;

        Line {
            x: start.0,
            y: start.1,
            dx: dx,
            dy: dy,
            x1: end.0,
            diff: dy - dx,
            octant: octant,
            steps: (if dx > dy { dx } else { dy }) + 1,
            current: 0,
        }
    }
}

impl Iterator for Line {
    type Item = (Point, f32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.x1 {
            return None;
        }

        let p = Point::new(self.x, self.y);

        if self.diff >= 0 {
            self.y += 1;
            self.diff -= self.dx;
        }

        self.diff += self.dy;

        // loop inc
        self.x += 1;
        self.current += 1;
        Some((
            self.octant.from_octant0(p),
            self.current as f32 / self.steps as f32,
        ))
    }
}
