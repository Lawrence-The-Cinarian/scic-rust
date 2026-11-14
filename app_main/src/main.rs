use crates::extramath::{
    cah, calculate_by_pythagoras_add, calculate_by_pythagoras_sub, cosine_for_adj, cosine_for_hyp,
    cosine_rule_1, dicos, dinvc, dinvs, dinvt, disin, ditan, sin_two_angles_one_side,
    sin_two_sides_one_angle, sine_for_hyp, sine_for_opp, soh, tangent_for_adj, tangent_for_opp,
    toa,
};
use crates::libprint::{
    cosinefn, fams, forra, forra_option_1, forra_option_2, forst, mainmenu, rastc, sinefn, taaos,
    trignomenu, tsaoa,
};
use crates::read::{clear_terminal, read_number, read_string};

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
        } else if option == 2 {
            rastc();
            let (_, option) = read_number("Select an option: ");
            if option == 1 {
                forra();
                let (_, option) = read_number("Select an option: ");
                if option == 1 {
                    forra_option_1();
                    let (_, option) = read_number("Select an option: ");
                    match option {
                        1 => {
                            let (b, _) = read_number("Enter value for b: ");
                            let (c, _) = read_number("Enter value for c: ");
                            answer = calculate_by_pythagoras_sub(c, b);
                        }

                        2 => {
                            let (a, _) = read_number("Enter value for a: ");
                            let (c, _) = read_number("Enter value for c: ");
                            answer = calculate_by_pythagoras_sub(c, a);
                        }

                        3 => {
                            let (a, _) = read_number("Enter value for a: ");
                            let (b, _) = read_number("Enter value for b: ");
                            answer = calculate_by_pythagoras_add(a, b);
                        }

                        _ => {
                            read = 1;
                            println!("Invalid option");
                        }
                    }
                } else if option == 2 {
                    forra_option_2();
                    let (_, option) = read_number("Select an option: ");
                    match option {
                        1 => {
                            println!("\nEnter 0 if the value was not given\n");
                            let (sine, _) = read_number("Enter Sine in degrees: ");
                            let (opposite, _) = read_number("Enter opposite: ");
                            let (hypotenuse, _) = read_number("Enter hypotenuse: ");

                            if sine == 0.0 {
                                answer = soh(opposite, hypotenuse);
                            } else if opposite == 0.0 {
                                answer = sine_for_opp(sine, hypotenuse)
                            } else if hypotenuse == 0.0 {
                                answer = sine_for_hyp(sine, opposite);
                            } else {
                                read = 1;
                                println!(
                                    "Values are completed, Please one of the values must be 0.."
                                );
                            }
                        }

                        2 => {
                            println!("\nEnter 0 if the value was not given\n");
                            let (cosine, _) = read_number("Enter Cosine in degree: ");
                            let (adjacent, _) = read_number("Enter adjacent: ");
                            let (hypotenuse, _) = read_number("Enter hypotenuse: ");

                            if cosine == 0.0 {
                                answer = cah(adjacent, hypotenuse);
                            } else if adjacent == 0.0 {
                                answer = cosine_for_adj(cosine, hypotenuse);
                            } else if hypotenuse == 0.0 {
                                answer = cosine_for_hyp(cosine, adjacent);
                            } else {
                                read = 1;
                                println!(
                                    "Values are completed, Please one of the values must be 0.."
                                );
                            }
                        }

                        3 => {
                            println!("\nEnter 0 if the value was not given\n");
                            let (tangent, _) = read_number("Enter Tangent in degree: ");
                            let (opposite, _) = read_number("Enter opposite: ");
                            let (adjacent, _) = read_number("Enter adjacent: ");

                            if tangent == 0.0 {
                                answer = toa(opposite, adjacent);
                            } else if opposite == 0.0 {
                                answer = tangent_for_opp(tangent, adjacent);
                            } else if adjacent == 0.0 {
                                answer = tangent_for_adj(tangent, opposite);
                            } else {
                                read = 1;
                                println!(
                                    "Values are completed, Please one of the values must be 0.."
                                );
                            }
                        }

                        _ => {
                            read = 1;
                            println!("Invalid option");
                        }
                    }
                }
            } else if option == 2 {
                forst();
                let (_, option) = read_number("Select an option: ");
                if option == 1 {
                    sinefn();
                    let (_, option) = read_number("Select an option: ");
                    if option == 1 {
                        taaos();
                        let (angle1, _) = read_number("Enter the first angle: ");
                        let (angle2, _) = read_number("Enter the second angle: ");
                        let (side, _) = read_number("Enter the first side: ");
                        answer = sin_two_angles_one_side(angle1, angle2, side);
                    } else if option == 2 {
                        tsaoa();
                        let (side1, _) = read_number("Enter the first side: ");
                        let (side2, _) = read_number("Enter the second side: ");
                        let (angle, _) = read_number("Enter the first angle: ");
                        answer = sin_two_sides_one_angle(side1, side2, angle);
                    } else {
                        break;
                    }
                } else if option == 2 {
                    cosinefn();
                    let (_, option) = read_number("Select an option: ");
                    if option == 1 {
                        fams();
                        let (_, option) = read_number("Select an option: ");
                        match option {
                            1 => {
                                let (b, _) = read_number("Enter value for b: ");
                                let (c, _) = read_number("Enter value for c: ");
                                let (biga, _) = read_number("Enter value for A: ");
                                answer = cosine_rule_1(b, c, biga);
                            }

                            2 => {
                                let (a, _) = read_number("Enter value for a: ");
                                let (c, _) = read_number("Enter value for c: ");
                                let (bigb, _) = read_number("Enter value for B: ");
                                answer = cosine_rule_1(a, c, bigb);
                            }

                            3 => {
                                let (a, _) = read_number("Enter value for a: ");
                                let (b, _) = read_number("Enter value for b: ");
                                let (bigc, _) = read_number("Enter value for C: ");
                                answer = cosine_rule_1(a, b, bigc);
                            }

                            _ => {
                                println!("Invalid option");
                            }
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        } else {
            break;
        }
        if read == 0 {
            println!();
            println!("Answer = {}", answer);
            println!();
        }
        let repeat_or_break = read_string("Would you like to continue Yes or No: ");
        println!();
        if repeat_or_break != "yes" && repeat_or_break != "y" {
            break;
        }
    }
}
