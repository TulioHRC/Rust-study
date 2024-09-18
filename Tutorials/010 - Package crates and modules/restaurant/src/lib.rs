mod front_of_house {
    // the function example has access to this module because it is exposed to the same path of the function, but its child not
    pub mod hosting { // pub exposes this path to whoever have access to the parent path
        pub fn add_to_waitlist() {}
    }

    fn test() {
        hosting::add_to_waitlist();
        super::example(); // super goes to the parent of the actual, in this case goes to crate
    }
}

pub fn example() {
    crate::front_of_house::hosting::add_to_waitlist(); // absolute path
    front_of_house::hosting::add_to_waitlist(); // relative path
}
