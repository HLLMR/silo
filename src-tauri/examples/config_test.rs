fn main() {
    let dir = silo_lib::fsgame::user_dir().unwrap();
    let g = std::fs::read_to_string(dir.join("game.xml")).unwrap();
    let gv = silo_lib::xmlconfig::get_values(&g, &[
        "game.graphic.scalability.shadowMapSize".into(),
        "game.graphic.scalability.fsr@quality".into(),
        "game.graphic.display.vsync".into(),
    ]);
    println!("game.xml: {:?}", gv);

    let s = std::fs::read_to_string(dir.join("savegame1/careerSavegame.xml")).unwrap();
    let sv = silo_lib::xmlconfig::get_values(&s, &[
        "careerSavegame.settings.economicDifficulty".into(),
        "careerSavegame.settings.timeScale".into(),
        "careerSavegame.settings.weedsEnabled".into(),
        "careerSavegame.settings.savegameName".into(),
    ]);
    println!("savegame1: {:?}", sv);
    // round-trip check (in-memory, not written): flip weeds
    let out = silo_lib::xmlconfig::set_values(&s, &[silo_lib::xmlconfig::Edit{
        path:"careerSavegame.settings.weedsEnabled".into(), value:"false".into()}]).unwrap();
    println!("set weeds=false ok: {}", out.contains("<weedsEnabled>false</weedsEnabled>"));
    println!("timeScale preserved: {}", out.contains("<timeScale>10.000000</timeScale>"));
}
