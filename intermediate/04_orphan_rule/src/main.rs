/*
    ORPHAN RULE
    Un Trait puede ser implementado en un tipo si uno de los dos pertenece al crate actual. Es decir, si se ambos el trait y el tipo son de crates distintos al actual no puede implmentarse el trait en ese tipo.
*/
use orphan_rule::Point;

// Definimos una struct para envolver el struct externa y tener una local que pueda implementar el trait
struct PointWrapper(Point);

impl PartialEq for PointWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0.x == other.0.x && self.0.y == other.0.y
    }
}

fn main() {
    let p1 = PointWrapper(Point { x: 1, y: 1 });
    let p2 = PointWrapper(Point { x: 1, y: 1 });

    println!("{}", p1 == p2);
}
