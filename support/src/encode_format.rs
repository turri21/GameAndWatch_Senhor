use std::{
    fs::{self, File},
    io::{self, Read, Seek},
    path::{Path, PathBuf},
};

use bitvec::{
    field::BitField,
    prelude::{bitvec, Lsb0},
};

use sha1::{Digest, Sha1};

use crate::{
    manifest::{Action, CPUType, NamedAction, PlatformSpecification, Port, Screen},
    HEIGHT, WIDTH,
};

pub fn encode(
    background_bytes: &[u8],
    mask_bytes: &[u8],
    pixels_to_mask_id: &[Option<u16>],
    platform: &PlatformSpecification,
    asset_dir: &Path,
    output_dir: &Path,
) -> Result<PathBuf, String> {
    // Build config
    let mut config = build_config(platform)?;

    // Build image
    let background_iter = background_bytes.into_iter();
    let mask_iter = mask_bytes.into_iter();

    let mut count = 0;

    let mut image_block = background_iter
        .zip(mask_iter)
        .filter(|_| {
            let prev_count = count;
            // Drop every 4th (alpha) byte
            if count < 3 {
                count += 1;
            } else {
                count = 0;
            }
            prev_count != 3
        })
        // Background is low byte
        .flat_map(|(background_byte, mask_byte)| [*background_byte, *mask_byte])
        .collect::<Vec<u8>>();

    config.append(&mut image_block);

    // Build mask config
    let mut mask_block = build_mask_map(pixels_to_mask_id)?;

    config.append(&mut mask_block);

    // Add ROM
    // TODO: Add melody ROM
    let rom_path = asset_dir.join(&platform.rom.rom);

    let mut rom_data = match fs::read(&rom_path) {
        Ok(data) => Ok(data),
        Err(_) => match find_rom_by_hash(&platform.rom.rom_hash, asset_dir) {
            Ok(data) => Ok(data),
            Err(err) => Err(format!("{err}\nCould not open ROM {rom_path:?}")),
        },
    }?;

    config.append(&mut rom_data);

    let mut game_name = platform.metadata.name.clone();

    if game_name.to_lowercase().starts_with("game & watch:") {
        game_name = game_name.chars().skip("Game & Watch:".len()).collect();
    }

    game_name = game_name.replace(":", " -");
    let game_name = game_name.trim();

    let output_path: PathBuf = output_dir.join(format!("{game_name}.gnw"));
    fs::write(&output_path, config).unwrap();

    Ok(output_path)
}

fn find_rom_by_hash(target_hash: &String, asset_dir: &Path) -> Result<Vec<u8>, String> {
    for entry in fs::read_dir(asset_dir).expect("Could not open temp directory") {
        if let Ok(entry) = entry {
            let mut file = match File::open(entry.path()) {
                Ok(file) => file,
                Err(_) => continue,
            };
            let mut hasher = Sha1::new();
            let _ = match io::copy(&mut file, &mut hasher) {
                Ok(_) => {}
                Err(_) => continue,
            };
            let hash = hasher.finalize();

            let hash = hex::encode(hash);

            if &hash == target_hash {
                let mut buffer = Vec::new();
                if let Err(_) = file.seek(io::SeekFrom::Start(0)) {
                    return Err("Could not reread from file after hash check".into());
                }
                if let Err(_) = file.read_to_end(&mut buffer) {
                    return Err(format!("Could not open SHA matched ROM {:?}", entry.path()));
                }

                return Ok(buffer);
            }
        }
    }

    Err(format!("No SHA matched ROM found"))
}

fn build_config(platform: &PlatformSpecification) -> Result<Vec<u8>, String> {
    let mut config = Vec::<u8>::with_capacity(0x100);
    // Version
    config.push(1);

    // MPU version
    let version = match platform.device.cpu {
        CPUType::SM510 => 0,
        CPUType::SM511 => 1,
        CPUType::SM512 => 2,
        CPUType::SM530 => 3,
        CPUType::SM5a => 4,
        CPUType::SM510Tiger => 5,
        CPUType::SM511Tiger1Bit => 6,
        CPUType::SM511Tiger2Bit => 7,
        CPUType::KB1013VK12 => 8,
    };

    config.push(version);

    // Screen configuration
    let (screen, width, height) = match &platform.device.screen {
        Screen::Single { width, height } => (0, *width, *height),
        Screen::DualVertical { top, bottom } => {
            if top != bottom {
                println!("Top and bottom screen sizes don't match");
            }

            (1, top.width, top.height)
        }
        Screen::DualHorizontal { left, right } => {
            if left != right {
                println!("Left and right screen sizes don't match");
            }

            (2, left.width, left.height)
        }
    };
    config.push(screen);

    let mut data: bitvec::vec::BitVec<u8> = bitvec![u8, Lsb0; 0; 3*8];
    let width = width.round() as u16;
    let height = height.round() as u16;
    data[0..10].store(width);
    data[10..20].store(height);

    config.append(&mut data.into());

    // Reserved
    config.push(0);
    config.push(0);

    // Input mapping
    let mut s_ports: [Option<[Option<NamedAction>; 4]>; 8] = Default::default();
    let mut b_port: Option<NamedAction> = None;
    let mut ba_port: Option<NamedAction> = None;
    let mut acl_port: Option<NamedAction> = None;

    for port in &platform.port_map.ports {
        match port {
            Port::S { index, bitmap } => {
                if *index > 7 {
                    return Err(format!("Port index {index} is out of bounds"));
                }

                s_ports[*index] = Some(bitmap.clone());
            }
            Port::ACL { bit } => acl_port = bit.clone(),
            Port::B { bit } => b_port = bit.clone(),
            Port::BA { bit } => ba_port = bit.clone(),
        }
    }

    for port in s_ports {
        if let Some(port) = port {
            for action in port {
                if let Some(action) = action {
                    config.push(input_value_for_port(action));
                } else {
                    config.push(0x7F);
                }
            }
        } else {
            // Write 4 zeros
            config.push(0x7F);
            config.push(0x7F);
            config.push(0x7F);
            config.push(0x7F);
        }
    }

    let unused_action = NamedAction {
        action: Action::Unused,
        active_low: true,
        name: None,
    };

    let b_port = if let Some(b_port) = b_port {
        input_value_for_port(b_port)
    } else {
        // B has a pull-up resistor. If not set, default it to high
        let mut action = unused_action.clone();
        action.active_low = true;
        input_value_for_port(action)
    };
    config.push(b_port);

    let ba_port = if let Some(ba_port) = ba_port {
        input_value_for_port(ba_port)
    } else {
        // BA has a pull-up resistor. If not set, default it to high
        let mut action = unused_action.clone();
        action.active_low = true;
        input_value_for_port(action)
    };
    config.push(ba_port);

    let acl_port = if let Some(acl_port) = acl_port {
        input_value_for_port(acl_port)
    } else {
        input_value_for_port(unused_action)
    };
    config.push(acl_port);

    let ground_index = if let Some(ground_last_index) = platform.port_map.ground_last_index {
        // Indexes start at 1
        ground_last_index + 1
    } else {
        // Unset
        0
    };

    config.push(ground_index);

    // Spacer pixels for input mapping
    for _ in 0..4 {
        config.push(0);
    }

    // Reserved space
    for _ in 0..0xC9 {
        config.push(0);
    }

    let sha = env!("VERGEN_GIT_SHA");

    if sha.len() < 1 {
        println!("Unknown git SHA");

        vec![0 as u8; 7].iter().for_each(|c| config.push(*c));
    } else {
        sha.chars()
            .take(7)
            .map(|c| c as u8)
            .for_each(|c| config.push(c));
    };

    Ok(config)
}

fn input_value_for_port(action: NamedAction) -> u8 {
    let mut input: u8 = match action.action {
        Action::JoyUp => 0,
        Action::JoyDown => 1,
        Action::JoyLeft => 2,
        Action::JoyRight => 3,
        Action::Button1 => 4,
        Action::Button2 => 5,
        Action::Button3 => 6,
        Action::Button4 => 7,
        Action::Button5 => 8,
        Action::Button6 => 9,
        Action::Button7 => 10,
        Action::Button8 => 11,
        Action::Select => 12,
        Action::Start1 => 13,
        Action::Start2 => 14,
        Action::Service1 => 15,
        Action::Service2 => 16,
        Action::LeftJoyUp => 17,
        Action::LeftJoyDown => 18,
        Action::LeftJoyLeft => 19,
        Action::LeftJoyRight => 20,
        Action::RightJoyUp => 21,
        Action::RightJoyDown => 22,
        Action::RightJoyLeft => 23,
        Action::RightJoyRight => 24,
        Action::VolumeDown => 25,
        Action::PowerOn => 26,
        Action::PowerOff => 27,
        Action::Keypad => 28,
        Action::Custom => 29,
        Action::Unused => 0x7F,
    };

    if action.active_low {
        input |= 0x80;
    }

    input
}

const BYTES_PER_ENTRY: usize = 5;
const AVERAGE_ENTRIES_PER_ROW: usize = 52;
const TOTAL_BYTE_LENGTH: usize = BYTES_PER_ENTRY * AVERAGE_ENTRIES_PER_ROW * HEIGHT;

fn insert_mask_entry_bytes(
    output: &mut Vec<u8>,
    byte_index: &mut usize,
    id: u16,
    length: usize,
    start_x: usize,
    y: usize,
) -> Result<(), String> {
    if *byte_index + BYTES_PER_ENTRY > TOTAL_BYTE_LENGTH {
        return Err(format!(
            "More entries ({}) than allowed ({TOTAL_BYTE_LENGTH})",
            *byte_index + BYTES_PER_ENTRY
        ));
    }

    output[*byte_index..*byte_index + BYTES_PER_ENTRY]
        .clone_from_slice(&entry_to_bytes(id, length, start_x, y));

    *byte_index += BYTES_PER_ENTRY;

    Ok(())
}

fn build_mask_map(pixels_to_mask_id: &[Option<u16>]) -> Result<Vec<u8>, String> {
    // 5 bytes per entry
    let mut output: Vec<u8> = vec![0; TOTAL_BYTE_LENGTH];
    let mut byte_index = 0;

    for y in 0..HEIGHT {
        let mut current_id: Option<u16> = None;
        let mut start_x: usize = 0;
        let mut length: usize = 0;

        for x in 0..WIDTH {
            if let Some(id) = pixels_to_mask_id[y * WIDTH + x] {
                // Has id
                match current_id {
                    Some(stored_id) => {
                        if stored_id == id {
                            // Increment current entry
                            length += 1;
                        } else {
                            // This is a new segment, finish the old segment and start a new one
                            insert_mask_entry_bytes(
                                &mut output,
                                &mut byte_index,
                                stored_id,
                                length,
                                start_x,
                                y,
                            )?;

                            current_id = Some(id);
                            start_x = x;
                            length = 1;
                        }
                    }
                    None => {
                        // Begin entry
                        current_id = Some(id);
                        start_x = x;
                        length = 1;
                    }
                }
            } else {
                // No id
                if let Some(id) = current_id {
                    // End entry
                    current_id = None;

                    insert_mask_entry_bytes(&mut output, &mut byte_index, id, length, start_x, y)?;
                }
            }
        }

        if let Some(id) = current_id {
            // Clean up straggler at the end of a row
            insert_mask_entry_bytes(&mut output, &mut byte_index, id, length, start_x, y)?;
        }
    }

    Ok(output)
}

fn entry_to_bytes(id: u16, length: usize, start_x: usize, y: usize) -> Vec<u8> {
    let mut data: bitvec::vec::BitVec<u8> = bitvec![u8, Lsb0; 0; 5*8];

    data[0..10].store::<u16>(id);
    data[10..20].store::<u16>(start_x as u16);
    data[20..30].store::<u16>(y as u16);
    data[30..40].store::<u16>(length as u16);

    data.into()
}
