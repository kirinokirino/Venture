pub mod road;
pub mod stone;

#[derive(Debug)]
pub enum Static {
    Stone(stone::Stone),
    Road(road::Segment),
}
