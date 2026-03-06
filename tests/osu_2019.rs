use akatsuki_pp::{Beatmap, osu_2019::OsuPP};

mod common;

#[test]
fn osu_2019_clock_rate() {
    let map = Beatmap::from_path(common::OSU).unwrap();

    let attrs_1_0 = OsuPP::from_map(&map)
        .calculate();

    let attrs_1_1 = OsuPP::from_map(&map)
        .clock_rate(1.1)
        .calculate();

    assert!(attrs_1_1.pp > attrs_1_0.pp);
    assert!(attrs_1_1.difficulty.stars > attrs_1_0.difficulty.stars);
}

#[test]
fn osu_2019_mods() {
    let map = Beatmap::from_path(common::OSU).unwrap();

    let nm = OsuPP::from_map(&map).calculate();
    let hd = OsuPP::from_map(&map).mods(8).calculate();
    let dt = OsuPP::from_map(&map).mods(64).calculate();
    let hr = OsuPP::from_map(&map).mods(16).calculate();
    let hddt = OsuPP::from_map(&map).mods(8 + 64).calculate();

    assert!(hd.pp > nm.pp);
    assert!(dt.pp > nm.pp);
    assert!(hr.pp > nm.pp);
    assert!(hddt.pp > dt.pp);
    assert!(hddt.pp > hd.pp);
}

#[test]
fn osu_2019_accuracy() {
    let map = Beatmap::from_path(common::OSU).unwrap();

    let acc100 = OsuPP::from_map(&map).accuracy(100.0).calculate();
    let acc98 = OsuPP::from_map(&map).accuracy(98.0).calculate();
    let acc95 = OsuPP::from_map(&map).accuracy(95.0).calculate();

    assert!(acc100.pp > acc98.pp);
    assert!(acc98.pp > acc95.pp);
}

#[test]
fn osu_2019_misses() {
    let map = Beatmap::from_path(common::OSU).unwrap();

    let fc = OsuPP::from_map(&map).calculate();
    let miss1 = OsuPP::from_map(&map).misses(1).calculate();
    let miss10 = OsuPP::from_map(&map).misses(10).calculate();

    assert!(fc.pp > miss1.pp);
    assert!(miss1.pp > miss10.pp);
}
