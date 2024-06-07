#[derive(Debug)]
pub enum Params {
    CMAES(CMAESParams),
    OtherParams,
}

#[derive(Debug)]
pub struct CMAESParams {
    // pub sigma_init: f32,
}

impl CMAESParams {
    pub fn default_params() -> Self {
        CMAESParams {}
    }
}
