pub fn disin(degree: f64) -> f64 {
    let radians = degree.to_radians();
    let result = radians.sin();
    result
}

pub fn dicos(degree: f64) -> f64 {
    let radians = degree.to_radians();
    let result = radians.cos();
    result
}

pub fn ditan(degree: f64) -> f64 {
    let radians = degree.to_radians();
    let result = radians.tan();
    result
}

pub fn dinvs(degree: f64) -> f64 {
    let result = degree.asin();
    result.to_degrees()
}

pub fn dinvc(degree: f64) -> f64 {
    let result = degree.acos();
    result.to_degrees()
}

pub fn dinvt(degree: f64) -> f64 {
    let result = degree.atan();
    result.to_degrees()
}
