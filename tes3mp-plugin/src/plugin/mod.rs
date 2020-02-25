#![allow(unused)]

pub mod generated;

pub use generated::*;

pub const LOG_VERBOSE: u16 = 0;
pub const LOG_INFO: u16 = 1;
pub const LOG_WARN: u16 = 2;
pub const LOG_ERROR: u16 = 3;
pub const LOG_FATAL: u16 = 4;

#[macro_export]
macro_rules! instance {
    ($call:tt, $($argument:expr),+) => {
        let instance = unsafe {
            EVENTS_INSTANCE
                .as_ref()
                .expect(format!("No events instance created: {}\n", stringify!($call)).as_str())
        };
        instance.$call($($argument),+);
    };

    ($call:tt) => {
        let instance = unsafe {
            EVENTS_INSTANCE
                .as_ref()
                .expect(format!("No events instance created: {}\n", stringify!($call)).as_str())
        };
        instance.$call();
    };
}

#[macro_export]
macro_rules! use_events {
    ($events:ident) => {
        use std::ffi::CStr;

        static mut EVENTS_INSTANCE: Option<$events> = None;

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnServerInit() {
            unsafe {
                if (EVENTS_INSTANCE.is_none()) {
                    EVENTS_INSTANCE = Some($events::new());
                }
            }

            instance!(on_server_init);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnServerPostInit() {
            instance!(on_server_post_init);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnServerExit(is_error: bool) {
            instance!(on_server_exit, is_error);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnActorAI(player_id: u16, description: *const i8) {
            instance!(on_actor_ai, player_id, unsafe {
                CStr::from_ptr(description).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnActorCellChange(player_id: u16, description: *const i8) {
            instance!(on_actor_cell_change, player_id, unsafe {
                CStr::from_ptr(description).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnActorDeath(player_id: u16, description: *const i8) {
            instance!(on_actor_death, player_id, unsafe {
                CStr::from_ptr(description).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnActorEquipment(player_id: u16, description: *const i8) {
            instance!(on_actor_equipment, player_id, unsafe {
                CStr::from_ptr(description).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnActorList(player_id: u16, description: *const i8) {
            instance!(on_actor_list, player_id, unsafe {
                CStr::from_ptr(description).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnActorTest(player_id: u16, description: *const i8) {
            instance!(on_actor_test, player_id, unsafe {
                CStr::from_ptr(description).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnCellDeletion(description: *const i8) {
            instance!(on_cell_deletion, unsafe {
                CStr::from_ptr(description).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnCellLoad(description: *const i8) {
            instance!(on_cell_load, unsafe {
                CStr::from_ptr(description).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnCellUnload(description: *const i8) {
            instance!(on_cell_unload, unsafe {
                CStr::from_ptr(description).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnContainer(player_id: u16, description: *const i8) {
            instance!(on_container, player_id, unsafe {
                CStr::from_ptr(description).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnDoorState(player_id: u16, description: *const i8) {
            instance!(on_door_state, player_id, unsafe {
                CStr::from_ptr(description).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnGUIAction(player_id: u16, message_box_id: i16, data: *const i8) {
            instance!(on_gui_action, player_id, message_box_id, unsafe {
                CStr::from_ptr(data).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnMpNumIncrement(current_mp_num: i16) {
            instance!(on_mp_num_increment, current_mp_num);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnObjectActivate(player_id: u16, description: *const i8) {
            instance!(on_object_activate, player_id, unsafe {
                CStr::from_ptr(description).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnObjectDelete(player_id: u16, description: *const i8) {
            instance!(on_object_delete, player_id, unsafe {
                CStr::from_ptr(description).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnObjectLock(player_id: u16, description: *const i8) {
            instance!(on_object_lock, player_id, unsafe {
                CStr::from_ptr(description).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnObjectPlace(player_id: u16, description: *const i8) {
            instance!(on_object_place, player_id, unsafe {
                CStr::from_ptr(description).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnObjectScale(player_id: u16, description: *const i8) {
            instance!(on_object_scale, player_id, unsafe {
                CStr::from_ptr(description).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnObjectSpawn(player_id: u16, description: *const i8) {
            instance!(on_object_spawn, player_id, unsafe {
                CStr::from_ptr(description).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnObjectState(player_id: u16, description: *const i8) {
            instance!(on_object_state, player_id, unsafe {
                CStr::from_ptr(description).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnObjectTrap(player_id: u16, description: *const i8) {
            instance!(on_object_trap, player_id, unsafe {
                CStr::from_ptr(description).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerAttribute(player_id: u16) {
            instance!(on_player_attribute, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerBook(player_id: u16) {
            instance!(on_player_book, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerBounty(player_id: u16) {
            instance!(on_player_bounty, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerCellChange(player_id: u16) {
            instance!(on_player_cell_change, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerConnect(player_id: u16) {
            instance!(on_player_connect, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerDeath(player_id: u16) {
            instance!(on_player_death, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerDisconnect(player_id: u16) {
            instance!(on_player_disconnect, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerDisposition(player_id: u16) {
            instance!(on_player_disposition, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerEndCharGen(player_id: u16) {
            instance!(on_player_end_char_gen, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerEquipment(player_id: u16) {
            instance!(on_player_equipment, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerFaction(player_id: u16) {
            instance!(on_player_faction, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerInput(player_id: u16) {
            instance!(on_player_input, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerInventory(player_id: u16) {
            instance!(on_player_inventory, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerItemUse(player_id: u16) {
            instance!(on_player_item_use, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerJournal(player_id: u16) {
            instance!(on_player_journal, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerLevel(player_id: u16) {
            instance!(on_player_level, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerMiscellaneous(player_id: u16) {
            instance!(on_player_miscellaneous, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerQuickKeys(player_id: u16) {
            instance!(on_player_quick_keys, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerReputation(player_id: u16) {
            instance!(on_player_reputation, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerRest(player_id: u16) {
            instance!(on_player_rest, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerResurrect(player_id: u16) {
            instance!(on_player_resurrect, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerSendMessage(player_id: u16, message: *const i8) {
            instance!(on_player_send_message, player_id, unsafe {
                CStr::from_ptr(message).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerShapeshift(player_id: u16) {
            instance!(on_player_shapeshift, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerSkill(player_id: u16) {
            instance!(on_player_skill, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerSpellbook(player_id: u16) {
            instance!(on_player_spellbook, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerTopic(player_id: u16) {
            instance!(on_player_topic, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnRecordDynamic(player_id: u16) {
            instance!(on_record_dynamic, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnRequestDataFileList() {
            instance!(on_request_data_file_list);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnScriptGlobalShort(player_id: u16) {
            instance!(on_script_global_short, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnServerScriptCrash(error: *const i8) {
            instance!(on_server_script_crash, unsafe {
                CStr::from_ptr(error).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnVideoPlay(player_id: u16, description: *const i8) {
            instance!(on_video_play, player_id, unsafe {
                CStr::from_ptr(description).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnWorldKillCount(player_id: u16) {
            instance!(on_world_kill_count, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnWorldMap(player_id: u16) {
            instance!(on_world_map, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnWorldWeather(player_id: u16) {
            instance!(on_world_weather, player_id);
        }
    };
}

pub trait Events: Sized {
    fn new() -> Self;

    fn on_actor_ai(&self, player_id: u16, description: &str) {}
    fn on_actor_cell_change(&self, player_id: u16, description: &str) {}
    fn on_actor_death(&self, player_id: u16, description: &str) {}
    fn on_actor_equipment(&self, player_id: u16, description: &str) {}
    fn on_actor_list(&self, player_id: u16, description: &str) {}
    fn on_actor_test(&self, player_id: u16, description: &str) {}

    fn on_cell_deletion(&self, description: &str) {}
    fn on_cell_load(&self, description: &str) {}
    fn on_cell_unload(&self, description: &str) {}

    fn on_container(&self, player_id: u16, description: &str) {}
    fn on_door_state(&self, player_id: u16, description: &str) {}

    fn on_gui_action(&self, player_id: u16, message_box_id: i16, data: &str) {}

    fn on_mp_num_increment(&self, current_mp_num: i16) {}

    fn on_object_activate(&self, player_id: u16, description: &str) {}
    fn on_object_delete(&self, player_id: u16, description: &str) {}
    fn on_object_lock(&self, player_id: u16, description: &str) {}
    fn on_object_place(&self, player_id: u16, description: &str) {}
    fn on_object_scale(&self, player_id: u16, description: &str) {}
    fn on_object_spawn(&self, player_id: u16, description: &str) {}
    fn on_object_state(&self, player_id: u16, description: &str) {}
    fn on_object_trap(&self, player_id: u16, description: &str) {}

    fn on_player_attribute(&self, player_id: u16) {}
    fn on_player_book(&self, player_id: u16) {}
    fn on_player_bounty(&self, player_id: u16) {}
    fn on_player_cell_change(&self, player_id: u16) {}
    fn on_player_connect(&self, player_id: u16) {}
    fn on_player_death(&self, player_id: u16) {}
    fn on_player_disconnect(&self, player_id: u16) {}
    fn on_player_disposition(&self, player_id: u16) {}
    fn on_player_end_char_gen(&self, player_id: u16) {}
    fn on_player_equipment(&self, player_id: u16) {}
    fn on_player_faction(&self, player_id: u16) {}
    fn on_player_input(&self, player_id: u16) {}
    fn on_player_inventory(&self, player_id: u16) {}
    fn on_player_item_use(&self, player_id: u16) {}
    fn on_player_journal(&self, player_id: u16) {}
    fn on_player_level(&self, player_id: u16) {}
    fn on_player_miscellaneous(&self, player_id: u16) {}
    fn on_player_quick_keys(&self, player_id: u16) {}
    fn on_player_reputation(&self, player_id: u16) {}
    fn on_player_rest(&self, player_id: u16) {}
    fn on_player_resurrect(&self, player_id: u16) {}
    fn on_player_send_message(&self, player_id: u16, message: &str) {}
    fn on_player_shapeshift(&self, player_id: u16) {}
    fn on_player_skill(&self, player_id: u16) {}
    fn on_player_spellbook(&self, player_id: u16) {}
    fn on_player_topic(&self, player_id: u16) {}

    fn on_record_dynamic(&self, player_id: u16) {}

    fn on_request_data_file_list(&self) {}

    fn on_script_global_short(&self, player_id: u16) {}

    fn on_server_exit(&self, is_error: bool) {}
    fn on_server_init(&self) {}
    fn on_server_post_init(&self) {}
    fn on_server_script_crash(&self, error: &str) {}

    fn on_video_play(&self, player_id: u16, description: &str) {}

    fn on_world_kill_count(&self, player_id: u16) {}
    fn on_world_map(&self, player_id: u16) {}
    fn on_world_weather(&self, player_id: u16) {}
}
