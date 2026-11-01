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

pub fn calculate_by_pythagoras_add(fresult: f64, sresult: f64) -> f64 {
    let result = (fresult * fresult) + (sresult * sresult);
    result.sqrt()
}

pub fn calculate_by_pythagoras_sub(fresult: f64, sresult: f64) -> f64 {
    let result = (fresult * fresult) - (sresult * sresult);
    result.sqrt()
}

pub fn soh(opposite: f64, hypotenuse: f64) -> f64 {
    let result = opposite / hypotenuse;
    let result = result.asin();
    result.to_degrees()
}

pub fn sine_for_opp(sine: f64, hypotenuse: f64) -> f64 {
    let fresult = sine.to_radians().sin();
    let sresult = fresult * hypotenuse;
    sresult
}

pub fn sine_for_hyp(sine: f64, opposite: f64) -> f64 {
    let fresult = sine.to_radians().sin();
    let sresult = opposite / fresult;
    sresult
}

pub fn cah(adjacent: f64, hypotenuse: f64) -> f64 {
    let result = adjacent / hypotenuse;
    let result = result.acos();
    result.to_degrees()
}

pub fn cosine_for_adj(cosine: f64, hypotenuse: f64) -> f64 {
    let fresult = cosine.to_radians().cos();
    let sresult = fresult * hypotenuse;
    sresult
}

pub fn cosine_for_hyp(cosine: f64, adjacent: f64) -> f64 {
    let fresult = cosine.to_radians().cos();
    let sresult = adjacent / fresult;
    sresult
}

pub fn toa(opposite: f64, adjacent: f64) -> f64 {
    let result = opposite / adjacent;
    let result = result.atan();
    result.to_degrees()
}

pub fn tangent_for_opp(tangent: f64, adjacent: f64) -> f64 {
    let fresult = tangent.to_radians().tan();
    let sresult = fresult * adjacent;
    sresult
}

pub fn tangent_for_adj(tangent: f64, opposite: f64) -> f64 {
    let fresult = tangent.to_radians().tan();
    let sresult = opposite / fresult;
    sresult
}
