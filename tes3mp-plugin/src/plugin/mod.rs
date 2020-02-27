#![allow(unused)]

pub mod generated;

pub use generated::*;
use std::os::raw::*;

pub const LOG_VERBOSE: c_ushort = 0;
pub const LOG_INFO: c_ushort = 1;
pub const LOG_WARN: c_ushort = 2;
pub const LOG_ERROR: c_ushort = 3;
pub const LOG_FATAL: c_ushort = 4;

pub const REGULAR: c_uint = 0;
pub const IMPERIAL_SHRINE: c_uint = 1;
pub const TRIBUNAL_TEMPLE: c_uint = 2;

pub const CLIENT_GAMEPLAY: c_uchar = 0;
pub const CLIENT_CONSOLE: c_uchar = 1;
pub const CLIENT_DIALOGUE: c_uchar = 2;
pub const CLIENT_SCRIPT_LOCAL: c_uchar = 3;
pub const CLIENT_SCRIPT_GLOBAL: c_uchar = 4;
pub const SERVER_SCRIPT: c_uchar = 5;

pub const NONE: c_uchar = 0;
pub const DRAG: c_uchar = 1;
pub const DROP: c_uchar = 2;
pub const TAKE_ALL: c_uchar = 3;

pub const ITEM: c_int = 0;
pub const ITEM_MAGIC: c_int = 1;
pub const MAGIC: c_int = 2;
pub const UNASSIGNED: c_int = 3;

pub const SET: c_uchar = 0;
pub const ADD: c_uchar = 1;
pub const REMOVE: c_uchar = 2;
pub const REQUEST: c_uchar = 3;

pub const LOAD: c_uint = 0;
pub const UNLOAD: c_uint = 1;

pub const RANK: c_uchar = 0;
pub const EXPULSION: c_uchar = 1;
pub const REPUTATION: c_uchar = 3;

pub const ENTRY: c_int = 0;
pub const INDEX: c_int = 1;

pub const SPELL: c_ushort = 0;
pub const POTION: c_ushort = 1;
pub const ENCHANTMENT: c_ushort = 2;
pub const NPC: c_ushort = 3;

/// Calls a function on `EVENTS_INSTANCE` with given parameters
#[macro_export]
macro_rules! call_instance {
    ($call:tt, $($argument:expr),+) => {
        let instance = unsafe {
            EVENTS_INSTANCE
                .as_mut()
                .expect(format!("No events instance created: {}\n", stringify!($call)).as_str())
        };
        instance.$call($($argument),+);
        instance.on_any(stringify!($call));
    };

    ($call:tt) => {
        let instance = unsafe {
            EVENTS_INSTANCE
                .as_mut()
                .expect(format!("No events instance created: {}\n", stringify!($call)).as_str())
        };
        instance.$call();
        instance.on_any(stringify!($call));
    };
}

///
/// create and bind C symbols to given struct, should implement Events
///
#[macro_export]
macro_rules! use_events {
    ($events:ident) => {
        use std::ffi::CStr;
        use std::os::raw::*;

        static mut EVENTS_INSTANCE: Option<$events> = None;

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnServerInit() {
            unsafe {
                if (EVENTS_INSTANCE.is_none()) {
                    EVENTS_INSTANCE = Some($events::new());
                }
            }

            call_instance!(on_server_init);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnServerPostInit() {
            call_instance!(on_server_post_init);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnServerExit(is_error: bool) {
            call_instance!(on_server_exit, is_error);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnActorAI(player_id: c_ushort, description: *const i8) {
            call_instance!(on_actor_ai, player_id, unsafe {
                CStr::from_ptr(description).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnActorCellChange(player_id: c_ushort, description: *const i8) {
            call_instance!(on_actor_cell_change, player_id, unsafe {
                CStr::from_ptr(description).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnActorDeath(player_id: c_ushort, description: *const i8) {
            call_instance!(on_actor_death, player_id, unsafe {
                CStr::from_ptr(description).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnActorEquipment(player_id: c_ushort, description: *const i8) {
            call_instance!(on_actor_equipment, player_id, unsafe {
                CStr::from_ptr(description).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnActorList(player_id: c_ushort, description: *const i8) {
            call_instance!(on_actor_list, player_id, unsafe {
                CStr::from_ptr(description).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnActorTest(player_id: c_ushort, description: *const i8) {
            call_instance!(on_actor_test, player_id, unsafe {
                CStr::from_ptr(description).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnCellDeletion(description: *const i8) {
            call_instance!(on_cell_deletion, unsafe {
                CStr::from_ptr(description).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnCellLoad(description: *const i8) {
            call_instance!(on_cell_load, unsafe {
                CStr::from_ptr(description).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnCellUnload(description: *const i8) {
            call_instance!(on_cell_unload, unsafe {
                CStr::from_ptr(description).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnContainer(player_id: c_ushort, description: *const i8) {
            call_instance!(on_container, player_id, unsafe {
                CStr::from_ptr(description).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnDoorState(player_id: c_ushort, description: *const i8) {
            call_instance!(on_door_state, player_id, unsafe {
                CStr::from_ptr(description).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnGUIAction(player_id: c_ushort, message_box_id: c_int, data: *const i8) {
            call_instance!(on_gui_action, player_id, message_box_id, unsafe {
                CStr::from_ptr(data).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnMpNumIncrement(current_mp_num: c_int) {
            call_instance!(on_mp_num_increment, current_mp_num);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnObjectActivate(player_id: c_ushort, description: *const i8) {
            call_instance!(on_object_activate, player_id, unsafe {
                CStr::from_ptr(description).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnObjectDelete(player_id: c_ushort, description: *const i8) {
            call_instance!(on_object_delete, player_id, unsafe {
                CStr::from_ptr(description).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnObjectLock(player_id: c_ushort, description: *const i8) {
            call_instance!(on_object_lock, player_id, unsafe {
                CStr::from_ptr(description).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnObjectPlace(player_id: c_ushort, description: *const i8) {
            call_instance!(on_object_place, player_id, unsafe {
                CStr::from_ptr(description).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnObjectScale(player_id: c_ushort, description: *const i8) {
            call_instance!(on_object_scale, player_id, unsafe {
                CStr::from_ptr(description).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnObjectSpawn(player_id: c_ushort, description: *const i8) {
            call_instance!(on_object_spawn, player_id, unsafe {
                CStr::from_ptr(description).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnObjectState(player_id: c_ushort, description: *const i8) {
            call_instance!(on_object_state, player_id, unsafe {
                CStr::from_ptr(description).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnObjectTrap(player_id: c_ushort, description: *const i8) {
            call_instance!(on_object_trap, player_id, unsafe {
                CStr::from_ptr(description).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerAttribute(player_id: c_ushort) {
            call_instance!(on_player_attribute, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerBook(player_id: c_ushort) {
            call_instance!(on_player_book, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerBounty(player_id: c_ushort) {
            call_instance!(on_player_bounty, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerCellChange(player_id: c_ushort) {
            call_instance!(on_player_cell_change, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerConnect(player_id: c_ushort) {
            call_instance!(on_player_connect, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerDeath(player_id: c_ushort) {
            call_instance!(on_player_death, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerDisconnect(player_id: c_ushort) {
            call_instance!(on_player_disconnect, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerDisposition(player_id: c_ushort) {
            call_instance!(on_player_disposition, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerEndCharGen(player_id: c_ushort) {
            call_instance!(on_player_end_char_gen, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerEquipment(player_id: c_ushort) {
            call_instance!(on_player_equipment, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerFaction(player_id: c_ushort) {
            call_instance!(on_player_faction, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerInput(player_id: c_ushort) {
            call_instance!(on_player_input, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerInventory(player_id: c_ushort) {
            call_instance!(on_player_inventory, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerItemUse(player_id: c_ushort) {
            call_instance!(on_player_item_use, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerJournal(player_id: c_ushort) {
            call_instance!(on_player_journal, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerLevel(player_id: c_ushort) {
            call_instance!(on_player_level, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerMiscellaneous(player_id: c_ushort) {
            call_instance!(on_player_miscellaneous, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerQuickKeys(player_id: c_ushort) {
            call_instance!(on_player_quick_keys, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerReputation(player_id: c_ushort) {
            call_instance!(on_player_reputation, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerRest(player_id: c_ushort) {
            call_instance!(on_player_rest, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerResurrect(player_id: c_ushort) {
            call_instance!(on_player_resurrect, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerSendMessage(player_id: c_ushort, message: *const i8) {
            call_instance!(on_player_send_message, player_id, unsafe {
                CStr::from_ptr(message).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerShapeshift(player_id: c_ushort) {
            call_instance!(on_player_shapeshift, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerSkill(player_id: c_ushort) {
            call_instance!(on_player_skill, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerSpellbook(player_id: c_ushort) {
            call_instance!(on_player_spellbook, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnPlayerTopic(player_id: c_ushort) {
            call_instance!(on_player_topic, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnRecordDynamic(player_id: c_ushort) {
            call_instance!(on_record_dynamic, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnRequestDataFileList() {
            call_instance!(on_request_data_file_list);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnServerScriptCrash(error: *const i8) {
            call_instance!(on_server_script_crash, unsafe {
                CStr::from_ptr(error).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnVideoPlay(player_id: c_ushort, description: *const i8) {
            call_instance!(on_video_play, player_id, unsafe {
                CStr::from_ptr(description).to_str().unwrap_or_default()
            });
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnWorldKillCount(player_id: c_ushort) {
            call_instance!(on_world_kill_count, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnWorldMap(player_id: c_ushort) {
            call_instance!(on_world_map, player_id);
        }

        #[no_mangle]
        #[allow(non_snake_case)]
        pub fn OnWorldWeather(player_id: c_ushort) {
            call_instance!(on_world_weather, player_id);
        }
    };
}

/// Trait implementing all known events TES3MP server can trigger
pub trait Events: Sized {
    fn new() -> Self;
    fn on_any(&mut self, event_name: &str) {}

    fn on_actor_ai(&mut self, player_id: c_ushort, description: &str) {}
    fn on_actor_cell_change(&mut self, player_id: c_ushort, description: &str) {}
    fn on_actor_death(&mut self, player_id: c_ushort, description: &str) {}
    fn on_actor_equipment(&mut self, player_id: c_ushort, description: &str) {}
    fn on_actor_list(&mut self, player_id: c_ushort, description: &str) {}
    fn on_actor_test(&mut self, player_id: c_ushort, description: &str) {}

    fn on_cell_deletion(&mut self, description: &str) {}
    fn on_cell_load(&mut self, description: &str) {}
    fn on_cell_unload(&mut self, description: &str) {}

    fn on_container(&mut self, player_id: c_ushort, description: &str) {}
    fn on_door_state(&mut self, player_id: c_ushort, description: &str) {}

    fn on_gui_action(&mut self, player_id: c_ushort, message_box_id: c_int, data: &str) {}

    fn on_mp_num_increment(&mut self, current_mp_num: c_int) {}

    fn on_object_activate(&mut self, player_id: c_ushort, description: &str) {}
    fn on_object_delete(&mut self, player_id: c_ushort, description: &str) {}
    fn on_object_lock(&mut self, player_id: c_ushort, description: &str) {}
    fn on_object_place(&mut self, player_id: c_ushort, description: &str) {}
    fn on_object_scale(&mut self, player_id: c_ushort, description: &str) {}
    fn on_object_spawn(&mut self, player_id: c_ushort, description: &str) {}
    fn on_object_state(&mut self, player_id: c_ushort, description: &str) {}
    fn on_object_trap(&mut self, player_id: c_ushort, description: &str) {}

    fn on_player_attribute(&mut self, player_id: c_ushort) {}
    fn on_player_book(&mut self, player_id: c_ushort) {}
    fn on_player_bounty(&mut self, player_id: c_ushort) {}
    fn on_player_cell_change(&mut self, player_id: c_ushort) {}
    fn on_player_connect(&mut self, player_id: c_ushort) {}
    fn on_player_death(&mut self, player_id: c_ushort) {}
    fn on_player_disconnect(&mut self, player_id: c_ushort) {}
    fn on_player_disposition(&mut self, player_id: c_ushort) {}
    fn on_player_end_char_gen(&mut self, player_id: c_ushort) {}
    fn on_player_equipment(&mut self, player_id: c_ushort) {}
    fn on_player_faction(&mut self, player_id: c_ushort) {}
    fn on_player_input(&mut self, player_id: c_ushort) {}
    fn on_player_inventory(&mut self, player_id: c_ushort) {}
    fn on_player_item_use(&mut self, player_id: c_ushort) {}
    fn on_player_journal(&mut self, player_id: c_ushort) {}
    fn on_player_level(&mut self, player_id: c_ushort) {}
    fn on_player_miscellaneous(&mut self, player_id: c_ushort) {}
    fn on_player_quick_keys(&mut self, player_id: c_ushort) {}
    fn on_player_reputation(&mut self, player_id: c_ushort) {}
    fn on_player_rest(&mut self, player_id: c_ushort) {}
    fn on_player_resurrect(&mut self, player_id: c_ushort) {}
    fn on_player_send_message(&mut self, player_id: c_ushort, message: &str) {}
    fn on_player_shapeshift(&mut self, player_id: c_ushort) {}
    fn on_player_skill(&mut self, player_id: c_ushort) {}
    fn on_player_spellbook(&mut self, player_id: c_ushort) {}
    fn on_player_topic(&mut self, player_id: c_ushort) {}

    fn on_record_dynamic(&mut self, player_id: c_ushort) {}

    fn on_request_data_file_list(&mut self) {}

    fn on_server_exit(&mut self, is_error: bool) {}
    fn on_server_init(&mut self) {}
    fn on_server_post_init(&mut self) {}
    fn on_server_script_crash(&mut self, error: &str) {}

    fn on_video_play(&mut self, player_id: c_ushort, description: &str) {}

    fn on_world_kill_count(&mut self, player_id: c_ushort) {}
    fn on_world_map(&mut self, player_id: c_ushort) {}
    fn on_world_weather(&mut self, player_id: c_ushort) {}
}
