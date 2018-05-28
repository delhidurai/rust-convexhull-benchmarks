//! Points types for finding canvex hull

///A basic representation of a point
///
///With x and y coordinate, a point2D
///is a point in 2D euclidean space
///
/// #Example
///
/// ```
/// let point = Point2D {x: 1.0, y: 2.0}
/// ```
#[derive(Debug)]
pub struct Point2D {
    /// x-coordinate value
    x: f64,
    /// y-coordinate value
    y: f64,
}

impl PartialEq for Point2D {
    /// compare 2 points using = sign
    /// and return true when both x and y
    /// coordinate are same
    fn eq(&self, other: &Point2D) -> bool {
        self.x == other.x && self.y == other.y
    }
}

///Implementation methods for Point2D type
///
/// Some handy methods find the convex hull
/// using the point
impl Point2D {
    ///Constructor for Point2D
    ///
    /// #Example
    /// ```
    /// let point = Point2D::new(1.0,2.0)
    /// ```
    pub fn new(x: f64, y: f64) -> Point2D {
        Point2D { x, y }
    }

    ///Comparision of point position relative to another point
    ///
    ///Given two points we need to left most point.
    /// This is used to find the pivot point of the vertex point
    /// of the hull.
    pub fn pick_left<'a>(&'a self, other: &'a Point2D) -> &'a Point2D {
        //when both the points are same, return the other point
        if self == other {
            return other;
        } else if self.y != other.y {
            // else return the point with min y-coordinate
            if self.y < other.y {
                return self;
            } else {
                return other;
            }
        } else {
            // when y-coordinates are same return the point with min x-coordinate
            if self.x < other.x {
                return self;
            } else {
                return other;
            }
        }
    }

    ///Determine the turn direction around the corner
    /// formed by the points a, b and c.
    ///
    /// Return true for counterclockwise turn
    /// and false for colinearity or clockwise turns.
    ///
    /// #Examples
    /// ```
    /// let point_a = Point2D::new(1.0, 1.0);
    /// let point_b = Point2D::new(2.0, 2.0);
    /// let point_c = Point2D::new(3.0, 2.5);
    /// assert_eq!(false, point_a.ccw(&point_b, &point_c));
    ///
    /// let point_a = Point2D::new(0.0, 0.0);
    /// let point_b = Point2D::new(1.0, 1.0);
    /// let point_c = Point2D::new(2.0, 0.0);
    /// assert_eq!(true, point_a.ccw(&point_c, &point_b));
    /// ```
    ///
    pub fn ccw(&self, point_b: &Point2D, point_c: &Point2D) -> bool {
        (point_b.x - self.x) * (point_c.y - self.y) - (point_b.y - self.y) * (point_c.x - self.x)
            > 0.0
    }

    ///Determine the distance between 2 points
    fn compute_distance(point1: &Point2D, point2: &Point2D) -> f64 {
        ((point1.x - point2.x).powi(2) + (point1.y - point2.y).powi(2)).sqrt()
    }

    ///Determine the polarangle between 2 points
    fn compute_angle(point1: &Point2D, point2: &Point2D) -> f64 {
        (point2.y - point1.y).atan2(point2.x - point1.x)
    }
}
