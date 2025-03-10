#[macro_use]
extern crate guard;

use std::{collections::HashMap, env::temp_dir, fs, path::PathBuf};

use clap::{Parser, Subcommand, ValueEnum};

use colored::Colorize;

use assets::get_assets;
use layout::parse_layout;
use manifest::PlatformSpecification;

use crate::{encode_format::encode, manifest::CPUType, render::RenderedData};

mod assets;
mod encode_format;
mod layout;
mod manifest;
mod render;
mod svg_manage;

const WIDTH: usize = 720;
const HEIGHT: usize = WIDTH;

#[derive(Subcommand, Clone, Debug)]
enum FilterArg {
    /// Match a particular game
    Specific { name: String },
    /// Match the games that use a particular CPU
    CPU { name: CPUType },
    /// Match the specific CPU types supported by the core currently. These are the SM510 (inc. Tiger) and SM5a CPUs
    Supported,
    /// All game types specified in the manifest.json
    All,
}

#[derive(ValueEnum, Clone, Debug)]
enum CompanyArg {
    Nintendo,
    Elektronika,
    Konami,
    Nelsonic,
    /// Tiger Electronics
    Tiger,
    Tronica,
    VTech,
}

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    filter: Option<FilterArg>,

    #[arg(short = 'i', long)]
    /// Only the games located in your MAME directory
    installed: bool,

    #[arg(short = 'm', long)]
    /// The path to your MAME directory containing your games
    mame_path: PathBuf,

    #[arg(short = 'a', long, default_value = "manifest.json")]
    /// The path to the included manifest file
    manifest_path: PathBuf,

    #[arg(short = 'o', long)]
    /// The path to the final ROM output directory
    output_path: PathBuf,

    #[arg(short = 'l', long)]
    /// The layout name specified in the MAME .lay file to use. Will fail if this layout is not found
    layout: Option<String>,

    #[arg(short = 'd', long)]
    /// Enable debug PNG output
    debug: bool,

    ///////////////////

    // Company filtering
    #[arg(short, long)]
    /// Filter to Nintendo games
    nintendo: bool,

    #[arg(short, long)]
    /// Filter to Elektronika games
    elektronika: bool,

    #[arg(short, long)]
    /// Filter to Konami games
    konami: bool,

    #[arg(short = 'c', long)]
    /// Filter to Nelsonic games
    nelsonic: bool,

    #[arg(short, long)]
    /// Filter to Tiger Electronics games
    tiger: bool,

    #[arg(short = 'r', long)]
    /// Filter to Tronica games
    tronica: bool,

    #[arg(short, long)]
    /// Filter to VTech games
    vtech: bool,

    #[arg(short = 'b', long)]
    /// Filter to Homebrew games
    homebrew: bool,
}

fn main() {
    let args = Args::parse();

    let temp_dir = temp_dir().join("gnw");

    let manifest_file = fs::read(args.manifest_path).expect("Could not find manifest file");

    let manifest: HashMap<String, PlatformSpecification> =
        serde_json::from_slice(manifest_file.as_slice()).expect("Could not parse manifest file");

    let output_path = args
        .output_path
        .canonicalize()
        .expect("Could not find output path");

    let company_filter = {
        let mut filter = vec![];

        if args.nintendo {
            filter.push("nintendo");
        }

        if args.elektronika {
            filter.push("elektronika");
            filter.push("bootleg (elektronika)");
        }

        if args.konami {
            filter.push("konami");
        }

        if args.nelsonic {
            filter.push("nelsonic");
        }

        if args.tiger {
            filter.push("tiger");
        }

        if args.tronica {
            filter.push("tronica");
        }

        if args.vtech {
            filter.push("vtech");
        }

        if args.homebrew {
            filter.push("homebrew");
        }

        filter
    };

    let filter_platforms =
        |platforms: Vec<CPUType>| -> Option<Vec<(String, &PlatformSpecification)>> {
            let result = manifest
                .iter()
                .filter(|(_, p)| platforms.contains(&p.device.cpu))
                .map(|(n, p)| (n.clone(), p))
                .collect::<Vec<(String, &PlatformSpecification)>>();

            if result.len() > 0 {
                Some(result)
            } else {
                None
            }
        };

    let platforms: Option<Vec<(String, &PlatformSpecification)>> = match &args.filter {
        Some(FilterArg::Specific { name }) => {
            let trimmed_name = name.trim().to_string();

            if let Some(entry) = manifest.get(&trimmed_name) {
                Some(vec![(trimmed_name, entry)])
            } else {
                None
            }
        }
        Some(FilterArg::Supported) => {
            filter_platforms(vec![CPUType::SM510, CPUType::SM510Tiger, CPUType::SM5a])
        }
        Some(FilterArg::CPU { name }) => filter_platforms(vec![name.clone()]),
        Some(FilterArg::All) | None => Some(manifest.iter().map(|(n, p)| (n.clone(), p)).collect()),
    };

    let installed = if args.filter.is_some() {
        args.installed
    } else {
        true
    };

    guard!(let Some(mut platforms) = platforms else {
        println!("No manifest listings for selected devices found");
        return;
    });

    platforms.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());

    let platforms = platforms.iter().filter(|(_, p)| {
        if company_filter.len() > 0 {
            for filter in &company_filter {
                if p.metadata.company.to_lowercase().starts_with(filter) {
                    return true;
                }
            }
        } else {
            return true;
        }

        false
    });

    let mut success_count = 0;
    let mut skip_count = 0;
    let mut fail_count = 0;
    let mut platform_count = 0;

    let mut fail = |name: &String, message: String| {
        println!("{message}");
        println!("{}", format!("Failing device {name}\n").red());

        fail_count += 1;
    };

    for (name, platform) in platforms {
        platform_count += 1;
        let asset_dir = temp_dir.join(name.clone());

        println!("-------------------------");
        println!("Processing device {}\n", name.green());

        if let Err(err) = get_assets(&name, &platform.rom.rom_owner, &args.mame_path, &asset_dir) {
            if !installed {
                // Only fail if we're not looking for only owned games
                fail(name, err);
            } else {
                // See `fail` above
                println!("{err}");
                println!(
                    "{}",
                    format!("Skipping device {name}: Not installed\n").red()
                );
                skip_count += 1;
            }
            continue;
        }

        let (layout_manifest, layout) = match parse_layout(&asset_dir, args.layout.as_ref()) {
            Ok(layout) => layout,
            Err(err) => {
                fail(name, err);
                continue;
            }
        };

        let RenderedData {
            background_bytes,
            mask_bytes,
            pixels_to_mask_id,
        } = match render::render(
            &name,
            &layout,
            &layout_manifest,
            &platform,
            &asset_dir,
            args.debug,
        ) {
            Ok(data) => data,
            Err(err) => {
                fail(name, err);
                continue;
            }
        };

        let data_path = encode(
            background_bytes.data(),
            mask_bytes.data(),
            pixels_to_mask_id.as_slice(),
            platform,
            &asset_dir,
            &output_path,
        );

        match data_path {
            Ok(path) => {
                println!(
                    "Successfully created device {} at {}\n",
                    name.green(),
                    path.display()
                );
                success_count += 1;
            }
            Err(err) => fail(name, err),
        }
    }

    println!("-------------------------");
    println!(
        "Total: {platform_count}, Success: {success_count}, Fail: {fail_count}, Skip: {skip_count}",
    );
}
