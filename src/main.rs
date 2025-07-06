use std::collections::HashMap;

use plotters::prelude::*;
use rand::{
    distr::{Distribution, StandardUniform},
    Rng,
};

#[derive(Debug)]
enum RollResult {
    FirstPrize,
    SecondPrize,
    ThirdPrize,
    FourthPrize,
    FifthPrize,
    SixthPrize,
    SeventhPrize,
    EighthPrize,
}

impl TryFrom<usize> for RollResult {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(RollResult::FirstPrize),
            1 => Ok(RollResult::SecondPrize),
            2 => Ok(RollResult::ThirdPrize),
            3 => Ok(RollResult::FourthPrize),
            4 => Ok(RollResult::FifthPrize),
            5 => Ok(RollResult::SixthPrize),
            6 => Ok(RollResult::SeventhPrize),
            7 => Ok(RollResult::EighthPrize),
            _ => Err("Can only convert 0..=7 to RollResult"),
        }
    }
}

impl From<&RollResult> for usize {
    fn from(value: &RollResult) -> Self {
        match value {
            RollResult::FirstPrize => 0,
            RollResult::SecondPrize => 1,
            RollResult::ThirdPrize => 2,
            RollResult::FourthPrize => 3,
            RollResult::FifthPrize => 4,
            RollResult::SixthPrize => 5,
            RollResult::SeventhPrize => 6,
            RollResult::EighthPrize => 7,
        }
    }
}

impl Distribution<RollResult> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RollResult {
        rng.random_range(0..8).try_into().unwrap()
    }
}

/// Keep rolling for prizes until all prizes have been earned, then return the result
fn run_sim() -> Vec<RollResult> {
    let mut results = Vec::new();
    let mut rng = rand::rng();
    //                       first  second third  fourth fifth  sixth  seventh eighth
    let mut earned_prizes = [false, false, false, false, false, false, false, false];

    while earned_prizes.iter().filter(|&&earned| earned).count() < 8 {
        let roll_result = if results.len() < 25 {
            rng.random::<RollResult>()
        } else {
            // It's not truly random, but after 25 rolls we get an unearned prize every time so it
            // doesn't matter
            earned_prizes
                .iter()
                .enumerate()
                .find_map(|(index, &earned)| if !earned { Some(index) } else { None })
                .unwrap() // okay to unwrap because there must be at least one unearned prize
                .try_into()
                .unwrap() // okay to unwrap because the index must be in range
        };

        earned_prizes[usize::from(&roll_result)] = true;
        results.push(roll_result);
    }

    results
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut results = Vec::new();
    let runs = 1000000;
    for _ in 0..runs {
        results.push(run_sim());
    }
    println!(
        "Average number of rolls to earn all prizes: {}",
        results.iter().fold(0, |sum, sim_res| sum + sim_res.len()) as f32 / runs as f32
    );

    let mut hist_data: HashMap<usize, usize> = HashMap::new();
    for sim_res in results {
        hist_data
            .entry(sim_res.len())
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let max_count = *hist_data.values().max().unwrap();
    let left_label_area_size = if max_count > 500 { 100 } else { 50 };

    let file_name = format!("output/{}-sim.png", runs);
    let root = BitMapBackend::new(&file_name, (1280, 720)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart_builder = ChartBuilder::on(&root);
    chart_builder
        .margin(5)
        .set_label_area_size(LabelAreaPosition::Bottom, 50)
        .set_label_area_size(LabelAreaPosition::Left, left_label_area_size)
        .caption(
            format!("Result of {} simulations of uma-bingo", runs),
            ("Calibri", 36),
        );

    let mut chart_context = chart_builder
        .build_cartesian_2d((8..35 as usize).into_segmented(), 0..max_count + 5)
        .unwrap();
    chart_context
        .configure_mesh()
        .label_style(("Calibri", 28))
        .x_desc("Number of rolls to earn all prizes")
        .y_desc("Number of samples")
        .draw()
        .unwrap();

    chart_context
        .draw_series(
            Histogram::vertical(&chart_context)
                .style(BLUE.filled())
                .margin(10)
                .data(hist_data),
        )
        .unwrap();

    Ok(())
}
