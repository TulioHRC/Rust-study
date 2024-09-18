mod my_restaurant {
    pub struct Breakfast { 
        pub snack: String, // Now you can access directly this variable and change it
        fruit: String
    }

    impl Breakfast {
        pub fn display_breakfast(&self) { // by default it's private
            println!("We have {}, and {} fruit.", self.snack, self.fruit);
        }

        pub fn build_breakfast(toast: String) -> Self {
            return Breakfast {
                snack: toast,
                fruit: String::from("apple")
            };
        }
    }   
}

fn summer_breakfast(toast: String) {
    let mut breakfast = my_restaurant::Breakfast::build_breakfast(toast);
    breakfast.display_breakfast();

    breakfast.snack = String::from("pizza");
    breakfast.display_breakfast();
} 

fn main() {
    summer_breakfast(String::from("bread"));
}