mod cli_errors;
mod maze;
mod pathfinder;

use image::{GenericImageView, Rgb, RgbImage};
use num::Num;
use structopt::StructOpt;

use std::boxed::Box;
use std::error::Error;
use std::ffi::OsString;
use std::fs;
use std::io::{self, ErrorKind};
use std::path::PathBuf;
use std::process::exit;
use std::str;
use std::time;
use std::u32;

use maze::node::Position;
use maze::Maze;

#[derive(Debug, StructOpt)]
struct Opt {
    /// Alogrithm. Options: bfs, dfs, dijk, astar
    #[structopt(short = "a", long = "algo", default_value = "bfs")]
    pub algo: String,
    /// Output directory
    #[structopt(
        short = "o",
        long = "output",
        default_value = "./out",
        parse(from_os_str)
    )]
    pub output: PathBuf,
    /// Input image
    #[structopt(parse(from_os_str))]
    pub image: PathBuf,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    const NS_S: u128 = 1_000_000_000;

    let time_total = time::Instant::now();

    let opt = Opt::from_args();
    let algo_string = opt.algo;
    let img_path = opt.image;
    let out_path = opt.output;

    if !img_path.exists() {
        return Err(Box::new(io::Error::new(
            ErrorKind::NotFound,
            "File path does not exists.",
        )));
    }
    if !img_path.is_file() {
        return Err(Box::new(io::Error::new(
            ErrorKind::Other,
            "File path is not a file.",
        )));
    }
    let algo = get_algo(&algo_string)?;

    let mut img: image::RgbImage;
    {
        println!("Loading Image");
        let dyn_img = image::open(&img_path)?;
        println!("{}", img_path.display());
        println!(
            "width: {}\nheight: {}",
            fmt_num(dyn_img.width()),
            fmt_num(dyn_img.height())
        );
        let (color_type, depth) = colortype_to_str(dyn_img.color());
        println!("color type: {}\ndepth: {}", color_type, depth);
        println!();

        img = dyn_img.to_rgb();
    }

    println!("Creating Maze");
    let time0 = time::Instant::now();
    let maze = Maze::new(&img)?;
    let dur0 = time0.elapsed().as_nanos();
    println!("Time elapsed: {}.{:09}", dur0 / NS_S, dur0 % NS_S);
    println!("Node Count: {}", fmt_num(maze.count()));
    println!();

    println!("Solving Maze");
    let time1 = time::Instant::now();
    let (maybe_path, count) = algo(&maze);
    let dur1 = time1.elapsed().as_nanos();
    println!("Time elapsed: {}.{:09}", dur1 / NS_S, dur1 % NS_S);
    println!("Nodes explored: {}", fmt_num(count));

    if let Some(maze_path) = maybe_path {
        println!("Path found\nLength: {}", fmt_num(maze_path.len()));
        println!();
        draw_path(&mut img, &maze_path);
        save_solved(&img, &img_path, &out_path, &algo_string)?;
    } else {
        println!("Path not found");
    }
    println!();

    let dur_total = time_total.elapsed().as_nanos();
    println!(
        "Total time elapsed: {}.{:09}",
        dur_total / NS_S,
        dur_total % NS_S
    );
    println!();

    Ok(())
}

fn draw_path(buf: &mut RgbImage, maze_path: &[Position]) {
    let length = maze_path.len();

    for i in 0..length - 1 {
        let a = maze_path[i];
        let b = maze_path[i + 1];

        // Blue - red
        let r = ((i as f32 / length as f32) * 255.0) as u8;
        let px = Rgb {
            data: [r, 0, 255 - r],
        };

        if a.row() == b.row() {
            for x in u32::min(a.col(), b.col())..=u32::max(a.col(), b.col()) {
                buf[(x, a.row())] = px;
            }
        } else {
            for y in u32::min(a.row(), b.row())..=u32::max(a.row(), b.row()) {
                buf[(a.col(), y)] = px;
            }
        }
    }
}

type AlgoResult =
    Result<fn(&Maze) -> (Option<Vec<Position>>, usize), cli_errors::InvalidAlgorithmError>;
fn get_algo(algo_str: &str) -> AlgoResult {
    match algo_str {
        "bfs" => Ok(pathfinder::bfs),
        "dijk" => Ok(pathfinder::dijk),
        "dijk2" => Ok(pathfinder::dijk2),
        "dfs" => Ok(pathfinder::dfs),
        "astar" => Ok(pathfinder::astar),
        _ => Err(cli_errors::InvalidAlgorithmError(algo_str.to_owned())),
    }
}

fn save_solved(
    img: &RgbImage,
    img_path: &PathBuf,
    out_path: &PathBuf,
    algo_str: &str,
) -> Result<(), Box<dyn Error>> {
    let name = img_path
        .file_name()
        .unwrap()
        .to_str()
        .expect("Could not convert filename to string");
    let dot_index = if let Some(i) = name.find('.') {
        i
    } else {
        name.len()
    };
    let mut name = name[..dot_index].to_owned();
    name.push_str(&format!("{}{}{}", "-", algo_str, ".png"));
    let name = OsString::from(name);

    println!("Saving Image");
    if !out_path.exists() {
        fs::create_dir(&out_path)?;
    }
    let file = out_path.join(name);
    img.save(&file)?;
    let full_path = file.canonicalize()?;
    println!("Saved to: {}", full_path.display());
    Ok(())
}

fn colortype_to_str(ct: image::ColorType) -> (&'static str, u8) {
    use image::ColorType::*;
    match ct {
        Gray(d) => ("Gray", d),
        RGB(d) => ("RGB", d),
        Palette(d) => ("Palette", d),
        GrayA(d) => ("GrayA", d),
        RGBA(d) => ("RGBA", d),
        BGR(d) => ("BGR", d),
        BGRA(d) => ("BGRA", d),
    }
}

fn fmt_num<T>(int: T) -> String
where
    T: Num + std::fmt::Display,
{
    let s = int.to_string();
    let mut neg = false;
    let mut flt = false;

    let start_index: usize;
    if s.starts_with('-') {
        start_index = 1;
        neg = true;
    } else {
        start_index = 0;
    }
    let point_index: usize;
    if let Some(i) = s.find('.') {
        point_index = i;
        flt = true;
    } else {
        point_index = s.len();
    }
    let b = s[start_index..point_index].as_bytes();

    let chunks = b.rchunks_exact(3);
    let mut s_chunks: Vec<_> = Vec::new();

    s_chunks.push(String::from(str::from_utf8(chunks.remainder()).unwrap()));

    let chunks = chunks.rev();

    for c in chunks {
        s_chunks.push(String::from(str::from_utf8(c).unwrap()));
    }

    // if length perfectly divisible by 3, the fist index will be empty.
    let mut out = if s_chunks[0] != "" {
        s_chunks.join(",")
    } else {
        s_chunks[1..].join(",")
    };

    if neg {
        out.insert(0, '-');
    }

    if flt {
        out.push_str(&s[point_index..]);
    }

    out
}
