pub mod basics;
pub mod structs;
pub mod ownership;
pub mod enums;
pub mod pattern_matching;

pub fn run() {
    basics::run();
    ownership::run();
    structs::run();
    enums::run();
    pattern_matching::run();
}