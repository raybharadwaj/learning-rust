pub fn run() {
    // structs_fundamentals();
    // structs_methods_associated_fns();
}

fn structs_fundamentals() {
    // Struct or Structure
    // holds related values together to make a meaningful group
    // Together with enums, fundamental to creating custom types
    {
        struct User {
            active: bool,
            username: String,
            email: String,
            sign_in_count: u64
        }
    }

    // Tuple vs Struct
    {
        let user: (bool, String, String, u64) = (true,
                                                     String::from("ray"),
                                                     String::from("hmsa.creator@gmail.com"),
                                                     355);

        let(active, username, email, sign_in_count) = user;
        println!("{} {} {} {}", active, username, email, sign_in_count);

        //Struct
        struct User {
            active: bool,
            username: String,
            email: String,
            sign_in_count: u64
        }

        // Entire instance should be mutable if you want any property to be mutable
        let mut ray = User {
            username: String::from("ray"),
            // no &str, this is to let the Struct have full ownership. not mere reference
            email: String::from("hmsa.developer@gmail.com"),
            active: false,
            sign_in_count: 358
        };

        ray.username = String::from("hmsa.ray");
        println!("{} {} {} {}", ray.active, ray.username, ray.email, ray.sign_in_count);
        // Behavior is written through methods
    }

    // Constructor with pre-determined values and Struct update
    {
        struct User {
            active: bool,
            username: String,
            email: String,
            sign_in_count: u64
        }



        fn build_user(email: String, username: String) -> User {
            User {
                email,
                username,
                active: true,
                sign_in_count: 359
            }
        }

        let sunny = build_user(String::from("sun@uni.edu"), String::from("sun"));
        println!("{} {} {} {}",
                 sunny.email,
                 sunny.username,
                 sunny.active,
                 sunny.sign_in_count);

        //struct update syntax
        let sunny2 = User {
            username: String::from("Ray"),
            ..sunny
        };

        println!("{} {} {} {}",
                 sunny2.email,
                 sunny2.username,
                 sunny2.active,
                 sunny2.sign_in_count);
    }

}

fn structs_methods_associated_fns() {
    // Programs to create a function to show area of a rectangle
    {
        // separate variables. not connected by any explicit means
        {
            let width = 30;
            let height= 50;

            println!("Area 1: {}", area(width, height));

            fn area(width: u32, height: u32) -> u32 {
                width * height
            }
        }

        //Tuple format - Connected together but missing labels
        {
            let rect = (40, 50);
            println!("Area 2: {}", area(rect));
            // Have to use the index in tuples
            fn area(dimensions: (u32, u32)) -> u32 {
                dimensions.0 * dimensions.1
            }
        }

        // Using Structs and methods
        {
            // structs definition are before main scope
            #[derive(Debug)] // imports debug functionality. can now use {:?}
            struct Rectangle {
                width: u32,
                height: u32
            }

            impl Rectangle {
                // methods
                // Take in &self to perform operations on the struct vals
                fn area(&self) -> u32 {
                    self.width * self.height
                }

                fn can_hold(&self, other: &Rectangle) -> bool {
                    self.width > other.width && self.height > other.height
                }

                fn print_struct(&self) {
                    println!("{:#?}", &self);
                }

                // associated function - Does not take in a &self
                fn square(size: u32) -> Self {
                    Self {
                        width: size,
                        height: size
                    }
                }

            }

            // fn main()
            {
                let rect1 = Rectangle {
                    width: 50,
                    height: 50
                };

                println!("Area 3: {}", area(&rect1));
                // println!("Rect1 is {}" rect1); // will not work
                // {} say we need something which implements std::fmt::Display
                println!("Rect1 is {:?}", rect1); // not pretty - one line
                println!("Rect1 is {:#?}", rect1); // pretty - use for larger structs
                dbg!(&rect1); // shows prettified objects

                // with struct method
                println!("Area 4: {}", rect1.area());

                let rect2 = Rectangle {
                    width: 70,
                    height: 40
                };

                let rect3 = Rectangle {
                    width: 100,
                    height: 300
                };

                println!("rect1: {:#?}", rect1);
                println!("rect2:",);
                rect2.print_struct();

                println!("rect3: {:#?}", rect3);

                println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
                println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));
                println!("Can rect3 hold rect2? {}", rect3.can_hold(&rect2));
            }


            fn area(rectangle : &Rectangle) -> u32 {
                rectangle.width * rectangle.height
            }
        }
    }
}