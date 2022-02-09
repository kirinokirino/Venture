pub mod road;
pub mod stone;
pub mod terrain;

use std::cmp::{Eq, Ord, Ordering, PartialEq};

#[derive(Debug)]
pub enum Static {
    Stone(stone::Stone),
    Road(road::Segment),
    Terrain(terrain::Terrain),
}

impl PartialEq for Static {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Stone(_) => match other {
                Self::Stone(_) => true,
                _ => false,
            },
            Self::Road(_) => match other {
                Self::Road(_) => true,
                _ => false,
            },
            Self::Terrain(_) => match other {
                Self::Terrain(_) => true,
                _ => false,
            },
        }
    }
}

impl Eq for Static {}

impl Ord for Static {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Self::Stone(_) => match other {
                Self::Stone(_) => Ordering::Equal,
                _ => Ordering::Greater,
            },
            Self::Road(_) => match other {
                Self::Road(_) => Ordering::Equal,
                Self::Stone(_) => Ordering::Less,
                _ => Ordering::Greater,
            },
            Self::Terrain(_) => match other {
                Self::Terrain(_) => Ordering::Equal,
                _ => Ordering::Less,
            },
        }
    }
}

impl PartialOrd for Static {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
