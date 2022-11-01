pub fn run() {
    // Define type and enumerating it's possible variants.
    // Enums say that the value is one of a set of possible values
    // Camel Case
    {

        {
            enum IpAddrKind {
                V4,
                V6,
            }

            struct IpAddr {
                kind: IpAddrKind,
                address: String,
            }

            let home = IpAddr {
                kind: IpAddrKind::V4,
                address: String::from("127.0.0.1"),
            };

            let loopback = IpAddr {
                kind: IpAddrKind::V6,
                address: String::from("::1"),
            };
        }

        // better to define types within the enum
        {
            enum IpAddr {
                V4(u8, u8, u8, u8),
                V6(String),
            }

            let home = IpAddr::V4(127,0,0,1);
            let loopback = IpAddr::V6(String::from("::4"));
        }

        // Enums can contain anything inside - Structs and even other enums
        {
            #[derive(Debug)]
            enum Message {
                Quit, // No data associated
                Move { x: i32, y: i32 }, // Struct
                Write(String), // Heap stored String
                ChangeColor(i32, i32, i32) // tuple with 3 i32 values
            }

            // the above is similar to defining structs for values
            {
                struct QuitMessage; // unit struct
                struct MoveMessage {
                    x: i32,
                    y: i32,
                }
                struct WriteMessage(String); // tuple struct
                struct ChangeColorMessage(i32, i32, i32); // tuple struct
            }
            // But there would now be 4 types instead of 1

            // We can define functions like structs using impl

            impl Message {
                fn print(&self) {
                    println!("{:?}", &self)
                }
            }

            let q = Message::Quit;
            let m = Message::Write(String::from("Task updated!"));
            m.print();
        }
    }

    // Option Enum
    {

    }


    // trials
    // {
    //     #[derive(Debug)]
    //     #[derive(PartialEq)]
    //     enum BasicPolygons {
    //         Triangle,
    //         Quadrilateral,
    //         Pentagon,
    //         Hexagon,
    //         Heptagon,
    //         Octagon,
    //         Nonagon,
    //         Decagon
    //     }
    //
    //     // Can now create instances of the 8 variants
    //     // namespaced under its identifier - BasicPolygons
    //     // Adds a lot more readability to the values
    //     let three_sides = BasicPolygons::Triangle;
    //     let seven_sides = BasicPolygons::Heptagon;
    //
    //     println!("{:?} {:?}", three_sides, seven_sides);
    //
    //     // we can also define a function that can take this new type as a property
    //     fn sides_of(polygon: BasicPolygons) -> u32 {
    //         if polygon == BasicPolygons::Triangle {
    //             3
    //         } else if polygon == BasicPolygons::Heptagon {
    //             7
    //         } else {
    //             0
    //         }
    //     }
    //
    //     println!("{}", sides_of(BasicPolygons::Heptagon));
    //
    //     #[derive(Debug)]
    //     struct Polygon {
    //         kind: BasicPolygons,
    //         sides: u32
    //     }
    //
    //     let triangle_1 = Polygon {
    //         kind: BasicPolygons::Triangle,
    //         sides: 3
    //     };
    //
    //     let triangle_2 = Polygon {
    //         kind: BasicPolygons::Triangle,
    //         sides: 3
    //     };
    //
    //     dbg!(&triangle_1);
    //     dbg!(&triangle_2);
    // }




}