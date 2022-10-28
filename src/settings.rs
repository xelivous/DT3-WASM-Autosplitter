pub struct SplitterSettings {
    /// Split whenever you start a boss fight
    /// In case you want to separate doing normal chapter movement from doing a boss fight
    pub split_on_every_boss_start: bool,
    /// Split after every boss
    /// Internally checks if the currentBossName value changes from "has something" to "doesn't have something".
    /// I don't think anything else sets this value so it should be fine.
    pub split_on_every_boss: bool,
    /// Split after every chapter starts
    /// There isn't a good way to do detect this.
    /// It uses a manually defined map of gameProgress values
    pub split_on_every_chapter: bool,
    /// Split whenever a Program Chip is obtained
    /// I'm not sure if this is useful since it's always obtained after a boss.
    /// maybe it can be used instead of splitting on bosses.
    pub split_on_every_program_chip: bool,
    /// Split when the gameCompleted value changes to true
    /// This gets set immediately before returning to the title after the epilogue.
    /// May or may not be useful.
    pub split_on_game_completed: bool,
    /// Split every time an achievement is gotten
    /// Is probably useful if you're running an AllAchievements% category?
    pub split_on_achievement: bool,
    /// Split every time an boss in the gallery is beaten
    /// Is probably useful if you're running an a bossgallery% category?
    pub split_on_boss_gallery: bool,
    /// Split every 10% of items found
    pub split_every_ten_percent: bool,
    /// Split at 100% items
    pub split_at_100_percent: bool,
    /// Split when entering the sacred temple
    /// Doesn't include entries from the exit map [19]
    pub split_on_sacred_temple: bool,
    /// Split whenever the map ID changes
    /// Not really recommended to turn this on.
    pub split_on_every_map_change: bool,
    /// Split whenever the map ID changes to a new map
    /// Stores an internal reference to map IDs and splits whenever it encounters a new map
    pub split_on_every_new_map_change: bool,

    /// Pause timer in a cutscene
    /// Unfortunateley I don't think this value is ever actually used internally in the game for anything so this does nothing.
    pub remove_cutscene_time: bool,
    /// Pause timer when the game is paused
    /// Sometimes the game pauses to do cutscenes, but this also handles the pause menu.
    pub remove_pause_time: bool,
    /// Pause Timer in the gameover screen
    /// This setting exists so that you're not forced to play on quick gameover mode.
    pub remove_gameover: bool,

    /// Start the timer when you continue the game
    /// I'm not sure if this is actually useful or not but maybe someone wants it.
    pub start_on_continue: bool,
    /// Start on map change
    /// This is useful for ILs i guess, since most of them start on portal entry. Doesn't count the title screen.
    pub start_on_map_change: bool,

    /// Reset timer when you go back to the title screen
    /// I'm not sure if this is actually useful or not but maybe someone wants it.
    pub reset_on_titlescreen: bool,

    /// Enable when using the double speed cheat
    /// There might be a way to detect this through pointers but it's annoying
    pub cheat_double_speed: bool,
}

impl Default for SplitterSettings {
    fn default() -> Self {
        Self {
            split_on_every_boss_start: true,
            split_on_every_boss: true,
            split_on_every_chapter: false,
            split_on_every_program_chip: false,
            split_on_game_completed: false,
            split_on_achievement: false,
            split_on_boss_gallery: false,
            split_every_ten_percent: false,
            split_at_100_percent: true,
            split_on_sacred_temple: false,
            split_on_every_map_change: false,
            split_on_every_new_map_change: false,
            remove_cutscene_time: false,
            remove_pause_time: false,
            remove_gameover: true,
            start_on_continue: false,
            start_on_map_change: false,
            reset_on_titlescreen: false,
            cheat_double_speed: false,
        }
    }
}
