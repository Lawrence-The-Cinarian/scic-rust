#[path = "../crate/extramath.rs"]
mod extramath;
#[path = "../crate/libprint.rs"]
mod libprint;
#[path = "../crate/read.rs"]
mod read;

use extramath::{dicos, dinvc, dinvs, dinvt, disin, ditan};
use libprint::{mainmenu, trignomenu};
use read::{clear_terminal, read_number, read_string};

fn main() {
    let mut answer: f64 = 0.0;
    let mut read: u8 = 0;
    loop {
        clear_terminal();
        mainmenu();
        let (_, option) = read_number("Select an option: ");
        if option == 1 {
            trignomenu();
            let (question, _) = read_number("Enter the number in degrees/decimals: ");
            println!();
            let (_, option) = read_number("Select an option: ");
            match option {
                1 => {
                    answer = disin(question);
                }
                2 => {
                    answer = dicos(question);
                }
                3 => {
                    answer = ditan(question);
                }
                4 => {
                    answer = dinvs(question);
                }
                5 => {
                    answer = dinvc(question);
                }
                6 => {
                    answer = dinvt(question);
                }
                _ => {
                    read = 1;
                    println!("Invalid Option");
                }
            }
            if read == 0 {
                println!("Answer = {}", answer);
            }
        } else if option == 1 {
            println!("1");
        } else {
            break;
        }

        let repeat_or_break = read_string("Would you like to continue Yes or No: ");
        if repeat_or_break != "yes" && repeat_or_break != "y" {
            break;
        }
    }
}
