pub fn degree_to_radian(degree: f32) -> f32 {
    degree * std::f32::consts::PI / 180.0
}

pub fn radian_to_degree(radian: f32) -> f32 {
    radian * 180. / std::f32::consts::PI
}
