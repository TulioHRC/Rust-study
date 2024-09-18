mod animals;

pub use crate::animals::fish;

pub fn display_fish() {
    fish::display();
}