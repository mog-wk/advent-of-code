#![allow(unused)]
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::time::Instant;

// R1: 2679
// R2: 77607

fn main() -> std::io::Result<()> {
    let time = Instant::now();
    let mut handler = std::io::BufWriter::new(std::io::stdout());
    let mut file = File::open("input.txt")?;
    let mut st = String::new();
    file.read_to_string(&mut st)?;

    let r1 = p_1(&st)?;
    let r2 = p_2(&st)?;

    writeln!(handler, "r1: {}", r1);
    writeln!(handler, "r2: {}", r2);
    writeln!(handler, "time: {:.2?}", time.elapsed());

    Ok(())
}
fn p_2(st: &str) -> std::io::Result<u32> {
    let game_rounds = st.split('\n').collect::<Vec<&str>>();

    let mut games_power_sum: u32 = 0;

    for round in game_rounds {
        if round == "" {
            continue
        }
        let id = round.split(':').collect::<Vec<&str>>();
        let (id, info) = (id[0], id[1]);
        let info = info.split(';').collect::<Vec<&str>>();

        // Red, Green, Blue
        let mut points: (u32, u32, u32) = (0, 0, 0);

        for data in info {
            let info = data.split(',').collect::<Vec<&str>>();
            for entry in info {
                let value = entry[1..].split(' ').collect::<Vec<&str>>();
                let (value, ty) = (value[0], value[1]);

                let value = value.parse::<u32>().unwrap();

                let b: Vec<u8> = ty.bytes().collect();
                let b = b[0];
                match b {
                    b'r' => {
                            if value > points.0 {
                                points.0 = value;
                            }
                    },
                    b'g' => {
                            if value > points.1 {
                                points.1 = value;
                            }
                    },
                    b'b' => {
                            if value > points.2 {
                                points.2 = value;
                            }
                    },
                    _ => panic!("{}", ty),
                }
            }
        }

        let power = points.0 * points.1 * points.2;

        games_power_sum += power;
    }

    Ok(games_power_sum)
}

fn p_1(st: &str) -> std::io::Result<u32> {
    let game_rounds = st.split('\n').collect::<Vec<&str>>();

    let mut games_id_sum: u32 = 0;

    let limit_points: (u32, u32, u32) = (12 ,13, 14);

    for round in game_rounds {
        if round == "" {
            continue
        }
        let id = round.split(':').collect::<Vec<&str>>();
        let (id, info) = (id[0], id[1]);
        let id = id.split(' ').collect::<Vec<&str>>()[1];
        let id = id.parse::<u32>().unwrap();
        let info = info.split(';').collect::<Vec<&str>>();

        let mut tern = false;

        for data in info {
            let info = data.split(',').collect::<Vec<&str>>();
            for entry in info {
                let value = entry[1..].split(' ').collect::<Vec<&str>>();
                let (value, ty) = (value[0], value[1]);

                let value = value.parse::<u32>().unwrap();

                let b: Vec<u8> = ty.bytes().collect();
                let b = b[0];
                match b {
                    b'r' => {
                        if value > limit_points.0 {
                            tern = true;
                            break;
                        }
                    },
                    b'g' => {
                        if value > limit_points.1 {
                            tern = true;
                            break;
                        }
                    },
                    b'b' => {
                        if value > limit_points.2 {
                            tern = true;
                            break;
                        }
                    },
                    _ => panic!("{}", ty),
                }
            }
            if tern == true {
                break
            }
        }
        if tern == true {
            continue
        }

        games_id_sum += id;
    }

    Ok(games_id_sum)
}
