use asr::timer::{self, TimerState};
use std::{collections::HashSet, sync::Mutex};

pub mod settings;
use settings::SplitterSettings;
pub mod game;
use game::{GameProcess, Variables};

static GAME_PROCESS: Mutex<Option<GameProcess>> = Mutex::new(None);

const CHAPTER_BREAKPOINTS: [f64; 21] = [
    0.0,    // ch1
    160.0,  // gate 1 mario
    530.0,  // ch3
    630.0,  // gate 2 zelda
    930.0,  // ch5
    1160.0, // gate 3 castlevania
    1540.0, // ch7
    1770.0, // gate 4 megaman
    2010.0, // ch9
    2170.0, // the vault
    2570.0, // ch11
    2600.0, // ch12
    2680.0, // gate 5 metroid
    3090.0, // ch14
    3400.0, // gate 6 rpg
    3880.0, // ch16
    3940.0, // ch17
    4210.0, // ch18
    4530.0, // ch19
    5050.0, // ch20
    5510.0, // ch21
];

const MAP_ID_TITLE_SCREEN: i32 = 1;
const MAP_ID_INTRO_SCENE: i32 = 43;
const MAP_ID_SACRED_TEMPLE: i32 = 236;
const MAP_ID_SACRED_TEMPLE_EXIT_MAP: i32 = 245;

#[no_mangle]
pub extern "C" fn update() {
    let mut mutex = GAME_PROCESS.lock().unwrap();

    if mutex.is_none() {
        *mutex = GameProcess::connect("DT3_v1.5.2.4");
    } else {
        let game = mutex.as_mut().unwrap();

        // Ensure we're still connected to the process before we go on
        if !game.process.is_open() {
            *mutex = None;
            return;
        }

        let vars = match game.state.update(&mut game.process) {
            Some(v) => v,
            None => {
                asr::print_message("Failed to get state fully");
                return;
            }
        };

        // if let Some(boss) = vars.current_boss {
        //     asr::print_message(&format!("BOSS: {:?}", Variables::get_as_string(&boss.current)));
        // } else {
        //     asr::print_message("Boss is none");
        // }

        handle_resets(&vars, &game.settings);
        handle_is_loading(&vars, &game.settings);

        match timer::state() {
            TimerState::NotRunning => {
                timer_not_running(&vars, &game.settings, &mut game.seen_map_ids)
            }
            TimerState::Running => {
                timer_running(&vars, &game.settings, &mut game.seen_map_ids)
            }
            TimerState::Paused => {}
            TimerState::Ended => {}
        }
    }
}

fn handle_is_loading(vars: &Variables, settings: &SplitterSettings) {
    if settings.remove_pause_time && vars.is_paused.current == 1.0 {
        timer::pause_game_time();
    } else if settings.remove_cutscene_time && vars.in_cutscene.current == 1.0 {
        timer::pause_game_time();
    } else if settings.remove_gameover && vars.in_game_over.current == 1.0 {
        timer::pause_game_time();
    } else {
        timer::resume_game_time();
    }
}

fn handle_resets(vars: &Variables, settings: &SplitterSettings) {
    // reset when we reach the title screen
    if settings.reset_on_titlescreen
        && vars.map_id.old != MAP_ID_TITLE_SCREEN
        && vars.map_id.current == MAP_ID_TITLE_SCREEN
    {
        asr::timer::reset();
    }
    // reset when we click on the new game button on the main menu
    else if vars.map_id.old == MAP_ID_TITLE_SCREEN
        && vars.map_id.current == MAP_ID_INTRO_SCENE
        && vars.game_progress.current == 0.0
    {
        asr::timer::reset();
    }
}

fn timer_not_running(vars: &Variables, settings: &SplitterSettings, map_ids: &mut HashSet<i32>) {
    map_ids.clear();
    let mut should_start = false;

    // map 1 is the title screen, and map 43 is the intro cutscene on that cliff
    // it's the first thing that appears after you press new game and can only be reached from that
    // however let's safeguard with game_progress of 0 as well in case weird stuff happens
    if vars.map_id.old == MAP_ID_TITLE_SCREEN
        && vars.map_id.current == MAP_ID_INTRO_SCENE
        && vars.game_progress.current == 0.0
    {
        asr::print_message(
            "Starting timer since we've found the intro scene and our game progress is 0",
        );
        should_start = true;
    }
    // this setting could be useful for categories like "bossgallery%" i guess
    else if settings.start_on_continue
        && vars.map_id.old == MAP_ID_TITLE_SCREEN
        && vars.map_id.current != MAP_ID_TITLE_SCREEN
    {
        asr::print_message("Starting timer due to `start_on_continue");
        should_start = true;
    }
    // useful for ILs probably; ignores the title screen
    else if settings.start_on_map_change
        && vars.map_id.old != MAP_ID_TITLE_SCREEN
        && vars.map_id.current != MAP_ID_TITLE_SCREEN
        && vars.map_id.old != vars.map_id.current
    {
        asr::print_message("Starting timer due to `start_on_map_change`");
        should_start = true;
    }

    if should_start {
        map_ids.insert(vars.map_id.current);
        asr::timer::start();
    }
}

fn timer_running(vars: &Variables, settings: &SplitterSettings, map_ids: &mut HashSet<i32>) {
    if settings.split_on_every_new_map_change
        && vars.map_id.old != vars.map_id.current
        && !map_ids.contains(&vars.map_id.current)
    {
        asr::print_message(&format!(
            "Found a new map while using `split_on_every_new_map_change`: {}",
            vars.map_id.current
        ));
        map_ids.insert(vars.map_id.current);

        if vars.map_id.old != MAP_ID_TITLE_SCREEN
            && vars.map_id.current != MAP_ID_TITLE_SCREEN
            && vars.map_id.old != vars.map_id.current
        {
            asr::print_message("Splitting due to `split_on_every_new_map_change`");
            asr::timer::split();
        }
    }
    // The "game completed" value changes right before heading to the title in the epilogue
    else if settings.split_on_game_completed && vars.game_completed.check(|&x| x == 1.0) {
        asr::print_message("Splitting due to `split_on_game_completed`");
        asr::timer::split();
    }
    // Check if the current boss name changed from "having something" to "not having something"
    else if settings.split_on_every_boss
        && vars.current_boss.is_some()
        && vars
            .current_boss
            .unwrap()
            .check(|&x| match Variables::get_as_string(&x) {
                Some(s) => s.len() == 0,
                None => true,
            })
    {
        // If we have the "split_on_boss_gallery" setting checked we always want to split on a boss death
        // otherwise only split if we're not currently in the boss gallery
        if settings.split_on_boss_gallery {
            asr::print_message(
                "Splitting due to `split_on_every_boss` and `split_on_boss_gallery`",
            );
            asr::timer::split();
        } else if vars.boss_gallery.current == 0.0 {
            asr::print_message(
                "Splitting due to `split_on_every_boss` and not currently in boss gallery",
            );
            asr::timer::split();
        } else {
            asr::print_message("split_on_every_boss: Completed a boss but it was in the boss_gallery, and we don't have that option enabled, so we're not splitting.");
        }
    }
    // Check if the current boss name changed from "having something" to "not having something"
    else if settings.split_on_every_boss_start
        && vars.current_boss.is_some()
        && vars
            .current_boss
            .unwrap()
            .check(|&x| match Variables::get_as_string(&x) {
                Some(s) => s.len() != 0,
                None => false,
            })
    {
        // If we have the "split_on_boss_gallery" setting checked we always want to split on a boss death
        // otherwise only split if we're not currently in the boss gallery
        if settings.split_on_boss_gallery {
            asr::print_message(
                "Splitting due to `split_on_every_boss_start` and `split_on_boss_gallery`",
            );
            asr::timer::split();
        } else if vars.boss_gallery.current == 0.0 {
            asr::print_message(
                "Splitting due to `split_on_every_boss_start` and not currently in boss gallery",
            );
            asr::timer::split();
        } else {
            asr::print_message("split_on_every_boss_start: Completed a boss but it was in the boss_gallery, and we don't have that option enabled, so we're not splitting.");
        }
    }
    // Check if our achievement strings were modified and split if one of them were
    else if settings.split_on_achievement
        && (vars.token_recognitions.current != vars.token_recognitions.old
            || vars.token_recognitions_two.current != vars.token_recognitions_two.old)
    {
        asr::print_message("Splitting due to `split_on_achievement`");
        asr::timer::split();
    }
    // just check if our old programchips value is different
    else if settings.split_on_every_program_chip
        && vars.program_chips.current != vars.program_chips.old
    {
        asr::print_message("Splitting due to `split_on_every_program_chip`");
        asr::timer::split();
    }
    //check in an array if we're at a chapter boundary
    else if settings.split_on_every_chapter
        && vars.game_progress.current != vars.game_progress.old
        && CHAPTER_BREAKPOINTS.contains(&vars.game_progress.current)
    {
        asr::print_message("Splitting due to `split_on_every_chapter`");
        asr::timer::split();
    }
    // Check a wraparound using modulo with a wide margin.
    else if settings.split_every_ten_percent
        && vars.game_percent.old % 10.0 >= 9.0
        && vars.game_percent.current % 10.0 <= 1.0
    {
        asr::print_message("Splitting due to `split_every_ten_percent`");
        asr::timer::split();
    }
    // Casted to int to floor the value and hopefully ensure we only trigger once we get the 100
    // and possibly avoid weird impreciseness problems with double?
    else if settings.split_at_100_percent
        && (vars.game_percent.old as u64) < 100
        && (vars.game_percent.current as u64) >= 100
    {
        asr::print_message("Splitting due to `split_at_100_percent`");
        asr::timer::split();
    }
    //Check for sacred temple entries
    else if settings.split_on_sacred_temple
        && vars.map_id.old != MAP_ID_SACRED_TEMPLE_EXIT_MAP
        && vars.map_id.old != MAP_ID_SACRED_TEMPLE
        && vars.map_id.current == MAP_ID_SACRED_TEMPLE
    {
        asr::print_message("Splitting due to `split_on_sacred_temple`");
        asr::timer::split();
    } else if settings.split_on_every_map_change && vars.map_id.old != vars.map_id.current {
        asr::print_message("Splitting due to `split_on_every_map_change`");
        asr::timer::split();
    }
}
