use anyhow::Result;
// use ndarray::Array2;

// mod fitness;
// use fitness::square_and_sum;

mod params;
// use ndarray::Array2;
use params::CmaesInitParams;
use strategies::Cmaes;

// mod states;

mod strategies;
// use strategies::Algo;

/// `Result<T, Error>`
///
/// This is a reasonable return type to use throughout your application but also
/// for `fn main`; if you do, failures will be printed along with any
/// [context][Context] and a backtrace if one was captured.
///
/// `anyhow::Result` may be used with one *or* two type parameters.
///
pub fn work() -> Result<()> {
    // Step 1: Choose initial parameters
    let init_params = CmaesInitParams {
        mean: vec![0.0, 1.0, 2.0],
        sigma: 1.0,
        // n_max_resampling: None, // Some(100)
        seed: None,    // Some(16)
        popsize: None, // Some(50)
        cov: None,     // Some(Array2::eye(3)),
    };
    // init_params = init_params.validate()?;  // encapsulated inside algo
    // dbg!(&init_params);

    // STEP 2: Instantiate Cmaes algorithm with intial parameters
    let mut cmaes = Cmaes::new(init_params)?;
    // dbg!(&cmaes);

    let _ = cmaes.ask_one();

    // // Step 2: Get its (default) Parmeters and...
    // let params = cmaes.default_params();
    // dbg!(&params);

    // // Step 3: Initiate its State
    // let mut state = cmaes.init_algorithm(&params);
    // // println!("{:+6.4?}", &state);

    // // Step 4: Ask-Tell
    // for _ in 0..100 {
    //     let pop: Array2<f32> = cmaes.ask(&state, &params);
    //     println!("{:+.4}", &pop);

    //     let fitness: Array2<f32> = square_and_sum(&pop);
    //     println!("{:+.4}", &fitness);

    //     state = cmaes.tell(pop, fitness, state, &params);
    //     println!("{:+.4?}", &state);
    //     println!("");
    // }
    // println!("{:+.4?}", &state);

    // let num_iters = 7;
    // for _i in 0..num_iters {
    // pop, state = cmaes.ask(state);
    // fit = fitness(&pop);
    // state = cmaes.tell(state, &pop, &fit, &params);
    // state.best_member, state.best_fitness

    //     break;
    // }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::work;

    #[test]
    // TODO: implement integration tests, similar to Robert Lange
    fn it_works() {
        _ = work();
    }
}
