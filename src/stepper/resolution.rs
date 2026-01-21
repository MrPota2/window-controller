pub struct ResolutionMatrix {
    ms1: bool,
    ms2: bool,
    ms3: bool,
}

pub enum Resolution {
    FULL,
    HALF,
    FOURTH,
    EIGTH,
    SIXTEENTH,
}

pub fn get_pin_settings_from(resolution: Resolution) -> ResolutionMatrix {
    let mut resolution_matrix = ResolutionMatrix
    match resolution {
        Resolution::FULL => ResolutionMatrix(true, true. true),
        Resolution::HALF => todo!(),
        Resolution::FOURTH => todo!(),
        Resolution::EIGTH => todo!(),
        Resolution::SIXTEENTH => todo!(),
    }
}
