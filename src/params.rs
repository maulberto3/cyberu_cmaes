use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
pub struct CmaesParams {
    // Required
    pub mean: Vec<f32>,
    pub sigma: f32,
    pub popsize: i32,
    // Optional
    // pub bounds
    // pub n_max_resampling: Option<i32>,
    // pub seed: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct CmaesParamsValid {
    pub mean: Vec<f32>,
    pub sigma: f32,
    pub popsize: i32,
    pub mu: i32,
}

impl CmaesParamsValid {
    pub fn validate(params: &CmaesParams) -> Result<CmaesParamsValid> {
        print!("Computing default parameters... ");
        let params_ = CmaesParamsValid::create_default_params(params)?;
        println!("Done.");

        print!("Validating initial parameters... ");
        match CmaesParamsValid::validate_params(&params_) {
            Ok(_) => {
                println!(" Done.");
                Ok(params_)
            }
            Err(e) => {
                eprint!("An initial Cmaes parameter is not following its constraint: ");
                eprintln!("{} \n", e);
                panic!();
            }
        }
    }

    fn create_default_params(params: &CmaesParams) -> Result<CmaesParamsValid> {
        let mean = params.mean.clone();
        let sigma = params.sigma;
        let num_dims = params.mean.len() as i32;
        let popsize = if params.popsize <= 5 {
            println!("Parameter popsize smaller than 5, recalculating default value.");
            CmaesParamsValid::calculate_popsize(&num_dims)?
        } else {
            params.popsize
        };
        let mu = params.popsize / 2;

        let params_ = CmaesParamsValid {
            mean,
            sigma,
            popsize,
            mu,
        };
        Ok(params_)
    }

    fn validate_params(params_: &CmaesParamsValid) -> Result<()> {
        CmaesParamsValid::check_mean_length(params_)?;
        CmaesParamsValid::check_sigma(params_)?;
        CmaesParamsValid::check_popsize(params_)?;
        Ok(())
    }

    fn check_mean_length(params_: &CmaesParamsValid) -> Result<()> {
        if params_.mean.len() <= 1 {
            return Err(anyhow!("==> number of dimensions must be > 1."));
        }
        Ok(())
    }

    fn check_sigma(params_: &CmaesParamsValid) -> Result<()> {
        if params_.sigma <= 0.0 {
            return Err(anyhow!("==> sigma must be > 0.0."));
        }
        Ok(())
    }

    fn check_popsize(params_: &CmaesParamsValid) -> Result<()> {
        if params_.popsize <= 5 {
            return Err(anyhow!("==> popsize must be > 5."));
        }
        Ok(())
    }

    fn calculate_popsize(num_dims: &i32) -> Result<i32> {
        Ok(4 + (3.0 * (*num_dims as f32).ln()).floor() as i32)
    }
}
