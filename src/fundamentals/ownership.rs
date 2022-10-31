pub fn run() {
    // borrowing();
    // references();
    // slice_type();
}

fn borrowing() {
    {
        let s1 = String::from("World");
        {
            let (s2, len) = calculate_length(s1);
            //s1 is moved to fn, returned as s2
            // println!("{}", s1); //s1 is no longer valid
            println!("{} is {} letters long", s2, len);
        }
        // println!("{}", s1); //s1 is no longer valid


        fn calculate_length(mut s: String) -> (String, usize) {
            s.push_str(", Hello!"); //can change borrowed values
            let length = s.len();
            //must reassign because .len() borrows and returns s
            //function has no ownership of the return value of len()
            //hence we reassign to take ownership of that generated val
            (s, length)
        }
    }
}

fn references() {

    // borrow immutable but sent back as a tuple and shadowed by s1
    {
        let s1 = String::from("...still here");
        let (s1, len) = calc_length(s1); //s from fn shadows over the moved s1
        println!("{} {}", s1, len);

        fn calc_length(s: String) -> (String, usize) {
            // (s, s.len()) // won't work because s will be borrowed for .len()
            let length = s.len(); // borrowed s is returned after operation
            (s, length)
        }
    }

    // borrow only reference, can still do operations with reference
    {
        let s1 = String::from("I will never leave");
        let len = calc_length(&s1);
        // no need of reassignment of s1 because the value was never moved
        println!("{} {}", s1, len);

        // s is a ref variable - only has a pointer stored in it.
        fn calc_length(s: &String) -> usize {
            s.len() // nothing is borrowed, only referenced
        }
    }

    // Changing values of immutable variables through a function, shadowing
    {
        // borrow immutable but take into function as immutable
        // assign returned value to be shadowed into its original name
        let s1 = String::from("Hello"); // immutable value
        let s1 = change(s1);
        // still immutable but value changed because shadowed | 'let'
        println!("{}", s1);

        fn change(mut s: String) -> String {
            s.push_str(", World!"); // can only change inside the function
            s
        }
    }

    // mutable references
    {
        let mut s1 = String::from("Hello");
        change(&mut s1); // Sending a mutable reference
        println!("{}", s1);

        fn change(s: &mut String) {
            s.push_str(", World"); // can change outside value. s is a pointer
        }

        // mutable references restrictions:
        {
            // can only borrow as mutable once at a time
            // this is to mutate in a controllable fashion,
            // prevent data races at compile time and prevent undefined behavior
            {
                let mut s = String::from("Hello");
                let r1 = &mut s;
                // let r2 = &mut s; // won't allow it
                // println!("{} {}", r1, r2);
                println!("{}", r1);
            }

            // could however do this:
            {
                let mut s = String::from("Hello");
                {
                    let _r1 = &mut s;
                    // added _ to prevent compiler from complaining about unused var
                } // r1 is now out of scope
                let r2 = &mut s;
                println!("{}", r2);
            }

            // can't have a mutable reference while we have an immutable ref -
            // to the same value
            {
                // let mut s = String::from("Hello");
                let s = String::from("Hello");
                let r1 = &s;
                let r2 = &s; // allowed because r2 has no ability to change s value
                // let r3 = &mut s;
                // can't do this because an immutable ref already exists
                // println!("{}, {}, and {}", r1, r2, r3);
                println!("{} and {}", r1, r2);
            }

            // NLL Ability of the compiler to find internal reference scope
            {
                let mut s = String::from("hello");

                let r1 = &s;
                let r2 = &s;
                println!("{} and {}", r1, r2);
                // Non-lexical-lifetimes Ability of the compiler -
                // when references are last used, an internal scope ends
                // will now allow this
                let r3 = &mut s;
                println!("{}", r3);
            }
        }
    }

    // dangling references
    {
        //     {
        //         let reference_to_nothing = dangle();
        //         println!("{}", reference_to_nothing);
        //     }
        //
        //     fn dangle() -> &String {
        //         let s = String::from("never meant to be");
        //         &s
        //     } // the memory held by s and s itself will be destroyed here
        //     // &s points to nothing
        //     // this function's return type contains a borrowed value,
        //     // but there is no value for it to be borrowed from
    }
}

fn slice_type() {
    // String Slice - Reference to a part of the String.
    {
        let s = String::from("Hello world");
        let len = s.len();

        let mut hello = &s[0..5];
        let mut world = &s[6..len];
        let mut hello_world = &s[0..len];
        println!("{} + {} = {}", hello, world, hello_world);

        hello = &s[..5];
        world = &s[6..];
        hello_world = &s[..];
        println!("{} + {} = {}", hello, world, hello_world);
    }

    // first word from string with spaces - explanation
    {

        // fn first_word(s: &String) -> &str {
        // &String is also a slice, hence &str
        fn first_word(s: &str) -> &str {
            let bytes = s.as_bytes();
            // string converted into a collection: array of bytes

            // iter - iterator returns each element in the collection
            // enumerate - returns each element as a tuple with (index, element)
            //(i, &item) - destructuring what enumerate gives back.
            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' { // byte literal syntax
                    return &s[..i]; // changing the reference bounds
                } // if expects a () so we use return
            }

            // s.len()
            // only valid in context of &String,
            // so doubtful it will be valid in the future

            // example of above invalidity
            {
                //     let mut s = String::from("hello world!");
                //     let word = first_word(&s);
                //
                //     s.clear(); // s becomes "",
                // will cause error because a mutable reference is requested
                //     // but word now has a value of the usize returned by fn.
            }

            &s[..] // sending back only a reference pointer with current bounds
            // string literals &str are slices hence returning &str
        }
    }

    // first word from string with spaces - program
    {
        let string = String::from("Hello World");
        let hello = first_word(&string);
        println!("{}", hello);

        let string_literal = "World Hello";
        let hello = first_word(string_literal); // static types won't be moved
        println!("{}", hello);
        println!("{}", string_literal);

        fn first_word(s: &str) -> &str {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[..i];
                }
            }

            &s[..]
        }
    }

    // array slice
    {
        let a = [1, 2, 3, 4, 5];
        let slice = &a[0..=2];

        for item in slice {
            println!("{}", item);
        }

        assert_eq!(slice, &[1, 2, 3]); // Test
    }
}