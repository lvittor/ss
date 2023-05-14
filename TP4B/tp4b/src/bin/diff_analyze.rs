use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};

use clap::Parser as _parser;
use pool::{models::Frame, parser::output_parser};

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    output1: PathBuf,

    #[arg(short, long)]
    output2: PathBuf,

    #[arg(short, long)]
    analysis: PathBuf,
}

fn main() {
    let args = Args::parse();
    let output_file1 = File::open(args.output1).unwrap();
    let output_file2 = File::open(args.output2).unwrap();

    let mut analysis_file = File::create(args.analysis).unwrap();

    analysis_file
        .write(b"t,phi\n")
        .unwrap();

    for (frame1, frame2) in output_parser(BufReader::new(output_file1).lines())
        .zip(output_parser(BufReader::new(output_file2).lines()))
    {
        let Frame {
            time: time1,
            balls: mut balls1,
        } = frame1;
        let Frame {
            time: time2,
            balls: mut balls2,
        } = frame2;

        //assert_eq!(time1, time2);
        let time_diff = (time1 - time2).abs();
        //println!("{time1} | {time2} | {time_diff}");
        assert!(time_diff < 1e-8);
        let time = time1;

        balls1.sort_by_key(|ball| ball.id);
        balls2.sort_by_key(|ball| ball.id);
        let phi: f64 = balls1
            .iter()
            .zip(balls2.iter())
            .map(|(ball1, ball2)| (ball2.position - ball1.position).magnitude())
            .sum();

        analysis_file
            .write_fmt(format_args!("{time},{phi}\n"))
            .unwrap();
    }
}
