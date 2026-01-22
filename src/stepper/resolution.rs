#[derive(Debug)]
pub enum SIGNAL {
    HIGH,
    LOW,
}

#[derive(Debug)]
pub struct MicroStepPins<MS1, MS2, MS3> {
    pub ms1_pin: MS1,
    pub ms2_pin: MS2,
    pub ms3_pin: MS3,
}

#[derive(Debug)]
pub struct ResolutionMatrix {
    pub ms1: SIGNAL,
    pub ms2: SIGNAL,
    pub ms3: SIGNAL,
}

pub enum Resolution {
    FULL,
    HALF,
    FOURTH,
    EIGTH,
    SIXTEENTH,
}

pub trait SetStepResolution<E> {
    fn set_step_resolution(&mut self, resolution: Resolution) -> Result<(), E>;
}

pub struct NoStepModeControl;

pub struct WithStepResolutionControl<MS1, MS2, MS3> {
    pub pins: MicroStepPins<MS1, MS2, MS3>,
}

pub trait EnableStepModeControl<Resources, E> {
    type WithStepModeControl: SetStepResolution<E>;
    fn enable_step_mode_control(self, res: Resources) -> Self::WithStepModeControl;
}

pub fn get_pin_settings_from(resolution: &Resolution) -> ResolutionMatrix {
    let resolution_matrix = ResolutionMatrix {
        ms1: SIGNAL::LOW,
        ms2: SIGNAL::LOW,
        ms3: SIGNAL::LOW,
    };

    match resolution {
        Resolution::FULL => resolution_matrix,
        Resolution::HALF => ResolutionMatrix {
            ms1: SIGNAL::HIGH,
            ..resolution_matrix
        },
        Resolution::FOURTH => ResolutionMatrix {
            ms2: SIGNAL::HIGH,
            ..resolution_matrix
        },
        Resolution::EIGTH => ResolutionMatrix {
            ms1: SIGNAL::HIGH,
            ms2: SIGNAL::HIGH,
            ..resolution_matrix
        },
        Resolution::SIXTEENTH => ResolutionMatrix {
            ms1: SIGNAL::HIGH,
            ms2: SIGNAL::HIGH,
            ms3: SIGNAL::HIGH,
        },
    }
}
