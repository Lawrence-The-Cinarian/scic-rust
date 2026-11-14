use colored::Colorize;
pub fn mainmenu() {
    println!();
    println!("{}", "\t == Cinari's Scientific Calculator == ".blue());
    println!();
    println!(
        "Instructions: Enter any of your preferred options,\nPress any number apart from these two below to exit"
    );
    println!("(1) Enter 1 for the trigonometric Calculations");
    println!("(2) Enter 2 for Right Angled / Scalene Triangle Calculations");
    println!();
}

pub fn trignomenu() {
    println!();
    println!("\t == Trigonometric Calculator == ");
    println!();
    println!("(1) Sine");
    println!("(2) Cosine");
    println!("(3) Tangent");
    println!("(4) Sine Inverse");
    println!("(5) Cosine Inverse");
    println!("(6) Tangent Inverse");
    println!();
}

pub fn rastc() {
    println!();
    println!("\t == R.A.S.T Calculator == ");
    println!();
    println!("(1) For right angles"); //forra();
    println!("(2) For Scalene Triangles"); //forst();
    println!();
}

pub fn forra() {
    println!();
    println!("\t == For right angles == ");
    println!();
    println!("(1) For side lengths using pythagoras theorm");
    println!("(2) For angles and sides using trigonometry");
    println!();
}

pub fn forra_option_1() {
    println!();
    println!("\t == For (1) response == ");
    println!();
    println!("(1) finding a");
    println!("(2) finding b");
    println!("(3) finding c");
    println!();
}

pub fn forra_option_2() {
    println!();
    println!("\t == For (2) response == ");
    println!();
    println!("(1) SOH");
    println!("(2) CAH");
    println!("(3) TOA");
    println!();
}

pub fn forst() {
    println!();
    println!("\t == For Scalene Triangles == ");
    println!();
    println!("(1) Sine Rule");
    println!("(2) Cosine Rule");
    println!();
}

pub fn sinefn() {
    println!();
    println!("\t == Sine Rule == ");
    println!();
    println!("(1) Two angles and one side => find the other side ");
    println!("(2) Two sides and one angle => find the other angle ()");
    println!();
}

pub fn taaos() {
    println!();
    println!("\t == Two angles and one side == ");
    println!();
}

pub fn tsaoa() {
    println!();
    println!("\t == Two sides and one angle == ");
    println!();
}

pub fn cosinefn() {
    println!();
    println!("\t == Cosine Rule == ");
    println!();
    println!("(1) Find a missing side");
    println!();
}

pub fn fams() {
    println!();
    println!("\t == Finding a missing side == ");
    println!();
    println!("(1) finding a");
    println!("(2) finding b");
    println!("(3) finding c");
    println!();
}
