use std::collections::HashSet;
use asr::{
    watcher::{Pair, Watcher},
    Process,
};

use crate::SplitterSettings;

pub struct GameProcess {
    pub process: asr::Process,
    pub state: State,
    pub settings: SplitterSettings,
    pub seen_map_ids: HashSet<i32>,
}
impl GameProcess {
    pub fn connect(process_name: &str) -> Option<Self> {
        let process = asr::Process::attach(process_name)?;
        let settings = SplitterSettings::default();
        let tick_rate = if settings.cheat_double_speed {
            60.0
        } else {
            30.0
        };

        asr::set_tick_rate(tick_rate);

        Some(Self {
            process,
            state: State::default(),
            settings,
            seen_map_ids: HashSet::new(),
        })
    }
}

#[derive(Default)]
pub struct Variable<T> {
    var: Watcher<T>,
    base_address: u32,
    address_path: Vec<u32>,
}
impl<T: bytemuck::Pod + std::fmt::Debug> Variable<T> {
    pub fn update(&mut self, process: &Process) -> Option<&Pair<T>> {
        self.var.update(
            process
                .read_pointer_path32(self.base_address, &self.address_path)
                .ok(),
        )
    }
}

pub struct State {
    /// internal gamemaker room mapping
    pub map_id: Variable<i32>,
    /// arbitrary internal progress number to check for cutscenes/etc
    pub game_progress: Variable<f64>,
    /// how many program chips we have
    pub program_chips: Variable<f64>,
    /// what percent of the items are found/etc
    pub game_percent: Variable<f64>,
    /// if the game is done v handy
    pub game_completed: Variable<f64>,

    /// Achievements part 1.
    /// Each entry in the list is either 0x31 for achieved or 0x30 for not
    pub token_recognitions: Variable<[u8; 50]>,
    /// Achievements part 2.
    /// Each entry in the list is either 0x31 for achieved or 0x30 for not.
    pub token_recognitions_two: Variable<[u8; 50]>,

    /// a lot of things aren't counted as a cutscene apparently
    pub in_cutscene: Variable<f64>,
    /// if the game over screen is displayed
    pub in_game_over: Variable<f64>,
    /// if the pause menu is up, or the game is just generally paused like in a ""cutscene""
    pub is_paused: Variable<f64>,
    /// the name of the boss if one exists
    pub current_boss: Variable<[u8; 100]>,
    /// 0 = no boss, 1+ = fighting boss?
    pub boss_track: Variable<f64>,
    /// 0 = not in boss gallery, 1 = in boss gallery
    pub boss_gallery: Variable<f64>,
}
impl Default for State {
    fn default() -> Self {
        let base_address = 0x400000;
        Self {
            map_id: Variable {
                base_address,
                address_path: vec![0x4452FC],
                var: Watcher::new(),
            },
            game_progress: Variable {
                base_address,
                address_path: vec![0x286AB4, 0x4, 0x4890],
                var: Watcher::new(),
            },
            program_chips: Variable {
                base_address,
                address_path: vec![0x286AB4, 0x4, 0x48B8],
                var: Watcher::new(),
            },
            game_percent: Variable {
                base_address,
                address_path: vec![0x286AB4, 0x4, 0x48E0],
                var: Watcher::new(),
            },
            game_completed: Variable {
                base_address,
                address_path: vec![0x286AB4, 0x4, 0x5498],
                var: Watcher::new(),
            },
            token_recognitions: Variable {
                base_address,
                address_path: vec![0x286AB4, 0x4, 0x54C8, 0x0],
                var: Watcher::new(),
            },
            token_recognitions_two: Variable {
                base_address,
                address_path: vec![0x286AB4, 0x4, 0x54F0, 0x0],
                var: Watcher::new(),
            },
            in_cutscene: Variable {
                base_address,
                address_path: vec![0x286AB4, 0x4, 0x4CA0],
                var: Watcher::new(),
            },
            in_game_over: Variable {
                base_address,
                address_path: vec![0x286AB4, 0x4, 0x4C50],
                var: Watcher::new(),
            },
            is_paused: Variable {
                base_address,
                address_path: vec![0x286AB4, 0x4, 0x6A50],
                var: Watcher::new(),
            },
            current_boss: Variable {
                base_address,
                address_path: vec![0x286AB4, 0x4, 0x4D98, 0x0],
                var: Watcher::new(),
            },
            boss_track: Variable {
                base_address,
                address_path: vec![0x286AB4, 0x4, 0x4DB8],
                var: Watcher::new(),
            },
            boss_gallery: Variable {
                base_address,
                address_path: vec![0x286AB4, 0x4, 0x5240],
                var: Watcher::new(),
            },
        }
    }
}
impl State {
    pub fn update(&mut self, process: &Process) -> Option<Variables> {
        Some(Variables {
            map_id: self.map_id.update(process)?,
            game_progress: self.game_progress.update(process)?,
            program_chips: self.program_chips.update(process)?,
            game_percent: self.game_percent.update(process)?,
            game_completed: self.game_completed.update(process)?,
            token_recognitions: self.token_recognitions.update(process)?,
            token_recognitions_two: self.token_recognitions_two.update(process)?,
            in_cutscene: self.in_cutscene.update(process)?,
            in_game_over: self.in_game_over.update(process)?,
            is_paused: self.is_paused.update(process)?,
            current_boss: self.current_boss.update(process),
            boss_track: self.boss_track.update(process)?,
            boss_gallery: self.boss_gallery.update(process)?,
        })
    }
}

pub struct Variables<'a> {
    pub map_id: &'a Pair<i32>,
    pub game_progress: &'a Pair<f64>,
    pub program_chips: &'a Pair<f64>,
    pub game_percent: &'a Pair<f64>,
    pub game_completed: &'a Pair<f64>,
    pub token_recognitions: &'a Pair<[u8; 50]>,
    pub token_recognitions_two: &'a Pair<[u8; 50]>,
    pub in_cutscene: &'a Pair<f64>,
    pub in_game_over: &'a Pair<f64>,
    pub is_paused: &'a Pair<f64>,
    pub current_boss: Option<&'a Pair<[u8; 100]>>,
    pub boss_track: &'a Pair<f64>,
    pub boss_gallery: &'a Pair<f64>,
}
impl<'a> Variables<'a> {
    pub fn get_as_string(var: &'a [u8]) -> Option<&'a str> {
        let null_pos = var.iter().position(|&x| x == b'\0').unwrap_or(var.len());

        std::str::from_utf8(&var[0..null_pos]).ok()
    }
}
