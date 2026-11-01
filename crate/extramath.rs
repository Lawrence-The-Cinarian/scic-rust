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
    let result = sine.to_radians().sin();
    let result = result * hypotenuse;
    result
}

pub fn sine_for_hyp(sine: f64, opposite: f64) -> f64 {
    let result = sine.to_radians().sin();
    let result = opposite / result;
    result
}

pub fn cah(adjacent: f64, hypotenuse: f64) -> f64 {
    let result = adjacent / hypotenuse;
    let result = result.acos();
    result.to_degrees()
}

pub fn cosine_for_adj(cosine: f64, hypotenuse: f64) -> f64 {
    let result = cosine.to_radians().cos();
    let result = result * hypotenuse;
    result
}

pub fn cosine_for_hyp(cosine: f64, adjacent: f64) -> f64 {
    let result = cosine.to_radians().cos();
    let result = adjacent / result;
    result
}

pub fn toa(opposite: f64, adjacent: f64) -> f64 {
    let result = opposite / adjacent;
    let result = result.atan();
    result.to_degrees()
}

pub fn tangent_for_opp(tangent: f64, adjacent: f64) -> f64 {
    let result = tangent.to_radians().tan();
    let result = result * adjacent;
    result
}

pub fn tangent_for_adj(tangent: f64, opposite: f64) -> f64 {
    let result = tangent.to_radians().tan();
    let result = opposite / result;
    result
}

pub fn sin_two_angles_one_side(angle_1: f64, angle_2: f64, side: f64) -> f64 {
	let result = angle_1.to_radians().sin();
	let result1 = angle_2.to_radians().sin();
	let result = (side * result1) / result;
	result
}

pub fn sin_two_sides_one_angle(side_1: f64, side_2: f64, angle: f64) -> f64 {
	let angle = angle.to_radians().sin();
	let result = (side_2 * angle) / side_1;
	result.asin().to_degrees()
}

pub fn cosine_rule_1(firstv: f64, secondv: f64, value: f64) -> f64 {
	let value =  value.to_radians().cos();
	let power = (firstv * firstv) + (secondv * secondv) - (2.0 * firstv * secondv * value);
	power.sqrt()
}
