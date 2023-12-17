use grid::Grid;

#[macro_export]
macro_rules! regex {
    ($re:expr $(,)?) => {{
        static LAZY: once_cell::sync::Lazy<regex::Regex> =
            once_cell::sync::Lazy::new(|| regex::Regex::new($re).unwrap());
        &LAZY
    }};
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}


pub trait CoordExt {
    fn left(&self) -> (i64, i64);
    fn right(&self) -> (i64, i64);
    fn up(&self) -> (i64, i64);
    fn down(&self) -> (i64, i64);
}

impl CoordExt for (i64, i64) {
    fn left(&self) -> (i64, i64) {
        (self.0, self.1 - 1)
    }

    fn right(&self) -> (i64, i64) {
        (self.0, self.1 + 1)
    }

    fn up(&self) -> (i64, i64) {
        (self.0 - 1, self.1)
    }

    fn down(&self) -> (i64, i64) {
        (self.0 + 1, self.1)
    }
}

pub trait GridExt<T> {
    // Grid expects usize, and we always need to manually check for bounds which is annoying.
    fn get_coordinate(&self, coord: (i64, i64)) -> Option<&T>;
}

impl<T> GridExt<T> for Grid<T> {
    fn get_coordinate(&self, coord: (i64, i64)) -> Option<&T> {
        // if we didn't fall off map (usize)
        if (coord.0 < 0) || (coord.1 < 0) {
            None
        } else {
            self.get(coord.0 as usize, coord.1 as usize)
        }
    }
}
