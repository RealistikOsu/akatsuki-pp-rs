use akatsuki_pp::Beatmap;
use akatsuki_pp::osu_2019::OsuPP;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 8 {
        eprintln!("Usage: pp_cli <map_path> <mods> <combo> <n300> <n100> <n50> <misses>");
        return;
    }

    let map_path = &args[1];
    let mods: u32 = args[2].parse().unwrap();
    let combo: u32 = args[3].parse().unwrap();
    let n300: u32 = args[4].parse().unwrap();
    let n100: u32 = args[5].parse().unwrap();
    let n50: u32 = args[6].parse().unwrap();
    let misses: u32 = args[7].parse().unwrap();

    let map = Beatmap::from_path(map_path).expect("Failed to parse map");

    let attrs = OsuPP::from_map(&map)
        .mods(mods)
        .combo(combo)
        .n300(n300)
        .n100(n100)
        .n50(n50)
        .misses(misses)
        .calculate();

    println!("{}", attrs.pp);
}
