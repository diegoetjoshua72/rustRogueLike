pub struct Rect {
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Rect {
        //x y here are the coordinates of the rectangle start
        //so all we need is sort of this L to define the rectangle

        //x1 y1 are the coordinates for the first pixel
        //y0 is the first row of characters starting from the top
        //x0 is the first character to the left
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    //this is a pure function i see because we pass in parameters but it can't modify anything
    //as everything that is passed as parameter is immutable
    pub fn intersect(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    pub fn center(&self) -> (i32, i32) {
        ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }
}
