use anyhow::{anyhow, Result};

#[derive(Debug)]
pub enum Params {
    CMAES(CMAESParams),
    OtherParams,
}

#[derive(Debug)]
pub struct CMAESParams {
    pub mu: i32,
}

impl CMAESParams {
    pub fn default_params(popsize: &i32) -> Result<CMAESParams> {
        if *popsize > 1 {
            Ok(CMAESParams { mu: *popsize / 2 })
        } else {
            Err(anyhow!("popsize must bre greater than 1, got {:?}", popsize))
        }
    }
}
