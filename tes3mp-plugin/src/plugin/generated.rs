use std::ffi::{CStr, CString};

#[allow(non_upper_case_globals)]
pub mod raw {
    #[no_mangle]
    pub static mut prefix: [u8; 4] = *b"rust";
    #[no_mangle]
    pub static mut rustCreateTimer: fn(fn(), i16) -> i16 = |_, _| { unreachable!("CreateTimer was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustMakePublic: fn(fn(), *const i8, i8, *const i8) = |_, _, _, _| { unreachable!("MakePublic was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustStartTimer: fn(i16) = |_| { unreachable!("StartTimer was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustStopTimer: fn(i16) = |_| { unreachable!("StopTimer was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustRestartTimer: fn(i16, i16) = |_, _| { unreachable!("RestartTimer was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustFreeTimer: fn(i16) = |_| { unreachable!("FreeTimer was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustIsTimerElapsed: fn(i16) -> bool = |_| { unreachable!("IsTimerElapsed was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustReadReceivedActorList: fn() = || { unreachable!("ReadReceivedActorList was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustReadCellActorList: fn(*const i8) = |_| { unreachable!("ReadCellActorList was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustClearActorList: fn() = || { unreachable!("ClearActorList was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetActorListPid: fn(u16) = |_| { unreachable!("SetActorListPid was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustCopyReceivedActorListToStore: fn() = || { unreachable!("CopyReceivedActorListToStore was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetActorListSize: fn() -> u16 = || { unreachable!("GetActorListSize was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetActorListAction: fn() -> u8 = || { unreachable!("GetActorListAction was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetActorCell: fn(u16) -> *const i8 = |_| { unreachable!("GetActorCell was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetActorRefId: fn(u16) -> *const i8 = |_| { unreachable!("GetActorRefId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetActorRefNum: fn(u16) -> u16 = |_| { unreachable!("GetActorRefNum was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetActorMpNum: fn(u16) -> u16 = |_| { unreachable!("GetActorMpNum was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetActorPosX: fn(u16) -> f64 = |_| { unreachable!("GetActorPosX was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetActorPosY: fn(u16) -> f64 = |_| { unreachable!("GetActorPosY was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetActorPosZ: fn(u16) -> f64 = |_| { unreachable!("GetActorPosZ was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetActorRotX: fn(u16) -> f64 = |_| { unreachable!("GetActorRotX was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetActorRotY: fn(u16) -> f64 = |_| { unreachable!("GetActorRotY was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetActorRotZ: fn(u16) -> f64 = |_| { unreachable!("GetActorRotZ was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetActorHealthBase: fn(u16) -> f64 = |_| { unreachable!("GetActorHealthBase was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetActorHealthCurrent: fn(u16) -> f64 = |_| { unreachable!("GetActorHealthCurrent was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetActorHealthModified: fn(u16) -> f64 = |_| { unreachable!("GetActorHealthModified was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetActorMagickaBase: fn(u16) -> f64 = |_| { unreachable!("GetActorMagickaBase was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetActorMagickaCurrent: fn(u16) -> f64 = |_| { unreachable!("GetActorMagickaCurrent was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetActorMagickaModified: fn(u16) -> f64 = |_| { unreachable!("GetActorMagickaModified was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetActorFatigueBase: fn(u16) -> f64 = |_| { unreachable!("GetActorFatigueBase was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetActorFatigueCurrent: fn(u16) -> f64 = |_| { unreachable!("GetActorFatigueCurrent was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetActorFatigueModified: fn(u16) -> f64 = |_| { unreachable!("GetActorFatigueModified was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetActorEquipmentItemRefId: fn(u16, u16) -> *const i8 = |_, _| { unreachable!("GetActorEquipmentItemRefId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetActorEquipmentItemCount: fn(u16, u16) -> i16 = |_, _| { unreachable!("GetActorEquipmentItemCount was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetActorEquipmentItemCharge: fn(u16, u16) -> i16 = |_, _| { unreachable!("GetActorEquipmentItemCharge was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetActorEquipmentItemEnchantmentCharge: fn(u16, u16) -> f64 = |_, _| { unreachable!("GetActorEquipmentItemEnchantmentCharge was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustDoesActorHavePlayerKiller: fn(u16) -> bool = |_| { unreachable!("DoesActorHavePlayerKiller was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetActorKillerPid: fn(u16) -> i16 = |_| { unreachable!("GetActorKillerPid was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetActorKillerRefId: fn(u16) -> *const i8 = |_| { unreachable!("GetActorKillerRefId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetActorKillerRefNum: fn(u16) -> u16 = |_| { unreachable!("GetActorKillerRefNum was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetActorKillerMpNum: fn(u16) -> u16 = |_| { unreachable!("GetActorKillerMpNum was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetActorKillerName: fn(u16) -> *const i8 = |_| { unreachable!("GetActorKillerName was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustDoesActorHavePosition: fn(u16) -> bool = |_| { unreachable!("DoesActorHavePosition was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustDoesActorHaveStatsDynamic: fn(u16) -> bool = |_| { unreachable!("DoesActorHaveStatsDynamic was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetActorListCell: fn(*const i8) = |_| { unreachable!("SetActorListCell was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetActorListAction: fn(u8) = |_| { unreachable!("SetActorListAction was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetActorCell: fn(*const i8) = |_| { unreachable!("SetActorCell was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetActorRefId: fn(*const i8) = |_| { unreachable!("SetActorRefId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetActorRefNum: fn(i16) = |_| { unreachable!("SetActorRefNum was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetActorMpNum: fn(i16) = |_| { unreachable!("SetActorMpNum was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetActorPosition: fn(f64, f64, f64) = |_, _, _| { unreachable!("SetActorPosition was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetActorRotation: fn(f64, f64, f64) = |_, _, _| { unreachable!("SetActorRotation was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetActorHealthBase: fn(f64) = |_| { unreachable!("SetActorHealthBase was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetActorHealthCurrent: fn(f64) = |_| { unreachable!("SetActorHealthCurrent was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetActorHealthModified: fn(f64) = |_| { unreachable!("SetActorHealthModified was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetActorMagickaBase: fn(f64) = |_| { unreachable!("SetActorMagickaBase was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetActorMagickaCurrent: fn(f64) = |_| { unreachable!("SetActorMagickaCurrent was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetActorMagickaModified: fn(f64) = |_| { unreachable!("SetActorMagickaModified was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetActorFatigueBase: fn(f64) = |_| { unreachable!("SetActorFatigueBase was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetActorFatigueCurrent: fn(f64) = |_| { unreachable!("SetActorFatigueCurrent was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetActorFatigueModified: fn(f64) = |_| { unreachable!("SetActorFatigueModified was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetActorSound: fn(*const i8) = |_| { unreachable!("SetActorSound was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetActorAIAction: fn(u16) = |_| { unreachable!("SetActorAIAction was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetActorAITargetToPlayer: fn(u16) = |_| { unreachable!("SetActorAITargetToPlayer was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetActorAITargetToObject: fn(i16, i16) = |_, _| { unreachable!("SetActorAITargetToObject was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetActorAICoordinates: fn(f64, f64, f64) = |_, _, _| { unreachable!("SetActorAICoordinates was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetActorAIDistance: fn(u16) = |_| { unreachable!("SetActorAIDistance was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetActorAIDuration: fn(u16) = |_| { unreachable!("SetActorAIDuration was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetActorAIRepetition: fn(bool) = |_| { unreachable!("SetActorAIRepetition was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustEquipActorItem: fn(u16, *const i8, u16, i16, f64) = |_, _, _, _, _| { unreachable!("EquipActorItem was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustUnequipActorItem: fn(u16) = |_| { unreachable!("UnequipActorItem was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustAddActor: fn() = || { unreachable!("AddActor was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendActorList: fn() = || { unreachable!("SendActorList was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendActorAuthority: fn() = || { unreachable!("SendActorAuthority was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendActorPosition: fn(bool, bool) = |_, _| { unreachable!("SendActorPosition was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendActorStatsDynamic: fn(bool, bool) = |_, _| { unreachable!("SendActorStatsDynamic was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendActorEquipment: fn(bool, bool) = |_, _| { unreachable!("SendActorEquipment was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendActorSpeech: fn(bool, bool) = |_, _| { unreachable!("SendActorSpeech was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendActorAI: fn(bool, bool) = |_, _| { unreachable!("SendActorAI was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendActorCellChange: fn(bool, bool) = |_, _| { unreachable!("SendActorCellChange was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustReadLastActorList: fn() = || { unreachable!("ReadLastActorList was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustInitializeActorList: fn(u16) = |_| { unreachable!("InitializeActorList was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustCopyLastActorListToStore: fn() = || { unreachable!("CopyLastActorListToStore was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetActorRefNumIndex: fn(u16) -> u16 = |_| { unreachable!("GetActorRefNumIndex was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetActorKillerRefNumIndex: fn(u16) -> u16 = |_| { unreachable!("GetActorKillerRefNumIndex was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetActorRefNumIndex: fn(i16) = |_| { unreachable!("SetActorRefNumIndex was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustClearBookChanges: fn(u16) = |_| { unreachable!("ClearBookChanges was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetBookChangesSize: fn(u16) -> u16 = |_| { unreachable!("GetBookChangesSize was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustAddBook: fn(u16, *const i8) = |_, _| { unreachable!("AddBook was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetBookId: fn(u16, u16) -> *const i8 = |_, _| { unreachable!("GetBookId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendBookChanges: fn(u16, bool, bool) = |_, _, _| { unreachable!("SendBookChanges was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustInitializeBookChanges: fn(u16) = |_| { unreachable!("InitializeBookChanges was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetCellStateChangesSize: fn(u16) -> u16 = |_| { unreachable!("GetCellStateChangesSize was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetCellStateType: fn(u16, u16) -> u16 = |_, _| { unreachable!("GetCellStateType was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetCellStateDescription: fn(u16, u16) -> *const i8 = |_, _| { unreachable!("GetCellStateDescription was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetCell: fn(u16) -> *const i8 = |_| { unreachable!("GetCell was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetExteriorX: fn(u16) -> i16 = |_| { unreachable!("GetExteriorX was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetExteriorY: fn(u16) -> i16 = |_| { unreachable!("GetExteriorY was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustIsInExterior: fn(u16) -> bool = |_| { unreachable!("IsInExterior was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetRegion: fn(u16) -> *const i8 = |_| { unreachable!("GetRegion was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustIsChangingRegion: fn(u16) -> bool = |_| { unreachable!("IsChangingRegion was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetCell: fn(u16, *const i8) = |_, _| { unreachable!("SetCell was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetExteriorCell: fn(u16, i16, i16) = |_, _, _| { unreachable!("SetExteriorCell was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendCell: fn(u16) = |_| { unreachable!("SendCell was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetDefaultClass: fn(u16) -> *const i8 = |_| { unreachable!("GetDefaultClass was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetClassName: fn(u16) -> *const i8 = |_| { unreachable!("GetClassName was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetClassDesc: fn(u16) -> *const i8 = |_| { unreachable!("GetClassDesc was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetClassMajorAttribute: fn(u16, u8) -> i16 = |_, _| { unreachable!("GetClassMajorAttribute was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetClassSpecialization: fn(u16) -> i16 = |_| { unreachable!("GetClassSpecialization was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetClassMajorSkill: fn(u16, u8) -> i16 = |_, _| { unreachable!("GetClassMajorSkill was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetClassMinorSkill: fn(u16, u8) -> i16 = |_, _| { unreachable!("GetClassMinorSkill was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustIsClassDefault: fn(u16) -> i16 = |_| { unreachable!("IsClassDefault was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetDefaultClass: fn(u16, *const i8) = |_, _| { unreachable!("SetDefaultClass was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetClassName: fn(u16, *const i8) = |_, _| { unreachable!("SetClassName was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetClassDesc: fn(u16, *const i8) = |_, _| { unreachable!("SetClassDesc was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetClassMajorAttribute: fn(u16, u8, i16) = |_, _, _| { unreachable!("SetClassMajorAttribute was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetClassSpecialization: fn(u16, i16) = |_, _| { unreachable!("SetClassSpecialization was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetClassMajorSkill: fn(u16, u8, i16) = |_, _, _| { unreachable!("SetClassMajorSkill was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetClassMinorSkill: fn(u16, u8, i16) = |_, _, _| { unreachable!("SetClassMinorSkill was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendClass: fn(u16) = |_| { unreachable!("SendClass was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendMessage: fn(u16, *const i8, bool, bool) = |_, _, _, _| { unreachable!("SendMessage was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustCleanChatForPid: fn() = || { unreachable!("CleanChatForPid was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustCleanChat: fn(u16) = |_| { unreachable!("CleanChat was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustClearTopicChanges: fn(u16) = |_| { unreachable!("ClearTopicChanges was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetTopicChangesSize: fn(u16) -> u16 = |_| { unreachable!("GetTopicChangesSize was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustAddTopic: fn(u16, *const i8) = |_, _| { unreachable!("AddTopic was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetTopicId: fn(u16, u16) -> *const i8 = |_, _| { unreachable!("GetTopicId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendTopicChanges: fn(u16, bool, bool) = |_, _, _| { unreachable!("SendTopicChanges was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustPlayAnimation: fn(u16, *const i8, i16, i16, bool) = |_, _, _, _, _| { unreachable!("PlayAnimation was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustPlaySpeech: fn(u16, *const i8) = |_, _| { unreachable!("PlaySpeech was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustInitializeTopicChanges: fn(u16) = |_| { unreachable!("InitializeTopicChanges was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustClearFactionChanges: fn(u16) = |_| { unreachable!("ClearFactionChanges was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetFactionChangesSize: fn(u16) -> u16 = |_| { unreachable!("GetFactionChangesSize was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetFactionChangesAction: fn(u16) -> u8 = |_| { unreachable!("GetFactionChangesAction was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetFactionId: fn(u16, u16) -> *const i8 = |_, _| { unreachable!("GetFactionId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetFactionRank: fn(u16, u16) -> i16 = |_, _| { unreachable!("GetFactionRank was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetFactionExpulsionState: fn(u16, u16) -> bool = |_, _| { unreachable!("GetFactionExpulsionState was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetFactionReputation: fn(u16, u16) -> i16 = |_, _| { unreachable!("GetFactionReputation was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetFactionChangesAction: fn(u16, u8) = |_, _| { unreachable!("SetFactionChangesAction was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetFactionId: fn(*const i8) = |_| { unreachable!("SetFactionId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetFactionRank: fn(u16) = |_| { unreachable!("SetFactionRank was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetFactionExpulsionState: fn(bool) = |_| { unreachable!("SetFactionExpulsionState was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetFactionReputation: fn(i16) = |_| { unreachable!("SetFactionReputation was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustAddFaction: fn(u16) = |_| { unreachable!("AddFaction was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendFactionChanges: fn(u16, bool, bool) = |_, _, _| { unreachable!("SendFactionChanges was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustInitializeFactionChanges: fn(u16) = |_| { unreachable!("InitializeFactionChanges was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustCustomMessageBox: fn(u16, i16, *const i8, *const i8) = |_, _, _, _| { unreachable!("CustomMessageBox was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustInputDialog: fn(u16, i16, *const i8, *const i8) = |_, _, _, _| { unreachable!("InputDialog was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustPasswordDialog: fn(u16, i16, *const i8, *const i8) = |_, _, _, _| { unreachable!("PasswordDialog was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustListBox: fn(u16, i16, *const i8, *const i8) = |_, _, _, _| { unreachable!("ListBox was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustClearQuickKeyChanges: fn(u16) = |_| { unreachable!("ClearQuickKeyChanges was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetQuickKeyChangesSize: fn(u16) -> u16 = |_| { unreachable!("GetQuickKeyChangesSize was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetQuickKeySlot: fn(u16, u16) -> i16 = |_, _| { unreachable!("GetQuickKeySlot was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetQuickKeyType: fn(u16, u16) -> i16 = |_, _| { unreachable!("GetQuickKeyType was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetQuickKeyItemId: fn(u16, u16) -> *const i8 = |_, _| { unreachable!("GetQuickKeyItemId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustAddQuickKey: fn(u16, u16, i16, *const i8) = |_, _, _, _| { unreachable!("AddQuickKey was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendQuickKeyChanges: fn(u16) = |_| { unreachable!("SendQuickKeyChanges was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetMapVisibility: fn(u16, u16, u16) = |_, _, _| { unreachable!("SetMapVisibility was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetMapVisibilityAll: fn(u16, u16) = |_, _| { unreachable!("SetMapVisibilityAll was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustInitializeQuickKeyChanges: fn(u16) = |_| { unreachable!("InitializeQuickKeyChanges was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustClearInventoryChanges: fn(u16) = |_| { unreachable!("ClearInventoryChanges was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetEquipmentSize: fn() -> i16 = || { unreachable!("GetEquipmentSize was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetInventoryChangesSize: fn(u16) -> u16 = |_| { unreachable!("GetInventoryChangesSize was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetInventoryChangesAction: fn(u16) -> u16 = |_| { unreachable!("GetInventoryChangesAction was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetInventoryChangesAction: fn(u16, u8) = |_, _| { unreachable!("SetInventoryChangesAction was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustEquipItem: fn(u16, u16, *const i8, u16, i16, f64) = |_, _, _, _, _, _| { unreachable!("EquipItem was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustUnequipItem: fn(u16, u16) = |_, _| { unreachable!("UnequipItem was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustAddItemChange: fn(u16, *const i8, u16, i16, f64, *const i8) = |_, _, _, _, _, _| { unreachable!("AddItemChange was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustHasItemEquipped: fn(u16, *const i8) -> bool = |_, _| { unreachable!("HasItemEquipped was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetEquipmentItemRefId: fn(u16, u16) -> *const i8 = |_, _| { unreachable!("GetEquipmentItemRefId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetEquipmentItemCount: fn(u16, u16) -> i16 = |_, _| { unreachable!("GetEquipmentItemCount was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetEquipmentItemCharge: fn(u16, u16) -> i16 = |_, _| { unreachable!("GetEquipmentItemCharge was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetEquipmentItemEnchantmentCharge: fn(u16, u16) -> f64 = |_, _| { unreachable!("GetEquipmentItemEnchantmentCharge was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetInventoryItemRefId: fn(u16, u16) -> *const i8 = |_, _| { unreachable!("GetInventoryItemRefId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetInventoryItemCount: fn(u16, u16) -> i16 = |_, _| { unreachable!("GetInventoryItemCount was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetInventoryItemCharge: fn(u16, u16) -> i16 = |_, _| { unreachable!("GetInventoryItemCharge was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetInventoryItemEnchantmentCharge: fn(u16, u16) -> f64 = |_, _| { unreachable!("GetInventoryItemEnchantmentCharge was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetInventoryItemSoul: fn(u16, u16) -> *const i8 = |_, _| { unreachable!("GetInventoryItemSoul was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetUsedItemRefId: fn(u16) -> *const i8 = |_| { unreachable!("GetUsedItemRefId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetUsedItemCount: fn(u16) -> i16 = |_| { unreachable!("GetUsedItemCount was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetUsedItemCharge: fn(u16) -> i16 = |_| { unreachable!("GetUsedItemCharge was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetUsedItemEnchantmentCharge: fn(u16) -> f64 = |_| { unreachable!("GetUsedItemEnchantmentCharge was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetUsedItemSoul: fn(u16) -> *const i8 = |_| { unreachable!("GetUsedItemSoul was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendEquipment: fn(u16) = |_| { unreachable!("SendEquipment was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendInventoryChanges: fn(u16, bool, bool) = |_, _, _| { unreachable!("SendInventoryChanges was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendItemUse: fn(u16) = |_| { unreachable!("SendItemUse was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustInitializeInventoryChanges: fn(u16) = |_| { unreachable!("InitializeInventoryChanges was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustAddItem: fn(u16, *const i8, u16, i16, f64, *const i8) = |_, _, _, _, _, _| { unreachable!("AddItem was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetMiscellaneousChangeType: fn(u16) -> u8 = |_| { unreachable!("GetMiscellaneousChangeType was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetMarkCell: fn(u16) -> *const i8 = |_| { unreachable!("GetMarkCell was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetMarkPosX: fn(u16) -> f64 = |_| { unreachable!("GetMarkPosX was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetMarkPosY: fn(u16) -> f64 = |_| { unreachable!("GetMarkPosY was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetMarkPosZ: fn(u16) -> f64 = |_| { unreachable!("GetMarkPosZ was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetMarkRotX: fn(u16) -> f64 = |_| { unreachable!("GetMarkRotX was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetMarkRotZ: fn(u16) -> f64 = |_| { unreachable!("GetMarkRotZ was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetSelectedSpellId: fn(u16) -> *const i8 = |_| { unreachable!("GetSelectedSpellId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustDoesPlayerHavePlayerKiller: fn(u16) -> bool = |_| { unreachable!("DoesPlayerHavePlayerKiller was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetPlayerKillerPid: fn(u16) -> i16 = |_| { unreachable!("GetPlayerKillerPid was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetPlayerKillerRefId: fn(u16) -> *const i8 = |_| { unreachable!("GetPlayerKillerRefId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetPlayerKillerRefNum: fn(u16) -> u16 = |_| { unreachable!("GetPlayerKillerRefNum was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetPlayerKillerMpNum: fn(u16) -> u16 = |_| { unreachable!("GetPlayerKillerMpNum was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetPlayerKillerName: fn(u16) -> *const i8 = |_| { unreachable!("GetPlayerKillerName was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetDrawState: fn(u16) -> u16 = |_| { unreachable!("GetDrawState was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetSneakState: fn(u16) -> bool = |_| { unreachable!("GetSneakState was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetMarkCell: fn(u16, *const i8) = |_, _| { unreachable!("SetMarkCell was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetMarkPos: fn(u16, f64, f64, f64) = |_, _, _, _| { unreachable!("SetMarkPos was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetMarkRot: fn(u16, f64, f64) = |_, _, _| { unreachable!("SetMarkRot was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetSelectedSpellId: fn(u16, *const i8) = |_, _| { unreachable!("SetSelectedSpellId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendMarkLocation: fn(u16) = |_| { unreachable!("SendMarkLocation was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendSelectedSpell: fn(u16) = |_| { unreachable!("SendSelectedSpell was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustJail: fn(u16, i16, bool, bool, *const i8, *const i8) = |_, _, _, _, _, _| { unreachable!("Jail was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustResurrect: fn(u16, u16) = |_, _| { unreachable!("Resurrect was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetDeathReason: fn(u16) -> *const i8 = |_| { unreachable!("GetDeathReason was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetPlayerKillerRefNumIndex: fn(u16) -> u16 = |_| { unreachable!("GetPlayerKillerRefNumIndex was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGenerateRandomString: fn(u16) -> *const i8 = |_| { unreachable!("GenerateRandomString was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetSHA256Hash: fn(*const i8) -> *const i8 = |_| { unreachable!("GetSHA256Hash was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetLastPlayerId: fn() -> u16 = || { unreachable!("GetLastPlayerId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetCurrentMpNum: fn() -> i16 = || { unreachable!("GetCurrentMpNum was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetCurrentMpNum: fn(i16) = |_| { unreachable!("SetCurrentMpNum was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetPosX: fn(u16) -> f64 = |_| { unreachable!("GetPosX was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetPosY: fn(u16) -> f64 = |_| { unreachable!("GetPosY was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetPosZ: fn(u16) -> f64 = |_| { unreachable!("GetPosZ was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetPreviousCellPosX: fn(u16) -> f64 = |_| { unreachable!("GetPreviousCellPosX was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetPreviousCellPosY: fn(u16) -> f64 = |_| { unreachable!("GetPreviousCellPosY was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetPreviousCellPosZ: fn(u16) -> f64 = |_| { unreachable!("GetPreviousCellPosZ was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetRotX: fn(u16) -> f64 = |_| { unreachable!("GetRotX was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetRotZ: fn(u16) -> f64 = |_| { unreachable!("GetRotZ was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetPos: fn(u16, f64, f64, f64) = |_, _, _, _| { unreachable!("SetPos was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRot: fn(u16, f64, f64) = |_, _, _| { unreachable!("SetRot was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetMomentum: fn(u16, f64, f64, f64) = |_, _, _, _| { unreachable!("SetMomentum was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendPos: fn(u16) = |_| { unreachable!("SendPos was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendMomentum: fn(u16) = |_| { unreachable!("SendMomentum was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustClearJournalChanges: fn(u16) = |_| { unreachable!("ClearJournalChanges was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetJournalChangesSize: fn(u16) -> u16 = |_| { unreachable!("GetJournalChangesSize was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustAddJournalEntry: fn(u16, *const i8, u16, *const i8) = |_, _, _, _| { unreachable!("AddJournalEntry was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustAddJournalEntryWithTimestamp: fn(u16, *const i8, u16, *const i8, u16, u16, u16) = |_, _, _, _, _, _, _| { unreachable!("AddJournalEntryWithTimestamp was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustAddJournalIndex: fn(u16, *const i8, u16) = |_, _, _| { unreachable!("AddJournalIndex was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetReputation: fn(u16, i16) = |_, _| { unreachable!("SetReputation was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetJournalItemQuest: fn(u16, u16) -> *const i8 = |_, _| { unreachable!("GetJournalItemQuest was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetJournalItemIndex: fn(u16, u16) -> i16 = |_, _| { unreachable!("GetJournalItemIndex was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetJournalItemType: fn(u16, u16) -> i16 = |_, _| { unreachable!("GetJournalItemType was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetJournalItemActorRefId: fn(u16, u16) -> *const i8 = |_, _| { unreachable!("GetJournalItemActorRefId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetReputation: fn(u16) -> i16 = |_| { unreachable!("GetReputation was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendJournalChanges: fn(u16, bool, bool) = |_, _, _| { unreachable!("SendJournalChanges was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendReputation: fn(u16, bool, bool) = |_, _, _| { unreachable!("SendReputation was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustInitializeJournalChanges: fn(u16) = |_| { unreachable!("InitializeJournalChanges was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustClearRecords: fn() = || { unreachable!("ClearRecords was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetRecordType: fn() -> u16 = || { unreachable!("GetRecordType was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetRecordCount: fn() -> u16 = || { unreachable!("GetRecordCount was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetRecordEffectCount: fn(u16) -> u16 = |_| { unreachable!("GetRecordEffectCount was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetRecordId: fn(u16) -> *const i8 = |_| { unreachable!("GetRecordId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetRecordBaseId: fn(u16) -> *const i8 = |_| { unreachable!("GetRecordBaseId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetRecordSubtype: fn(u16) -> i16 = |_| { unreachable!("GetRecordSubtype was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetRecordName: fn(u16) -> *const i8 = |_| { unreachable!("GetRecordName was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetRecordModel: fn(u16) -> *const i8 = |_| { unreachable!("GetRecordModel was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetRecordIcon: fn(u16) -> *const i8 = |_| { unreachable!("GetRecordIcon was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetRecordScript: fn(u16) -> *const i8 = |_| { unreachable!("GetRecordScript was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetRecordEnchantmentId: fn(u16) -> *const i8 = |_| { unreachable!("GetRecordEnchantmentId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetRecordEnchantmentCharge: fn(u16) -> i16 = |_| { unreachable!("GetRecordEnchantmentCharge was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetRecordAutoCalc: fn(u16) -> i16 = |_| { unreachable!("GetRecordAutoCalc was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetRecordCharge: fn(u16) -> i16 = |_| { unreachable!("GetRecordCharge was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetRecordCost: fn(u16) -> i16 = |_| { unreachable!("GetRecordCost was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetRecordFlags: fn(u16) -> i16 = |_| { unreachable!("GetRecordFlags was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetRecordValue: fn(u16) -> i16 = |_| { unreachable!("GetRecordValue was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetRecordWeight: fn(u16) -> f64 = |_| { unreachable!("GetRecordWeight was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetRecordEffectId: fn(u16, u16) -> u16 = |_, _| { unreachable!("GetRecordEffectId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetRecordEffectAttribute: fn(u16, u16) -> i16 = |_, _| { unreachable!("GetRecordEffectAttribute was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetRecordEffectSkill: fn(u16, u16) -> i16 = |_, _| { unreachable!("GetRecordEffectSkill was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetRecordEffectRangeType: fn(u16, u16) -> u16 = |_, _| { unreachable!("GetRecordEffectRangeType was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetRecordEffectArea: fn(u16, u16) -> i16 = |_, _| { unreachable!("GetRecordEffectArea was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetRecordEffectDuration: fn(u16, u16) -> i16 = |_, _| { unreachable!("GetRecordEffectDuration was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetRecordEffectMagnitudeMax: fn(u16, u16) -> i16 = |_, _| { unreachable!("GetRecordEffectMagnitudeMax was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetRecordEffectMagnitudeMin: fn(u16, u16) -> i16 = |_, _| { unreachable!("GetRecordEffectMagnitudeMin was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordType: fn(u16) = |_| { unreachable!("SetRecordType was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordId: fn(*const i8) = |_| { unreachable!("SetRecordId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordBaseId: fn(*const i8) = |_| { unreachable!("SetRecordBaseId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordInventoryBaseId: fn(*const i8) = |_| { unreachable!("SetRecordInventoryBaseId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordSubtype: fn(u16) = |_| { unreachable!("SetRecordSubtype was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordName: fn(*const i8) = |_| { unreachable!("SetRecordName was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordModel: fn(*const i8) = |_| { unreachable!("SetRecordModel was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordIcon: fn(*const i8) = |_| { unreachable!("SetRecordIcon was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordScript: fn(*const i8) = |_| { unreachable!("SetRecordScript was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordEnchantmentId: fn(*const i8) = |_| { unreachable!("SetRecordEnchantmentId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordEnchantmentCharge: fn(i16) = |_| { unreachable!("SetRecordEnchantmentCharge was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordAutoCalc: fn(i16) = |_| { unreachable!("SetRecordAutoCalc was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordCharge: fn(i16) = |_| { unreachable!("SetRecordCharge was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordCost: fn(i16) = |_| { unreachable!("SetRecordCost was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordFlags: fn(i16) = |_| { unreachable!("SetRecordFlags was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordValue: fn(i16) = |_| { unreachable!("SetRecordValue was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordWeight: fn(f64) = |_| { unreachable!("SetRecordWeight was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordQuality: fn(f64) = |_| { unreachable!("SetRecordQuality was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordUses: fn(i16) = |_| { unreachable!("SetRecordUses was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordTime: fn(i16) = |_| { unreachable!("SetRecordTime was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordRadius: fn(i16) = |_| { unreachable!("SetRecordRadius was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordColor: fn(u16, u16, u16) = |_, _, _| { unreachable!("SetRecordColor was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordArmorRating: fn(i16) = |_| { unreachable!("SetRecordArmorRating was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordHealth: fn(i16) = |_| { unreachable!("SetRecordHealth was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordDamageChop: fn(u16, u16) = |_, _| { unreachable!("SetRecordDamageChop was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordDamageSlash: fn(u16, u16) = |_, _| { unreachable!("SetRecordDamageSlash was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordDamageThrust: fn(u16, u16) = |_, _| { unreachable!("SetRecordDamageThrust was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordReach: fn(f64) = |_| { unreachable!("SetRecordReach was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordSpeed: fn(f64) = |_| { unreachable!("SetRecordSpeed was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordKeyState: fn(bool) = |_| { unreachable!("SetRecordKeyState was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordScrollState: fn(bool) = |_| { unreachable!("SetRecordScrollState was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordSkillId: fn(i16) = |_| { unreachable!("SetRecordSkillId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordText: fn(*const i8) = |_| { unreachable!("SetRecordText was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordHair: fn(*const i8) = |_| { unreachable!("SetRecordHair was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordHead: fn(*const i8) = |_| { unreachable!("SetRecordHead was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordGender: fn(u16) = |_| { unreachable!("SetRecordGender was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordRace: fn(*const i8) = |_| { unreachable!("SetRecordRace was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordClass: fn(*const i8) = |_| { unreachable!("SetRecordClass was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordFaction: fn(*const i8) = |_| { unreachable!("SetRecordFaction was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordScale: fn(f64) = |_| { unreachable!("SetRecordScale was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordBloodType: fn(i16) = |_| { unreachable!("SetRecordBloodType was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordLevel: fn(i16) = |_| { unreachable!("SetRecordLevel was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordMagicka: fn(i16) = |_| { unreachable!("SetRecordMagicka was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordFatigue: fn(i16) = |_| { unreachable!("SetRecordFatigue was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordAIFight: fn(i16) = |_| { unreachable!("SetRecordAIFight was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordAIFlee: fn(i16) = |_| { unreachable!("SetRecordAIFlee was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordAIAlarm: fn(i16) = |_| { unreachable!("SetRecordAIAlarm was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordAIServices: fn(i16) = |_| { unreachable!("SetRecordAIServices was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordSound: fn(*const i8) = |_| { unreachable!("SetRecordSound was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordOpenSound: fn(*const i8) = |_| { unreachable!("SetRecordOpenSound was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordCloseSound: fn(*const i8) = |_| { unreachable!("SetRecordCloseSound was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordScriptText: fn(*const i8) = |_| { unreachable!("SetRecordScriptText was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordIdByIndex: fn(u16, *const i8) = |_, _| { unreachable!("SetRecordIdByIndex was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordEnchantmentIdByIndex: fn(u16, *const i8) = |_, _| { unreachable!("SetRecordEnchantmentIdByIndex was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordEffectId: fn(u16) = |_| { unreachable!("SetRecordEffectId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordEffectAttribute: fn(i16) = |_| { unreachable!("SetRecordEffectAttribute was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordEffectSkill: fn(i16) = |_| { unreachable!("SetRecordEffectSkill was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordEffectRangeType: fn(u16) = |_| { unreachable!("SetRecordEffectRangeType was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordEffectArea: fn(i16) = |_| { unreachable!("SetRecordEffectArea was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordEffectDuration: fn(i16) = |_| { unreachable!("SetRecordEffectDuration was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordEffectMagnitudeMax: fn(i16) = |_| { unreachable!("SetRecordEffectMagnitudeMax was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordEffectMagnitudeMin: fn(i16) = |_| { unreachable!("SetRecordEffectMagnitudeMin was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordBodyPartType: fn(u16) = |_| { unreachable!("SetRecordBodyPartType was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordBodyPartIdForMale: fn(*const i8) = |_| { unreachable!("SetRecordBodyPartIdForMale was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordBodyPartIdForFemale: fn(*const i8) = |_| { unreachable!("SetRecordBodyPartIdForFemale was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordInventoryItemId: fn(*const i8) = |_| { unreachable!("SetRecordInventoryItemId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRecordInventoryItemCount: fn(u16) = |_| { unreachable!("SetRecordInventoryItemCount was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustAddRecord: fn() = || { unreachable!("AddRecord was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustAddRecordEffect: fn() = || { unreachable!("AddRecordEffect was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustAddRecordBodyPart: fn() = || { unreachable!("AddRecordBodyPart was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustAddRecordInventoryItem: fn() = || { unreachable!("AddRecordInventoryItem was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendRecordDynamic: fn(u16, bool, bool) = |_, _, _| { unreachable!("SendRecordDynamic was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetScale: fn(u16) -> f64 = |_| { unreachable!("GetScale was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustIsWerewolf: fn(u16) -> bool = |_| { unreachable!("IsWerewolf was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetCreatureRefId: fn(u16) -> *const i8 = |_| { unreachable!("GetCreatureRefId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetCreatureNameDisplayState: fn(u16) -> bool = |_| { unreachable!("GetCreatureNameDisplayState was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetScale: fn(u16, f64) = |_, _| { unreachable!("SetScale was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetWerewolfState: fn(u16, bool) = |_, _| { unreachable!("SetWerewolfState was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetCreatureRefId: fn(u16, *const i8) = |_, _| { unreachable!("SetCreatureRefId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetCreatureNameDisplayState: fn(u16, bool) = |_, _| { unreachable!("SetCreatureNameDisplayState was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendShapeshift: fn(u16) = |_| { unreachable!("SendShapeshift was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustLogMessage: fn(u16, *const i8) = |_, _| { unreachable!("LogMessage was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustLogAppend: fn(u16, *const i8) = |_, _| { unreachable!("LogAppend was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustStopServer: fn(i16) = |_| { unreachable!("StopServer was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustKick: fn(u16) = |_| { unreachable!("Kick was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustBanAddress: fn(*const i8) = |_| { unreachable!("BanAddress was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustUnbanAddress: fn(*const i8) = |_| { unreachable!("UnbanAddress was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustDoesFilePathExist: fn(*const i8) -> bool = |_| { unreachable!("DoesFilePathExist was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetCaseInsensitiveFilename: fn(*const i8, *const i8) -> *const i8 = |_, _| { unreachable!("GetCaseInsensitiveFilename was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetDataPath: fn() -> *const i8 = || { unreachable!("GetDataPath was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetMillisecondsSinceServerStart: fn() -> u16 = || { unreachable!("GetMillisecondsSinceServerStart was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetOperatingSystemType: fn() -> *const i8 = || { unreachable!("GetOperatingSystemType was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetArchitectureType: fn() -> *const i8 = || { unreachable!("GetArchitectureType was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetServerVersion: fn() -> *const i8 = || { unreachable!("GetServerVersion was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetProtocolVersion: fn() -> *const i8 = || { unreachable!("GetProtocolVersion was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetAvgPing: fn(u16) -> i16 = |_| { unreachable!("GetAvgPing was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetIP: fn(u16) -> *const i8 = |_| { unreachable!("GetIP was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetMaxPlayers: fn() -> u16 = || { unreachable!("GetMaxPlayers was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetPort: fn() -> u16 = || { unreachable!("GetPort was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustHasPassword: fn() -> bool = || { unreachable!("HasPassword was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetDataFileEnforcementState: fn() -> bool = || { unreachable!("GetDataFileEnforcementState was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetScriptErrorIgnoringState: fn() -> bool = || { unreachable!("GetScriptErrorIgnoringState was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetGameMode: fn(*const i8) = |_| { unreachable!("SetGameMode was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetHostname: fn(*const i8) = |_| { unreachable!("SetHostname was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetServerPassword: fn(*const i8) = |_| { unreachable!("SetServerPassword was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetDataFileEnforcementState: fn(bool) = |_| { unreachable!("SetDataFileEnforcementState was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetScriptErrorIgnoringState: fn(bool) = |_| { unreachable!("SetScriptErrorIgnoringState was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRuleString: fn(*const i8, *const i8) = |_, _| { unreachable!("SetRuleString was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRuleValue: fn(*const i8, f64) = |_, _| { unreachable!("SetRuleValue was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustAddDataFileRequirement: fn(*const i8, *const i8) = |_, _| { unreachable!("AddDataFileRequirement was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustDoesFileExist: fn(*const i8) -> bool = |_| { unreachable!("DoesFileExist was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetModDir: fn() -> *const i8 = || { unreachable!("GetModDir was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetPluginEnforcementState: fn() -> bool = || { unreachable!("GetPluginEnforcementState was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetPluginEnforcementState: fn(bool) = |_| { unreachable!("SetPluginEnforcementState was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustAddPluginHash: fn(*const i8, *const i8) = |_, _| { unreachable!("AddPluginHash was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetDifficulty: fn(u16, i16) = |_, _| { unreachable!("SetDifficulty was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetEnforcedLogLevel: fn(u16, i16) = |_, _| { unreachable!("SetEnforcedLogLevel was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetPhysicsFramerate: fn(u16, f64) = |_, _| { unreachable!("SetPhysicsFramerate was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetConsoleAllowed: fn(u16, bool) = |_, _| { unreachable!("SetConsoleAllowed was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetBedRestAllowed: fn(u16, bool) = |_, _| { unreachable!("SetBedRestAllowed was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetWildernessRestAllowed: fn(u16, bool) = |_, _| { unreachable!("SetWildernessRestAllowed was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetWaitAllowed: fn(u16, bool) = |_, _| { unreachable!("SetWaitAllowed was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendSettings: fn(u16) = |_| { unreachable!("SendSettings was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustClearSpellbookChanges: fn(u16) = |_| { unreachable!("ClearSpellbookChanges was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetSpellbookChangesSize: fn(u16) -> u16 = |_| { unreachable!("GetSpellbookChangesSize was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetSpellbookChangesAction: fn(u16) -> u16 = |_| { unreachable!("GetSpellbookChangesAction was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetSpellbookChangesAction: fn(u16, u8) = |_, _| { unreachable!("SetSpellbookChangesAction was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustAddSpell: fn(u16, *const i8) = |_, _| { unreachable!("AddSpell was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetSpellId: fn(u16, u16) -> *const i8 = |_, _| { unreachable!("GetSpellId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendSpellbookChanges: fn(u16, bool, bool) = |_, _, _| { unreachable!("SendSpellbookChanges was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustInitializeSpellbookChanges: fn(u16) = |_| { unreachable!("InitializeSpellbookChanges was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetAttributeCount: fn() -> i16 = || { unreachable!("GetAttributeCount was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetSkillCount: fn() -> i16 = || { unreachable!("GetSkillCount was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetAttributeId: fn(*const i8) -> i16 = |_| { unreachable!("GetAttributeId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetSkillId: fn(*const i8) -> i16 = |_| { unreachable!("GetSkillId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetAttributeName: fn(u16) -> *const i8 = |_| { unreachable!("GetAttributeName was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetSkillName: fn(u16) -> *const i8 = |_| { unreachable!("GetSkillName was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetName: fn(u16) -> *const i8 = |_| { unreachable!("GetName was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetRace: fn(u16) -> *const i8 = |_| { unreachable!("GetRace was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetHead: fn(u16) -> *const i8 = |_| { unreachable!("GetHead was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetHair: fn(u16) -> *const i8 = |_| { unreachable!("GetHair was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetIsMale: fn(u16) -> i16 = |_| { unreachable!("GetIsMale was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetBirthsign: fn(u16) -> *const i8 = |_| { unreachable!("GetBirthsign was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetLevel: fn(u16) -> i16 = |_| { unreachable!("GetLevel was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetLevelProgress: fn(u16) -> i16 = |_| { unreachable!("GetLevelProgress was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetHealthBase: fn(u16) -> f64 = |_| { unreachable!("GetHealthBase was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetHealthCurrent: fn(u16) -> f64 = |_| { unreachable!("GetHealthCurrent was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetMagickaBase: fn(u16) -> f64 = |_| { unreachable!("GetMagickaBase was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetMagickaCurrent: fn(u16) -> f64 = |_| { unreachable!("GetMagickaCurrent was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetFatigueBase: fn(u16) -> f64 = |_| { unreachable!("GetFatigueBase was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetFatigueCurrent: fn(u16) -> f64 = |_| { unreachable!("GetFatigueCurrent was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetAttributeBase: fn(u16, u16) -> i16 = |_, _| { unreachable!("GetAttributeBase was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetAttributeModifier: fn(u16, u16) -> i16 = |_, _| { unreachable!("GetAttributeModifier was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetAttributeDamage: fn(u16, u16) -> f64 = |_, _| { unreachable!("GetAttributeDamage was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetSkillBase: fn(u16, u16) -> i16 = |_, _| { unreachable!("GetSkillBase was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetSkillModifier: fn(u16, u16) -> i16 = |_, _| { unreachable!("GetSkillModifier was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetSkillDamage: fn(u16, u16) -> f64 = |_, _| { unreachable!("GetSkillDamage was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetSkillProgress: fn(u16, u16) -> f64 = |_, _| { unreachable!("GetSkillProgress was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetSkillIncrease: fn(u16, u16) -> i16 = |_, _| { unreachable!("GetSkillIncrease was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetBounty: fn(u16) -> i16 = |_| { unreachable!("GetBounty was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetName: fn(u16, *const i8) = |_, _| { unreachable!("SetName was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetRace: fn(u16, *const i8) = |_, _| { unreachable!("SetRace was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetHead: fn(u16, *const i8) = |_, _| { unreachable!("SetHead was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetHair: fn(u16, *const i8) = |_, _| { unreachable!("SetHair was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetIsMale: fn(u16, i16) = |_, _| { unreachable!("SetIsMale was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetBirthsign: fn(u16, *const i8) = |_, _| { unreachable!("SetBirthsign was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetResetStats: fn(u16, bool) = |_, _| { unreachable!("SetResetStats was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetLevel: fn(u16, i16) = |_, _| { unreachable!("SetLevel was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetLevelProgress: fn(u16, i16) = |_, _| { unreachable!("SetLevelProgress was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetHealthBase: fn(u16, f64) = |_, _| { unreachable!("SetHealthBase was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetHealthCurrent: fn(u16, f64) = |_, _| { unreachable!("SetHealthCurrent was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetMagickaBase: fn(u16, f64) = |_, _| { unreachable!("SetMagickaBase was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetMagickaCurrent: fn(u16, f64) = |_, _| { unreachable!("SetMagickaCurrent was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetFatigueBase: fn(u16, f64) = |_, _| { unreachable!("SetFatigueBase was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetFatigueCurrent: fn(u16, f64) = |_, _| { unreachable!("SetFatigueCurrent was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetAttributeBase: fn(u16, u16, i16) = |_, _, _| { unreachable!("SetAttributeBase was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustClearAttributeModifier: fn(u16, u16) = |_, _| { unreachable!("ClearAttributeModifier was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetAttributeDamage: fn(u16, u16, f64) = |_, _, _| { unreachable!("SetAttributeDamage was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetSkillBase: fn(u16, u16, i16) = |_, _, _| { unreachable!("SetSkillBase was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustClearSkillModifier: fn(u16, u16) = |_, _| { unreachable!("ClearSkillModifier was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetSkillDamage: fn(u16, u16, f64) = |_, _, _| { unreachable!("SetSkillDamage was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetSkillProgress: fn(u16, u16, f64) = |_, _, _| { unreachable!("SetSkillProgress was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetSkillIncrease: fn(u16, u16, i16) = |_, _, _| { unreachable!("SetSkillIncrease was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetBounty: fn(u16, i16) = |_, _| { unreachable!("SetBounty was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetCharGenStage: fn(u16, i16, i16) = |_, _, _| { unreachable!("SetCharGenStage was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendBaseInfo: fn(u16) = |_| { unreachable!("SendBaseInfo was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendStatsDynamic: fn(u16) = |_| { unreachable!("SendStatsDynamic was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendAttributes: fn(u16) = |_| { unreachable!("SendAttributes was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendSkills: fn(u16) = |_| { unreachable!("SendSkills was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendLevel: fn(u16) = |_| { unreachable!("SendLevel was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendBounty: fn(u16) = |_| { unreachable!("SendBounty was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustReadReceivedObjectList: fn() = || { unreachable!("ReadReceivedObjectList was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustClearObjectList: fn() = || { unreachable!("ClearObjectList was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetObjectListPid: fn(u16) = |_| { unreachable!("SetObjectListPid was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustCopyReceivedObjectListToStore: fn() = || { unreachable!("CopyReceivedObjectListToStore was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetObjectListSize: fn() -> u16 = || { unreachable!("GetObjectListSize was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetObjectListOrigin: fn() -> u8 = || { unreachable!("GetObjectListOrigin was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetObjectListClientScript: fn() -> *const i8 = || { unreachable!("GetObjectListClientScript was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetObjectListAction: fn() -> u8 = || { unreachable!("GetObjectListAction was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetObjectListContainerSubAction: fn() -> u8 = || { unreachable!("GetObjectListContainerSubAction was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustIsObjectPlayer: fn(u16) -> bool = |_| { unreachable!("IsObjectPlayer was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetObjectPid: fn(u16) -> i16 = |_| { unreachable!("GetObjectPid was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetObjectRefId: fn(u16) -> *const i8 = |_| { unreachable!("GetObjectRefId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetObjectRefNum: fn(u16) -> u16 = |_| { unreachable!("GetObjectRefNum was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetObjectMpNum: fn(u16) -> u16 = |_| { unreachable!("GetObjectMpNum was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetObjectCount: fn(u16) -> i16 = |_| { unreachable!("GetObjectCount was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetObjectCharge: fn(u16) -> i16 = |_| { unreachable!("GetObjectCharge was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetObjectEnchantmentCharge: fn(u16) -> f64 = |_| { unreachable!("GetObjectEnchantmentCharge was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetObjectSoul: fn(u16) -> *const i8 = |_| { unreachable!("GetObjectSoul was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetObjectGoldValue: fn(u16) -> i16 = |_| { unreachable!("GetObjectGoldValue was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetObjectScale: fn(u16) -> f64 = |_| { unreachable!("GetObjectScale was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetObjectState: fn(u16) -> bool = |_| { unreachable!("GetObjectState was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetObjectDoorState: fn(u16) -> i16 = |_| { unreachable!("GetObjectDoorState was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetObjectLockLevel: fn(u16) -> i16 = |_| { unreachable!("GetObjectLockLevel was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustDoesObjectHavePlayerActivating: fn(u16) -> bool = |_| { unreachable!("DoesObjectHavePlayerActivating was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetObjectActivatingPid: fn(u16) -> i16 = |_| { unreachable!("GetObjectActivatingPid was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetObjectActivatingRefId: fn(u16) -> *const i8 = |_| { unreachable!("GetObjectActivatingRefId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetObjectActivatingRefNum: fn(u16) -> u16 = |_| { unreachable!("GetObjectActivatingRefNum was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetObjectActivatingMpNum: fn(u16) -> u16 = |_| { unreachable!("GetObjectActivatingMpNum was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetObjectActivatingName: fn(u16) -> *const i8 = |_| { unreachable!("GetObjectActivatingName was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetObjectSummonState: fn(u16) -> bool = |_| { unreachable!("GetObjectSummonState was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetObjectSummonDuration: fn(u16) -> f64 = |_| { unreachable!("GetObjectSummonDuration was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustDoesObjectHavePlayerSummoner: fn(u16) -> bool = |_| { unreachable!("DoesObjectHavePlayerSummoner was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetObjectSummonerPid: fn(u16) -> i16 = |_| { unreachable!("GetObjectSummonerPid was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetObjectSummonerRefId: fn(u16) -> *const i8 = |_| { unreachable!("GetObjectSummonerRefId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetObjectSummonerRefNum: fn(u16) -> u16 = |_| { unreachable!("GetObjectSummonerRefNum was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetObjectSummonerMpNum: fn(u16) -> u16 = |_| { unreachable!("GetObjectSummonerMpNum was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetObjectPosX: fn(u16) -> f64 = |_| { unreachable!("GetObjectPosX was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetObjectPosY: fn(u16) -> f64 = |_| { unreachable!("GetObjectPosY was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetObjectPosZ: fn(u16) -> f64 = |_| { unreachable!("GetObjectPosZ was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetObjectRotX: fn(u16) -> f64 = |_| { unreachable!("GetObjectRotX was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetObjectRotY: fn(u16) -> f64 = |_| { unreachable!("GetObjectRotY was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetObjectRotZ: fn(u16) -> f64 = |_| { unreachable!("GetObjectRotZ was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetVideoFilename: fn(u16) -> *const i8 = |_| { unreachable!("GetVideoFilename was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetScriptVariableName: fn(u16) -> *const i8 = |_| { unreachable!("GetScriptVariableName was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetScriptVariableShortValue: fn(u16) -> i16 = |_| { unreachable!("GetScriptVariableShortValue was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetContainerChangesSize: fn(u16) -> u16 = |_| { unreachable!("GetContainerChangesSize was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetContainerItemRefId: fn(u16, u16) -> *const i8 = |_, _| { unreachable!("GetContainerItemRefId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetContainerItemCount: fn(u16, u16) -> i16 = |_, _| { unreachable!("GetContainerItemCount was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetContainerItemCharge: fn(u16, u16) -> i16 = |_, _| { unreachable!("GetContainerItemCharge was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetContainerItemEnchantmentCharge: fn(u16, u16) -> f64 = |_, _| { unreachable!("GetContainerItemEnchantmentCharge was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetContainerItemSoul: fn(u16, u16) -> *const i8 = |_, _| { unreachable!("GetContainerItemSoul was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetContainerItemActionCount: fn(u16, u16) -> i16 = |_, _| { unreachable!("GetContainerItemActionCount was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustDoesObjectHaveContainer: fn(u16) -> bool = |_| { unreachable!("DoesObjectHaveContainer was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetObjectListCell: fn(*const i8) = |_| { unreachable!("SetObjectListCell was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetObjectListAction: fn(u8) = |_| { unreachable!("SetObjectListAction was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetObjectListConsoleCommand: fn(*const i8) = |_| { unreachable!("SetObjectListConsoleCommand was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetObjectRefId: fn(*const i8) = |_| { unreachable!("SetObjectRefId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetObjectRefNum: fn(i16) = |_| { unreachable!("SetObjectRefNum was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetObjectMpNum: fn(i16) = |_| { unreachable!("SetObjectMpNum was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetObjectCount: fn(i16) = |_| { unreachable!("SetObjectCount was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetObjectCharge: fn(i16) = |_| { unreachable!("SetObjectCharge was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetObjectEnchantmentCharge: fn(f64) = |_| { unreachable!("SetObjectEnchantmentCharge was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetObjectSoul: fn(*const i8) = |_| { unreachable!("SetObjectSoul was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetObjectGoldValue: fn(i16) = |_| { unreachable!("SetObjectGoldValue was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetObjectScale: fn(f64) = |_| { unreachable!("SetObjectScale was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetObjectState: fn(bool) = |_| { unreachable!("SetObjectState was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetObjectLockLevel: fn(i16) = |_| { unreachable!("SetObjectLockLevel was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetObjectDisarmState: fn(bool) = |_| { unreachable!("SetObjectDisarmState was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetObjectSummonDuration: fn(f32) = |_| { unreachable!("SetObjectSummonDuration was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetObjectSummonState: fn(bool) = |_| { unreachable!("SetObjectSummonState was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetObjectPosition: fn(f64, f64, f64) = |_, _, _| { unreachable!("SetObjectPosition was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetObjectRotation: fn(f64, f64, f64) = |_, _, _| { unreachable!("SetObjectRotation was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetObjectActivatingPid: fn(u16) = |_| { unreachable!("SetObjectActivatingPid was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetObjectDoorState: fn(i16) = |_| { unreachable!("SetObjectDoorState was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetObjectDoorTeleportState: fn(bool) = |_| { unreachable!("SetObjectDoorTeleportState was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetObjectDoorDestinationCell: fn(*const i8) = |_| { unreachable!("SetObjectDoorDestinationCell was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetObjectDoorDestinationPosition: fn(f64, f64, f64) = |_, _, _| { unreachable!("SetObjectDoorDestinationPosition was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetObjectDoorDestinationRotation: fn(f64, f64) = |_, _| { unreachable!("SetObjectDoorDestinationRotation was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetScriptVariableName: fn(*const i8) = |_| { unreachable!("SetScriptVariableName was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetScriptVariableShortValue: fn(i16) = |_| { unreachable!("SetScriptVariableShortValue was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetPlayerAsObject: fn(u16) = |_| { unreachable!("SetPlayerAsObject was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetContainerItemRefId: fn(*const i8) = |_| { unreachable!("SetContainerItemRefId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetContainerItemCount: fn(i16) = |_| { unreachable!("SetContainerItemCount was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetContainerItemCharge: fn(i16) = |_| { unreachable!("SetContainerItemCharge was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetContainerItemEnchantmentCharge: fn(f64) = |_| { unreachable!("SetContainerItemEnchantmentCharge was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetContainerItemSoul: fn(*const i8) = |_| { unreachable!("SetContainerItemSoul was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetContainerItemActionCountByIndex: fn(u16, u16, i16) = |_, _, _| { unreachable!("SetContainerItemActionCountByIndex was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustAddObject: fn() = || { unreachable!("AddObject was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustAddContainerItem: fn() = || { unreachable!("AddContainerItem was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendObjectActivate: fn(bool, bool) = |_, _| { unreachable!("SendObjectActivate was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendObjectPlace: fn(bool, bool) = |_, _| { unreachable!("SendObjectPlace was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendObjectSpawn: fn(bool, bool) = |_, _| { unreachable!("SendObjectSpawn was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendObjectDelete: fn(bool, bool) = |_, _| { unreachable!("SendObjectDelete was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendObjectLock: fn(bool, bool) = |_, _| { unreachable!("SendObjectLock was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendObjectTrap: fn(bool, bool) = |_, _| { unreachable!("SendObjectTrap was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendObjectScale: fn(bool, bool) = |_, _| { unreachable!("SendObjectScale was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendObjectState: fn(bool, bool) = |_, _| { unreachable!("SendObjectState was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendDoorState: fn(bool, bool) = |_, _| { unreachable!("SendDoorState was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendDoorDestination: fn(bool, bool) = |_, _| { unreachable!("SendDoorDestination was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendContainer: fn(bool, bool) = |_, _| { unreachable!("SendContainer was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendVideoPlay: fn(bool, bool) = |_, _| { unreachable!("SendVideoPlay was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendScriptGlobalShort: fn(bool, bool) = |_, _| { unreachable!("SendScriptGlobalShort was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendConsoleCommand: fn(bool, bool) = |_, _| { unreachable!("SendConsoleCommand was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustReadLastObjectList: fn() = || { unreachable!("ReadLastObjectList was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustReadLastEvent: fn() = || { unreachable!("ReadLastEvent was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustInitializeObjectList: fn(u16) = |_| { unreachable!("InitializeObjectList was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustInitializeEvent: fn(u16) = |_| { unreachable!("InitializeEvent was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustCopyLastObjectListToStore: fn() = || { unreachable!("CopyLastObjectListToStore was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetObjectChangesSize: fn() -> u16 = || { unreachable!("GetObjectChangesSize was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetEventAction: fn() -> u8 = || { unreachable!("GetEventAction was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetEventContainerSubAction: fn() -> u8 = || { unreachable!("GetEventContainerSubAction was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetObjectRefNumIndex: fn(u16) -> u16 = |_| { unreachable!("GetObjectRefNumIndex was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetObjectSummonerRefNumIndex: fn(u16) -> u16 = |_| { unreachable!("GetObjectSummonerRefNumIndex was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetEventCell: fn(*const i8) = |_| { unreachable!("SetEventCell was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetEventAction: fn(u8) = |_| { unreachable!("SetEventAction was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetEventConsoleCommand: fn(*const i8) = |_| { unreachable!("SetEventConsoleCommand was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetObjectRefNumIndex: fn(i16) = |_| { unreachable!("SetObjectRefNumIndex was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustAddWorldObject: fn() = || { unreachable!("AddWorldObject was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustReadReceivedWorldstate: fn() = || { unreachable!("ReadReceivedWorldstate was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustCopyReceivedWorldstateToStore: fn() = || { unreachable!("CopyReceivedWorldstateToStore was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustClearKillChanges: fn() = || { unreachable!("ClearKillChanges was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustClearMapChanges: fn() = || { unreachable!("ClearMapChanges was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetKillChangesSize: fn() -> u16 = || { unreachable!("GetKillChangesSize was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetMapChangesSize: fn() -> u16 = || { unreachable!("GetMapChangesSize was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetKillRefId: fn(u16) -> *const i8 = |_| { unreachable!("GetKillRefId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetKillNumber: fn(u16) -> i16 = |_| { unreachable!("GetKillNumber was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetWeatherRegion: fn() -> *const i8 = || { unreachable!("GetWeatherRegion was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetWeatherCurrent: fn() -> i16 = || { unreachable!("GetWeatherCurrent was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetWeatherNext: fn() -> i16 = || { unreachable!("GetWeatherNext was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetWeatherQueued: fn() -> i16 = || { unreachable!("GetWeatherQueued was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetWeatherTransitionFactor: fn() -> f64 = || { unreachable!("GetWeatherTransitionFactor was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetMapTileCellX: fn(u16) -> i16 = |_| { unreachable!("GetMapTileCellX was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustGetMapTileCellY: fn(u16) -> i16 = |_| { unreachable!("GetMapTileCellY was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetAuthorityRegion: fn(*const i8) = |_| { unreachable!("SetAuthorityRegion was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetWeatherRegion: fn(*const i8) = |_| { unreachable!("SetWeatherRegion was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetWeatherForceState: fn(bool) = |_| { unreachable!("SetWeatherForceState was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetWeatherCurrent: fn(i16) = |_| { unreachable!("SetWeatherCurrent was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetWeatherNext: fn(i16) = |_| { unreachable!("SetWeatherNext was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetWeatherQueued: fn(i16) = |_| { unreachable!("SetWeatherQueued was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetWeatherTransitionFactor: fn(f64) = |_| { unreachable!("SetWeatherTransitionFactor was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetHour: fn(f64) = |_| { unreachable!("SetHour was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetDay: fn(i16) = |_| { unreachable!("SetDay was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetMonth: fn(i16) = |_| { unreachable!("SetMonth was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetYear: fn(i16) = |_| { unreachable!("SetYear was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetDaysPassed: fn(i16) = |_| { unreachable!("SetDaysPassed was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetTimeScale: fn(f64) = |_| { unreachable!("SetTimeScale was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetPlayerCollisionState: fn(bool) = |_| { unreachable!("SetPlayerCollisionState was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetActorCollisionState: fn(bool) = |_| { unreachable!("SetActorCollisionState was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSetPlacedObjectCollisionState: fn(bool) = |_| { unreachable!("SetPlacedObjectCollisionState was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustUseActorCollisionForPlacedObjects: fn(bool) = |_| { unreachable!("UseActorCollisionForPlacedObjects was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustAddKill: fn(*const i8, i16) = |_, _| { unreachable!("AddKill was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustAddSynchronizedClientScriptId: fn(*const i8) = |_| { unreachable!("AddSynchronizedClientScriptId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustAddSynchronizedClientGlobalId: fn(*const i8) = |_| { unreachable!("AddSynchronizedClientGlobalId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustAddEnforcedCollisionRefId: fn(*const i8) = |_| { unreachable!("AddEnforcedCollisionRefId was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustClearSynchronizedClientScriptIds: fn() = || { unreachable!("ClearSynchronizedClientScriptIds was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustClearSynchronizedClientGlobalIds: fn() = || { unreachable!("ClearSynchronizedClientGlobalIds was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustClearEnforcedCollisionRefIds: fn() = || { unreachable!("ClearEnforcedCollisionRefIds was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSaveMapTileImageFile: fn(u16, *const i8) = |_, _| { unreachable!("SaveMapTileImageFile was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustLoadMapTileImageFile: fn(i16, i16, *const i8) = |_, _, _| { unreachable!("LoadMapTileImageFile was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendClientScriptSettings: fn(u16, bool, bool) = |_, _, _| { unreachable!("SendClientScriptSettings was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendWorldKillCount: fn(u16, bool, bool) = |_, _, _| { unreachable!("SendWorldKillCount was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendWorldMap: fn(u16, bool, bool) = |_, _, _| { unreachable!("SendWorldMap was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendWorldTime: fn(u16, bool, bool) = |_, _, _| { unreachable!("SendWorldTime was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendWorldWeather: fn(u16, bool, bool) = |_, _, _| { unreachable!("SendWorldWeather was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendWorldCollisionOverride: fn(u16, bool, bool) = |_, _, _| { unreachable!("SendWorldCollisionOverride was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustSendWorldRegionAuthority: fn(u16) = |_| { unreachable!("SendWorldRegionAuthority was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustReadLastWorldstate: fn() = || { unreachable!("ReadLastWorldstate was called before set by TES3MP"); };
    #[no_mangle]
    pub static mut rustCopyLastWorldstateToStore: fn() = || { unreachable!("CopyLastWorldstateToStore was called before set by TES3MP"); };
}

///
///  Create a timer that will run a script function after a certain interval.
///
///  [`callback`] The Lua script function.
///  [`msec`] The interval in miliseconds.
///
///  Returns the ID of the timer thus created.
///
pub fn create_timer(callback: fn(), msec: i16) -> i16 {
    unsafe {
        raw::rustCreateTimer(callback, msec)
    }
}

pub fn make_public(public: fn(), name: &str, ret_type: i8, def: &str) {
    unsafe {
        raw::rustMakePublic(public, CString::new(name).unwrap_or_default().as_ptr(), ret_type, CString::new(def).unwrap_or_default().as_ptr())
    }
}

///
///  Start the timer with a certain ID.
///
///  [`timer_id`] The timer ID.
///
///  Returns void
///
pub fn start_timer(timer_id: i16) {
    unsafe {
        raw::rustStartTimer(timer_id)
    }
}

///
///  Stop the timer with a certain ID.
///
///  [`timer_id`] The timer ID.
///
///  Returns void
///
pub fn stop_timer(timer_id: i16) {
    unsafe {
        raw::rustStopTimer(timer_id)
    }
}

///
///  Restart the timer with a certain ID for a certain interval.
///
///  [`timer_id`] The timer ID.
///  [`msec`] The interval in miliseconds.
///
///  Returns void
///
pub fn restart_timer(timer_id: i16, msec: i16) {
    unsafe {
        raw::rustRestartTimer(timer_id, msec)
    }
}

///
///  Free the timer with a certain ID.
///
///  [`timer_id`] The timer ID.
///
///  Returns void
///
pub fn free_timer(timer_id: i16) {
    unsafe {
        raw::rustFreeTimer(timer_id)
    }
}

///
///  Check whether a timer is elapsed.
///
///  [`timer_id`] The timer ID.
///
///  Returns whether the timer is elapsed.
///
pub fn is_timer_elapsed(timer_id: i16) -> bool {
    unsafe {
        raw::rustIsTimerElapsed(timer_id)
    }
}

///
///  Use the last actor list received by the server as the one being read.
///
///
///  Returns void
///
pub fn read_received_actor_list() {
    unsafe {
        raw::rustReadReceivedActorList()
    }
}

///
///  Use the temporary actor list stored for a cell as the one being read.
///
///  This type of actor list is used to store actor positions and dynamic stats and is deleted
///  when the cell is unloaded.
///
///  [`cell_description`] The description of the cell whose actor list should be read.
///
///  Returns void
///
pub fn read_cell_actor_list(cell_description: &str) {
    unsafe {
        raw::rustReadCellActorList(CString::new(cell_description).unwrap_or_default().as_ptr())
    }
}

///
///  Clear the data from the actor list stored on the server.
///
///
///  Returns void
///
pub fn clear_actor_list() {
    unsafe {
        raw::rustClearActorList()
    }
}

///
///  Set the pid attached to the ActorList.
///
///  [`pid`] The player ID to whom the actor list should be attached.
///
///  Returns void
///
pub fn set_actor_list_pid(pid: u16) {
    unsafe {
        raw::rustSetActorListPid(pid)
    }
}

///
///  Take the contents of the read-only actor list last received by the
///         server from a player and move its contents to the stored object list
///         that can be sent by the server.
///
///
///  Returns void
///
pub fn copy_received_actor_list_to_store() {
    unsafe {
        raw::rustCopyReceivedActorListToStore()
    }
}

///
///  Get the number of indexes in the read actor list.
///
///
///  Returns the number of indexes.
///
pub fn get_actor_list_size() -> u16 {
    unsafe {
        raw::rustGetActorListSize()
    }
}

///
///  Get the action type used in the read actor list.
///
///
///  Returns the action type (0 for SET, 1 for ADD, 2 for REMOVE, 3 for REQUEST).
///
pub fn get_actor_list_action() -> u8 {
    unsafe {
        raw::rustGetActorListAction()
    }
}

///
///  Get the cell description of the actor at a certain index in the read actor list.
///
///  [`index`] The index of the actor.
///
///  Returns the cell description.
///
pub fn get_actor_cell(index: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetActorCell(index))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the refId of the actor at a certain index in the read actor list.
///
///  [`index`] The index of the actor.
///
///  Returns the refId.
///
pub fn get_actor_ref_id(index: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetActorRefId(index))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the refNum of the actor at a certain index in the read actor list.
///
///  [`index`] The index of the actor.
///
///  Returns the refNum.
///
pub fn get_actor_ref_num(index: u16) -> u16 {
    unsafe {
        raw::rustGetActorRefNum(index)
    }
}

///
///  Get the mpNum of the actor at a certain index in the read actor list.
///
///  [`index`] The index of the actor.
///
///  Returns the mpNum.
///
pub fn get_actor_mp_num(index: u16) -> u16 {
    unsafe {
        raw::rustGetActorMpNum(index)
    }
}

///
///  Get the X position of the actor at a certain index in the read actor list.
///
///  [`index`] The index of the actor.
///
///  Returns the X position.
///
pub fn get_actor_pos_x(index: u16) -> f64 {
    unsafe {
        raw::rustGetActorPosX(index)
    }
}

///
///  Get the Y position of the actor at a certain index in the read actor list.
///
///  [`index`] The index of the actor.
///
///  Returns the Y position.
///
pub fn get_actor_pos_y(index: u16) -> f64 {
    unsafe {
        raw::rustGetActorPosY(index)
    }
}

///
///  Get the Z position of the actor at a certain index in the read actor list.
///
///  [`index`] The index of the actor.
///
///  Returns the Z position.
///
pub fn get_actor_pos_z(index: u16) -> f64 {
    unsafe {
        raw::rustGetActorPosZ(index)
    }
}

///
///  Get the X rotation of the actor at a certain index in the read actor list.
///
///  [`index`] The index of the actor.
///
///  Returns the X rotation.
///
pub fn get_actor_rot_x(index: u16) -> f64 {
    unsafe {
        raw::rustGetActorRotX(index)
    }
}

///
///  Get the Y rotation of the actor at a certain index in the read actor list.
///
///  [`index`] The index of the actor.
///
///  Returns the Y rotation.
///
pub fn get_actor_rot_y(index: u16) -> f64 {
    unsafe {
        raw::rustGetActorRotY(index)
    }
}

///
///  Get the Z rotation of the actor at a certain index in the read actor list.
///
///  [`index`] The index of the actor.
///
///  Returns the Z rotation.
///
pub fn get_actor_rot_z(index: u16) -> f64 {
    unsafe {
        raw::rustGetActorRotZ(index)
    }
}

///
///  Get the base health of the actor at a certain index in the read actor list.
///
///  [`index`] The index of the actor.
///
///  Returns the base health.
///
pub fn get_actor_health_base(index: u16) -> f64 {
    unsafe {
        raw::rustGetActorHealthBase(index)
    }
}

///
///  Get the current health of the actor at a certain index in the read actor list.
///
///  [`index`] The index of the actor.
///
///  Returns the current health.
///
pub fn get_actor_health_current(index: u16) -> f64 {
    unsafe {
        raw::rustGetActorHealthCurrent(index)
    }
}

///
///  Get the modified health of the actor at a certain index in the read actor list.
///
///  [`index`] The index of the actor.
///
///  Returns the modified health.
///
pub fn get_actor_health_modified(index: u16) -> f64 {
    unsafe {
        raw::rustGetActorHealthModified(index)
    }
}

///
///  Get the base magicka of the actor at a certain index in the read actor list.
///
///  [`index`] The index of the actor.
///
///  Returns the base magicka.
///
pub fn get_actor_magicka_base(index: u16) -> f64 {
    unsafe {
        raw::rustGetActorMagickaBase(index)
    }
}

///
///  Get the current magicka of the actor at a certain index in the read actor list.
///
///  [`index`] The index of the actor.
///
///  Returns the current magicka.
///
pub fn get_actor_magicka_current(index: u16) -> f64 {
    unsafe {
        raw::rustGetActorMagickaCurrent(index)
    }
}

///
///  Get the modified magicka of the actor at a certain index in the read actor list.
///
///  [`index`] The index of the actor.
///
///  Returns the modified magicka.
///
pub fn get_actor_magicka_modified(index: u16) -> f64 {
    unsafe {
        raw::rustGetActorMagickaModified(index)
    }
}

///
///  Get the base fatigue of the actor at a certain index in the read actor list.
///
///  [`index`] The index of the actor.
///
///  Returns the base fatigue.
///
pub fn get_actor_fatigue_base(index: u16) -> f64 {
    unsafe {
        raw::rustGetActorFatigueBase(index)
    }
}

///
///  Get the current fatigue of the actor at a certain index in the read actor list.
///
///  [`index`] The index of the actor.
///
///  Returns the current fatigue.
///
pub fn get_actor_fatigue_current(index: u16) -> f64 {
    unsafe {
        raw::rustGetActorFatigueCurrent(index)
    }
}

///
///  Get the modified fatigue of the actor at a certain index in the read actor list.
///
///  [`index`] The index of the actor.
///
///  Returns the modified fatigue.
///
pub fn get_actor_fatigue_modified(index: u16) -> f64 {
    unsafe {
        raw::rustGetActorFatigueModified(index)
    }
}

///
///  Get the refId of the item in a certain slot of the equipment of the actor at a
///  certain index in the read actor list.
///
///  [`index`] The index of the actor.
///  [`slot`] The slot of the equipment item.
///
///  Returns the refId.
///
pub fn get_actor_equipment_item_ref_id(index: u16, slot: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetActorEquipmentItemRefId(index, slot))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the count of the item in a certain slot of the equipment of the actor at a
///  certain index in the read actor list.
///
///  [`index`] The index of the actor.
///  [`slot`] The slot of the equipment item.
///
///  Returns the item count.
///
pub fn get_actor_equipment_item_count(index: u16, slot: u16) -> i16 {
    unsafe {
        raw::rustGetActorEquipmentItemCount(index, slot)
    }
}

///
///  Get the charge of the item in a certain slot of the equipment of the actor at a
///  certain index in the read actor list.
///
///  [`index`] The index of the actor.
///  [`slot`] The slot of the equipment item.
///
///  Returns the charge.
///
pub fn get_actor_equipment_item_charge(index: u16, slot: u16) -> i16 {
    unsafe {
        raw::rustGetActorEquipmentItemCharge(index, slot)
    }
}

///
///  Get the enchantment charge of the item in a certain slot of the equipment of the actor at a
///  certain index in the read actor list.
///
///  [`index`] The index of the actor.
///  [`slot`] The slot of the equipment item.
///
///  Returns the enchantment charge.
///
pub fn get_actor_equipment_item_enchantment_charge(index: u16, slot: u16) -> f64 {
    unsafe {
        raw::rustGetActorEquipmentItemEnchantmentCharge(index, slot)
    }
}

///
///  Check whether the killer of the actor at a certain index in the read actor list is a player.
///
///  [`index`] The index of the actor.
///
///  Returns whether the actor was killed by a player.
///
pub fn does_actor_have_player_killer(index: u16) -> bool {
    unsafe {
        raw::rustDoesActorHavePlayerKiller(index)
    }
}

///
///  Get the player ID of the killer of the actor at a certain index in the read actor list.
///
///  [`index`] The index of the actor.
///
///  Returns the player ID of the killer.
///
pub fn get_actor_killer_pid(index: u16) -> i16 {
    unsafe {
        raw::rustGetActorKillerPid(index)
    }
}

///
///  Get the refId of the actor killer of the actor at a certain index in the read actor list.
///
///  [`index`] The index of the actor.
///
///  Returns the refId of the killer.
///
pub fn get_actor_killer_ref_id(index: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetActorKillerRefId(index))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the refNum of the actor killer of the actor at a certain index in the read actor list.
///
///  [`index`] The index of the actor.
///
///  Returns the refNum of the killer.
///
pub fn get_actor_killer_ref_num(index: u16) -> u16 {
    unsafe {
        raw::rustGetActorKillerRefNum(index)
    }
}

///
///  Get the mpNum of the actor killer of the actor at a certain index in the read actor list.
///
///  [`index`] The index of the actor.
///
///  Returns the mpNum of the killer.
///
pub fn get_actor_killer_mp_num(index: u16) -> u16 {
    unsafe {
        raw::rustGetActorKillerMpNum(index)
    }
}

///
///  Get the name of the actor killer of the actor at a certain index in the read actor list.
///
///  [`index`] The index of the actor.
///
///  Returns the name of the killer.
///
pub fn get_actor_killer_name(index: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetActorKillerName(index))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Check whether there is any positional data for the actor at a certain index in
///  the read actor list.
///
///  This is only useful when reading the actor list data recorded for a particular cell.
///
///  [`index`] The index of the actor.
///
///  Returns whether the read actor list contains positional data.
///
pub fn does_actor_have_position(index: u16) -> bool {
    unsafe {
        raw::rustDoesActorHavePosition(index)
    }
}

///
///  Check whether there is any dynamic stats data for the actor at a certain index in
///  the read actor list.
///
///  This is only useful when reading the actor list data recorded for a particular cell.
///
///  [`index`] The index of the actor.
///
///  Returns whether the read actor list contains dynamic stats data.
///
pub fn does_actor_have_stats_dynamic(index: u16) -> bool {
    unsafe {
        raw::rustDoesActorHaveStatsDynamic(index)
    }
}

///
///  Set the cell of the temporary actor list stored on the server.
///
///  The cell is determined to be an exterior cell if it fits the pattern of a number followed
///  by a comma followed by another number.
///
///  [`cell_description`] The description of the cell.
///
///  Returns void
///
pub fn set_actor_list_cell(cell_description: &str) {
    unsafe {
        raw::rustSetActorListCell(CString::new(cell_description).unwrap_or_default().as_ptr())
    }
}

///
///  Set the action type of the temporary actor list stored on the server.
///
///  [`action`] The action type (0 for SET, 1 for ADD, 2 for REMOVE, 3 for REQUEST).
///
///  Returns void
///
pub fn set_actor_list_action(action: u8) {
    unsafe {
        raw::rustSetActorListAction(action)
    }
}

///
///  Set the cell of the temporary actor stored on the server.
///
///  Used for ActorCellChange packets, where a specific actor's cell now differs from that of the
///  actor list.
///
///  The cell is determined to be an exterior cell if it fits the pattern of a number followed
///  by a comma followed by another number.
///
///  [`cell_description`] The description of the cell.
///
///  Returns void
///
pub fn set_actor_cell(cell_description: &str) {
    unsafe {
        raw::rustSetActorCell(CString::new(cell_description).unwrap_or_default().as_ptr())
    }
}

///
///  Set the refId of the temporary actor stored on the server.
///
///  [`ref_id`] The refId.
///
///  Returns void
///
pub fn set_actor_ref_id(ref_id: &str) {
    unsafe {
        raw::rustSetActorRefId(CString::new(ref_id).unwrap_or_default().as_ptr())
    }
}

///
///  Set the refNum of the temporary actor stored on the server.
///
///  [`ref_num`] The refNum.
///
///  Returns void
///
pub fn set_actor_ref_num(ref_num: i16) {
    unsafe {
        raw::rustSetActorRefNum(ref_num)
    }
}

///
///  Set the mpNum of the temporary actor stored on the server.
///
///  [`mp_num`] The mpNum.
///
///  Returns void
///
pub fn set_actor_mp_num(mp_num: i16) {
    unsafe {
        raw::rustSetActorMpNum(mp_num)
    }
}

///
///  Set the position of the temporary actor stored on the server.
///
///  [`x`] The X position.
///  [`y`] The Y position.
///  [`z`] The Z position.
///
///  Returns void
///
pub fn set_actor_position(x: f64, y: f64, z: f64) {
    unsafe {
        raw::rustSetActorPosition(x, y, z)
    }
}

///
///  Set the rotation of the temporary actor stored on the server.
///
///  [`x`] The X rotation.
///  [`y`] The Y rotation.
///  [`z`] The Z rotation.
///
///  Returns void
///
pub fn set_actor_rotation(x: f64, y: f64, z: f64) {
    unsafe {
        raw::rustSetActorRotation(x, y, z)
    }
}

///
///  Set the base health of the temporary actor stored on the server.
///
///  [`value`] The new value.
///
///  Returns void
///
pub fn set_actor_health_base(value: f64) {
    unsafe {
        raw::rustSetActorHealthBase(value)
    }
}

///
///  Set the current health of the temporary actor stored on the server.
///
///  [`value`] The new value.
///
///  Returns void
///
pub fn set_actor_health_current(value: f64) {
    unsafe {
        raw::rustSetActorHealthCurrent(value)
    }
}

///
///  Set the modified health of the temporary actor stored on the server.
///
///  [`value`] The new value.
///
///  Returns void
///
pub fn set_actor_health_modified(value: f64) {
    unsafe {
        raw::rustSetActorHealthModified(value)
    }
}

///
///  Set the base magicka of the temporary actor stored on the server.
///
///  [`value`] The new value.
///
///  Returns void
///
pub fn set_actor_magicka_base(value: f64) {
    unsafe {
        raw::rustSetActorMagickaBase(value)
    }
}

///
///  Set the current magicka of the temporary actor stored on the server.
///
///  [`value`] The new value.
///
///  Returns void
///
pub fn set_actor_magicka_current(value: f64) {
    unsafe {
        raw::rustSetActorMagickaCurrent(value)
    }
}

///
///  Set the modified magicka of the temporary actor stored on the server.
///
///  [`value`] The new value.
///
///  Returns void
///
pub fn set_actor_magicka_modified(value: f64) {
    unsafe {
        raw::rustSetActorMagickaModified(value)
    }
}

///
///  Set the base fatigue of the temporary actor stored on the server.
///
///  [`value`] The new value.
///
///  Returns void
///
pub fn set_actor_fatigue_base(value: f64) {
    unsafe {
        raw::rustSetActorFatigueBase(value)
    }
}

///
///  Set the current fatigue of the temporary actor stored on the server.
///
///  [`value`] The new value.
///
///  Returns void
///
pub fn set_actor_fatigue_current(value: f64) {
    unsafe {
        raw::rustSetActorFatigueCurrent(value)
    }
}

///
///  Set the modified fatigue of the temporary actor stored on the server.
///
///  [`value`] The new value.
///
///  Returns void
///
pub fn set_actor_fatigue_modified(value: f64) {
    unsafe {
        raw::rustSetActorFatigueModified(value)
    }
}

///
///  Set the sound of the temporary actor stored on the server.
///
///  [`sound`] The sound.
///
///  Returns void
///
pub fn set_actor_sound(sound: &str) {
    unsafe {
        raw::rustSetActorSound(CString::new(sound).unwrap_or_default().as_ptr())
    }
}

///
///  Set the AI action of the temporary actor stored on the server.
///
///  [`action`] The new action.
///
///  Returns void
///
pub fn set_actor_ai_action(action: u16) {
    unsafe {
        raw::rustSetActorAIAction(action)
    }
}

///
///  Set a player as the AI target of the temporary actor stored on the server.
///
///  [`pid`] The player ID.
///
///  Returns void
///
pub fn set_actor_ai_target_to_player(pid: u16) {
    unsafe {
        raw::rustSetActorAITargetToPlayer(pid)
    }
}

///
///  Set another object as the AI target of the temporary actor stored on the server.
///
///  [`ref_num`] The refNum of the target object.
///  [`mp_num`] The mpNum of the target object.
///
///  Returns void
///
pub fn set_actor_ai_target_to_object(ref_num: i16, mp_num: i16) {
    unsafe {
        raw::rustSetActorAITargetToObject(ref_num, mp_num)
    }
}

///
///  Set the coordinates for the AI package associated with the current AI action.
///
///  [`x`] The X coordinate.
///  [`y`] The Y coordinate.
///  [`z`] The Z coordinate.
///
///  Returns void
///
pub fn set_actor_ai_coordinates(x: f64, y: f64, z: f64) {
    unsafe {
        raw::rustSetActorAICoordinates(x, y, z)
    }
}

///
///  Set the distance of the AI package associated with the current AI action.
///
///  [`duration`] The distance of the package.
///
///  Returns void
///
pub fn set_actor_ai_distance(distance: u16) {
    unsafe {
        raw::rustSetActorAIDistance(distance)
    }
}

///
///  Set the duration of the AI package associated with the current AI action.
///
///  [`duration`] The duration of the package.
///
///  Returns void
///
pub fn set_actor_ai_duration(duration: u16) {
    unsafe {
        raw::rustSetActorAIDuration(duration)
    }
}

///
///  Set whether the current AI package should be repeated.
///
///  Note: This only has an effect on the WANDER package.
///
///  [`should_repeat`] Whether the package should be repeated.
///
///  Returns void
///
pub fn set_actor_ai_repetition(should_repeat: bool) {
    unsafe {
        raw::rustSetActorAIRepetition(should_repeat)
    }
}

///
///  Equip an item in a certain slot of the equipment of the temporary actor stored
///  on the server.
///
///  [`slot`] The equipment slot.
///  [`ref_id`] The refId of the item.
///  [`count`] The count of the item.
///  [`charge`] The charge of the item.
///  [`enchantment_charge`] The enchantment charge of the item.
///
///  Returns void
///
pub fn equip_actor_item(slot: u16, ref_id: &str, count: u16, charge: i16, enchantment_charge: f64) {
    unsafe {
        raw::rustEquipActorItem(slot, CString::new(ref_id).unwrap_or_default().as_ptr(), count, charge, enchantment_charge)
    }
}

///
///  Unequip the item in a certain slot of the equipment of the temporary actor stored
///  on the server.
///
///  [`slot`] The equipment slot.
///
///  Returns void
///
pub fn unequip_actor_item(slot: u16) {
    unsafe {
        raw::rustUnequipActorItem(slot)
    }
}

///
///  Add a copy of the server's temporary actor to the server's temporary actor list.
///
///  In the process, the server's temporary actor will automatically be cleared so a new
///  one can be set up.
///
///
///  Returns void
///
pub fn add_actor() {
    unsafe {
        raw::rustAddActor()
    }
}

///
///  Send an ActorList packet.
///
///  It is sent only to the player for whom the current actor list was initialized.
///
///
///  Returns void
///
pub fn send_actor_list() {
    unsafe {
        raw::rustSendActorList()
    }
}

///
///  Send an ActorAuthority packet.
///
///  The player for whom the current actor list was initialized is recorded in the server memory
///  as the new actor authority for the actor list's cell.
///
///  The packet is sent to that player as well as all other players who have the cell loaded.
///
///
///  Returns void
///
pub fn send_actor_authority() {
    unsafe {
        raw::rustSendActorAuthority()
    }
}

///
///  Send an ActorPosition packet.
///
///  [`send_to_other_visitors`] Whether this packet should be sent to cell visitors other
///                             than the player attached to the packet (false by default).
///  [`skip_attached_player`] Whether the packet should skip being sent to the player attached
///                            to the packet (false by default).
///
///
///  Returns void
///
pub fn send_actor_position(send_to_other_visitors: bool, skip_attached_player: bool) {
    unsafe {
        raw::rustSendActorPosition(send_to_other_visitors, skip_attached_player)
    }
}

///
///  Send an ActorStatsDynamic packet.
///
///  [`send_to_other_visitors`] Whether this packet should be sent to cell visitors other
///                             than the player attached to the packet (false by default).
///  [`skip_attached_player`] Whether the packet should skip being sent to the player attached
///                            to the packet (false by default).
///
///
///  Returns void
///
pub fn send_actor_stats_dynamic(send_to_other_visitors: bool, skip_attached_player: bool) {
    unsafe {
        raw::rustSendActorStatsDynamic(send_to_other_visitors, skip_attached_player)
    }
}

///
///  Send an ActorEquipment packet.
///
///  [`send_to_other_visitors`] Whether this packet should be sent to cell visitors other
///                             than the player attached to the packet (false by default).
///  [`skip_attached_player`] Whether the packet should skip being sent to the player attached
///                            to the packet (false by default).
///
///
///  Returns void
///
pub fn send_actor_equipment(send_to_other_visitors: bool, skip_attached_player: bool) {
    unsafe {
        raw::rustSendActorEquipment(send_to_other_visitors, skip_attached_player)
    }
}

///
///  Send an ActorSpeech packet.
///
///  [`send_to_other_visitors`] Whether this packet should be sent to cell visitors other
///                             than the player attached to the packet (false by default).
///  [`skip_attached_player`] Whether the packet should skip being sent to the player attached
///                            to the packet (false by default).
///
///  Returns void
///
pub fn send_actor_speech(send_to_other_visitors: bool, skip_attached_player: bool) {
    unsafe {
        raw::rustSendActorSpeech(send_to_other_visitors, skip_attached_player)
    }
}

///
///  Send an ActorAI packet.
///
///  [`send_to_other_visitors`] Whether this packet should be sent to cell visitors other
///                             than the player attached to the packet (false by default).
///  [`skip_attached_player`] Whether the packet should skip being sent to the player attached
///                            to the packet (false by default).
///
///  Returns void
///
pub fn send_actor_ai(send_to_other_visitors: bool, skip_attached_player: bool) {
    unsafe {
        raw::rustSendActorAI(send_to_other_visitors, skip_attached_player)
    }
}

///
///  Send an ActorCellChange packet.
///
///  [`send_to_other_visitors`] Whether this packet should be sent to cell visitors other
///                             than the player attached to the packet (false by default).
///  [`skip_attached_player`] Whether the packet should skip being sent to the player attached
///                            to the packet (false by default).
///
///
///  Returns void
///
pub fn send_actor_cell_change(send_to_other_visitors: bool, skip_attached_player: bool) {
    unsafe {
        raw::rustSendActorCellChange(send_to_other_visitors, skip_attached_player)
    }
}

pub fn read_last_actor_list() {
    unsafe {
        raw::rustReadLastActorList()
    }
}

pub fn initialize_actor_list(pid: u16) {
    unsafe {
        raw::rustInitializeActorList(pid)
    }
}

pub fn copy_last_actor_list_to_store() {
    unsafe {
        raw::rustCopyLastActorListToStore()
    }
}

pub fn get_actor_ref_num_index(index: u16) -> u16 {
    unsafe {
        raw::rustGetActorRefNumIndex(index)
    }
}

pub fn get_actor_killer_ref_num_index(index: u16) -> u16 {
    unsafe {
        raw::rustGetActorKillerRefNumIndex(index)
    }
}

pub fn set_actor_ref_num_index(ref_num: i16) {
    unsafe {
        raw::rustSetActorRefNumIndex(ref_num)
    }
}

///
///  Clear the last recorded book changes for a player.
///
///  This is used to initialize the sending of new PlayerBook packets.
///
///  [`pid`] The player ID whose book changes should be used.
///
///  Returns void
///
pub fn clear_book_changes(pid: u16) {
    unsafe {
        raw::rustClearBookChanges(pid)
    }
}

///
///  Get the number of indexes in a player's latest book changes.
///
///  [`pid`] The player ID whose book changes should be used.
///
///  Returns the number of indexes.
///
pub fn get_book_changes_size(pid: u16) -> u16 {
    unsafe {
        raw::rustGetBookChangesSize(pid)
    }
}

///
///  Add a new book to the book changes for a player.
///
///  [`pid`] The player ID whose book changes should be used.
///  [`book_id`] The bookId of the book.
///
///  Returns void
///
pub fn add_book(pid: u16, book_id: &str) {
    unsafe {
        raw::rustAddBook(pid, CString::new(book_id).unwrap_or_default().as_ptr())
    }
}

///
///  Get the bookId at a certain index in a player's latest book changes.
///
///  [`pid`] The player ID whose book changes should be used.
///  [`index`] The index of the book.
///
///  Returns the bookId.
///
pub fn get_book_id(pid: u16, index: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetBookId(pid, index))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Send a PlayerBook packet with a player's recorded book changes.
///
///  [`pid`] The player ID whose book changes should be used.
///  [`send_to_other_players`] Whether this packet should be sent to players other than the
///                            player attached to the packet (false by default).
///  [`skip_attached_player`] Whether the packet should skip being sent to the player attached
///                            to the packet (false by default).
///
///  Returns void
///
pub fn send_book_changes(pid: u16, send_to_other_players: bool, skip_attached_player: bool) {
    unsafe {
        raw::rustSendBookChanges(pid, send_to_other_players, skip_attached_player)
    }
}

pub fn initialize_book_changes(pid: u16) {
    unsafe {
        raw::rustInitializeBookChanges(pid)
    }
}

///
///  Get the number of indexes in a player's latest cell state changes.
///
///  [`pid`] The player ID whose cell state changes should be used.
///
///  Returns the number of indexes.
///
pub fn get_cell_state_changes_size(pid: u16) -> u16 {
    unsafe {
        raw::rustGetCellStateChangesSize(pid)
    }
}

///
///  Get the cell state type at a certain index in a player's latest cell state changes.
///
///  [`pid`] The player ID whose cell state changes should be used.
///  [`index`] The index of the cell state.
///
///  Returns the cell state type (0 for LOAD, 1 for UNLOAD).
///
pub fn get_cell_state_type(pid: u16, index: u16) -> u16 {
    unsafe {
        raw::rustGetCellStateType(pid, index)
    }
}

///
///  Get the cell description at a certain index in a player's latest cell state changes.
///
///  [`pid`] The player ID whose cell state changes should be used.
///  [`index`] The index of the cell state.
///
///  Returns the cell description.
///
pub fn get_cell_state_description(pid: u16, index: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetCellStateDescription(pid, index))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the cell description of a player's cell.
///
///  [`pid`] The player ID.
///
///  Returns the cell description.
///
pub fn get_cell(pid: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetCell(pid))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the X coordinate of the player's exterior cell.
///
///  [`pid`] The player ID.
///
///  Returns the X coordinate of the cell.
///
pub fn get_exterior_x(pid: u16) -> i16 {
    unsafe {
        raw::rustGetExteriorX(pid)
    }
}

///
///  Get the Y coordinate of the player's exterior cell.
///
///  [`pid`] The player ID.
///
///  Returns the Y coordinate of the cell.
///
pub fn get_exterior_y(pid: u16) -> i16 {
    unsafe {
        raw::rustGetExteriorY(pid)
    }
}

///
///  Check whether the player is in an exterior cell or not.
///
///  [`pid`] The player ID.
///
///  Returns whether the player is in an exterior cell.
///
pub fn is_in_exterior(pid: u16) -> bool {
    unsafe {
        raw::rustIsInExterior(pid)
    }
}

///
///  Get the region of the player's exterior cell.
///
///  A blank value will be returned if the player is in an interior.
///
///  [`pid`] The player ID.
///
///  Returns the region.
///
pub fn get_region(pid: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetRegion(pid))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Check whether the player's last cell change has involved a region change.
///
///  [`pid`] The player ID.
///
///  Returns whether the player has changed their region.
///
pub fn is_changing_region(pid: u16) -> bool {
    unsafe {
        raw::rustIsChangingRegion(pid)
    }
}

///
///  Set the cell of a player.
///
///  This changes the cell recorded for that player in the server memory, but does not by itself
///  send a packet.
///
///  The cell is determined to be an exterior cell if it fits the pattern of a number followed
///  by a comma followed by another number.
///
///  [`pid`] The player ID.
///  [`cell_description`] The cell description.
///
///  Returns void
///
pub fn set_cell(pid: u16, cell_description: &str) {
    unsafe {
        raw::rustSetCell(pid, CString::new(cell_description).unwrap_or_default().as_ptr())
    }
}

///
///  Set the cell of a player to an exterior cell.
///
///  This changes the cell recorded for that player in the server memory, but does not by itself
///  send a packet.
///
///  [`pid`] The player ID.
///  [`x`] The X coordinate of the cell.
///  [`y`] The Y coordinate of the cell.
///
///  Returns void
///
pub fn set_exterior_cell(pid: u16, x: i16, y: i16) {
    unsafe {
        raw::rustSetExteriorCell(pid, x, y)
    }
}

///
///  Send a PlayerCellChange packet about a player.
///
///  It is only sent to the affected player.
///
///  [`pid`] The player ID.
///
///  Returns void
///
pub fn send_cell(pid: u16) {
    unsafe {
        raw::rustSendCell(pid)
    }
}

///
///  Get the default class used by a player.
///
///  [`pid`] The player ID.
///
///  Returns the ID of the default class.
///
pub fn get_default_class(pid: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetDefaultClass(pid))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the name of the custom class used by a player.
///
///  [`pid`] The player ID.
///
///  Returns the name of the custom class.
///
pub fn get_class_name(pid: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetClassName(pid))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the description of the custom class used by a player.
///
///  [`pid`] The player ID.
///
///  Returns the description of the custom class.
///
pub fn get_class_desc(pid: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetClassDesc(pid))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the ID of one of the two major attributes of a custom class used by a player.
///
///  [`pid`] The player ID.
///  [`slot`] The slot of the major attribute (0 or 1).
///
///  Returns the ID of the major attribute.
///
pub fn get_class_major_attribute(pid: u16, slot: u8) -> i16 {
    unsafe {
        raw::rustGetClassMajorAttribute(pid, slot)
    }
}

///
///  Get the specialization ID of the custom class used by a player.
///
///  [`pid`] The player ID.
///
///  Returns the specialization ID of the custom class (0 for Combat, 1 for Magic, 2 for Stealth).
///
pub fn get_class_specialization(pid: u16) -> i16 {
    unsafe {
        raw::rustGetClassSpecialization(pid)
    }
}

///
///  Get the ID of one of the five major skills of a custom class used by a player.
///
///  [`pid`] The player ID.
///  [`slot`] The slot of the major skill (0 to 4).
///
///  Returns the ID of the major skill.
///
pub fn get_class_major_skill(pid: u16, slot: u8) -> i16 {
    unsafe {
        raw::rustGetClassMajorSkill(pid, slot)
    }
}

///
///  Get the ID of one of the five minor skills of a custom class used by a player.
///
///  [`pid`] The player ID.
///  [`slot`] The slot of the minor skill (0 to 4).
///
///  Returns the ID of the minor skill.
///
pub fn get_class_minor_skill(pid: u16, slot: u8) -> i16 {
    unsafe {
        raw::rustGetClassMinorSkill(pid, slot)
    }
}

///
///  Check whether the player is using a default class instead of a custom one.
///
///  [`pid`] The player ID.
///
///  Returns whether the player is using a default class.
///
pub fn is_class_default(pid: u16) -> i16 {
    unsafe {
        raw::rustIsClassDefault(pid)
    }
}

///
///  Set the default class used by a player.
///
///  If this is left blank, the custom class data set for the player will be used instead.
///
///  [`pid`] The player ID.
///  [`id`] The ID of the default class.
///
///  Returns void
///
pub fn set_default_class(pid: u16, id: &str) {
    unsafe {
        raw::rustSetDefaultClass(pid, CString::new(id).unwrap_or_default().as_ptr())
    }
}

///
///  Set the name of the custom class used by a player.
///
///  [`pid`] The player ID.
///  [`name`] The name of the custom class.
///
///  Returns void
///
pub fn set_class_name(pid: u16, name: &str) {
    unsafe {
        raw::rustSetClassName(pid, CString::new(name).unwrap_or_default().as_ptr())
    }
}

///
///  Set the description of the custom class used by a player.
///
///  [`pid`] The player ID.
///  [`desc`] The description of the custom class.
///
///  Returns void
///
pub fn set_class_desc(pid: u16, desc: &str) {
    unsafe {
        raw::rustSetClassDesc(pid, CString::new(desc).unwrap_or_default().as_ptr())
    }
}

///
///  Set the ID of one of the two major attributes of the custom class used by a player.
///
///  [`pid`] The player ID.
///  [`slot`] The slot of the major attribute (0 or 1).
///  [`attr_id`] The ID to use for the attribute.
///
///  Returns void
///
pub fn set_class_major_attribute(pid: u16, slot: u8, attr_id: i16) {
    unsafe {
        raw::rustSetClassMajorAttribute(pid, slot, attr_id)
    }
}

///
///  Set the specialization of the custom class used by a player.
///
///  [`pid`] The player ID.
///  [`spec`] The specialization ID to use (0 for Combat, 1 for Magic, 2 for Stealth).
///
///  Returns void
///
pub fn set_class_specialization(pid: u16, spec: i16) {
    unsafe {
        raw::rustSetClassSpecialization(pid, spec)
    }
}

///
///  Set the ID of one of the five major skills of the custom class used by a player.
///
///  [`pid`] The player ID.
///  [`slot`] The slot of the major skill (0 to 4).
///  [`skill_id`] The ID to use for the skill.
///
///  Returns void
///
pub fn set_class_major_skill(pid: u16, slot: u8, skill_id: i16) {
    unsafe {
        raw::rustSetClassMajorSkill(pid, slot, skill_id)
    }
}

///
///  Set the ID of one of the five minor skills of the custom class used by a player.
///
///  [`pid`] The player ID.
///  [`slot`] The slot of the minor skill (0 to 4).
///  [`skill_id`] The ID to use for the skill.
///
///  Returns void
///
pub fn set_class_minor_skill(pid: u16, slot: u8, skill_id: i16) {
    unsafe {
        raw::rustSetClassMinorSkill(pid, slot, skill_id)
    }
}

///
///  Send a PlayerCharClass packet about a player.
///
///  It is only sent to the affected player.
///
///  [`pid`] The player ID.
///
///  Returns void
///
pub fn send_class(pid: u16) {
    unsafe {
        raw::rustSendClass(pid)
    }
}

///
///  Send a message to a certain player.
///
///  [`pid`] The player ID.
///  [`message`] The contents of the message.
///  [`send_to_other_players`] Whether this packet should be sent to players other than the
///                            player attached to the packet (false by default).
///  [`skip_attached_player`] Whether the packet should skip being sent to the player attached
///                            to the packet (false by default).
///
///  Returns void
///
pub fn send_message(pid: u16, message: &str, send_to_other_players: bool, skip_attached_player: bool) {
    unsafe {
        raw::rustSendMessage(pid, CString::new(message).unwrap_or_default().as_ptr(), send_to_other_players, skip_attached_player)
    }
}

///
///  Remove all messages from chat for everyone on the server.
///
///
///  Returns void
///
pub fn clean_chat_for_pid() {
    unsafe {
        raw::rustCleanChatForPid()
    }
}

///
///  Remove all messages from chat for a certain player.
///
///  [`pid`] The player ID.
///
///  Returns void
///
pub fn clean_chat(pid: u16) {
    unsafe {
        raw::rustCleanChat(pid)
    }
}

///
///  Clear the last recorded topic changes for a player.
///
///  This is used to initialize the sending of new PlayerTopic packets.
///
///  [`pid`] The player ID whose topic changes should be used.
///
///  Returns void
///
pub fn clear_topic_changes(pid: u16) {
    unsafe {
        raw::rustClearTopicChanges(pid)
    }
}

///
///  Get the number of indexes in a player's latest topic changes.
///
///  [`pid`] The player ID whose topic changes should be used.
///
///  Returns the number of indexes.
///
pub fn get_topic_changes_size(pid: u16) -> u16 {
    unsafe {
        raw::rustGetTopicChangesSize(pid)
    }
}

///
///  Add a new topic to the topic changes for a player.
///
///  [`pid`] The player ID whose topic changes should be used.
///  [`topic_id`] The topicId of the topic.
///
///  Returns void
///
pub fn add_topic(pid: u16, topic_id: &str) {
    unsafe {
        raw::rustAddTopic(pid, CString::new(topic_id).unwrap_or_default().as_ptr())
    }
}

///
///  Get the topicId at a certain index in a player's latest topic changes.
///
///  [`pid`] The player ID whose topic changes should be used.
///  [`index`] The index of the topic.
///
///  Returns the topicId.
///
pub fn get_topic_id(pid: u16, index: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetTopicId(pid, index))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Send a PlayerTopic packet with a player's recorded topic changes.
///
///  [`pid`] The player ID whose topic changes should be used.
///  [`send_to_other_players`] Whether this packet should be sent to players other than the
///                            player attached to the packet (false by default).
///  [`skip_attached_player`] Whether the packet should skip being sent to the player attached
///                            to the packet (false by default).
///
///  Returns void
///
pub fn send_topic_changes(pid: u16, send_to_other_players: bool, skip_attached_player: bool) {
    unsafe {
        raw::rustSendTopicChanges(pid, send_to_other_players, skip_attached_player)
    }
}

///
///  Play a certain animation on a player's character by sending a PlayerAnimation
///         packet.
///
///  [`pid`] The player ID of the character playing the animation.
///  [`groupname`] The groupname of the animation.
///  [`mode`] The mode of the animation.
///  [`count`] The number of times the animation should be played.
///  [`bool`] Whether the animation should persist or not.
///
///  Returns void
///
pub fn play_animation(pid: u16, groupname: &str, mode: i16, count: i16, persist: bool) {
    unsafe {
        raw::rustPlayAnimation(pid, CString::new(groupname).unwrap_or_default().as_ptr(), mode, count, persist)
    }
}

///
///  Play a certain sound for a player as spoken by their character by sending
///         a PlayerSpeech packet.
///
///  [`pid`] The player ID of the character playing the sound.
///  [`sound`] The path of the sound file.
///
///  Returns void
///
pub fn play_speech(pid: u16, sound: &str) {
    unsafe {
        raw::rustPlaySpeech(pid, CString::new(sound).unwrap_or_default().as_ptr())
    }
}

pub fn initialize_topic_changes(pid: u16) {
    unsafe {
        raw::rustInitializeTopicChanges(pid)
    }
}

///
///  Clear the last recorded faction changes for a player.
///
///  This is used to initialize the sending of new PlayerFaction packets.
///
///  [`pid`] The player ID whose faction changes should be used.
///
///  Returns void
///
pub fn clear_faction_changes(pid: u16) {
    unsafe {
        raw::rustClearFactionChanges(pid)
    }
}

///
///  Get the number of indexes in a player's latest faction changes.
///
///  [`pid`] The player ID whose faction changes should be used.
///
///  Returns the number of indexes.
///
pub fn get_faction_changes_size(pid: u16) -> u16 {
    unsafe {
        raw::rustGetFactionChangesSize(pid)
    }
}

///
///  Get the action type used in a player's latest faction changes.
///
///  [`pid`] The player ID whose faction changes should be used.
///
///  Returns the action type (0 for RANK, 1 for EXPULSION, 2 for REPUTATION).
///
pub fn get_faction_changes_action(pid: u16) -> u8 {
    unsafe {
        raw::rustGetFactionChangesAction(pid)
    }
}

///
///  Get the factionId at a certain index in a player's latest faction changes.
///
///  [`pid`] The player ID whose faction changes should be used.
///  [`index`] The index of the faction.
///
///  Returns the factionId.
///
pub fn get_faction_id(pid: u16, index: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetFactionId(pid, index))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the rank at a certain index in a player's latest faction changes.
///
///  [`pid`] The player ID whose faction changes should be used.
///  [`index`] The index of the faction.
///
///  Returns the rank.
///
pub fn get_faction_rank(pid: u16, index: u16) -> i16 {
    unsafe {
        raw::rustGetFactionRank(pid, index)
    }
}

///
///  Get the expulsion state at a certain index in a player's latest faction changes.
///
///  [`pid`] The player ID whose faction changes should be used.
///  [`index`] The index of the faction.
///
///  Returns the expulsion state.
///
pub fn get_faction_expulsion_state(pid: u16, index: u16) -> bool {
    unsafe {
        raw::rustGetFactionExpulsionState(pid, index)
    }
}

///
///  Get the reputation at a certain index in a player's latest faction changes.
///
///  [`pid`] The player ID whose faction changes should be used.
///  [`index`] The index of the faction.
///
///  Returns the reputation.
///
pub fn get_faction_reputation(pid: u16, index: u16) -> i16 {
    unsafe {
        raw::rustGetFactionReputation(pid, index)
    }
}

///
///  Set the action type in a player's faction changes.
///
///  [`pid`] The player ID whose faction changes should be used.
///  [`action`] The action (0 for RANK, 1 for EXPULSION, 2 for REPUTATION).
///
///  Returns void
///
pub fn set_faction_changes_action(pid: u16, action: u8) {
    unsafe {
        raw::rustSetFactionChangesAction(pid, action)
    }
}

///
///  Set the factionId of the temporary faction stored on the server.
///
///  [`faction_id`] The factionId.
///
///  Returns void
///
pub fn set_faction_id(faction_id: &str) {
    unsafe {
        raw::rustSetFactionId(CString::new(faction_id).unwrap_or_default().as_ptr())
    }
}

///
///  Set the rank of the temporary faction stored on the server.
///
///  [`rank`] The rank.
///
///  Returns void
///
pub fn set_faction_rank(rank: u16) {
    unsafe {
        raw::rustSetFactionRank(rank)
    }
}

///
///  Set the expulsion state of the temporary faction stored on the server.
///
///  [`expulsion_state`] The expulsion state.
///
///  Returns void
///
pub fn set_faction_expulsion_state(expulsion_state: bool) {
    unsafe {
        raw::rustSetFactionExpulsionState(expulsion_state)
    }
}

///
///  Set the reputation of the temporary faction stored on the server.
///
///  [`reputation`] The reputation.
///
///  Returns void
///
pub fn set_faction_reputation(reputation: i16) {
    unsafe {
        raw::rustSetFactionReputation(reputation)
    }
}

///
///  Add the server's temporary faction to the faction changes for a player.
///
///  In the process, the server's temporary faction will automatically be cleared so a new one
///  can be set up.
///
///  [`pid`] The player ID whose faction changes should be used.
///
///  Returns void
///
pub fn add_faction(pid: u16) {
    unsafe {
        raw::rustAddFaction(pid)
    }
}

///
///  Send a PlayerFaction packet with a player's recorded faction changes.
///
///  [`pid`] The player ID whose faction changes should be used.
///  [`send_to_other_players`] Whether this packet should be sent to players other than the
///                            player attached to the packet (false by default).
///  [`skip_attached_player`] Whether the packet should skip being sent to the player attached
///                            to the packet (false by default).
///
///  Returns void
///
pub fn send_faction_changes(pid: u16, send_to_other_players: bool, skip_attached_player: bool) {
    unsafe {
        raw::rustSendFactionChanges(pid, send_to_other_players, skip_attached_player)
    }
}

pub fn initialize_faction_changes(pid: u16) {
    unsafe {
        raw::rustInitializeFactionChanges(pid)
    }
}

///
///  Display an interactive messagebox at the center of the screen that
///         vanishes only when one of its buttons is clicked.
///
///  [`pid`] The player ID for whom the messagebox should appear.
///  [`id`] The numerical ID of the messagebox.
///  [`label`] The text in the messagebox.
///  \parm buttons The captions of the buttons, separated by semicolons (e.g. "Yes;No;Maybe").
///
///  Returns void
///
pub fn custom_message_box(pid: u16, id: i16, label: &str, buttons: &str) {
    unsafe {
        raw::rustCustomMessageBox(pid, id, CString::new(label).unwrap_or_default().as_ptr(), CString::new(buttons).unwrap_or_default().as_ptr())
    }
}

///
///  Display an input dialog at the center of the screen.
///
///  [`pid`] The player ID for whom the input dialog should appear.
///  [`id`] The numerical ID of the input dialog.
///  [`label`] The text at the top of the input dialog.
///  \parm note The text at the bottom of the input dialog.
///
///  Returns void
///
pub fn input_dialog(pid: u16, id: i16, label: &str, note: &str) {
    unsafe {
        raw::rustInputDialog(pid, id, CString::new(label).unwrap_or_default().as_ptr(), CString::new(note).unwrap_or_default().as_ptr())
    }
}

///
///  Display a password dialog at the center of the screen.
///
///  Although similar to an input dialog, the password dialog replaces all
///  input characters with asterisks.
///
///  [`pid`] The player ID for whom the password dialog should appear.
///  [`id`] The numerical ID of the password dialog.
///  [`label`] The text at the top of the password dialog.
///  \parm note The text at the bottom of the password dialog.
///
///  Returns void
///
pub fn password_dialog(pid: u16, id: i16, label: &str, note: &str) {
    unsafe {
        raw::rustPasswordDialog(pid, id, CString::new(label).unwrap_or_default().as_ptr(), CString::new(note).unwrap_or_default().as_ptr())
    }
}

///
///  Display a listbox at the center of the screen where each item takes up
///         a row and is selectable, with the listbox only vanishing once the Ok button
///         is pressed.
///
///  [`pid`] The player ID for whom the listbox should appear.
///  [`id`] The numerical ID of the listbox.
///  [`label`] The text at the top of the listbox.
///  \parm items The items in the listbox, separated by newlines (e.g. "Item 1\nItem 2").
///
///  Returns void
///
pub fn list_box(pid: u16, id: i16, label: &str, items: &str) {
    unsafe {
        raw::rustListBox(pid, id, CString::new(label).unwrap_or_default().as_ptr(), CString::new(items).unwrap_or_default().as_ptr())
    }
}

///
///  Clear the last recorded quick key changes for a player.
///
///  This is used to initialize the sending of new PlayerQuickKeys packets.
///
///  [`pid`] The player ID whose quick key changes should be used.
///
///  Returns void
///
pub fn clear_quick_key_changes(pid: u16) {
    unsafe {
        raw::rustClearQuickKeyChanges(pid)
    }
}

///
///  Get the number of indexes in a player's latest quick key changes.
///
///  [`pid`] The player ID whose quick key changes should be used.
///
///  Returns the number of indexes.
///
pub fn get_quick_key_changes_size(pid: u16) -> u16 {
    unsafe {
        raw::rustGetQuickKeyChangesSize(pid)
    }
}

///
///  Get the slot of the quick key at a certain index in a player's latest quick key changes.
///
///  [`pid`] The player ID whose quick key changes should be used.
///  [`index`] The index of the quick key in the quick key changes vector.
///
///  Returns the slot.
///
pub fn get_quick_key_slot(pid: u16, index: u16) -> i16 {
    unsafe {
        raw::rustGetQuickKeySlot(pid, index)
    }
}

///
///  Get the type of the quick key at a certain index in a player's latest quick key changes.
///
///  [`pid`] The player ID whose quick key changes should be used.
///  [`index`] The index of the quick key in the quick key changes vector.
///
///  Returns the quick key type.
///
pub fn get_quick_key_type(pid: u16, index: u16) -> i16 {
    unsafe {
        raw::rustGetQuickKeyType(pid, index)
    }
}

///
///  Get the itemId at a certain index in a player's latest quick key changes.
///
///  [`pid`] The player ID whose quick key changes should be used.
///  [`index`] The index of the quick key in the quick key changes vector.
///
///  Returns the itemId.
///
pub fn get_quick_key_item_id(pid: u16, index: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetQuickKeyItemId(pid, index))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Add a new quick key to the quick key changes for a player.
///
///  [`pid`] The player ID whose quick key changes should be used.
///  [`slot`] The slot to be used.
///  [`slot`] The type of the quick key (0 for ITEM, 1 for ITEM_MAGIC, 2 for MAGIC, 3 for UNASSIGNED).
///  [`item_id`] The itemId of the item.
///
///  Returns void
///
pub fn add_quick_key(pid: u16, slot: u16, _type: i16, item_id: &str) {
    unsafe {
        raw::rustAddQuickKey(pid, slot, _type, CString::new(item_id).unwrap_or_default().as_ptr())
    }
}

///
///  Send a PlayerQuickKeys packet with a player's recorded quick key changes.
///
///  [`pid`] The player ID whose quick key changes should be used.
///
///  Returns void
///
pub fn send_quick_key_changes(pid: u16) {
    unsafe {
        raw::rustSendQuickKeyChanges(pid)
    }
}

///
///  Determine whether a player can see the map marker of another player.
///
///  Note: This currently has no effect, and is just an unimplemented stub.
///
///  [`target_pid`] The player ID whose map marker should be hidden or revealed.
///  [`affected_pid`] The player ID for whom the map marker will be hidden or revealed.
///  [`state`] The state of the map marker (false to hide, true to reveal).
///
///  Returns void
///
pub fn set_map_visibility(target_pid: u16, affected_pid: u16, state: u16) {
    unsafe {
        raw::rustSetMapVisibility(target_pid, affected_pid, state)
    }
}

///
///  Determine whether a player's map marker can be seen by all other players.
///
///  Note: This currently has no effect, and is just an unimplemented stub.
///
///  [`target_pid`] The player ID whose map marker should be hidden or revealed.
///  [`state`] The state of the map marker (false to hide, true to reveal).
///
///  Returns void
///
pub fn set_map_visibility_all(target_pid: u16, state: u16) {
    unsafe {
        raw::rustSetMapVisibilityAll(target_pid, state)
    }
}

pub fn initialize_quick_key_changes(pid: u16) {
    unsafe {
        raw::rustInitializeQuickKeyChanges(pid)
    }
}

///
///  Clear the last recorded inventory changes for a player.
///
///  This is used to initialize the sending of new PlayerInventory packets.
///
///  [`pid`] The player ID whose inventory changes should be used.
///
///  Returns void
///
pub fn clear_inventory_changes(pid: u16) {
    unsafe {
        raw::rustClearInventoryChanges(pid)
    }
}

///
///  Get the number of slots used for equipment.
///
///  The number is 19 before any dehardcoding is done in OpenMW.
///
///
///  Returns the number of slots.
///
pub fn get_equipment_size() -> i16 {
    unsafe {
        raw::rustGetEquipmentSize()
    }
}

///
///  Get the number of indexes in a player's latest inventory changes.
///
///  [`pid`] The player ID whose inventory changes should be used.
///
///  Returns the number of indexes.
///
pub fn get_inventory_changes_size(pid: u16) -> u16 {
    unsafe {
        raw::rustGetInventoryChangesSize(pid)
    }
}

///
///  Get the action type used in a player's latest inventory changes.
///
///  [`pid`] The player ID whose inventory changes should be used.
///
///  Returns the action type (0 for SET, 1 for ADD, 2 for REMOVE).
///
pub fn get_inventory_changes_action(pid: u16) -> u16 {
    unsafe {
        raw::rustGetInventoryChangesAction(pid)
    }
}

///
///  Set the action type in a player's inventory changes.
///
///  [`pid`] The player ID whose inventory changes should be used.
///  [`action`] The action (0 for SET, 1 for ADD, 2 for REMOVE).
///
///  Returns void
///
pub fn set_inventory_changes_action(pid: u16, action: u8) {
    unsafe {
        raw::rustSetInventoryChangesAction(pid, action)
    }
}

///
///  Equip an item in a certain slot of the equipment of a player.
///
///  [`pid`] The player ID.
///  [`slot`] The equipment slot.
///  [`ref_id`] The refId of the item.
///  [`count`] The count of the item.
///  [`charge`] The charge of the item.
///  [`enchantment_charge`] The enchantment charge of the item.
///
///  Returns void
///
pub fn equip_item(pid: u16, slot: u16, ref_id: &str, count: u16, charge: i16, enchantment_charge: f64) {
    unsafe {
        raw::rustEquipItem(pid, slot, CString::new(ref_id).unwrap_or_default().as_ptr(), count, charge, enchantment_charge)
    }
}

///
///  Unequip the item in a certain slot of the equipment of a player.
///
///  [`pid`] The player ID.
///  [`slot`] The equipment slot.
///
///  Returns void
///
pub fn unequip_item(pid: u16, slot: u16) {
    unsafe {
        raw::rustUnequipItem(pid, slot)
    }
}

///
///  Add an item change to a player's inventory changes.
///
///  [`pid`] The player ID.
///  [`ref_id`] The refId of the item.
///  [`count`] The count of the item.
///  [`charge`] The charge of the item.
///  [`enchantment_charge`] The enchantment charge of the item.
///  [`soul`] The soul of the item.
///
///  Returns void
///
pub fn add_item_change(pid: u16, ref_id: &str, count: u16, charge: i16, enchantment_charge: f64, soul: &str) {
    unsafe {
        raw::rustAddItemChange(pid, CString::new(ref_id).unwrap_or_default().as_ptr(), count, charge, enchantment_charge, CString::new(soul).unwrap_or_default().as_ptr())
    }
}

///
///  Check whether a player has equipped an item with a certain refId in any slot.
///
///  [`pid`] The player ID.
///  [`ref_id`] The refId of the item.
///
///  Returns whether the player has the item equipped.
///
pub fn has_item_equipped(pid: u16, ref_id: &str) -> bool {
    unsafe {
        raw::rustHasItemEquipped(pid, CString::new(ref_id).unwrap_or_default().as_ptr())
    }
}

///
///  Get the refId of the item in a certain slot of the equipment of a player.
///
///  [`pid`] The player ID.
///  [`slot`] The slot of the equipment item.
///
///  Returns the refId.
///
pub fn get_equipment_item_ref_id(pid: u16, slot: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetEquipmentItemRefId(pid, slot))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the count of the item in a certain slot of the equipment of a player.
///
///  [`pid`] The player ID.
///  [`slot`] The slot of the equipment item.
///
///  Returns the item count.
///
pub fn get_equipment_item_count(pid: u16, slot: u16) -> i16 {
    unsafe {
        raw::rustGetEquipmentItemCount(pid, slot)
    }
}

///
///  Get the charge of the item in a certain slot of the equipment of a player.
///
///  [`pid`] The player ID.
///  [`slot`] The slot of the equipment item.
///
///  Returns the charge.
///
pub fn get_equipment_item_charge(pid: u16, slot: u16) -> i16 {
    unsafe {
        raw::rustGetEquipmentItemCharge(pid, slot)
    }
}

///
///  Get the enchantment charge of the item in a certain slot of the equipment of
///         a player.
///
///  [`pid`] The player ID.
///  [`slot`] The slot of the equipment item.
///
///  Returns the enchantment charge.
///
pub fn get_equipment_item_enchantment_charge(pid: u16, slot: u16) -> f64 {
    unsafe {
        raw::rustGetEquipmentItemEnchantmentCharge(pid, slot)
    }
}

///
///  Get the refId of the item at a certain index in a player's latest inventory
///         changes.
///
///  [`pid`] The player ID whose inventory changes should be used.
///  [`index`] The index of the inventory item.
///
///  Returns the refId.
///
pub fn get_inventory_item_ref_id(pid: u16, index: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetInventoryItemRefId(pid, index))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the count of the item at a certain index in a player's latest inventory
///         changes.
///
///  [`pid`] The player ID whose inventory changes should be used.
///  [`index`] The index of the inventory item.
///
///  Returns the item count.
///
pub fn get_inventory_item_count(pid: u16, index: u16) -> i16 {
    unsafe {
        raw::rustGetInventoryItemCount(pid, index)
    }
}

///
///  Get the charge of the item at a certain index in a player's latest inventory
///         changes.
///
///  [`pid`] The player ID whose inventory changes should be used.
///  [`index`] The index of the inventory item.
///
///  Returns the charge.
///
pub fn get_inventory_item_charge(pid: u16, index: u16) -> i16 {
    unsafe {
        raw::rustGetInventoryItemCharge(pid, index)
    }
}

///
///  Get the enchantment charge of the item at a certain index in a player's
///         latest inventory changes.
///
///  [`pid`] The player ID whose inventory changes should be used.
///  [`index`] The index of the inventory item.
///
///  Returns the enchantment charge.
///
pub fn get_inventory_item_enchantment_charge(pid: u16, index: u16) -> f64 {
    unsafe {
        raw::rustGetInventoryItemEnchantmentCharge(pid, index)
    }
}

///
///  Get the soul of the item at a certain index in a player's latest inventory
///         changes.
///
///  [`pid`] The player ID whose inventory changes should be used.
///  [`index`] The index of the inventory item.
///
///  Returns the soul.
///
pub fn get_inventory_item_soul(pid: u16, index: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetInventoryItemSoul(pid, index))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the refId of the item last used by a player.
///
///  [`pid`] The player ID.
///
///  Returns the refId.
///
pub fn get_used_item_ref_id(pid: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetUsedItemRefId(pid))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the count of the item last used by a player.
///
///  [`pid`] The player ID.
///
///  Returns the item count.
///
pub fn get_used_item_count(pid: u16) -> i16 {
    unsafe {
        raw::rustGetUsedItemCount(pid)
    }
}

///
///  Get the charge of the item last used by a player.
///
///  [`pid`] The player ID.
///
///  Returns the charge.
///
pub fn get_used_item_charge(pid: u16) -> i16 {
    unsafe {
        raw::rustGetUsedItemCharge(pid)
    }
}

///
///  Get the enchantment charge of the item last used by a player.
///
///  [`pid`] The player ID.
///
///  Returns the enchantment charge.
///
pub fn get_used_item_enchantment_charge(pid: u16) -> f64 {
    unsafe {
        raw::rustGetUsedItemEnchantmentCharge(pid)
    }
}

///
///  Get the soul of the item last used by a player.
///
///  [`pid`] The player ID.
///
///  Returns the soul.
///
pub fn get_used_item_soul(pid: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetUsedItemSoul(pid))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Send a PlayerEquipment packet with a player's equipment.
///
///  It is always sent to all players.
///
///  [`pid`] The player ID whose equipment should be sent.
///
///  Returns void
///
pub fn send_equipment(pid: u16) {
    unsafe {
        raw::rustSendEquipment(pid)
    }
}

///
///  Send a PlayerInventory packet with a player's recorded inventory changes.
///
///  [`pid`] The player ID whose inventory changes should be used.
///  [`send_to_other_players`] Whether this packet should be sent to players other than the
///                            player attached to the packet (false by default).
///  [`skip_attached_player`] Whether the packet should skip being sent to the player attached
///                            to the packet (false by default).
///
///  Returns void
///
pub fn send_inventory_changes(pid: u16, send_to_other_players: bool, skip_attached_player: bool) {
    unsafe {
        raw::rustSendInventoryChanges(pid, send_to_other_players, skip_attached_player)
    }
}

///
///  Send a PlayerItemUse causing a player to use their recorded usedItem.
///
///  [`pid`] The player ID affected.
///
///  Returns void
///
pub fn send_item_use(pid: u16) {
    unsafe {
        raw::rustSendItemUse(pid)
    }
}

pub fn initialize_inventory_changes(pid: u16) {
    unsafe {
        raw::rustInitializeInventoryChanges(pid)
    }
}

pub fn add_item(pid: u16, ref_id: &str, count: u16, charge: i16, enchantment_charge: f64, soul: &str) {
    unsafe {
        raw::rustAddItem(pid, CString::new(ref_id).unwrap_or_default().as_ptr(), count, charge, enchantment_charge, CString::new(soul).unwrap_or_default().as_ptr())
    }
}

///
///  Get the type of a PlayerMiscellaneous packet.
///
///  [`pid`] The player ID.
///
///  Returns the type.
///
pub fn get_miscellaneous_change_type(pid: u16) -> u8 {
    unsafe {
        raw::rustGetMiscellaneousChangeType(pid)
    }
}

///
///  Get the cell description of a player's Mark cell.
///
///  [`pid`] The player ID.
///
///  Returns the cell description.
///
pub fn get_mark_cell(pid: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetMarkCell(pid))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the X position of a player's Mark.
///
///  [`pid`] The player ID.
///
///  Returns the X position.
///
pub fn get_mark_pos_x(pid: u16) -> f64 {
    unsafe {
        raw::rustGetMarkPosX(pid)
    }
}

///
///  Get the Y position of a player's Mark.
///
///  [`pid`] The player ID.
///
///  Returns the Y position.
///
pub fn get_mark_pos_y(pid: u16) -> f64 {
    unsafe {
        raw::rustGetMarkPosY(pid)
    }
}

///
///  Get the Z position of a player's Mark.
///
///  [`pid`] The player ID.
///
///  Returns the Z position.
///
pub fn get_mark_pos_z(pid: u16) -> f64 {
    unsafe {
        raw::rustGetMarkPosZ(pid)
    }
}

///
///  Get the X rotation of a player's Mark.
///
///  [`pid`] The player ID.
///
///  Returns the X rotation.
///
pub fn get_mark_rot_x(pid: u16) -> f64 {
    unsafe {
        raw::rustGetMarkRotX(pid)
    }
}

///
///  Get the Z rotation of a player's Mark.
///
///  [`pid`] The player ID.
///
///  Returns the X rotation.
///
pub fn get_mark_rot_z(pid: u16) -> f64 {
    unsafe {
        raw::rustGetMarkRotZ(pid)
    }
}

///
///  Get the ID of a player's selected spell.
///
///  [`pid`] The player ID.
///
///  Returns the spell ID.
///
pub fn get_selected_spell_id(pid: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetSelectedSpellId(pid))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Check whether the killer of a certain player is also a player.
///
///  [`pid`] The player ID of the killed player.
///
///  Returns whether the player was killed by another player.
///
pub fn does_player_have_player_killer(pid: u16) -> bool {
    unsafe {
        raw::rustDoesPlayerHavePlayerKiller(pid)
    }
}

///
///  Get the player ID of the killer of a certain player.
///
///  [`pid`] The player ID of the killed player.
///
///  Returns the player ID of the killer.
///
pub fn get_player_killer_pid(pid: u16) -> i16 {
    unsafe {
        raw::rustGetPlayerKillerPid(pid)
    }
}

///
///  Get the refId of the actor killer of a certain player.
///
///  [`pid`] The player ID of the killed player.
///
///  Returns the refId of the killer.
///
pub fn get_player_killer_ref_id(pid: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetPlayerKillerRefId(pid))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the refNum of the actor killer of a certain player.
///
///  [`pid`] The player ID of the killed player.
///
///  Returns the refNum of the killer.
///
pub fn get_player_killer_ref_num(pid: u16) -> u16 {
    unsafe {
        raw::rustGetPlayerKillerRefNum(pid)
    }
}

///
///  Get the mpNum of the actor killer of a certain player.
///
///  [`pid`] The player ID of the killed player.
///
///  Returns the mpNum of the killer.
///
pub fn get_player_killer_mp_num(pid: u16) -> u16 {
    unsafe {
        raw::rustGetPlayerKillerMpNum(pid)
    }
}

///
///  Get the name of the actor killer of a certain player.
///
///  [`pid`] The player ID of the killed player.
///
///  Returns the name of the killer.
///
pub fn get_player_killer_name(pid: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetPlayerKillerName(pid))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the draw state of a player (0 for nothing, 1 for drawn weapon,
///         2 for drawn spell).
///
///  [`pid`] The player ID.
///
///  Returns the draw state.
///
pub fn get_draw_state(pid: u16) -> u16 {
    unsafe {
        raw::rustGetDrawState(pid)
    }
}

///
///  Get the sneak state of a player.
///
///  [`pid`] The player ID.
///
///  Returns whether the player is sneaking.
///
pub fn get_sneak_state(pid: u16) -> bool {
    unsafe {
        raw::rustGetSneakState(pid)
    }
}

///
///  Set the Mark cell of a player.
///
///  This changes the Mark cell recorded for that player in the server memory, but does not by itself
///  send a packet.
///
///  The cell is determined to be an exterior cell if it fits the pattern of a number followed
///  by a comma followed by another number.
///
///  [`pid`] The player ID.
///  [`cell_description`] The cell description.
///
///  Returns void
///
pub fn set_mark_cell(pid: u16, cell_description: &str) {
    unsafe {
        raw::rustSetMarkCell(pid, CString::new(cell_description).unwrap_or_default().as_ptr())
    }
}

///
///  Set the Mark position of a player.
///
///  This changes the Mark positional coordinates recorded for that player in the server memory, but
///  does not by itself send a packet.
///
///  [`pid`] The player ID.
///  [`x`] The X position.
///  [`y`] The Y position.
///  [`z`] The Z position.
///
///  Returns void
///
pub fn set_mark_pos(pid: u16, x: f64, y: f64, z: f64) {
    unsafe {
        raw::rustSetMarkPos(pid, x, y, z)
    }
}

///
///  Set the Mark rotation of a player.
///
///  This changes the Mark positional coordinates recorded for that player in the server memory, but
///  does not by itself send a packet.
///
///  [`pid`] The player ID.
///  [`x`] The X rotation.
///  [`z`] The Z rotation.
///
///  Returns void
///
pub fn set_mark_rot(pid: u16, x: f64, z: f64) {
    unsafe {
        raw::rustSetMarkRot(pid, x, z)
    }
}

///
///  Set the ID of a player's selected spell.
///
///  This changes the spell ID recorded for that player in the server memory, but does not by itself
///  send a packet.
///
///  [`pid`] The player ID.
///  [`spell_id`] The spell ID.
///
///  Returns void
///
pub fn set_selected_spell_id(pid: u16, spell_id: &str) {
    unsafe {
        raw::rustSetSelectedSpellId(pid, CString::new(spell_id).unwrap_or_default().as_ptr())
    }
}

///
///  Send a PlayerMiscellaneous packet with a Mark location to a player.
///
///  [`pid`] The player ID.
///
///  Returns void
///
pub fn send_mark_location(pid: u16) {
    unsafe {
        raw::rustSendMarkLocation(pid)
    }
}

///
///  Send a PlayerMiscellaneous packet with a selected spell ID to a player.
///
///  [`pid`] The player ID.
///
///  Returns void
///
pub fn send_selected_spell(pid: u16) {
    unsafe {
        raw::rustSendSelectedSpell(pid)
    }
}

///
///  Send a PlayerJail packet about a player.
///
///  This is similar to the player being jailed by a guard, but provides extra parameters for
///  increased flexibility.
///
///  It is only sent to the player being jailed, as the other players will be informed of the
///  jailing's actual consequences via other packets sent by the affected client.
///
///  [`pid`] The player ID.
///  [`jail_days`] The number of days to spend jailed, where each day affects one skill point.
///  [`ignore_jail_teleportation`] Whether the player being teleported to the nearest jail
///                                 marker should be overridden.
///  [`ignore_jail_skill_increase`] Whether the player's Sneak and Security skills should be
///                                 prevented from increasing as a result of the jailing,
///                                 overriding default behavior.
///  [`jail_progress_text`] The text that should be displayed while jailed.
///  [`jail_end_text`] The text that should be displayed once the jailing period is over.
///
///  Returns void
///
pub fn jail(pid: u16, jail_days: i16, ignore_jail_teleportation: bool, ignore_jail_skill_increases: bool, jail_progress_text: &str, jail_end_text: &str) {
    unsafe {
        raw::rustJail(pid, jail_days, ignore_jail_teleportation, ignore_jail_skill_increases, CString::new(jail_progress_text).unwrap_or_default().as_ptr(), CString::new(jail_end_text).unwrap_or_default().as_ptr())
    }
}

///
///  Send a PlayerResurrect packet about a player.
///
///  This sends the packet to all players connected to the server.
///
///  [`pid`] The player ID.
///  [`_type`] The type of resurrection (0 for REGULAR, 1 for IMPERIAL_SHRINE,
///              2 for TRIBUNAL_TEMPLE).
///
///  Returns void
///
pub fn resurrect(pid: u16, _type: u16) {
    unsafe {
        raw::rustResurrect(pid, _type)
    }
}

pub fn get_death_reason(pid: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetDeathReason(pid))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

pub fn get_player_killer_ref_num_index(pid: u16) -> u16 {
    unsafe {
        raw::rustGetPlayerKillerRefNumIndex(pid)
    }
}

///
///  Generate a random string of a particular length that only contains
///         letters and numbers.
///
///  [`length`] The length of the generated string.
///
///  Returns the generated string.
///
pub fn generate_random_string(length: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGenerateRandomString(length))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the SHA256 hash corresponding to an input string.
///
///  [`input_string`] The input string.
///
///  Returns the SHA256 hash.
///
pub fn get_sha256_hash(input_string: &str) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetSHA256Hash(CString::new(input_string).unwrap_or_default().as_ptr()))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the last player ID currently connected to the server.
///
///  Every player receives a unique numerical index known as their player ID upon joining the
///  server.
///
///
///  Returns the player ID.
///
pub fn get_last_player_id() -> u16 {
    unsafe {
        raw::rustGetLastPlayerId()
    }
}

///
///  Get the current (latest) mpNum generated by the server.
///
///  Every object that did not exist in an .ESM or .ESP data file and has instead been placed or
///  spawned through a server-sent packet has a numerical index known as its mpNum.
///
///  When ObjectPlace and ObjectSpawn packets are received from players, their objects lack mpNums,
///  so the server assigns them some based on incrementing the server's current mpNum, with the
///  operation's final mpNum becoming the server's new current mpNum.
///
///
///  Returns the mpNum.
///
pub fn get_current_mp_num() -> i16 {
    unsafe {
        raw::rustGetCurrentMpNum()
    }
}

///
///  Set the current (latest) mpNum generated by the server.
///
///  When restarting a server, it is important to revert to the previous current (latest) mpNum
///  as stored in the server's data, so as to avoid starting over from 0 and ending up assigning
///  duplicate mpNums to objects.
///
///  [`mp_num`] The number that should be used as the new current mpNum.
///
///  Returns void
///
pub fn set_current_mp_num(mp_num: i16) {
    unsafe {
        raw::rustSetCurrentMpNum(mp_num)
    }
}

///
///  Get the X position of a player.
///
///  [`pid`] The player ID.
///
///  Returns the X position.
///
pub fn get_pos_x(pid: u16) -> f64 {
    unsafe {
        raw::rustGetPosX(pid)
    }
}

///
///  Get the Y position of a player.
///
///  [`pid`] The player ID.
///
///  Returns the Y position.
///
pub fn get_pos_y(pid: u16) -> f64 {
    unsafe {
        raw::rustGetPosY(pid)
    }
}

///
///  Get the Z position of a player.
///
///  [`pid`] The player ID.
///
///  Returns the Z position.
///
pub fn get_pos_z(pid: u16) -> f64 {
    unsafe {
        raw::rustGetPosZ(pid)
    }
}

///
///  Get the X position of a player from before their latest cell change.
///
///  [`pid`] The player ID.
///
///  Returns the X position.
///
pub fn get_previous_cell_pos_x(pid: u16) -> f64 {
    unsafe {
        raw::rustGetPreviousCellPosX(pid)
    }
}

///
///  Get the Y position of a player from before their latest cell change.
///
///  [`pid`] The player ID.
///
///  Returns the Y position.
///
pub fn get_previous_cell_pos_y(pid: u16) -> f64 {
    unsafe {
        raw::rustGetPreviousCellPosY(pid)
    }
}

///
///  Get the Z position of a player from before their latest cell change.
///
///  [`pid`] The player ID.
///
///  Returns the Z position.
///
pub fn get_previous_cell_pos_z(pid: u16) -> f64 {
    unsafe {
        raw::rustGetPreviousCellPosZ(pid)
    }
}

///
///  Get the X rotation of a player.
///
///  [`pid`] The player ID.
///
///  Returns the X rotation.
///
pub fn get_rot_x(pid: u16) -> f64 {
    unsafe {
        raw::rustGetRotX(pid)
    }
}

///
///  Get the Z rotation of a player.
///
///  [`pid`] The player ID.
///
///  Returns the Z rotation.
///
pub fn get_rot_z(pid: u16) -> f64 {
    unsafe {
        raw::rustGetRotZ(pid)
    }
}

///
///  Set the position of a player.
///
///  This changes the positional coordinates recorded for that player in the server memory, but
///  does not by itself send a packet.
///
///  [`pid`] The player ID.
///  [`x`] The X position.
///  [`y`] The Y position.
///  [`z`] The Z position.
///
///  Returns void
///
pub fn set_pos(pid: u16, x: f64, y: f64, z: f64) {
    unsafe {
        raw::rustSetPos(pid, x, y, z)
    }
}

///
///  Set the rotation of a player.
///
///  This changes the rotational coordinates recorded for that player in the server memory, but
///  does not by itself send a packet.
///
///  A player's Y rotation is always 0, which is why there is no Y rotation parameter.
///
///  [`pid`] The player ID.
///  [`x`] The X position.
///  [`z`] The Z position.
///
///  Returns void
///
pub fn set_rot(pid: u16, x: f64, z: f64) {
    unsafe {
        raw::rustSetRot(pid, x, z)
    }
}

///
///  Set the momentum of a player.
///
///  This changes the coordinates recorded for that player's momentum in the server memory, but
///  does not by itself send a packet.
///
///  [`pid`] The player ID.
///  [`x`] The X momentum.
///  [`y`] The Y momentum.
///  [`z`] The Z momentum.
///
///  Returns void
///
pub fn set_momentum(pid: u16, x: f64, y: f64, z: f64) {
    unsafe {
        raw::rustSetMomentum(pid, x, y, z)
    }
}

///
///  Send a PlayerPosition packet about a player.
///
///  It is only sent to the affected player.
///
///  [`pid`] The player ID.
///
///  Returns void
///
pub fn send_pos(pid: u16) {
    unsafe {
        raw::rustSendPos(pid)
    }
}

///
///  Send a PlayerMomentum packet about a player.
///
///  It is only sent to the affected player.
///
///  [`pid`] The player ID.
///
///  Returns void
///
pub fn send_momentum(pid: u16) {
    unsafe {
        raw::rustSendMomentum(pid)
    }
}

///
///  Clear the last recorded journal changes for a player.
///
///  This is used to initialize the sending of new PlayerJournal packets.
///
///  [`pid`] The player ID whose journal changes should be used.
///
///  Returns void
///
pub fn clear_journal_changes(pid: u16) {
    unsafe {
        raw::rustClearJournalChanges(pid)
    }
}

///
///  Get the number of indexes in a player's latest journal changes.
///
///  [`pid`] The player ID whose journal changes should be used.
///
///  Returns the number of indexes.
///
pub fn get_journal_changes_size(pid: u16) -> u16 {
    unsafe {
        raw::rustGetJournalChangesSize(pid)
    }
}

///
///  Add a new journal item of type ENTRY to the journal changes for a player,
///   with a specific timestamp.
///
///  [`pid`] The player ID whose journal changes should be used.
///  [`quest`] The quest of the journal item.
///  [`index`] The quest index of the journal item.
///  [`actor_ref_id`] The actor refId of the journal item.
///
///  Returns void
///
pub fn add_journal_entry(pid: u16, quest: &str, index: u16, actor_ref_id: &str) {
    unsafe {
        raw::rustAddJournalEntry(pid, CString::new(quest).unwrap_or_default().as_ptr(), index, CString::new(actor_ref_id).unwrap_or_default().as_ptr())
    }
}

///
///  Add a new journal item of type ENTRY to the journal changes for a player,
///   with a specific timestamp.
///
///  [`pid`] The player ID whose journal changes should be used.
///  [`quest`] The quest of the journal item.
///  [`index`] The quest index of the journal item.
///  [`actor_ref_id`] The actor refId of the journal item.
///  [`the`] daysPassed for the journal item.
///  [`the`] month for the journal item.
///  [`the`] day of the month for the journal item.
///
///  Returns void
///
pub fn add_journal_entry_with_timestamp(pid: u16, quest: &str, index: u16, actor_ref_id: &str, days_passed: u16, month: u16, day: u16) {
    unsafe {
        raw::rustAddJournalEntryWithTimestamp(pid, CString::new(quest).unwrap_or_default().as_ptr(), index, CString::new(actor_ref_id).unwrap_or_default().as_ptr(), days_passed, month, day)
    }
}

///
///  Add a new journal item of type INDEX to the journal changes for a player.
///
///  [`pid`] The player ID whose journal changes should be used.
///  [`quest`] The quest of the journal item.
///  [`index`] The quest index of the journal item.
///
///  Returns void
///
pub fn add_journal_index(pid: u16, quest: &str, index: u16) {
    unsafe {
        raw::rustAddJournalIndex(pid, CString::new(quest).unwrap_or_default().as_ptr(), index)
    }
}

///
///  Set the reputation of a certain player.
///
///  [`pid`] The player ID.
///  [`value`] The reputation.
///
///  Returns void
///
pub fn set_reputation(pid: u16, value: i16) {
    unsafe {
        raw::rustSetReputation(pid, value)
    }
}

///
///  Get the quest at a certain index in a player's latest journal changes.
///
///  [`pid`] The player ID whose journal changes should be used.
///  [`index`] The index of the journalItem.
///
///  Returns the quest.
///
pub fn get_journal_item_quest(pid: u16, index: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetJournalItemQuest(pid, index))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the quest index at a certain index in a player's latest journal changes.
///
///  [`pid`] The player ID whose journal changes should be used.
///  [`index`] The index of the journalItem.
///
///  Returns the quest index.
///
pub fn get_journal_item_index(pid: u16, index: u16) -> i16 {
    unsafe {
        raw::rustGetJournalItemIndex(pid, index)
    }
}

///
///  Get the journal item type at a certain index in a player's latest journal changes.
///
///  [`pid`] The player ID whose journal changes should be used.
///  [`index`] The index of the journalItem.
///
///  Returns the type (0 for ENTRY, 1 for INDEX).
///
pub fn get_journal_item_type(pid: u16, index: u16) -> i16 {
    unsafe {
        raw::rustGetJournalItemType(pid, index)
    }
}

///
///  Get the actor refId at a certain index in a player's latest journal changes.
///
///  Every journal change has an associated actor, which is usually the quest giver.
///
///  [`pid`] The player ID whose journal changes should be used.
///  [`index`] The index of the journalItem.
///
///  Returns the actor refId.
///
pub fn get_journal_item_actor_ref_id(pid: u16, index: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetJournalItemActorRefId(pid, index))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the a certain player's reputation.
///
///  [`pid`] The player ID.
///
///  Returns the reputation.
///
pub fn get_reputation(pid: u16) -> i16 {
    unsafe {
        raw::rustGetReputation(pid)
    }
}

///
///  Send a PlayerJournal packet with a player's recorded journal changes.
///
///  [`pid`] The player ID whose journal changes should be used.
///  [`send_to_other_players`] Whether this packet should be sent to players other than the
///                            player attached to the packet (false by default).
///  [`skip_attached_player`] Whether the packet should skip being sent to the player attached
///                            to the packet (false by default).
///
///  Returns void
///
pub fn send_journal_changes(pid: u16, send_to_other_players: bool, skip_attached_player: bool) {
    unsafe {
        raw::rustSendJournalChanges(pid, send_to_other_players, skip_attached_player)
    }
}

///
///  Send a PlayerReputation packet with a player's recorded reputation.
///
///  [`pid`] The player ID whose reputation should be used.
///  [`send_to_other_players`] Whether this packet should be sent to players other than the
///                            player attached to the packet (false by default).
///  [`skip_attached_player`] Whether the packet should skip being sent to the player attached
///                            to the packet (false by default).
///
///  Returns void
///
pub fn send_reputation(pid: u16, send_to_other_players: bool, skip_attached_player: bool) {
    unsafe {
        raw::rustSendReputation(pid, send_to_other_players, skip_attached_player)
    }
}

pub fn initialize_journal_changes(pid: u16) {
    unsafe {
        raw::rustInitializeJournalChanges(pid)
    }
}

///
///  Clear the data from the records stored on the server.
///
///
///  Returns void
///
pub fn clear_records() {
    unsafe {
        raw::rustClearRecords()
    }
}

///
///  Get the type of records in the read worldstate's dynamic records.
///
///
///  Returns the type of records (0 for SPELL, 1 for POTION, 2 for ENCHANTMENT,
///          3 for NPC).
///
pub fn get_record_type() -> u16 {
    unsafe {
        raw::rustGetRecordType()
    }
}

///
///  Get the number of records in the read worldstate's dynamic records.
///
///
///  Returns the number of records.
///
pub fn get_record_count() -> u16 {
    unsafe {
        raw::rustGetRecordCount()
    }
}

///
///  Get the number of effects for the record at a certain index in the read
///  worldstate's current records.
///
///  [`record_index`] The index of the record.
///
///  Returns the number of effects.
///
pub fn get_record_effect_count(record_index: u16) -> u16 {
    unsafe {
        raw::rustGetRecordEffectCount(record_index)
    }
}

///
///  Get the id of the record at a certain index in the read worldstate's
///  dynamic records of the current type.
///
///  [`index`] The index of the record.
///
///  Returns the id of the record.
///
pub fn get_record_id(index: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetRecordId(index))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the base id (i.e. the id this record should inherit default
///  values from) of the record at a certain index in the read worldstate's
///  dynamic records of the current type.
///
///  [`index`] The index of the record.
///
///  Returns the base id of the record.
///
pub fn get_record_base_id(index: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetRecordBaseId(index))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the subtype of the record at a certain index in the read worldstate's
///  dynamic records of the current type.
///
///  [`index`] The index of the record.
///
///  Returns the type of the record.
///
pub fn get_record_subtype(index: u16) -> i16 {
    unsafe {
        raw::rustGetRecordSubtype(index)
    }
}

///
///  Get the name of the record at a certain index in the read worldstate's
///  dynamic records of the current type.
///
///  [`index`] The index of the record.
///
///  Returns the name of the record.
///
pub fn get_record_name(index: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetRecordName(index))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the model of the record at a certain index in the read worldstate's
///  dynamic records of the current type.
///
///  [`index`] The index of the record.
///
///  Returns the model of the record.
///
pub fn get_record_model(index: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetRecordModel(index))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the icon of the record at a certain index in the read worldstate's
///  dynamic records of the current type.
///
///  [`index`] The index of the record.
///
///  Returns the icon of the record.
///
pub fn get_record_icon(index: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetRecordIcon(index))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the script of the record at a certain index in the read worldstate's
///  dynamic records of the current type.
///
///  [`index`] The index of the record.
///
///  Returns the script of the record.
///
pub fn get_record_script(index: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetRecordScript(index))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the enchantment id of the record at a certain index in the read
///  worldstate's dynamic records of the current type.
///
///  [`index`] The index of the record.
///
///  Returns the enchantment id of the record.
///
pub fn get_record_enchantment_id(index: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetRecordEnchantmentId(index))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the enchantment charge of the record at a certain index in
///  the read worldstate's dynamic records of the current type.
///
///  [`index`] The index of the record.
///
///  Returns the enchantment charge of the record.
///
pub fn get_record_enchantment_charge(index: u16) -> i16 {
    unsafe {
        raw::rustGetRecordEnchantmentCharge(index)
    }
}

///
///  Get the auto-calculation flag value of the record at a certain index in
///  the read worldstate's dynamic records of the current type.
///
///  [`index`] The index of the record.
///
///  Returns the auto-calculation flag value of the record.
///
pub fn get_record_auto_calc(index: u16) -> i16 {
    unsafe {
        raw::rustGetRecordAutoCalc(index)
    }
}

///
///  Get the charge of the record at a certain index in the read worldstate's
///  dynamic records of the current type.
///
///  [`index`] The index of the record.
///
///  Returns the charge of the record.
///
pub fn get_record_charge(index: u16) -> i16 {
    unsafe {
        raw::rustGetRecordCharge(index)
    }
}

///
///  Get the cost of the record at a certain index in the read worldstate's
///  dynamic records of the current type.
///
///  [`index`] The index of the record.
///
///  Returns the cost of the record.
///
pub fn get_record_cost(index: u16) -> i16 {
    unsafe {
        raw::rustGetRecordCost(index)
    }
}

///
///  Get the flags of the record at a certain index in the read worldstate's
///  dynamic records of the current type.
///
///  [`index`] The index of the record.
///
///  Returns the flags of the spell as an integer.
///
pub fn get_record_flags(index: u16) -> i16 {
    unsafe {
        raw::rustGetRecordFlags(index)
    }
}

///
///  Get the value of the record at a certain index in the read worldstate's
///  dynamic records of the current type.
///
///  [`index`] The index of the record.
///
///  Returns the value of the record.
///
pub fn get_record_value(index: u16) -> i16 {
    unsafe {
        raw::rustGetRecordValue(index)
    }
}

///
///  Get the weight of the record at a certain index in the read worldstate's
///  dynamic records of the current type.
///
///  [`index`] The index of the record.
///
///  Returns the weight of the record.
///
pub fn get_record_weight(index: u16) -> f64 {
    unsafe {
        raw::rustGetRecordWeight(index)
    }
}

///
///  Get the ID of the effect at a certain index in the read worldstate's
///  current records.
///
///  [`record_index`] The index of the record.
///  [`effect_index`] The index of the effect.
///
///  Returns the ID of the effect.
///
pub fn get_record_effect_id(record_index: u16, effect_index: u16) -> u16 {
    unsafe {
        raw::rustGetRecordEffectId(record_index, effect_index)
    }
}

///
///  Get the ID of the attribute modified by the effect at a certain index in the
///  read worldstate's current records.
///
///  [`record_index`] The index of the record.
///  [`effect_index`] The index of the effect.
///
///  Returns the attribute ID for the effect.
///
pub fn get_record_effect_attribute(record_index: u16, effect_index: u16) -> i16 {
    unsafe {
        raw::rustGetRecordEffectAttribute(record_index, effect_index)
    }
}

///
///  Get the ID of the skill modified by the effect at a certain index in the
///  read worldstate's current records.
///
///  [`record_index`] The index of the record.
///  [`effect_index`] The index of the effect.
///
///  Returns the skill ID for the effect.
///
pub fn get_record_effect_skill(record_index: u16, effect_index: u16) -> i16 {
    unsafe {
        raw::rustGetRecordEffectSkill(record_index, effect_index)
    }
}

///
///  Get the range type of the effect at a certain index in the read worldstate's
///  current records (0 for self, 1 for touch, 2 for target).
///
///  [`record_index`] The index of the record.
///  [`effect_index`] The index of the effect.
///
///  Returns the range of the effect.
///
pub fn get_record_effect_range_type(record_index: u16, effect_index: u16) -> u16 {
    unsafe {
        raw::rustGetRecordEffectRangeType(record_index, effect_index)
    }
}

///
///  Get the area of the effect at a certain index in the read worldstate's current
///  records.
///
///  [`record_index`] The index of the record.
///  [`effect_index`] The index of the effect.
///
///  Returns the area of the effect.
///
pub fn get_record_effect_area(record_index: u16, effect_index: u16) -> i16 {
    unsafe {
        raw::rustGetRecordEffectArea(record_index, effect_index)
    }
}

///
///  Get the duration of the effect at a certain index in the read worldstate's current
///  records.
///
///  [`record_index`] The index of the record.
///  [`effect_index`] The index of the effect.
///
///  Returns the duration of the effect.
///
pub fn get_record_effect_duration(record_index: u16, effect_index: u16) -> i16 {
    unsafe {
        raw::rustGetRecordEffectDuration(record_index, effect_index)
    }
}

///
///  Get the maximum magnitude of the effect at a certain index in the read
///  worldstate's current records.
///
///  [`record_index`] The index of the record.
///  [`effect_index`] The index of the effect.
///
///  Returns the maximum magnitude of the effect.
///
pub fn get_record_effect_magnitude_max(record_index: u16, effect_index: u16) -> i16 {
    unsafe {
        raw::rustGetRecordEffectMagnitudeMax(record_index, effect_index)
    }
}

///
///  Get the minimum magnitude of the effect at a certain index in the read
///  worldstate's current records.
///
///  [`record_index`] The index of the record.
///  [`effect_index`] The index of the effect.
///
///  Returns the minimum magnitude of the effect.
///
pub fn get_record_effect_magnitude_min(record_index: u16, effect_index: u16) -> i16 {
    unsafe {
        raw::rustGetRecordEffectMagnitudeMin(record_index, effect_index)
    }
}

///
///  Set which type of temporary records stored on the server should have
///  their data changed via setter functions.
///
///  [`_type`] The type of records.
///
///  Returns void
///
pub fn set_record_type(_type: u16) {
    unsafe {
        raw::rustSetRecordType(_type)
    }
}

///
///  Set the id of the temporary record stored on the server for the
///  currently specified record type.
///
///  [`id`] The id of the record.
///
///  Returns void
///
pub fn set_record_id(id: &str) {
    unsafe {
        raw::rustSetRecordId(CString::new(id).unwrap_or_default().as_ptr())
    }
}

///
///  Set the base id (i.e. the id this record should inherit default
///  values from) of the temporary record stored on the server for the
///  currently specified record type.
///
///  [`base_id`] The baseId of the record.
///
///  Returns void
///
pub fn set_record_base_id(base_id: &str) {
    unsafe {
        raw::rustSetRecordBaseId(CString::new(base_id).unwrap_or_default().as_ptr())
    }
}

///
///  Set the inventory base id (i.e. the id this record should inherit
///  its inventory contents from) of the temporary record stored on the server for
///  the currently specified record type.
///
///  [`inventory_base_id`] The inventoryBaseId of the record.
///
///  Returns void
///
pub fn set_record_inventory_base_id(inventory_base_id: &str) {
    unsafe {
        raw::rustSetRecordInventoryBaseId(CString::new(inventory_base_id).unwrap_or_default().as_ptr())
    }
}

///
///  Set the subtype of the temporary record stored on the server for
///  the currently specified record type.
///
///  [`_type`] The spell type.
///
///  Returns void
///
pub fn set_record_subtype(subtype: u16) {
    unsafe {
        raw::rustSetRecordSubtype(subtype)
    }
}

///
///  Set the name of the temporary record stored on the server for the
///  currently specified record type.
///
///  [`name`] The name of the record.
///
///  Returns void
///
pub fn set_record_name(name: &str) {
    unsafe {
        raw::rustSetRecordName(CString::new(name).unwrap_or_default().as_ptr())
    }
}

///
///  Set the model of the temporary record stored on the server for the
///  currently specified record type.
///
///  [`model`] The model of the record.
///
///  Returns void
///
pub fn set_record_model(model: &str) {
    unsafe {
        raw::rustSetRecordModel(CString::new(model).unwrap_or_default().as_ptr())
    }
}

///
///  Set the icon of the temporary record stored on the server for the
///  currently specified record type.
///
///  [`icon`] The icon of the record.
///
///  Returns void
///
pub fn set_record_icon(icon: &str) {
    unsafe {
        raw::rustSetRecordIcon(CString::new(icon).unwrap_or_default().as_ptr())
    }
}

///
///  Set the script of the temporary record stored on the server for the
///  currently specified record type.
///
///  [`script`] The script of the record.
///
///  Returns void
///
pub fn set_record_script(script: &str) {
    unsafe {
        raw::rustSetRecordScript(CString::new(script).unwrap_or_default().as_ptr())
    }
}

///
///  Set the enchantment id of the temporary record stored on the server
///  for the currently specified record type.
///
///  [`enchantment_id`] The enchantment id of the record.
///
///  Returns void
///
pub fn set_record_enchantment_id(enchantment_id: &str) {
    unsafe {
        raw::rustSetRecordEnchantmentId(CString::new(enchantment_id).unwrap_or_default().as_ptr())
    }
}

///
///  Set the enchantment charge of the temporary record stored on the server
///  for the currently specified record type.
///
///  [`enchantment_charge`] The enchantmentCharge of the record.
///
///  Returns void
///
pub fn set_record_enchantment_charge(enchantment_charge: i16) {
    unsafe {
        raw::rustSetRecordEnchantmentCharge(enchantment_charge)
    }
}

///
///  Set the auto-calculation flag value of the temporary record stored
///  on the server for the currently specified record type.
///
///  [`auto_calc`] The auto-calculation flag value of the record.
///
///  Returns void
///
pub fn set_record_auto_calc(auto_calc: i16) {
    unsafe {
        raw::rustSetRecordAutoCalc(auto_calc)
    }
}

///
///  Set the charge of the temporary record stored on the server for the
///  currently specified record type.
///
///  [`charge`] The charge of the record.
///
///  Returns void
///
pub fn set_record_charge(charge: i16) {
    unsafe {
        raw::rustSetRecordCharge(charge)
    }
}

///
///  Set the cost of the temporary record stored on the server for the
///  currently specified record type.
///
///  [`cost`] The cost of the record.
///
///  Returns void
///
pub fn set_record_cost(cost: i16) {
    unsafe {
        raw::rustSetRecordCost(cost)
    }
}

///
///  Set the flags of the temporary record stored on the server for the
///  currently specified record type.
///
///  [`flags`] The flags of the record.
///
///  Returns void
///
pub fn set_record_flags(flags: i16) {
    unsafe {
        raw::rustSetRecordFlags(flags)
    }
}

///
///  Set the value of the temporary record stored on the server for the
///  currently specified record type.
///
///  [`value`] The value of the record.
///
///  Returns void
///
pub fn set_record_value(value: i16) {
    unsafe {
        raw::rustSetRecordValue(value)
    }
}

///
///  Set the weight of the temporary record stored on the server for the
///  currently specified record type.
///
///  [`weight`] The weight of the record.
///
///  Returns void
///
pub fn set_record_weight(weight: f64) {
    unsafe {
        raw::rustSetRecordWeight(weight)
    }
}

///
///  Set the item quality of the temporary record stored on the server for the
///  currently specified record type.
///
///  [`weight`] The weight of the record.
///
///  Returns void
///
pub fn set_record_quality(quality: f64) {
    unsafe {
        raw::rustSetRecordQuality(quality)
    }
}

///
///  Set the number of uses of the temporary record stored on the server for the
///  currently specified record type.
///
///  [`uses`] The number of uses of the record.
///
///  Returns void
///
pub fn set_record_uses(uses: i16) {
    unsafe {
        raw::rustSetRecordUses(uses)
    }
}

///
///  Set the time of the temporary record stored on the server for the currently
///  specified record type.
///
///  [`time`] The time of the record.
///
///  Returns void
///
pub fn set_record_time(time: i16) {
    unsafe {
        raw::rustSetRecordTime(time)
    }
}

///
///  Set the radius of the temporary record stored on the server for the currently
///  specified record type.
///
///  [`uses`] The radius of the record.
///
///  Returns void
///
pub fn set_record_radius(radius: i16) {
    unsafe {
        raw::rustSetRecordRadius(radius)
    }
}

///
///  Set the color of the temporary record stored on the server for the currently
///  specified record type.
///
///  [`color`] The color of the record.
///
///  Returns void
///
pub fn set_record_color(red: u16, green: u16, blue: u16) {
    unsafe {
        raw::rustSetRecordColor(red, green, blue)
    }
}

///
///  Set the armor rating of the temporary record stored on the server
///  for the currently specified record type.
///
///  [`armor_rating`] The armor rating of the record.
///
///  Returns void
///
pub fn set_record_armor_rating(armor_rating: i16) {
    unsafe {
        raw::rustSetRecordArmorRating(armor_rating)
    }
}

///
///  Set the health of the temporary record stored on the server for the
///  currently specified record type.
///
///  [`health`] The health of the record.
///
///  Returns void
///
pub fn set_record_health(health: i16) {
    unsafe {
        raw::rustSetRecordHealth(health)
    }
}

///
///  Set the chop damage of the temporary record stored on the server for the
///  currently specified record type.
///
///  [`min_damage`] The minimum damage of the record.
///  [`max_damage`] The maximum damage of the record.
///
///  Returns void
///
pub fn set_record_damage_chop(min_damage: u16, max_damage: u16) {
    unsafe {
        raw::rustSetRecordDamageChop(min_damage, max_damage)
    }
}

///
///  Set the slash damage of the temporary record stored on the server for the
///  currently specified record type.
///
///  [`min_damage`] The minimum damage of the record.
///  [`max_damage`] The maximum damage of the record.
///
///  Returns void
///
pub fn set_record_damage_slash(min_damage: u16, max_damage: u16) {
    unsafe {
        raw::rustSetRecordDamageSlash(min_damage, max_damage)
    }
}

///
///  Set the thrust damage of the temporary record stored on the server for the
///  currently specified record type.
///
///  [`min_damage`] The minimum damage of the record.
///  [`max_damage`] The maximum damage of the record.
///
///  Returns void
///
pub fn set_record_damage_thrust(min_damage: u16, max_damage: u16) {
    unsafe {
        raw::rustSetRecordDamageThrust(min_damage, max_damage)
    }
}

///
///  Set the reach of the temporary record stored on the server for the
///  currently specified record type.
///
///  [`reach`] The reach of the record.
///
///  Returns void
///
pub fn set_record_reach(reach: f64) {
    unsafe {
        raw::rustSetRecordReach(reach)
    }
}

///
///  Set the speed of the temporary record stored on the server for the
///  currently specified record type.
///
///  [`speed`] The speed of the record.
///
///  Returns void
///
pub fn set_record_speed(speed: f64) {
    unsafe {
        raw::rustSetRecordSpeed(speed)
    }
}

///
///  Set whether the temporary record stored on the server for the
///  currently specified record type is a key.
///
///  Note: This is only applicable to Miscellaneous records.
///
///  [`key_state`] Whether the record is a key.
///
///  Returns void
///
pub fn set_record_key_state(key_state: bool) {
    unsafe {
        raw::rustSetRecordKeyState(key_state)
    }
}

///
///  Set whether the temporary record stored on the server for the
///  currently specified record type is a scroll.
///
///  Note: This is only applicable to Book records.
///
///  [`scroll_state`] Whether the record is a scroll.
///
///  Returns void
///
pub fn set_record_scroll_state(scroll_state: bool) {
    unsafe {
        raw::rustSetRecordScrollState(scroll_state)
    }
}

///
///  Set the skill ID of the temporary record stored on the server for the
///  currently specified record type.
///
///  [`skill_id`] The skill ID of the record.
///
///  Returns void
///
pub fn set_record_skill_id(skill_id: i16) {
    unsafe {
        raw::rustSetRecordSkillId(skill_id)
    }
}

///
///  Set the text of the temporary record stored on the server for the
///  currently specified record type.
///
///  [`text`] The text of the record.
///
///  Returns void
///
pub fn set_record_text(text: &str) {
    unsafe {
        raw::rustSetRecordText(CString::new(text).unwrap_or_default().as_ptr())
    }
}

///
///  Set the hair of the temporary record stored on the server for the
///  currently specified record type.
///
///  [`hair`] The hair of the record.
///
///  Returns void
///
pub fn set_record_hair(hair: &str) {
    unsafe {
        raw::rustSetRecordHair(CString::new(hair).unwrap_or_default().as_ptr())
    }
}

///
///  Set the head of the temporary record stored on the server for the
///  currently specified record type.
///
///  [`hair`] The head of the record.
///
///  Returns void
///
pub fn set_record_head(head: &str) {
    unsafe {
        raw::rustSetRecordHead(CString::new(head).unwrap_or_default().as_ptr())
    }
}

///
///  Set the gender of the temporary record stored on the server for the
///  currently specified record type (0 for female, 1 for male).
///
///  [`hair`] The race of the record.
///
///  Returns void
///
pub fn set_record_gender(gender: u16) {
    unsafe {
        raw::rustSetRecordGender(gender)
    }
}

///
///  Set the race of the temporary record stored on the server for the
///  currently specified record type.
///
///  [`hair`] The race of the record.
///
///  Returns void
///
pub fn set_record_race(race: &str) {
    unsafe {
        raw::rustSetRecordRace(CString::new(race).unwrap_or_default().as_ptr())
    }
}

///
///  Set the character class of the temporary record stored on the server
///  for the currently specified record type.
///
///  [`hair`] The character class of the record.
///
///  Returns void
///
pub fn set_record_class(char_class: &str) {
    unsafe {
        raw::rustSetRecordClass(CString::new(char_class).unwrap_or_default().as_ptr())
    }
}

///
///  Set the faction of the temporary record stored on the server for the
///  currently specified record type.
///
///  [`faction`] The faction of the record.
///
///  Returns void
///
pub fn set_record_faction(faction: &str) {
    unsafe {
        raw::rustSetRecordFaction(CString::new(faction).unwrap_or_default().as_ptr())
    }
}

///
///  Set the scale of the temporary record stored on the server for the
///  currently specified record type.
///
///  [`scale`] The scale of the record.
///
///  Returns void
///
pub fn set_record_scale(scale: f64) {
    unsafe {
        raw::rustSetRecordScale(scale)
    }
}

///
///  Set the blood type of the temporary record stored on the server for the
///  currently specified record type.
///
///  [`blood_type`] The blood type of the record.
///
///  Returns void
///
pub fn set_record_blood_type(blood_type: i16) {
    unsafe {
        raw::rustSetRecordBloodType(blood_type)
    }
}

///
///  Set the level of the temporary record stored on the server for the
///  currently specified record type.
///
///  [`level`] The level of the record.
///
///  Returns void
///
pub fn set_record_level(level: i16) {
    unsafe {
        raw::rustSetRecordLevel(level)
    }
}

///
///  Set the magicka of the temporary record stored on the server for the
///  currently specified record type.
///
///  [`magicka`] The magicka of the record.
///
///  Returns void
///
pub fn set_record_magicka(magicka: i16) {
    unsafe {
        raw::rustSetRecordMagicka(magicka)
    }
}

///
///  Set the fatigue of the temporary record stored on the server for the
///  currently specified record type.
///
///  [`fatigue`] The fatigue of the record.
///
///  Returns void
///
pub fn set_record_fatigue(fatigue: i16) {
    unsafe {
        raw::rustSetRecordFatigue(fatigue)
    }
}

///
///  Set the AI fight value of the temporary record stored on the server for the
///  currently specified record type.
///
///  [`ai_fight`] The AI fight value of the record.
///
///  Returns void
///
pub fn set_record_ai_fight(ai_fight: i16) {
    unsafe {
        raw::rustSetRecordAIFight(ai_fight)
    }
}

///
///  Set the AI flee value of the temporary record stored on the server for the
///  currently specified record type.
///
///  [`ai_flee`] The AI flee value of the record.
///
///  Returns void
///
pub fn set_record_ai_flee(ai_flee: i16) {
    unsafe {
        raw::rustSetRecordAIFlee(ai_flee)
    }
}

///
///  Set the AI alarm value of the temporary record stored on the server for the
///  currently specified record type.
///
///  [`ai_alarm`] The AI alarm value of the record.
///
///  Returns void
///
pub fn set_record_ai_alarm(ai_alarm: i16) {
    unsafe {
        raw::rustSetRecordAIAlarm(ai_alarm)
    }
}

///
///  Set the AI services value of the temporary record stored on the server for the
///  currently specified record type.
///
///  [`ai_services`] The AI services value of the record.
///
///  Returns void
///
pub fn set_record_ai_services(ai_services: i16) {
    unsafe {
        raw::rustSetRecordAIServices(ai_services)
    }
}

///
///  Set the sound of the temporary record stored on the server for the currently
///  specified record type.
///
///  [`sound`] The sound of the record.
///
///  Returns void
///
pub fn set_record_sound(sound: &str) {
    unsafe {
        raw::rustSetRecordSound(CString::new(sound).unwrap_or_default().as_ptr())
    }
}

///
///  Set the opening sound of the temporary record stored on the server for the
///  currently specified record type.
///
///  [`sound`] The opening sound of the record.
///
///  Returns void
///
pub fn set_record_open_sound(sound: &str) {
    unsafe {
        raw::rustSetRecordOpenSound(CString::new(sound).unwrap_or_default().as_ptr())
    }
}

///
///  Set the closing sound of the temporary record stored on the server for the
///  currently specified record type.
///
///  [`sound`] The closing sound of the record.
///
///  Returns void
///
pub fn set_record_close_sound(sound: &str) {
    unsafe {
        raw::rustSetRecordCloseSound(CString::new(sound).unwrap_or_default().as_ptr())
    }
}

///
///  Set the script text of the temporary record stored on the server for the
///  currently specified record type.
///
///  [`sound`] The script text of the record.
///
///  Returns void
///
pub fn set_record_script_text(script_text: &str) {
    unsafe {
        raw::rustSetRecordScriptText(CString::new(script_text).unwrap_or_default().as_ptr())
    }
}

///
///  Set the id of the record at a certain index in the records stored on the server.
///
///  When resending a received RecordsDynamic packet, this allows you to set the server-generated
///  id of a record without having to clear and recreate the packet.
///
///  [`index`] The index of the record.
///  [`id`] The id of the record.
///
///  Returns void
///
pub fn set_record_id_by_index(index: u16, id: &str) {
    unsafe {
        raw::rustSetRecordIdByIndex(index, CString::new(id).unwrap_or_default().as_ptr())
    }
}

///
///  Set the enchantment id of the record at a certain index in the records stored on
///  the server.
///
///  When resending a received RecordsDynamic packet, this allows you to set the server-generated
///  enchantment id of a record without having to clear and recreate the packet.
///
///  [`index`] The index of the record.
///  [`enchantment_id`] The enchantment id of the record.
///
///  Returns void
///
pub fn set_record_enchantment_id_by_index(index: u16, enchantment_id: &str) {
    unsafe {
        raw::rustSetRecordEnchantmentIdByIndex(index, CString::new(enchantment_id).unwrap_or_default().as_ptr())
    }
}

///
///  Set the ID of the temporary effect stored on the server.
///
///  [`effect_id`] The ID of the effect.
///
///  Returns void
///
pub fn set_record_effect_id(effect_id: u16) {
    unsafe {
        raw::rustSetRecordEffectId(effect_id)
    }
}

///
///  Set the ID of the attribute modified by the temporary effect stored on
///  the server.
///
///  [`attribute_id`] The ID of the attribute.
///
///  Returns void
///
pub fn set_record_effect_attribute(attribute_id: i16) {
    unsafe {
        raw::rustSetRecordEffectAttribute(attribute_id)
    }
}

///
///  Set the ID of the skill modified by the temporary effect stored on the
///  server.
///
///  [`skill_id`] The ID of the skill.
///
///  Returns void
///
pub fn set_record_effect_skill(skill_id: i16) {
    unsafe {
        raw::rustSetRecordEffectSkill(skill_id)
    }
}

///
///  Set the range type of the temporary effect stored on the server (0 for
///  self, 1 for touch, 2 for target).
///
///  [`range_type`] The range type of the effect.
///
///  Returns void
///
pub fn set_record_effect_range_type(range_type: u16) {
    unsafe {
        raw::rustSetRecordEffectRangeType(range_type)
    }
}

///
///  Set the area of the temporary effect stored on the server.
///
///  [`area`] The area of the effect.
///
///  Returns void
///
pub fn set_record_effect_area(area: i16) {
    unsafe {
        raw::rustSetRecordEffectArea(area)
    }
}

///
///  Set the duration of the temporary effect stored on the server.
///
///  [`duration`] The duration of the effect.
///
///  Returns void
///
pub fn set_record_effect_duration(duration: i16) {
    unsafe {
        raw::rustSetRecordEffectDuration(duration)
    }
}

///
///  Set the maximum magnitude of the temporary effect stored on the server.
///
///  [`magnitude_max`] The maximum magnitude of the effect.
///
///  Returns void
///
pub fn set_record_effect_magnitude_max(magnitude_max: i16) {
    unsafe {
        raw::rustSetRecordEffectMagnitudeMax(magnitude_max)
    }
}

///
///  Set the minimum magnitude of the temporary effect stored on the server.
///
///  [`magnitude_min`] The minimum magnitude of the effect.
///
///  Returns void
///
pub fn set_record_effect_magnitude_min(magnitude_min: i16) {
    unsafe {
        raw::rustSetRecordEffectMagnitudeMin(magnitude_min)
    }
}

///
///  Set the type of the temporary body part stored on the server.
///
///  [`part_type`] The type of the body part.
///
///  Returns void
///
pub fn set_record_body_part_type(part_type: u16) {
    unsafe {
        raw::rustSetRecordBodyPartType(part_type)
    }
}

///
///  Set the id of the male version of the temporary body part stored on the
///  server.
///
///  [`part_id`] The id of the body part.
///
///  Returns void
///
pub fn set_record_body_part_id_for_male(part_id: &str) {
    unsafe {
        raw::rustSetRecordBodyPartIdForMale(CString::new(part_id).unwrap_or_default().as_ptr())
    }
}

///
///  Set the id of the female version of the temporary body part stored on the
///  server.
///
///  [`part_id`] The id of the body part.
///
///  Returns void
///
pub fn set_record_body_part_id_for_female(part_id: &str) {
    unsafe {
        raw::rustSetRecordBodyPartIdForFemale(CString::new(part_id).unwrap_or_default().as_ptr())
    }
}

///
///  Set the id of the of the temporary inventory item stored on the server.
///
///  [`part_id`] The id of the inventory item.
///
///  Returns void
///
pub fn set_record_inventory_item_id(item_id: &str) {
    unsafe {
        raw::rustSetRecordInventoryItemId(CString::new(item_id).unwrap_or_default().as_ptr())
    }
}

///
///  Set the count of the of the temporary inventory item stored on the server.
///
///  [`count`] The count of the inventory item.
///
///  Returns void
///
pub fn set_record_inventory_item_count(count: u16) {
    unsafe {
        raw::rustSetRecordInventoryItemCount(count)
    }
}

///
///  Add a copy of the server's temporary record of the current specified
///  type to the stored records.
///
///  In the process, the server's temporary record will automatically be cleared
///  so a new one can be set up.
///
///
///  Returns void
///
pub fn add_record() {
    unsafe {
        raw::rustAddRecord()
    }
}

///
///  Add a copy of the server's temporary effect to the temporary record
///  of the current specified type.
///
///  In the process, the server's temporary effect will automatically be cleared
///  so a new one can be set up.
///
///
///  Returns void
///
pub fn add_record_effect() {
    unsafe {
        raw::rustAddRecordEffect()
    }
}

///
///  Add a copy of the server's temporary body part to the temporary record
///  of the current specified type.
///
///  In the process, the server's temporary body part will automatically be cleared
///  so a new one can be set up.
///
///
///  Returns void
///
pub fn add_record_body_part() {
    unsafe {
        raw::rustAddRecordBodyPart()
    }
}

///
///  Add a copy of the server's temporary inventory item to the temporary record
///  of the current specified type.
///
///  In the process, the server's temporary inventory item will automatically be cleared
///  so a new one can be set up.
///
///  Note: Any items added this way will be ignored if the record already has a valid
///        inventoryBaseId.
///
///
///  Returns void
///
pub fn add_record_inventory_item() {
    unsafe {
        raw::rustAddRecordInventoryItem()
    }
}

///
///  Send a RecordDynamic packet with the current specified record type.
///
///  [`pid`] The player ID attached to the packet.
///  [`send_to_other_players`] Whether this packet should be sent to players other than the
///                            player attached to the packet (false by default).
///  [`skip_attached_player`] Whether the packet should skip being sent to the player attached
///                            to the packet (false by default).
///
///  Returns void
///
pub fn send_record_dynamic(pid: u16, send_to_other_players: bool, skip_attached_player: bool) {
    unsafe {
        raw::rustSendRecordDynamic(pid, send_to_other_players, skip_attached_player)
    }
}

///
///  Get the scale of a player.
///
///  [`pid`] The player ID.
///
///  Returns the scale.
///
pub fn get_scale(pid: u16) -> f64 {
    unsafe {
        raw::rustGetScale(pid)
    }
}

///
///  Check whether a player is a werewolf.
///
///  This is based on the last PlayerShapeshift packet received or sent for that player.
///
///  [`pid`] The player ID.
///
///  Returns the werewolf state.
///
pub fn is_werewolf(pid: u16) -> bool {
    unsafe {
        raw::rustIsWerewolf(pid)
    }
}

///
///  Get the refId of the creature the player is disguised as.
///
///  [`pid`] The player ID.
///
///  Returns the creature refId.
///
pub fn get_creature_ref_id(pid: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetCreatureRefId(pid))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Check whether a player's name is replaced by that of the creature they are
///         disguised as when other players hover over them.
///
///  This is based on the last PlayerShapeshift packet received or sent for that player.
///
///  [`pid`] The player ID.
///
///  Returns the creature name display state.
///
pub fn get_creature_name_display_state(pid: u16) -> bool {
    unsafe {
        raw::rustGetCreatureNameDisplayState(pid)
    }
}

///
///  Set the scale of a player.
///
///  This changes the scale recorded for that player in the server memory, but
///  does not by itself send a packet.
///
///  [`pid`] The player ID.
///  [`scale`] The new scale.
///
///  Returns void
///
pub fn set_scale(pid: u16, scale: f64) {
    unsafe {
        raw::rustSetScale(pid, scale)
    }
}

///
///  Set the werewolf state of a player.
///
///  This changes the werewolf state recorded for that player in the server memory, but
///  does not by itself send a packet.
///
///  [`pid`] The player ID.
///  [`is_werewolf`] The new werewolf state.
///
///  Returns void
///
pub fn set_werewolf_state(pid: u16, is_werewolf: bool) {
    unsafe {
        raw::rustSetWerewolfState(pid, is_werewolf)
    }
}

///
///  Set the refId of the creature a player is disguised as.
///
///  This changes the creature refId recorded for that player in the server memory, but
///  does not by itself send a packet.
///
///  [`pid`] The player ID.
///  [`ref_id`] The creature refId.
///  [`displays_creature_name`] Whether the player's name appears as that of the creature
///                              when hovered over by others.
///
///  Returns void
///
pub fn set_creature_ref_id(pid: u16, ref_id: &str) {
    unsafe {
        raw::rustSetCreatureRefId(pid, CString::new(ref_id).unwrap_or_default().as_ptr())
    }
}

///
///  Set whether a player's name is replaced by that of the creature they are
///         disguised as when other players hover over them.
///
///  [`pid`] The player ID.
///  [`display_state`] The creature name display state.
///
///  Returns void
///
pub fn set_creature_name_display_state(pid: u16, display_state: bool) {
    unsafe {
        raw::rustSetCreatureNameDisplayState(pid, display_state)
    }
}

///
///  Send a PlayerShapeshift packet about a player.
///
///  This sends the packet to all players connected to the server. It is currently used
///  only to communicate werewolf states.
///
///  [`pid`] The player ID.
///
///  Returns void
///
pub fn send_shapeshift(pid: u16) {
    unsafe {
        raw::rustSendShapeshift(pid)
    }
}

///
///  Write a log message with its own timestamp.
///
///  It will have `[Script]:` prepended to it so as to mark it as a script-generated log message.
///
///  [`level`] The logging level used (0 for LOG_VERBOSE, 1 for LOG_INFO, 2 for LOG_WARN,
///               3 for LOG_ERROR, 4 for LOG_FATAL).
///  [`message`] The message logged.
///
///  Returns void
///
pub fn log_message(level: u16, message: &str) {
    unsafe {
        raw::rustLogMessage(level, CString::new(message).unwrap_or_default().as_ptr())
    }
}

///
///  Write a log message without its own timestamp.
///
///  It will have `[Script]:` prepended to it so as to mark it as a script-generated log message.
///
///  [`level`] The logging level used (0 for LOG_VERBOSE, 1 for LOG_INFO, 2 for LOG_WARN,
///               3 for LOG_ERROR, 4 for LOG_FATAL).
///  [`message`] The message logged.
///
///  Returns void
///
pub fn log_append(level: u16, message: &str) {
    unsafe {
        raw::rustLogAppend(level, CString::new(message).unwrap_or_default().as_ptr())
    }
}

///
///  Shut down the server.
///
///  [`code`] The shutdown code.
///
///  Returns void
///
pub fn stop_server(code: i16) {
    unsafe {
        raw::rustStopServer(code)
    }
}

///
///  Kick a certain player from the server.
///
///  [`pid`] The player ID.
///
///  Returns void
///
pub fn kick(pid: u16) {
    unsafe {
        raw::rustKick(pid)
    }
}

///
///  Ban a certain IP address from the server.
///
///  [`ip_address`] The IP address.
///
///  Returns void
///
pub fn ban_address(ip_address: &str) {
    unsafe {
        raw::rustBanAddress(CString::new(ip_address).unwrap_or_default().as_ptr())
    }
}

///
///  Unban a certain IP address from the server.
///
///  [`ip_address`] The IP address.
///
///  Returns void
///
pub fn unban_address(ip_address: &str) {
    unsafe {
        raw::rustUnbanAddress(CString::new(ip_address).unwrap_or_default().as_ptr())
    }
}

///
///  Check whether a certain file path exists.
///
///  This will be a case sensitive check on case sensitive filesystems.
///
///  Whenever you want to enforce case insensitivity, use GetCaseInsensitiveFilename() instead.
///
///
///  Returns whether the file exists or not.
///
pub fn does_file_path_exist(file_path: &str) -> bool {
    unsafe {
        raw::rustDoesFilePathExist(CString::new(file_path).unwrap_or_default().as_ptr())
    }
}

///
///  Get the first filename in a folder that has a case insensitive match with the filename
///  argument.
///
///  This is used to retain case insensitivity when opening data files on Linux.
///
///
///  Returns the filename that matches.
///
pub fn get_case_insensitive_filename(folder_path: &str, filename: &str) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetCaseInsensitiveFilename(CString::new(folder_path).unwrap_or_default().as_ptr(), CString::new(filename).unwrap_or_default().as_ptr()))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the path of the server's data folder.
///
///
///  Returns the data path.
///
pub fn get_data_path() -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetDataPath())
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the milliseconds elapsed since the server was started.
///
///
///  Returns the time since the server's startup in milliseconds.
///
pub fn get_milliseconds_since_server_start() -> u16 {
    unsafe {
        raw::rustGetMillisecondsSinceServerStart()
    }
}

///
///  Get the type of the operating system used by the server.
///
///  Note: Currently, the type can be "Windows", "Linux", "OS X" or "Unknown OS".
///
///
///  Returns the type of the operating system.
///
pub fn get_operating_system_type() -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetOperatingSystemType())
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the architecture type used by the server.
///
///  Note: Currently, the type can be "64-bit", "32-bit", "ARMv#" or "Unknown architecture".
///
///
///  Returns the architecture type.
///
pub fn get_architecture_type() -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetArchitectureType())
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the TES3MP version of the server.
///
///
///  Returns the server version.
///
pub fn get_server_version() -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetServerVersion())
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the protocol version of the server.
///
///
///  Returns the protocol version.
///
pub fn get_protocol_version() -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetProtocolVersion())
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the average ping of a certain player.
///
///  [`pid`] The player ID.
///
///  Returns the average ping.
///
pub fn get_avg_ping(pid: u16) -> i16 {
    unsafe {
        raw::rustGetAvgPing(pid)
    }
}

///
///  Get the IP address of a certain player.
///
///  [`pid`] The player ID.
///
///  Returns the IP address.
///
pub fn get_ip(pid: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetIP(pid))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the maximum number of players.
///
///
///  Returns max players
///
pub fn get_max_players() -> u16 {
    unsafe {
        raw::rustGetMaxPlayers()
    }
}

///
///  Get the port used by the server.
///
///
///  Returns the port.
///
pub fn get_port() -> u16 {
    unsafe {
        raw::rustGetPort()
    }
}

///
///  Checking if the server requires a password to connect.
///
///  @return
///
pub fn has_password() -> bool {
    unsafe {
        raw::rustHasPassword()
    }
}

///
///  Get the data file enforcement state of the server.
///
///  If true, clients are required to use the same data files as set for the server.
///
///
///  Returns the enforcement state.
///
pub fn get_data_file_enforcement_state() -> bool {
    unsafe {
        raw::rustGetDataFileEnforcementState()
    }
}

///
///  Get the script error ignoring state of the server.
///
///  If true, script errors will not crash the server.
///
///
///  Returns the script error ignoring state.
///
pub fn get_script_error_ignoring_state() -> bool {
    unsafe {
        raw::rustGetScriptErrorIgnoringState()
    }
}

///
///  Set the game mode of the server, as displayed in the server browser.
///
///  [`name`] The new game mode.
///
///  Returns void
///
pub fn set_game_mode(game_mode: &str) {
    unsafe {
        raw::rustSetGameMode(CString::new(game_mode).unwrap_or_default().as_ptr())
    }
}

///
///  Set the name of the server, as displayed in the server browser.
///
///  [`name`] The new name.
///
///  Returns void
///
pub fn set_hostname(name: &str) {
    unsafe {
        raw::rustSetHostname(CString::new(name).unwrap_or_default().as_ptr())
    }
}

///
///  Set the password required to join the server.
///
///  [`password`] The password.
///
///  Returns void
///
pub fn set_server_password(password: &str) {
    unsafe {
        raw::rustSetServerPassword(CString::new(password).unwrap_or_default().as_ptr())
    }
}

///
///  Set the data file enforcement state of the server.
///
///  If true, clients are required to use the same data files as set for the server.
///
///  [`state`] The new enforcement state.
///
///  Returns void
///
pub fn set_data_file_enforcement_state(state: bool) {
    unsafe {
        raw::rustSetDataFileEnforcementState(state)
    }
}

///
///  Set whether script errors should be ignored or not.
///
///  If true, script errors will not crash the server, but could have any number
///  of unforeseen consequences, which is why this is a highly experimental
///  setting.
///
///  [`state`] The new script error ignoring state.
///
///  Returns void
///
pub fn set_script_error_ignoring_state(state: bool) {
    unsafe {
        raw::rustSetScriptErrorIgnoringState(state)
    }
}

///
///  Set a rule string for the server details displayed in the server browser.
///
///  [`key`] The name of the rule.
///  [`value`] The string value of the rule.
///
///  Returns void
///
pub fn set_rule_string(key: &str, value: &str) {
    unsafe {
        raw::rustSetRuleString(CString::new(key).unwrap_or_default().as_ptr(), CString::new(value).unwrap_or_default().as_ptr())
    }
}

///
///  Set a rule value for the server details displayed in the server browser.
///
///  [`key`] The name of the rule.
///  [`value`] The numerical value of the rule.
///
///  Returns void
///
pub fn set_rule_value(key: &str, value: f64) {
    unsafe {
        raw::rustSetRuleValue(CString::new(key).unwrap_or_default().as_ptr(), value)
    }
}

///
///  Add a data file and a corresponding CRC32 checksum to the data file loadout
///         that connecting clients need to match.
///
///  It can be used multiple times to set multiple checksums for the same data file.
///
///  Note: If an empty string is provided for the checksum, a checksum will not be
///        required for that data file.
///
///  @param dataFilename The filename of the data file.
///  @param checksumString A string with the CRC32 checksum required.
///
pub fn add_data_file_requirement(data_filename: &str, checksum_string: &str) {
    unsafe {
        raw::rustAddDataFileRequirement(CString::new(data_filename).unwrap_or_default().as_ptr(), CString::new(checksum_string).unwrap_or_default().as_ptr())
    }
}

pub fn does_file_exist(file_path: &str) -> bool {
    unsafe {
        raw::rustDoesFileExist(CString::new(file_path).unwrap_or_default().as_ptr())
    }
}

pub fn get_mod_dir() -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetModDir())
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

pub fn get_plugin_enforcement_state() -> bool {
    unsafe {
        raw::rustGetPluginEnforcementState()
    }
}

pub fn set_plugin_enforcement_state(state: bool) {
    unsafe {
        raw::rustSetPluginEnforcementState(state)
    }
}

pub fn add_plugin_hash(plugin_name: &str, checksum_string: &str) {
    unsafe {
        raw::rustAddPluginHash(CString::new(plugin_name).unwrap_or_default().as_ptr(), CString::new(checksum_string).unwrap_or_default().as_ptr())
    }
}

///
///  Set the difficulty for a player.
///
///  This changes the difficulty for that player in the server memory, but does not by itself
///  send a packet.
///
///  [`pid`] The player ID.
///  [`difficulty`] The difficulty.
///
///  Returns void
///
pub fn set_difficulty(pid: u16, difficulty: i16) {
    unsafe {
        raw::rustSetDifficulty(pid, difficulty)
    }
}

///
///  Set the client log level enforced for a player.
///
///  This changes the enforced log level for that player in the server memory, but does not by itself
///  send a packet.
///
///  Enforcing a certain log level is necessary to prevent players from learning information from
///  their console window that they are otherwise unable to obtain, such as the locations of
///  other players.
///
///  If you do not wish to enforce a log level, simply set enforcedLogLevel to -1
///
///  [`pid`] The player ID.
///  [`enforced_log_level`] The enforced log level.
///
///  Returns void
///
pub fn set_enforced_log_level(pid: u16, enforced_log_level: i16) {
    unsafe {
        raw::rustSetEnforcedLogLevel(pid, enforced_log_level)
    }
}

///
///  Set the physics framerate for a player.
///
///  This changes the physics framerate for that player in the server memory, but does not by itself
///  send a packet.
///
///  [`pid`] The player ID.
///  [`physics_framerate`] The physics framerate.
///
///  Returns void
///
pub fn set_physics_framerate(pid: u16, physics_framerate: f64) {
    unsafe {
        raw::rustSetPhysicsFramerate(pid, physics_framerate)
    }
}

///
///  Set whether the console is allowed for a player.
///
///  This changes the console permission for that player in the server memory, but does not
///  by itself send a packet.
///
///  [`pid`] The player ID.
///  [`state`] The console permission state.
///
///  Returns void
///
pub fn set_console_allowed(pid: u16, state: bool) {
    unsafe {
        raw::rustSetConsoleAllowed(pid, state)
    }
}

///
///  Set whether resting in beds is allowed for a player.
///
///  This changes the resting permission for that player in the server memory, but does not
///  by itself send a packet.
///
///  [`pid`] The player ID.
///  [`state`] The resting permission state.
///
///  Returns void
///
pub fn set_bed_rest_allowed(pid: u16, state: bool) {
    unsafe {
        raw::rustSetBedRestAllowed(pid, state)
    }
}

///
///  Set whether resting in the wilderness is allowed for a player.
///
///  This changes the resting permission for that player in the server memory, but does not
///  by itself send a packet.
///
///  [`pid`] The player ID.
///  [`state`] The resting permission state.
///
///  Returns void
///
pub fn set_wilderness_rest_allowed(pid: u16, state: bool) {
    unsafe {
        raw::rustSetWildernessRestAllowed(pid, state)
    }
}

///
///  Set whether waiting is allowed for a player.
///
///  This changes the waiting permission for that player in the server memory, but does not
///  by itself send a packet.
///
///  [`pid`] The player ID.
///  [`state`] The waiting permission state.
///
///  Returns void
///
pub fn set_wait_allowed(pid: u16, state: bool) {
    unsafe {
        raw::rustSetWaitAllowed(pid, state)
    }
}

///
///  Send a PlayerSettings packet to the player affected by it.
///
///  [`pid`] The player ID to send it to.
///
///  Returns void
///
pub fn send_settings(pid: u16) {
    unsafe {
        raw::rustSendSettings(pid)
    }
}

///
///  Clear the last recorded spellbook changes for a player.
///
///  This is used to initialize the sending of new PlayerSpellbook packets.
///
///  [`pid`] The player ID whose spellbook changes should be used.
///
///  Returns void
///
pub fn clear_spellbook_changes(pid: u16) {
    unsafe {
        raw::rustClearSpellbookChanges(pid)
    }
}

///
///  Get the number of indexes in a player's latest spellbook changes.
///
///  [`pid`] The player ID whose spellbook changes should be used.
///
///  Returns the number of indexes.
///
pub fn get_spellbook_changes_size(pid: u16) -> u16 {
    unsafe {
        raw::rustGetSpellbookChangesSize(pid)
    }
}

///
///  Get the action type used in a player's latest spellbook changes.
///
///  [`pid`] The player ID whose spellbook changes should be used.
///
///  Returns the action type (0 for SET, 1 for ADD, 2 for REMOVE).
///
pub fn get_spellbook_changes_action(pid: u16) -> u16 {
    unsafe {
        raw::rustGetSpellbookChangesAction(pid)
    }
}

///
///  Set the action type in a player's spellbook changes.
///
///  [`pid`] The player ID whose spellbook changes should be used.
///  [`action`] The action (0 for SET, 1 for ADD, 2 for REMOVE).
///
///  Returns void
///
pub fn set_spellbook_changes_action(pid: u16, action: u8) {
    unsafe {
        raw::rustSetSpellbookChangesAction(pid, action)
    }
}

///
///  Add a new spell to the spellbook changes for a player.
///
///  [`pid`] The player ID whose spellbook changes should be used.
///  [`spell_id`] The spellId of the spell.
///
///  Returns void
///
pub fn add_spell(pid: u16, spell_id: &str) {
    unsafe {
        raw::rustAddSpell(pid, CString::new(spell_id).unwrap_or_default().as_ptr())
    }
}

///
///  Get the spellId at a certain index in a player's latest spellbook changes.
///
///  [`pid`] The player ID whose spellbook changes should be used.
///  [`index`] The index of the spell.
///
///  Returns the spellId.
///
pub fn get_spell_id(pid: u16, index: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetSpellId(pid, index))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Send a PlayerSpellbook packet with a player's recorded spellbook changes.
///
///  [`pid`] The player ID whose spellbook changes should be used.
///  [`send_to_other_players`] Whether this packet should be sent to players other than the
///                            player attached to the packet (false by default).
///  [`skip_attached_player`] Whether the packet should skip being sent to the player attached
///                            to the packet (false by default).
///
///  Returns void
///
pub fn send_spellbook_changes(pid: u16, send_to_other_players: bool, skip_attached_player: bool) {
    unsafe {
        raw::rustSendSpellbookChanges(pid, send_to_other_players, skip_attached_player)
    }
}

pub fn initialize_spellbook_changes(pid: u16) {
    unsafe {
        raw::rustInitializeSpellbookChanges(pid)
    }
}

///
///  Get the number of attributes.
///
///  The number is 8 before any dehardcoding is done in OpenMW.
///
///
///  Returns the number of attributes.
///
pub fn get_attribute_count() -> i16 {
    unsafe {
        raw::rustGetAttributeCount()
    }
}

///
///  Get the number of skills.
///
///  The number is 27 before any dehardcoding is done in OpenMW.
///
///
///  Returns the number of skills.
///
pub fn get_skill_count() -> i16 {
    unsafe {
        raw::rustGetSkillCount()
    }
}

///
///  Get the numerical ID of an attribute with a certain name.
///
///  If an invalid name is used, the ID returned is -1
///
///  [`name`] The name of the attribute.
///
///  Returns the ID of the attribute.
///
pub fn get_attribute_id(name: &str) -> i16 {
    unsafe {
        raw::rustGetAttributeId(CString::new(name).unwrap_or_default().as_ptr())
    }
}

///
///  Get the numerical ID of a skill with a certain name.
///
///  If an invalid name is used, the ID returned is -1
///
///  [`name`] The name of the skill.
///
///  Returns the ID of the skill.
///
pub fn get_skill_id(name: &str) -> i16 {
    unsafe {
        raw::rustGetSkillId(CString::new(name).unwrap_or_default().as_ptr())
    }
}

///
///  Get the name of the attribute with a certain numerical ID.
///
///  If an invalid ID is used, "invalid" is returned.
///
///  [`attribute_id`] The ID of the attribute.
///
///  Returns the name of the attribute.
///
pub fn get_attribute_name(attribute_id: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetAttributeName(attribute_id))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the name of the skill with a certain numerical ID.
///
///  If an invalid ID is used, "invalid" is returned.
///
///  [`skill_id`] The ID of the skill.
///
///  Returns the name of the skill.
///
pub fn get_skill_name(skill_id: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetSkillName(skill_id))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the name of a player.
///
///  [`pid`] The player ID.
///
///  Returns the name of the player.
///
pub fn get_name(pid: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetName(pid))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the race of a player.
///
///  [`pid`] The player ID.
///
///  Returns the race of the player.
///
pub fn get_race(pid: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetRace(pid))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the head mesh used by a player.
///
///  [`pid`] The player ID.
///
///  Returns the head mesh of the player.
///
pub fn get_head(pid: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetHead(pid))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the hairstyle mesh used by a player.
///
///  [`pid`] The player ID.
///
///  Returns the hairstyle mesh of the player.
///
pub fn get_hair(pid: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetHair(pid))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Check whether a player is male or not.
///
///  [`pid`] The player ID.
///
///  Returns whether the player is male.
///
pub fn get_is_male(pid: u16) -> i16 {
    unsafe {
        raw::rustGetIsMale(pid)
    }
}

///
///  Get the birthsign of a player.
///
///  [`pid`] The player ID.
///
///  Returns the birthsign of the player.
///
pub fn get_birthsign(pid: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetBirthsign(pid))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the character level of a player.
///
///  [`pid`] The player ID.
///
///  Returns the level of the player.
///
pub fn get_level(pid: u16) -> i16 {
    unsafe {
        raw::rustGetLevel(pid)
    }
}

///
///  Get the player's progress to their next character level.
///
///  [`pid`] The player ID.
///
///  Returns the level progress.
///
pub fn get_level_progress(pid: u16) -> i16 {
    unsafe {
        raw::rustGetLevelProgress(pid)
    }
}

///
///  Get the base health of the player.
///
///  [`pid`] The player ID.
///
///  Returns the base health.
///
pub fn get_health_base(pid: u16) -> f64 {
    unsafe {
        raw::rustGetHealthBase(pid)
    }
}

///
///  Get the current health of the player.
///
///  [`pid`] The player ID.
///
///  Returns the current health.
///
pub fn get_health_current(pid: u16) -> f64 {
    unsafe {
        raw::rustGetHealthCurrent(pid)
    }
}

///
///  Get the base magicka of the player.
///
///  [`pid`] The player ID.
///
///  Returns the base magicka.
///
pub fn get_magicka_base(pid: u16) -> f64 {
    unsafe {
        raw::rustGetMagickaBase(pid)
    }
}

///
///  Get the current magicka of the player.
///
///  [`pid`] The player ID.
///
///  Returns the current magicka.
///
pub fn get_magicka_current(pid: u16) -> f64 {
    unsafe {
        raw::rustGetMagickaCurrent(pid)
    }
}

///
///  Get the base fatigue of the player.
///
///  [`pid`] The player ID.
///
///  Returns the base fatigue.
///
pub fn get_fatigue_base(pid: u16) -> f64 {
    unsafe {
        raw::rustGetFatigueBase(pid)
    }
}

///
///  Get the current fatigue of the player.
///
///  [`pid`] The player ID.
///
///  Returns the current fatigue.
///
pub fn get_fatigue_current(pid: u16) -> f64 {
    unsafe {
        raw::rustGetFatigueCurrent(pid)
    }
}

///
///  Get the base value of a player's attribute.
///
///  [`pid`] The player ID.
///  [`attribute_id`] The attribute ID.
///
///  Returns the base value of the attribute.
///
pub fn get_attribute_base(pid: u16, attribute_id: u16) -> i16 {
    unsafe {
        raw::rustGetAttributeBase(pid, attribute_id)
    }
}

///
///  Get the modifier value of a player's attribute.
///
///  [`pid`] The player ID.
///  [`attribute_id`] The attribute ID.
///
///  Returns the modifier value of the attribute.
///
pub fn get_attribute_modifier(pid: u16, attribute_id: u16) -> i16 {
    unsafe {
        raw::rustGetAttributeModifier(pid, attribute_id)
    }
}

///
///  Get the amount of damage (as caused through the Damage Attribute effect)
///         to a player's attribute.
///
///  [`pid`] The player ID.
///  [`attribute_id`] The attribute ID.
///
///  Returns the amount of damage to the attribute.
///
pub fn get_attribute_damage(pid: u16, attribute_id: u16) -> f64 {
    unsafe {
        raw::rustGetAttributeDamage(pid, attribute_id)
    }
}

///
///  Get the base value of a player's skill.
///
///  [`pid`] The player ID.
///  [`skill_id`] The skill ID.
///
///  Returns the base value of the skill.
///
pub fn get_skill_base(pid: u16, skill_id: u16) -> i16 {
    unsafe {
        raw::rustGetSkillBase(pid, skill_id)
    }
}

///
///  Get the modifier value of a player's skill.
///
///  [`pid`] The player ID.
///  [`skill_id`] The skill ID.
///
///  Returns the modifier value of the skill.
///
pub fn get_skill_modifier(pid: u16, skill_id: u16) -> i16 {
    unsafe {
        raw::rustGetSkillModifier(pid, skill_id)
    }
}

///
///  Get the amount of damage (as caused through the Damage Skill effect)
///         to a player's skill.
///
///  [`pid`] The player ID.
///  [`skill_id`] The skill ID.
///
///  Returns the amount of damage to the skill.
///
pub fn get_skill_damage(pid: u16, skill_id: u16) -> f64 {
    unsafe {
        raw::rustGetSkillDamage(pid, skill_id)
    }
}

///
///  Get the progress the player has made towards increasing a certain skill by 1.
///
///  [`pid`] The player ID.
///  [`skill_id`] The skill ID.
///
///  Returns the skill progress.
///
pub fn get_skill_progress(pid: u16, skill_id: u16) -> f64 {
    unsafe {
        raw::rustGetSkillProgress(pid, skill_id)
    }
}

///
///  Get the bonus applied to a certain attribute at the next level up as a result
///         of associated skill increases.
///
///  Although confusing, the term "skill increase" for this is taken from OpenMW itself.
///
///  [`pid`] The player ID.
///  [`skill_id`] The attribute ID.
///
///  Returns the increase in the attribute caused by skills.
///
pub fn get_skill_increase(pid: u16, attribute_id: u16) -> i16 {
    unsafe {
        raw::rustGetSkillIncrease(pid, attribute_id)
    }
}

///
///  Get the bounty of the player.
///
///  [`pid`] The player ID.
///
///  Returns the bounty.
///
pub fn get_bounty(pid: u16) -> i16 {
    unsafe {
        raw::rustGetBounty(pid)
    }
}

///
///  Set the name of a player.
///
///  [`pid`] The player ID.
///  [`name`] The new name of the player.
///
///  Returns void
///
pub fn set_name(pid: u16, name: &str) {
    unsafe {
        raw::rustSetName(pid, CString::new(name).unwrap_or_default().as_ptr())
    }
}

///
///  Set the race of a player.
///
///  [`pid`] The player ID.
///  [`race`] The new race of the player.
///
///  Returns void
///
pub fn set_race(pid: u16, race: &str) {
    unsafe {
        raw::rustSetRace(pid, CString::new(race).unwrap_or_default().as_ptr())
    }
}

///
///  Set the head mesh used by a player.
///
///  [`pid`] The player ID.
///  [`head`] The new head mesh of the player.
///
///  Returns void
///
pub fn set_head(pid: u16, head: &str) {
    unsafe {
        raw::rustSetHead(pid, CString::new(head).unwrap_or_default().as_ptr())
    }
}

///
///  Set the hairstyle mesh used by a player.
///
///  [`pid`] The player ID.
///  [`hairstyle`] The new hairstyle mesh of the player.
///
///  Returns void
///
pub fn set_hair(pid: u16, hairstyle: &str) {
    unsafe {
        raw::rustSetHair(pid, CString::new(hairstyle).unwrap_or_default().as_ptr())
    }
}

///
///  Set whether a player is male or not.
///
///  [`pid`] The player ID.
///  [`state`] Whether the player is male.
///
///  Returns void
///
pub fn set_is_male(pid: u16, state: i16) {
    unsafe {
        raw::rustSetIsMale(pid, state)
    }
}

///
///  Set the birthsign of a player.
///
///  [`pid`] The player ID.
///  [`name`] The new birthsign of the player.
///
///  Returns void
///
pub fn set_birthsign(pid: u16, name: &str) {
    unsafe {
        raw::rustSetBirthsign(pid, CString::new(name).unwrap_or_default().as_ptr())
    }
}

///
///  Set whether the player's stats should be reset based on their
///         current race as the result of a PlayerBaseInfo packet.
///
///  This changes the resetState for that player in the server memory, but does not by itself
///  send a packet.
///
///  [`pid`] The player ID.
///  [`reset_stats`] The stat reset state.
///
///  Returns void
///
pub fn set_reset_stats(pid: u16, reset_stats: bool) {
    unsafe {
        raw::rustSetResetStats(pid, reset_stats)
    }
}

///
///  Set the character level of a player.
///
///  [`pid`] The player ID.
///  [`value`] The new level of the player.
///
///  Returns void
///
pub fn set_level(pid: u16, value: i16) {
    unsafe {
        raw::rustSetLevel(pid, value)
    }
}

///
///  Set the player's progress to their next character level.
///
///  [`pid`] The player ID.
///  [`value`] The new level progress of the player.
///
///  Returns void
///
pub fn set_level_progress(pid: u16, value: i16) {
    unsafe {
        raw::rustSetLevelProgress(pid, value)
    }
}

///
///  Set the base health of a player.
///
///  [`pid`] The player ID.
///  [`name`] The new base health of the player.
///
///  Returns void
///
pub fn set_health_base(pid: u16, value: f64) {
    unsafe {
        raw::rustSetHealthBase(pid, value)
    }
}

///
///  Set the current health of a player.
///
///  [`pid`] The player ID.
///  [`name`] The new current health of the player.
///
///  Returns void
///
pub fn set_health_current(pid: u16, value: f64) {
    unsafe {
        raw::rustSetHealthCurrent(pid, value)
    }
}

///
///  Set the base magicka of a player.
///
///  [`pid`] The player ID.
///  [`name`] The new base magicka of the player.
///
///  Returns void
///
pub fn set_magicka_base(pid: u16, value: f64) {
    unsafe {
        raw::rustSetMagickaBase(pid, value)
    }
}

///
///  Set the current magicka of a player.
///
///  [`pid`] The player ID.
///  [`name`] The new current magicka of the player.
///
///  Returns void
///
pub fn set_magicka_current(pid: u16, value: f64) {
    unsafe {
        raw::rustSetMagickaCurrent(pid, value)
    }
}

///
///  Set the base fatigue of a player.
///
///  [`pid`] The player ID.
///  [`name`] The new base fatigue of the player.
///
///  Returns void
///
pub fn set_fatigue_base(pid: u16, value: f64) {
    unsafe {
        raw::rustSetFatigueBase(pid, value)
    }
}

///
///  Set the current fatigue of a player.
///
///  [`pid`] The player ID.
///  [`name`] The new current fatigue of the player.
///
///  Returns void
///
pub fn set_fatigue_current(pid: u16, value: f64) {
    unsafe {
        raw::rustSetFatigueCurrent(pid, value)
    }
}

///
///  Set the base value of a player's attribute.
///
///  [`pid`] The player ID.
///  [`attribute_id`] The attribute ID.
///  [`value`] The new base value of the player's attribute.
///
///  Returns void
///
pub fn set_attribute_base(pid: u16, attribute_id: u16, value: i16) {
    unsafe {
        raw::rustSetAttributeBase(pid, attribute_id, value)
    }
}

///
///  Clear the modifier value of a player's attribute.
///
///  There's no way to set a modifier to a specific value because it can come from
///  multiple different sources, but clearing it is a straightforward process that
///  dispels associated effects on a client and, if necessary, unequips associated
///  items.
///
///  [`pid`] The player ID.
///  [`attribute_id`] The attribute ID.
///
///  Returns void
///
pub fn clear_attribute_modifier(pid: u16, attribute_id: u16) {
    unsafe {
        raw::rustClearAttributeModifier(pid, attribute_id)
    }
}

///
///  Set the amount of damage (as caused through the Damage Attribute effect) to
///         a player's attribute.
///
///  [`pid`] The player ID.
///  [`attribute_id`] The attribute ID.
///  [`value`] The amount of damage to the player's attribute.
///
///  Returns void
///
pub fn set_attribute_damage(pid: u16, attribute_id: u16, value: f64) {
    unsafe {
        raw::rustSetAttributeDamage(pid, attribute_id, value)
    }
}

///
///  Set the base value of a player's skill.
///
///  [`pid`] The player ID.
///  [`skill_id`] The skill ID.
///  [`value`] The new base value of the player's skill.
///
///  Returns void
///
pub fn set_skill_base(pid: u16, skill_id: u16, value: i16) {
    unsafe {
        raw::rustSetSkillBase(pid, skill_id, value)
    }
}

///
///  Clear the modifier value of a player's skill.
///
///  There's no way to set a modifier to a specific value because it can come from
///  multiple different sources, but clearing it is a straightforward process that
///  dispels associated effects on a client and, if necessary, unequips associated
///  items.
///
///  [`pid`] The player ID.
///  [`skill_id`] The skill ID.
///
///  Returns void
///
pub fn clear_skill_modifier(pid: u16, skill_id: u16) {
    unsafe {
        raw::rustClearSkillModifier(pid, skill_id)
    }
}

///
///  Set the amount of damage (as caused through the Damage Skill effect) to
///         a player's skill.
///
///  [`pid`] The player ID.
///  [`skill_id`] The skill ID.
///  [`value`] The amount of damage to the player's skill.
///
///  Returns void
///
pub fn set_skill_damage(pid: u16, skill_id: u16, value: f64) {
    unsafe {
        raw::rustSetSkillDamage(pid, skill_id, value)
    }
}

///
///  Set the progress the player has made towards increasing a certain skill by 1.
///
///  [`pid`] The player ID.
///  [`skill_id`] The skill ID.
///  [`value`] The progress value.
///
///  Returns void
///
pub fn set_skill_progress(pid: u16, skill_id: u16, value: f64) {
    unsafe {
        raw::rustSetSkillProgress(pid, skill_id, value)
    }
}

///
///  Set the bonus applied to a certain attribute at the next level up as a result
///         of associated skill increases.
///
///  Although confusing, the term "skill increase" for this is taken from OpenMW itself.
///
///  [`pid`] The player ID.
///  [`skill_id`] The attribute ID.
///  [`value`] The increase in the attribute caused by skills.
///
///  Returns void
///
pub fn set_skill_increase(pid: u16, attribute_id: u16, value: i16) {
    unsafe {
        raw::rustSetSkillIncrease(pid, attribute_id, value)
    }
}

///
///  Set the bounty of a player.
///
///  [`pid`] The player ID.
///  [`value`] The new bounty.
///
///  Returns void
///
pub fn set_bounty(pid: u16, value: i16) {
    unsafe {
        raw::rustSetBounty(pid, value)
    }
}

///
///  Set the current and ending stages of character generation for a player.
///
///  This is used to repeat part of character generation or to only go through part of it.
///
///  [`pid`] The player ID.
///  [`current_stage`] The new current stage.
///  [`end_stage`] The new ending stage.
///
///  Returns void
///
pub fn set_char_gen_stage(pid: u16, current_stage: i16, end_stage: i16) {
    unsafe {
        raw::rustSetCharGenStage(pid, current_stage, end_stage)
    }
}

///
///  Send a PlayerBaseInfo packet with a player's name, race, head mesh,
///         hairstyle mesh, birthsign and stat reset state.
///
///  It is always sent to all players.
///
///  [`pid`] The player ID.
///
///  Returns void
///
pub fn send_base_info(pid: u16) {
    unsafe {
        raw::rustSendBaseInfo(pid)
    }
}

///
///  Send a PlayerStatsDynamic packet with a player's dynamic stats (health,
///         magicka and fatigue).
///
///  It is always sent to all players.
///
///  [`pid`] The player ID.
///
///  Returns void
///
pub fn send_stats_dynamic(pid: u16) {
    unsafe {
        raw::rustSendStatsDynamic(pid)
    }
}

///
///  Send a PlayerAttribute packet with a player's attributes and bonuses
///         to those attributes at the next level up (the latter being called
///         "skill increases" as in OpenMW).
///
///  It is always sent to all players.
///
///  [`pid`] The player ID.
///
///  Returns void
///
pub fn send_attributes(pid: u16) {
    unsafe {
        raw::rustSendAttributes(pid)
    }
}

///
///  Send a PlayerSkill packet with a player's skills.
///
///  It is always sent to all players.
///
///  [`pid`] The player ID.
///
///  Returns void
///
pub fn send_skills(pid: u16) {
    unsafe {
        raw::rustSendSkills(pid)
    }
}

///
///  Send a PlayerLevel packet with a player's character level and
///         progress towards the next level up
///
///  It is always sent to all players.
///
///  [`pid`] The player ID.
///
///  Returns void
///
pub fn send_level(pid: u16) {
    unsafe {
        raw::rustSendLevel(pid)
    }
}

///
///  Send a PlayerBounty packet with a player's bounty.
///
///  It is always sent to all players.
///
///  [`pid`] The player ID.
///
///  Returns void
///
pub fn send_bounty(pid: u16) {
    unsafe {
        raw::rustSendBounty(pid)
    }
}

///
///  Use the last object list received by the server as the one being read.
///
///
///  Returns void
///
pub fn read_received_object_list() {
    unsafe {
        raw::rustReadReceivedObjectList()
    }
}

///
///  Clear the data from the object list stored on the server.
///
///
///  Returns void
///
pub fn clear_object_list() {
    unsafe {
        raw::rustClearObjectList()
    }
}

///
///  Set the pid attached to the ObjectList.
///
///  [`pid`] The player ID to whom the object list should be attached.
///
///  Returns void
///
pub fn set_object_list_pid(pid: u16) {
    unsafe {
        raw::rustSetObjectListPid(pid)
    }
}

///
///  Take the contents of the read-only object list last received by the
///         server from a player and move its contents to the stored object list
///         that can be sent by the server.
///
///
///  Returns void
///
pub fn copy_received_object_list_to_store() {
    unsafe {
        raw::rustCopyReceivedObjectListToStore()
    }
}

///
///  Get the number of indexes in the read object list.
///
///
///  Returns the number of indexes.
///
pub fn get_object_list_size() -> u16 {
    unsafe {
        raw::rustGetObjectListSize()
    }
}

///
///  Get the origin of the read object list.
///
///
///  Returns the origin (0 for CLIENT_GAMEPLAY, 1 for CLIENT_CONSOLE, 2 for
///  CLIENT_DIALOGUE, 3 for CLIENT_SCRIPT_LOCAL, 4 for CLIENT_SCRIPT_GLOBAL,
///  5 for SERVER_SCRIPT).
///
pub fn get_object_list_origin() -> u8 {
    unsafe {
        raw::rustGetObjectListOrigin()
    }
}

///
///  Get the client script that the read object list originated from.
///
///  Note: This is not yet implemented.
///
///
///  Returns the ID of the client script.
///
pub fn get_object_list_client_script() -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetObjectListClientScript())
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the action type used in the read object list.
///
///
///  Returns the action type (0 for SET, 1 for ADD, 2 for REMOVE, 3 for REQUEST).
///
pub fn get_object_list_action() -> u8 {
    unsafe {
        raw::rustGetObjectListAction()
    }
}

///
///  Get the container subaction type used in the read object list.
///
///
///  Returns the action type (0 for NONE, 1 for DRAG, 2 for DROP, 3 for TAKE_ALL).
///
pub fn get_object_list_container_sub_action() -> u8 {
    unsafe {
        raw::rustGetObjectListContainerSubAction()
    }
}

///
///  Check whether the object at a certain index in the read object list is a
///  player.
///
///  Note: Although most player data and events are dealt with in Player packets,
///        object activation is general enough for players themselves to be included
///        as objects in ObjectActivate packets.
///
///  [`index`] The index of the object.
///
///  Returns whether the object is a player.
///
pub fn is_object_player(index: u16) -> bool {
    unsafe {
        raw::rustIsObjectPlayer(index)
    }
}

///
///  Get the player ID of the object at a certain index in the read object list,
///  only valid if the object is a player.
///
///  Note: Currently, players can only be objects in ObjectActivate and ConsoleCommand
///        packets.
///
///  [`index`] The index of the object.
///
///  Returns the player ID of the object.
///
pub fn get_object_pid(index: u16) -> i16 {
    unsafe {
        raw::rustGetObjectPid(index)
    }
}

///
///  Get the refId of the object at a certain index in the read object list.
///
///  [`index`] The index of the object.
///
///  Returns the refId.
///
pub fn get_object_ref_id(index: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetObjectRefId(index))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the refNum of the object at a certain index in the read object list.
///
///  [`index`] The index of the object.
///
///  Returns the refNum.
///
pub fn get_object_ref_num(index: u16) -> u16 {
    unsafe {
        raw::rustGetObjectRefNum(index)
    }
}

///
///  Get the mpNum of the object at a certain index in the read object list.
///
///  [`index`] The index of the object.
///
///  Returns the mpNum.
///
pub fn get_object_mp_num(index: u16) -> u16 {
    unsafe {
        raw::rustGetObjectMpNum(index)
    }
}

///
///  Get the count of the object at a certain index in the read object list.
///
///  [`index`] The index of the object.
///
///  Returns the object count.
///
pub fn get_object_count(index: u16) -> i16 {
    unsafe {
        raw::rustGetObjectCount(index)
    }
}

///
///  Get the charge of the object at a certain index in the read object list.
///
///  [`index`] The index of the object.
///
///  Returns the charge.
///
pub fn get_object_charge(index: u16) -> i16 {
    unsafe {
        raw::rustGetObjectCharge(index)
    }
}

///
///  Get the enchantment charge of the object at a certain index in the read object list.
///
///  [`index`] The index of the object.
///
///  Returns the enchantment charge.
///
pub fn get_object_enchantment_charge(index: u16) -> f64 {
    unsafe {
        raw::rustGetObjectEnchantmentCharge(index)
    }
}

///
///  Get the soul of the object at a certain index in the read object list.
///
///  [`index`] The index of the object.
///
///  Returns the soul.
///
pub fn get_object_soul(index: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetObjectSoul(index))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the gold value of the object at a certain index in the read object list.
///
///  This is used solely to get the gold value of gold. It is not used for other objects.
///
///  [`index`] The index of the object.
///
///  Returns the gold value.
///
pub fn get_object_gold_value(index: u16) -> i16 {
    unsafe {
        raw::rustGetObjectGoldValue(index)
    }
}

///
///  Get the object scale of the object at a certain index in the read object list.
///
///  [`index`] The index of the object.
///
///  Returns the object scale.
///
pub fn get_object_scale(index: u16) -> f64 {
    unsafe {
        raw::rustGetObjectScale(index)
    }
}

///
///  Get the object state of the object at a certain index in the read object list.
///
///  [`index`] The index of the object.
///
///  Returns the object state.
///
pub fn get_object_state(index: u16) -> bool {
    unsafe {
        raw::rustGetObjectState(index)
    }
}

///
///  Get the door state of the object at a certain index in the read object list.
///
///  [`index`] The index of the object.
///
///  Returns the door state.
///
pub fn get_object_door_state(index: u16) -> i16 {
    unsafe {
        raw::rustGetObjectDoorState(index)
    }
}

///
///  Get the lock level of the object at a certain index in the read object list.
///
///  [`index`] The index of the object.
///
///  Returns the lock level.
///
pub fn get_object_lock_level(index: u16) -> i16 {
    unsafe {
        raw::rustGetObjectLockLevel(index)
    }
}

///
///  Check whether the object at a certain index in the read object list has been
///  activated by a player.
///
///  [`index`] The index of the object.
///
///  Returns whether the object has been activated by a player.
///
pub fn does_object_have_player_activating(index: u16) -> bool {
    unsafe {
        raw::rustDoesObjectHavePlayerActivating(index)
    }
}

///
///  Get the player ID of the player activating the object at a certain index in the
///  read object list.
///
///  [`index`] The index of the object.
///
///  Returns the player ID of the activating player.
///
pub fn get_object_activating_pid(index: u16) -> i16 {
    unsafe {
        raw::rustGetObjectActivatingPid(index)
    }
}

///
///  Get the refId of the actor activating the object at a certain index in the read
///  object list.
///
///  [`index`] The index of the object.
///
///  Returns the refId of the activating actor.
///
pub fn get_object_activating_ref_id(index: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetObjectActivatingRefId(index))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the refNum of the actor activating the object at a certain index in the read
///  object list.
///
///  [`index`] The index of the object.
///
///  Returns the refNum of the activating actor.
///
pub fn get_object_activating_ref_num(index: u16) -> u16 {
    unsafe {
        raw::rustGetObjectActivatingRefNum(index)
    }
}

///
///  Get the mpNum of the actor activating the object at a certain index in the read
///  object list.
///
///  [`index`] The index of the object.
///
///  Returns the mpNum of the activating actor.
///
pub fn get_object_activating_mp_num(index: u16) -> u16 {
    unsafe {
        raw::rustGetObjectActivatingMpNum(index)
    }
}

///
///  Get the name of the actor activating the object at a certain index in the read
///  object list.
///
///  [`index`] The index of the object.
///
///  Returns the name of the activating actor.
///
pub fn get_object_activating_name(index: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetObjectActivatingName(index))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Check whether the object at a certain index in the read object list is a
///  summon.
///
///  Only living actors can be summoned.
///
///
///  Returns the summon state.
///
pub fn get_object_summon_state(index: u16) -> bool {
    unsafe {
        raw::rustGetObjectSummonState(index)
    }
}

///
///  Get the summon duration of the object at a certain index in the read object list.
///
///  Note: Returns -1 if indefinite.
///
///  [`index`] The index of the object.
///
///  Returns the summon duration.
///
pub fn get_object_summon_duration(index: u16) -> f64 {
    unsafe {
        raw::rustGetObjectSummonDuration(index)
    }
}

///
///  Check whether the object at a certain index in the read object list has a player
///  as its summoner.
///
///  Only living actors can be summoned.
///
///  [`index`] The index of the object.
///
///  Returns whether a player is the summoner of the object.
///
pub fn does_object_have_player_summoner(index: u16) -> bool {
    unsafe {
        raw::rustDoesObjectHavePlayerSummoner(index)
    }
}

///
///  Get the player ID of the summoner of the object at a certain index in the read object
///  list.
///
///  [`index`] The index of the object.
///
///  Returns the player ID of the summoner.
///
pub fn get_object_summoner_pid(index: u16) -> i16 {
    unsafe {
        raw::rustGetObjectSummonerPid(index)
    }
}

///
///  Get the refId of the actor summoner of the object at a certain index in the read object
///  list.
///
///  [`index`] The index of the object.
///
///  Returns the refId of the summoner.
///
pub fn get_object_summoner_ref_id(index: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetObjectSummonerRefId(index))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the refNum of the actor summoner of the object at a certain index in the read object
///  list.
///
///  [`index`] The index of the object.
///
///  Returns the refNum of the summoner.
///
pub fn get_object_summoner_ref_num(index: u16) -> u16 {
    unsafe {
        raw::rustGetObjectSummonerRefNum(index)
    }
}

///
///  Get the mpNum of the actor summoner of the object at a certain index in the read object list.
///
///  [`index`] The index of the object.
///
///  Returns the mpNum of the summoner.
///
pub fn get_object_summoner_mp_num(index: u16) -> u16 {
    unsafe {
        raw::rustGetObjectSummonerMpNum(index)
    }
}

///
///  Get the X position of the object at a certain index in the read object list.
///
///  [`index`] The index of the object.
///
///  Returns the X position.
///
pub fn get_object_pos_x(index: u16) -> f64 {
    unsafe {
        raw::rustGetObjectPosX(index)
    }
}

///
///  Get the Y position of the object at a certain index in the read object list.
///
///  [`index`] The index of the object.
///
///  Returns the Y position.
///
pub fn get_object_pos_y(index: u16) -> f64 {
    unsafe {
        raw::rustGetObjectPosY(index)
    }
}

///
///  Get the Z position at a certain index in the read object list.
///
///  [`index`] The index of the object.
///
///  Returns the Z position.
///
pub fn get_object_pos_z(index: u16) -> f64 {
    unsafe {
        raw::rustGetObjectPosZ(index)
    }
}

///
///  Get the X rotation of the object at a certain index in the read object list.
///
///  [`index`] The index of the object.
///
///  Returns the X rotation.
///
pub fn get_object_rot_x(index: u16) -> f64 {
    unsafe {
        raw::rustGetObjectRotX(index)
    }
}

///
///  Get the Y rotation of the object at a certain index in the read object list.
///
///  [`index`] The index of the object.
///
///  Returns the Y rotation.
///
pub fn get_object_rot_y(index: u16) -> f64 {
    unsafe {
        raw::rustGetObjectRotY(index)
    }
}

///
///  Get the Z rotation of the object at a certain index in the read object list.
///
///  [`index`] The index of the object.
///
///  Returns the Z rotation.
///
pub fn get_object_rot_z(index: u16) -> f64 {
    unsafe {
        raw::rustGetObjectRotZ(index)
    }
}

///
///  Get the videoFilename of the object at a certain index in the read object list.
///
///
///  Returns the videoFilename.
///
pub fn get_video_filename(index: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetVideoFilename(index))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

pub fn get_script_variable_name(index: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetScriptVariableName(index))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

pub fn get_script_variable_short_value(index: u16) -> i16 {
    unsafe {
        raw::rustGetScriptVariableShortValue(index)
    }
}

///
///  Get the number of container item indexes of the object at a certain index in the
///  read object list.
///
///  [`index`] The index of the object.
///
///  Returns the number of container item indexes.
///
pub fn get_container_changes_size(object_index: u16) -> u16 {
    unsafe {
        raw::rustGetContainerChangesSize(object_index)
    }
}

///
///  Get the refId of the container item at a certain itemIndex in the container changes
///  of the object at a certain objectIndex in the read object list.
///
///  [`object_index`] The index of the object.
///  [`item_index`] The index of the container item.
///
///  Returns the refId.
///
pub fn get_container_item_ref_id(object_index: u16, item_index: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetContainerItemRefId(object_index, item_index))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the item count of the container item at a certain itemIndex in the container
///  changes of the object at a certain objectIndex in the read object list.
///
///  [`object_index`] The index of the object.
///  [`item_index`] The index of the container item.
///
///  Returns the item count.
///
pub fn get_container_item_count(object_index: u16, item_index: u16) -> i16 {
    unsafe {
        raw::rustGetContainerItemCount(object_index, item_index)
    }
}

///
///  Get the charge of the container item at a certain itemIndex in the container changes
///  of the object at a certain objectIndex in the read object list.
///
///  [`object_index`] The index of the object.
///  [`item_index`] The index of the container item.
///
///  Returns the charge.
///
pub fn get_container_item_charge(object_index: u16, item_index: u16) -> i16 {
    unsafe {
        raw::rustGetContainerItemCharge(object_index, item_index)
    }
}

///
///  Get the enchantment charge of the container item at a certain itemIndex in the container changes
///  of the object at a certain objectIndex in the read object list.
///
///  [`object_index`] The index of the object.
///  [`item_index`] The index of the container item.
///
///  Returns the enchantment charge.
///
pub fn get_container_item_enchantment_charge(object_index: u16, item_index: u16) -> f64 {
    unsafe {
        raw::rustGetContainerItemEnchantmentCharge(object_index, item_index)
    }
}

///
///  Get the soul of the container item at a certain itemIndex in the container changes
///  of the object at a certain objectIndex in the read object list.
///
///  [`object_index`] The index of the object.
///  [`item_index`] The index of the container item.
///
///  Returns the soul.
///
pub fn get_container_item_soul(object_index: u16, item_index: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetContainerItemSoul(object_index, item_index))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the action count of the container item at a certain itemIndex in the container
///  changes of the object at a certain objectIndex in the read object list.
///
///  [`object_index`] The index of the object.
///  [`item_index`] The index of the container item.
///
///  Returns the action count.
///
pub fn get_container_item_action_count(object_index: u16, item_index: u16) -> i16 {
    unsafe {
        raw::rustGetContainerItemActionCount(object_index, item_index)
    }
}

///
///  Check whether the object at a certain index in the read object list has a container.
///
///  Note: Only ObjectLists from ObjectPlace packets contain this information. Objects from
///        received ObjectSpawn packets can always be assumed to have a container.
///
///  [`index`] The index of the object.
///
///  Returns whether the object has a container.
///
pub fn does_object_have_container(index: u16) -> bool {
    unsafe {
        raw::rustDoesObjectHaveContainer(index)
    }
}

///
///  Set the cell of the temporary object list stored on the server.
///
///  The cell is determined to be an exterior cell if it fits the pattern of a number followed
///  by a comma followed by another number.
///
///  [`cell_description`] The description of the cell.
///
///  Returns void
///
pub fn set_object_list_cell(cell_description: &str) {
    unsafe {
        raw::rustSetObjectListCell(CString::new(cell_description).unwrap_or_default().as_ptr())
    }
}

///
///  Set the action type of the temporary object list stored on the server.
///
///  [`action`] The action type (0 for SET, 1 for ADD, 2 for REMOVE, 3 for REQUEST).
///
///  Returns void
///
pub fn set_object_list_action(action: u8) {
    unsafe {
        raw::rustSetObjectListAction(action)
    }
}

///
///  Set the console command of the temporary object list stored on the server.
///
///  When sent, the command will run once on every object added to the object list. If no objects
///  have been added, it will run once without any object reference.
///
///  [`console_command`] The console command.
///
///  Returns void
///
pub fn set_object_list_console_command(console_command: &str) {
    unsafe {
        raw::rustSetObjectListConsoleCommand(CString::new(console_command).unwrap_or_default().as_ptr())
    }
}

///
///  Set the refId of the temporary object stored on the server.
///
///  [`ref_id`] The refId.
///
///  Returns void
///
pub fn set_object_ref_id(ref_id: &str) {
    unsafe {
        raw::rustSetObjectRefId(CString::new(ref_id).unwrap_or_default().as_ptr())
    }
}

///
///  Set the refNum of the temporary object stored on the server.
///
///  Every object loaded from .ESM and .ESP data files has a unique refNum which needs to be
///  retained to refer to it in packets.
///
///  On the other hand, objects placed or spawned via the server should always have a refNum
///  of 0.
///
///  [`ref_num`] The refNum.
///
///  Returns void
///
pub fn set_object_ref_num(ref_num: i16) {
    unsafe {
        raw::rustSetObjectRefNum(ref_num)
    }
}

///
///  Set the mpNum of the temporary object stored on the server.
///
///  Every object placed or spawned via the server is assigned an mpNum by incrementing the last
///  mpNum stored on the server. Scripts should take care to ensure that mpNums are kept unique
///  for these objects.
///
///  Objects loaded from .ESM and .ESP data files should always have an mpNum of 0, because they
///  have unique refNumes instead.
///
///  [`mp_num`] The mpNum.
///
///  Returns void
///
pub fn set_object_mp_num(mp_num: i16) {
    unsafe {
        raw::rustSetObjectMpNum(mp_num)
    }
}

///
///  Set the object count of the temporary object stored on the server.
///
///  This determines the quantity of an object, with the exception of gold.
///
///  [`count`] The object count.
///
///  Returns void
///
pub fn set_object_count(count: i16) {
    unsafe {
        raw::rustSetObjectCount(count)
    }
}

///
///  Set the charge of the temporary object stored on the server.
///
///  Object durabilities are set through this value.
///
///  [`charge`] The charge.
///
///  Returns void
///
pub fn set_object_charge(charge: i16) {
    unsafe {
        raw::rustSetObjectCharge(charge)
    }
}

///
///  Set the enchantment charge of the temporary object stored on the server.
///
///  Object durabilities are set through this value.
///
///  [`charge`] The enchantment charge.
///
///  Returns void
///
pub fn set_object_enchantment_charge(enchantment_charge: f64) {
    unsafe {
        raw::rustSetObjectEnchantmentCharge(enchantment_charge)
    }
}

///
///  Set the soul of the temporary object stored on the server.
///
///  [`ref_id`] The soul.
///
///  Returns void
///
pub fn set_object_soul(soul: &str) {
    unsafe {
        raw::rustSetObjectSoul(CString::new(soul).unwrap_or_default().as_ptr())
    }
}

///
///  Set the gold value of the temporary object stored on the server.
///
///  This is used solely to set the gold value for gold. It has no effect on other objects.
///
///  [`gold_value`] The gold value.
///
///  Returns void
///
pub fn set_object_gold_value(gold_value: i16) {
    unsafe {
        raw::rustSetObjectGoldValue(gold_value)
    }
}

///
///  Set the scale of the temporary object stored on the server.
///
///  Objects are smaller or larger than their default size based on their scale.
///
///  [`scale`] The scale.
///
///  Returns void
///
pub fn set_object_scale(scale: f64) {
    unsafe {
        raw::rustSetObjectScale(scale)
    }
}

///
///  Set the object state of the temporary object stored on the server.
///
///  Objects are enabled or disabled based on their object state.
///
///  [`object_state`] The object state.
///
///  Returns void
///
pub fn set_object_state(object_state: bool) {
    unsafe {
        raw::rustSetObjectState(object_state)
    }
}

///
///  Set the lock level of the temporary object stored on the server.
///
///  [`lock_level`] The lock level.
///
///  Returns void
///
pub fn set_object_lock_level(lock_level: i16) {
    unsafe {
        raw::rustSetObjectLockLevel(lock_level)
    }
}

///
///  Set the disarm state of the temporary object stored on the server.
///
///  [`disarm_state`] The disarmState.
///
///  Returns void
///
pub fn set_object_disarm_state(disarm_state: bool) {
    unsafe {
        raw::rustSetObjectDisarmState(disarm_state)
    }
}

///
///  Set the summon duration of the temporary object stored on the server.
///
///  [`summon_duration`] The summon duration.
///
///  Returns void
///
pub fn set_object_summon_duration(summon_duration: f32) {
    unsafe {
        raw::rustSetObjectSummonDuration(summon_duration)
    }
}

///
///  Set the summon state of the temporary object stored on the server.
///
///  This only affects living actors and determines whether they are summons of another
///  living actor.
///
///  [`summon_state`] The summon state.
///
///  Returns void
///
pub fn set_object_summon_state(summon_state: bool) {
    unsafe {
        raw::rustSetObjectSummonState(summon_state)
    }
}

///
///  Set the position of the temporary object stored on the server.
///
///  [`x`] The X position.
///  [`y`] The Y position.
///  [`z`] The Z position.
///
///  Returns void
///
pub fn set_object_position(x: f64, y: f64, z: f64) {
    unsafe {
        raw::rustSetObjectPosition(x, y, z)
    }
}

///
///  Set the rotation of the temporary object stored on the server.
///
///  [`x`] The X rotation.
///  [`y`] The Y rotation.
///  [`z`] The Z rotation.
///
///  Returns void
///
pub fn set_object_rotation(x: f64, y: f64, z: f64) {
    unsafe {
        raw::rustSetObjectRotation(x, y, z)
    }
}

///
///  Set the player ID of the player activating the temporary object stored on the
///         server. Currently only used for ObjectActivate packets.
///
///  [`pid`] The pid of the player.
///
///  Returns void
///
pub fn set_object_activating_pid(pid: u16) {
    unsafe {
        raw::rustSetObjectActivatingPid(pid)
    }
}

///
///  Set the door state of the temporary object stored on the server.
///
///  Doors are open or closed based on their door state.
///
///  [`door_state`] The door state.
///
///  Returns void
///
pub fn set_object_door_state(door_state: i16) {
    unsafe {
        raw::rustSetObjectDoorState(door_state)
    }
}

///
///  Set the teleport state of the temporary object stored on the server.
///
///  If a door's teleport state is true, interacting with the door teleports a player to its
///  destination. If it's false, it opens and closes like a regular door.
///
///  [`teleport_state`] The teleport state.
///
///  Returns void
///
pub fn set_object_door_teleport_state(teleport_state: bool) {
    unsafe {
        raw::rustSetObjectDoorTeleportState(teleport_state)
    }
}

///
///  Set the door destination cell of the temporary object stored on the server.
///
///  The cell is determined to be an exterior cell if it fits the pattern of a number followed
///  by a comma followed by another number.
///
///  [`cell_description`] The description of the cell.
///
///  Returns void
///
pub fn set_object_door_destination_cell(cell_description: &str) {
    unsafe {
        raw::rustSetObjectDoorDestinationCell(CString::new(cell_description).unwrap_or_default().as_ptr())
    }
}

///
///  Set the door destination position of the temporary object stored on the server.
///
///  [`x`] The X position.
///  [`y`] The Y position.
///  [`z`] The Z position.
///
///  Returns void
///
pub fn set_object_door_destination_position(x: f64, y: f64, z: f64) {
    unsafe {
        raw::rustSetObjectDoorDestinationPosition(x, y, z)
    }
}

///
///  Set the door destination rotation of the temporary object stored on the server.
///
///  Note: Because this sets the rotation a player will have upon using the door, and rotation
///        on the Y axis has no effect on players, the Y value has been omitted as an argument.
///
///  [`x`] The X rotation.
///  [`z`] The Z rotation.
///
///  Returns void
///
pub fn set_object_door_destination_rotation(x: f64, z: f64) {
    unsafe {
        raw::rustSetObjectDoorDestinationRotation(x, z)
    }
}

pub fn set_script_variable_name(var_name: &str) {
    unsafe {
        raw::rustSetScriptVariableName(CString::new(var_name).unwrap_or_default().as_ptr())
    }
}

pub fn set_script_variable_short_value(short_val: i16) {
    unsafe {
        raw::rustSetScriptVariableShortValue(short_val)
    }
}

///
///  Set a player as the object in the temporary object stored on the server.
///         Currently only used for ConsoleCommand packets.
///
///  [`pid`] The pid of the player.
///
///  Returns void
///
pub fn set_player_as_object(pid: u16) {
    unsafe {
        raw::rustSetPlayerAsObject(pid)
    }
}

///
///  Set the refId of the temporary container item stored on the server.
///
///  [`ref_id`] The refId.
///
///  Returns void
///
pub fn set_container_item_ref_id(ref_id: &str) {
    unsafe {
        raw::rustSetContainerItemRefId(CString::new(ref_id).unwrap_or_default().as_ptr())
    }
}

///
///  Set the item count of the temporary container item stored on the server.
///
///  [`count`] The item count.
///
///  Returns void
///
pub fn set_container_item_count(count: i16) {
    unsafe {
        raw::rustSetContainerItemCount(count)
    }
}

///
///  Set the charge of the temporary container item stored on the server.
///
///  [`charge`] The charge.
///
///  Returns void
///
pub fn set_container_item_charge(charge: i16) {
    unsafe {
        raw::rustSetContainerItemCharge(charge)
    }
}

///
///  Set the enchantment charge of the temporary container item stored on the server.
///
///  [`charge`] The enchantment charge.
///
///  Returns void
///
pub fn set_container_item_enchantment_charge(enchantment_charge: f64) {
    unsafe {
        raw::rustSetContainerItemEnchantmentCharge(enchantment_charge)
    }
}

///
///  Set the soul of the temporary container item stored on the server.
///
///  [`ref_id`] The soul.
///
///  Returns void
///
pub fn set_container_item_soul(soul: &str) {
    unsafe {
        raw::rustSetContainerItemSoul(CString::new(soul).unwrap_or_default().as_ptr())
    }
}

///
///  Set the action count of the container item at a certain itemIndex in the container
///  changes of the object at a certain objectIndex in the object list stored on the server.
///
///  When resending a received Container packet, this allows you to correct the amount of items
///  removed from a container by a player when it conflicts with what other players have already
///  taken.
///
///  [`object_index`] The index of the object.
///  [`item_index`] The index of the container item.
///  [`action_count`] The action count.
///
///  Returns void
///
pub fn set_container_item_action_count_by_index(object_index: u16, item_index: u16, action_count: i16) {
    unsafe {
        raw::rustSetContainerItemActionCountByIndex(object_index, item_index, action_count)
    }
}

///
///  Add a copy of the server's temporary object to the server's currently stored object
///  list.
///
///  In the process, the server's temporary object will automatically be cleared so a new
///  one can be set up.
///
///
///  Returns void
///
pub fn add_object() {
    unsafe {
        raw::rustAddObject()
    }
}

///
///  Add a copy of the server's temporary container item to the container changes of the
///  server's temporary object.
///
///  In the process, the server's temporary container item will automatically be cleared so a new
///  one can be set up.
///
///
///  Returns void
///
pub fn add_container_item() {
    unsafe {
        raw::rustAddContainerItem()
    }
}

///
///  Send an ObjectActivate packet.
///
///  [`send_to_other_players`] Whether this packet should be sent to players other than the
///                            player attached to the packet (false by default).
///  [`skip_attached_player`] Whether the packet should skip being sent to the player attached
///                            to the packet (false by default).
///
///  Returns void
///
pub fn send_object_activate(send_to_other_players: bool, skip_attached_player: bool) {
    unsafe {
        raw::rustSendObjectActivate(send_to_other_players, skip_attached_player)
    }
}

///
///  Send an ObjectPlace packet.
///
///  [`send_to_other_players`] Whether this packet should be sent to players other than the
///                            player attached to the packet (false by default).
///  [`skip_attached_player`] Whether the packet should skip being sent to the player attached
///                            to the packet (false by default).
///
///  Returns void
///
pub fn send_object_place(send_to_other_players: bool, skip_attached_player: bool) {
    unsafe {
        raw::rustSendObjectPlace(send_to_other_players, skip_attached_player)
    }
}

///
///  Send an ObjectSpawn packet.
///
///  [`send_to_other_players`] Whether this packet should be sent to players other than the
///                            player attached to the packet (false by default).
///  [`skip_attached_player`] Whether the packet should skip being sent to the player attached
///                            to the packet (false by default).
///
///  Returns void
///
pub fn send_object_spawn(send_to_other_players: bool, skip_attached_player: bool) {
    unsafe {
        raw::rustSendObjectSpawn(send_to_other_players, skip_attached_player)
    }
}

///
///  Send an ObjectDelete packet.
///
///  [`broadcast`] Whether this packet should be sent only to the player for whom the current
///                   object list was initialized or to everyone on the server.
///
///
///  Returns void
///
pub fn send_object_delete(send_to_other_players: bool, skip_attached_player: bool) {
    unsafe {
        raw::rustSendObjectDelete(send_to_other_players, skip_attached_player)
    }
}

///
///  Send an ObjectLock packet.
///
///  [`send_to_other_players`] Whether this packet should be sent to players other than the
///                            player attached to the packet (false by default).
///  [`skip_attached_player`] Whether the packet should skip being sent to the player attached
///                            to the packet (false by default).
///
///  Returns void
///
pub fn send_object_lock(send_to_other_players: bool, skip_attached_player: bool) {
    unsafe {
        raw::rustSendObjectLock(send_to_other_players, skip_attached_player)
    }
}

///
///  Send an ObjectTrap packet.
///
///  [`broadcast`] Whether this packet should be sent only to the player for whom the current
///                   object list was initialized or to everyone on the server.
///
///
///  Returns void
///
pub fn send_object_trap(send_to_other_players: bool, skip_attached_player: bool) {
    unsafe {
        raw::rustSendObjectTrap(send_to_other_players, skip_attached_player)
    }
}

///
///  Send an ObjectScale packet.
///
///  [`send_to_other_players`] Whether this packet should be sent to players other than the
///                            player attached to the packet (false by default).
///  [`skip_attached_player`] Whether the packet should skip being sent to the player attached
///                            to the packet (false by default).
///
///  Returns void
///
pub fn send_object_scale(send_to_other_players: bool, skip_attached_player: bool) {
    unsafe {
        raw::rustSendObjectScale(send_to_other_players, skip_attached_player)
    }
}

///
///  Send an ObjectState packet.
///
///  [`send_to_other_players`] Whether this packet should be sent to players other than the
///                            player attached to the packet (false by default).
///  [`skip_attached_player`] Whether the packet should skip being sent to the player attached
///                            to the packet (false by default).
///
///  Returns void
///
pub fn send_object_state(send_to_other_players: bool, skip_attached_player: bool) {
    unsafe {
        raw::rustSendObjectState(send_to_other_players, skip_attached_player)
    }
}

///
///  Send a DoorState packet.
///
///  [`send_to_other_players`] Whether this packet should be sent to players other than the
///                            player attached to the packet (false by default).
///  [`skip_attached_player`] Whether the packet should skip being sent to the player attached
///                            to the packet (false by default).
///
///  Returns void
///
pub fn send_door_state(send_to_other_players: bool, skip_attached_player: bool) {
    unsafe {
        raw::rustSendDoorState(send_to_other_players, skip_attached_player)
    }
}

///
///  Send a DoorDestination packet.
///
///  [`send_to_other_players`] Whether this packet should be sent to players other than the
///                            player attached to the packet (false by default).
///  [`skip_attached_player`] Whether the packet should skip being sent to the player attached
///                            to the packet (false by default).
///
///  Returns void
///
pub fn send_door_destination(send_to_other_players: bool, skip_attached_player: bool) {
    unsafe {
        raw::rustSendDoorDestination(send_to_other_players, skip_attached_player)
    }
}

///
///  Send a Container packet.
///
///  [`send_to_other_players`] Whether this packet should be sent to players other than the
///                            player attached to the packet (false by default).
///  [`skip_attached_player`] Whether the packet should skip being sent to the player attached
///                            to the packet (false by default).
///
///  Returns void
///
pub fn send_container(send_to_other_players: bool, skip_attached_player: bool) {
    unsafe {
        raw::rustSendContainer(send_to_other_players, skip_attached_player)
    }
}

///
///  Send a VideoPlay packet.
///
///  [`send_to_other_players`] Whether this packet should be sent to players other than the
///                            player attached to the packet (false by default).
///  [`skip_attached_player`] Whether the packet should skip being sent to the player attached
///                            to the packet (false by default).
///
///  Returns void
///
pub fn send_video_play(send_to_other_players: bool, skip_attached_player: bool) {
    unsafe {
        raw::rustSendVideoPlay(send_to_other_players, skip_attached_player)
    }
}

pub fn send_script_global_short(send_to_other_players: bool, skip_attached_player: bool) {
    unsafe {
        raw::rustSendScriptGlobalShort(send_to_other_players, skip_attached_player)
    }
}

///
///  Send a ConsoleCommand packet.
///
///  [`send_to_other_players`] Whether this packet should be sent to players other than the
///                            player attached to the packet (false by default).
///  [`skip_attached_player`] Whether the packet should skip being sent to the player attached
///                            to the packet (false by default).
///
///  Returns void
///
pub fn send_console_command(send_to_other_players: bool, skip_attached_player: bool) {
    unsafe {
        raw::rustSendConsoleCommand(send_to_other_players, skip_attached_player)
    }
}

pub fn read_last_object_list() {
    unsafe {
        raw::rustReadLastObjectList()
    }
}

pub fn read_last_event() {
    unsafe {
        raw::rustReadLastEvent()
    }
}

pub fn initialize_object_list(pid: u16) {
    unsafe {
        raw::rustInitializeObjectList(pid)
    }
}

pub fn initialize_event(pid: u16) {
    unsafe {
        raw::rustInitializeEvent(pid)
    }
}

pub fn copy_last_object_list_to_store() {
    unsafe {
        raw::rustCopyLastObjectListToStore()
    }
}

pub fn get_object_changes_size() -> u16 {
    unsafe {
        raw::rustGetObjectChangesSize()
    }
}

pub fn get_event_action() -> u8 {
    unsafe {
        raw::rustGetEventAction()
    }
}

pub fn get_event_container_sub_action() -> u8 {
    unsafe {
        raw::rustGetEventContainerSubAction()
    }
}

pub fn get_object_ref_num_index(index: u16) -> u16 {
    unsafe {
        raw::rustGetObjectRefNumIndex(index)
    }
}

pub fn get_object_summoner_ref_num_index(index: u16) -> u16 {
    unsafe {
        raw::rustGetObjectSummonerRefNumIndex(index)
    }
}

pub fn set_event_cell(cell_description: &str) {
    unsafe {
        raw::rustSetEventCell(CString::new(cell_description).unwrap_or_default().as_ptr())
    }
}

pub fn set_event_action(action: u8) {
    unsafe {
        raw::rustSetEventAction(action)
    }
}

pub fn set_event_console_command(console_command: &str) {
    unsafe {
        raw::rustSetEventConsoleCommand(CString::new(console_command).unwrap_or_default().as_ptr())
    }
}

pub fn set_object_ref_num_index(ref_num: i16) {
    unsafe {
        raw::rustSetObjectRefNumIndex(ref_num)
    }
}

pub fn add_world_object() {
    unsafe {
        raw::rustAddWorldObject()
    }
}

///
///  Use the last worldstate received by the server as the one being read.
///
///
///  Returns void
///
pub fn read_received_worldstate() {
    unsafe {
        raw::rustReadReceivedWorldstate()
    }
}

///
///  Take the contents of the read-only worldstate last received by the
///         server from a player and move its contents to the stored worldstate
///         that can be sent by the server.
///
///
///  Returns void
///
pub fn copy_received_worldstate_to_store() {
    unsafe {
        raw::rustCopyReceivedWorldstateToStore()
    }
}

///
///  Clear the kill count changes for the write-only worldstate.
///
///  This is used to initialize the sending of new WorldKillCount packets.
///
///
///  Returns void
///
pub fn clear_kill_changes() {
    unsafe {
        raw::rustClearKillChanges()
    }
}

///
///  Clear the map changes for the write-only worldstate.
///
///  This is used to initialize the sending of new WorldMap packets.
///
///
///  Returns void
///
pub fn clear_map_changes() {
    unsafe {
        raw::rustClearMapChanges()
    }
}

///
///  Get the number of indexes in the read worldstate's kill changes.
///
///
///  Returns the number of indexes.
///
pub fn get_kill_changes_size() -> u16 {
    unsafe {
        raw::rustGetKillChangesSize()
    }
}

///
///  Get the number of indexes in the read worldstate's map changes.
///
///
///  Returns the number of indexes.
///
pub fn get_map_changes_size() -> u16 {
    unsafe {
        raw::rustGetMapChangesSize()
    }
}

///
///  Get the refId at a certain index in the read worldstate's kill count changes.
///
///  [`index`] The index of the kill count.
///
///  Returns the refId.
///
pub fn get_kill_ref_id(index: u16) -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetKillRefId(index))
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the number of kills at a certain index in the read worldstate's kill count changes.
///
///  [`index`] The index of the kill count.
///
///  Returns the number of kills.
///
pub fn get_kill_number(index: u16) -> i16 {
    unsafe {
        raw::rustGetKillNumber(index)
    }
}

///
///  Get the weather region in the read worldstate.
///
///
///  Returns the weather region.
///
pub fn get_weather_region() -> String {
    unsafe {
        CStr::from_ptr(raw::rustGetWeatherRegion())
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}

///
///  Get the current weather in the read worldstate.
///
///
///  Returns the current weather.
///
pub fn get_weather_current() -> i16 {
    unsafe {
        raw::rustGetWeatherCurrent()
    }
}

///
///  Get the next weather in the read worldstate.
///
///
///  Returns the next weather.
///
pub fn get_weather_next() -> i16 {
    unsafe {
        raw::rustGetWeatherNext()
    }
}

///
///  Get the queued weather in the read worldstate.
///
///
///  Returns the queued weather.
///
pub fn get_weather_queued() -> i16 {
    unsafe {
        raw::rustGetWeatherQueued()
    }
}

///
///  Get the transition factor of the weather in the read worldstate.
///
///
///  Returns the transition factor of the weather.
///
pub fn get_weather_transition_factor() -> f64 {
    unsafe {
        raw::rustGetWeatherTransitionFactor()
    }
}

///
///  Get the X coordinate of the cell corresponding to the map tile at a certain index in
///         the read worldstate's map tiles.
///
///  [`index`] The index of the map tile.
///
///  Returns the X coordinate of the cell.
///
pub fn get_map_tile_cell_x(index: u16) -> i16 {
    unsafe {
        raw::rustGetMapTileCellX(index)
    }
}

///
///  Get the Y coordinate of the cell corresponding to the map tile at a certain index in
///         the read worldstate's map tiles.
///
///  [`index`] The index of the map tile.
///
///  Returns the Y coordinate of the cell.
///
pub fn get_map_tile_cell_y(index: u16) -> i16 {
    unsafe {
        raw::rustGetMapTileCellY(index)
    }
}

///
///  Set the region affected by the next WorldRegionAuthority packet sent.
///
///  [`region`] The region.
///
///  Returns void
///
pub fn set_authority_region(authority_region: &str) {
    unsafe {
        raw::rustSetAuthorityRegion(CString::new(authority_region).unwrap_or_default().as_ptr())
    }
}

///
///  Set the weather region in the write-only worldstate stored on the server.
///
///  [`region`] The region.
///
///  Returns void
///
pub fn set_weather_region(region: &str) {
    unsafe {
        raw::rustSetWeatherRegion(CString::new(region).unwrap_or_default().as_ptr())
    }
}

///
///  Set the weather forcing state in the write-only worldstate stored on the server.
///
///  Players who receive a packet with forced weather will switch to that weather immediately.
///
///  [`force_state`] The weather forcing state.
///
///  Returns void
///
pub fn set_weather_force_state(force_state: bool) {
    unsafe {
        raw::rustSetWeatherForceState(force_state)
    }
}

///
///  Set the current weather in the write-only worldstate stored on the server.
///
///  [`current_weather`] The current weather.
///
///  Returns void
///
pub fn set_weather_current(current_weather: i16) {
    unsafe {
        raw::rustSetWeatherCurrent(current_weather)
    }
}

///
///  Set the next weather in the write-only worldstate stored on the server.
///
///  [`next_weather`] The next weather.
///
///  Returns void
///
pub fn set_weather_next(next_weather: i16) {
    unsafe {
        raw::rustSetWeatherNext(next_weather)
    }
}

///
///  Set the queued weather in the write-only worldstate stored on the server.
///
///  [`queued_weather`] The queued weather.
///
///  Returns void
///
pub fn set_weather_queued(queued_weather: i16) {
    unsafe {
        raw::rustSetWeatherQueued(queued_weather)
    }
}

///
///  Set the transition factor for the weather in the write-only worldstate stored on the server.
///
///  [`transition_factor`] The transition factor.
///
///  Returns void
///
pub fn set_weather_transition_factor(transition_factor: f64) {
    unsafe {
        raw::rustSetWeatherTransitionFactor(transition_factor)
    }
}

///
///  Set the world's hour in the write-only worldstate stored on the server.
///
///  [`hour`] The hour.
///
///  Returns void
///
pub fn set_hour(hour: f64) {
    unsafe {
        raw::rustSetHour(hour)
    }
}

///
///  Set the world's day in the write-only worldstate stored on the server.
///
///  [`day`] The day.
///
///  Returns void
///
pub fn set_day(day: i16) {
    unsafe {
        raw::rustSetDay(day)
    }
}

///
///  Set the world's month in the write-only worldstate stored on the server.
///
///  [`month`] The month.
///
///  Returns void
///
pub fn set_month(month: i16) {
    unsafe {
        raw::rustSetMonth(month)
    }
}

///
///  Set the world's year in the write-only worldstate stored on the server.
///
///  [`year`] The year.
///
///  Returns void
///
pub fn set_year(year: i16) {
    unsafe {
        raw::rustSetYear(year)
    }
}

///
///  Set the world's days passed in the write-only worldstate stored on the server.
///
///  [`days_passed`] The days passed.
///
///  Returns void
///
pub fn set_days_passed(days_passed: i16) {
    unsafe {
        raw::rustSetDaysPassed(days_passed)
    }
}

///
///  Set the world's time scale in the write-only worldstate stored on the server.
///
///  [`pid`] The player ID.
///  [`time_scale`] The time scale.
///
///  Returns void
///
pub fn set_time_scale(time_scale: f64) {
    unsafe {
        raw::rustSetTimeScale(time_scale)
    }
}

///
///  Set the collision state for other players in the write-only worldstate stored
///         on the server.
///
///  [`state`] The collision state.
///
///  Returns void
///
pub fn set_player_collision_state(state: bool) {
    unsafe {
        raw::rustSetPlayerCollisionState(state)
    }
}

///
///  Set the collision state for actors in the write-only worldstate stored on the
///         server.
///
///  [`state`] The collision state.
///
///  Returns void
///
pub fn set_actor_collision_state(state: bool) {
    unsafe {
        raw::rustSetActorCollisionState(state)
    }
}

///
///  Set the collision state for placed objects in the write-only worldstate stored
///         on the server.
///
///  [`state`] The collision state.
///
///  Returns void
///
pub fn set_placed_object_collision_state(state: bool) {
    unsafe {
        raw::rustSetPlacedObjectCollisionState(state)
    }
}

///
///  Whether placed objects with collision turned on should use actor collision, i.e.
///         whether they should be slippery and prevent players from standing on them.
///
///  [`use_actor_collision`] Whether to use actor collision.
///
///  Returns void
///
pub fn use_actor_collision_for_placed_objects(use_actor_collision: bool) {
    unsafe {
        raw::rustUseActorCollisionForPlacedObjects(use_actor_collision)
    }
}

///
///  Add a new kill count to the kill count changes.
///
///  [`ref_id`] The refId of the kill count.
///  [`number`] The number of kills in the kill count.
///
///  Returns void
///
pub fn add_kill(ref_id: &str, number: i16) {
    unsafe {
        raw::rustAddKill(CString::new(ref_id).unwrap_or_default().as_ptr(), number)
    }
}

///
///  Add an ID to the list of script IDs whose variable changes should be sent to the
///         the server by clients.
///
///  [`script_id`] The ID.
///
///  Returns void
///
pub fn add_synchronized_client_script_id(script_id: &str) {
    unsafe {
        raw::rustAddSynchronizedClientScriptId(CString::new(script_id).unwrap_or_default().as_ptr())
    }
}

///
///  Add an ID to the list of global IDs whose value changes should be sent to the
///         server by clients.
///
///  [`global_id`] The ID.
///
///  Returns void
///
pub fn add_synchronized_client_global_id(global_id: &str) {
    unsafe {
        raw::rustAddSynchronizedClientGlobalId(CString::new(global_id).unwrap_or_default().as_ptr())
    }
}

///
///  Add a refId to the list of refIds for which collision should be enforced
///         irrespective of other settings.
///
///  [`ref_id`] The refId.
///
///  Returns void
///
pub fn add_enforced_collision_ref_id(ref_id: &str) {
    unsafe {
        raw::rustAddEnforcedCollisionRefId(CString::new(ref_id).unwrap_or_default().as_ptr())
    }
}

///
///  Clear the list of script IDs whose variable changes should be sent to the
///         the server by clients.
///
///
///  Returns void
///
pub fn clear_synchronized_client_script_ids() {
    unsafe {
        raw::rustClearSynchronizedClientScriptIds()
    }
}

///
///  Clear the list of global IDs whose value changes should be sent to the
///         the server by clients.
///
///
///  Returns void
///
pub fn clear_synchronized_client_global_ids() {
    unsafe {
        raw::rustClearSynchronizedClientGlobalIds()
    }
}

///
///  Clear the list of refIds for which collision should be enforced irrespective
///         of other settings.
///
///
///  Returns void
///
pub fn clear_enforced_collision_ref_ids() {
    unsafe {
        raw::rustClearEnforcedCollisionRefIds()
    }
}

///
///  Save the .png image data of the map tile at a certain index in the read worldstate's
///         map changes.
///
///  [`index`] The index of the map tile.
///  [`file_path`] The file path of the resulting file.
///
///  Returns void
///
pub fn save_map_tile_image_file(index: u16, file_path: &str) {
    unsafe {
        raw::rustSaveMapTileImageFile(index, CString::new(file_path).unwrap_or_default().as_ptr())
    }
}

///
///  Load a .png file as the image data for a map tile and add it to the write-only worldstate
///         stored on the server.
///
///  [`cell_x`] The X coordinate of the cell corresponding to the map tile.
///  [`cell_y`] The Y coordinate of the cell corresponding to the map tile.
///  [`file_path`] The file path of the loaded file.
///
///  Returns void
///
pub fn load_map_tile_image_file(cell_x: i16, cell_y: i16, file_path: &str) {
    unsafe {
        raw::rustLoadMapTileImageFile(cell_x, cell_y, CString::new(file_path).unwrap_or_default().as_ptr())
    }
}

///
///  Send a ClientScriptSettings packet with the current client script settings in
///         the write-only worldstate.
///
///  [`pid`] The player ID attached to the packet.
///  [`send_to_other_players`] Whether this packet should be sent to players other than the
///                            player attached to the packet (false by default).
///  [`skip_attached_player`] Whether the packet should skip being sent to the player attached
///                            to the packet (false by default).
///
///  Returns void
///
pub fn send_client_script_settings(pid: u16, send_to_other_players: bool, skip_attached_player: bool) {
    unsafe {
        raw::rustSendClientScriptSettings(pid, send_to_other_players, skip_attached_player)
    }
}

///
///  Send a WorldKillCount packet with the current set of kill count changes in the write-only
///         worldstate.
///
///  [`pid`] The player ID attached to the packet.
///  [`send_to_other_players`] Whether this packet should be sent to players other than the
///                            player attached to the packet (false by default).
///  [`skip_attached_player`] Whether the packet should skip being sent to the player attached
///                            to the packet (false by default).
///
///  Returns void
///
pub fn send_world_kill_count(pid: u16, send_to_other_players: bool, skip_attached_player: bool) {
    unsafe {
        raw::rustSendWorldKillCount(pid, send_to_other_players, skip_attached_player)
    }
}

///
///  Send a WorldMap packet with the current set of map changes in the write-only
///         worldstate.
///
///  [`pid`] The player ID attached to the packet.
///  [`broadcast`] Whether this packet should be sent only to the attached player
///                   or to all players on the server.
///
///  Returns void
///
pub fn send_world_map(pid: u16, send_to_other_players: bool, skip_attached_player: bool) {
    unsafe {
        raw::rustSendWorldMap(pid, send_to_other_players, skip_attached_player)
    }
}

///
///  Send a WorldTime packet with the current time and time scale in the write-only
///         worldstate.
///
///  [`pid`] The player ID attached to the packet.
///  [`send_to_other_players`] Whether this packet should be sent to players other than the
///                            player attached to the packet (false by default).
///  [`skip_attached_player`] Whether the packet should skip being sent to the player attached
///                            to the packet (false by default).
///
///  Returns void
///
pub fn send_world_time(pid: u16, send_to_other_players: bool, skip_attached_player: bool) {
    unsafe {
        raw::rustSendWorldTime(pid, send_to_other_players, skip_attached_player)
    }
}

///
///  Send a WorldWeather packet with the current weather in the write-only worldstate.
///
///  [`pid`] The player ID attached to the packet.
///  [`send_to_other_players`] Whether this packet should be sent to players other than the
///                            player attached to the packet (false by default).
///  [`skip_attached_player`] Whether the packet should skip being sent to the player attached
///                            to the packet (false by default).
///
///  Returns void
///
pub fn send_world_weather(pid: u16, send_to_other_players: bool, skip_attached_player: bool) {
    unsafe {
        raw::rustSendWorldWeather(pid, send_to_other_players, skip_attached_player)
    }
}

///
///  Send a WorldCollisionOverride packet with the current collision overrides in
///         the write-only worldstate.
///
///  [`pid`] The player ID attached to the packet.
///  [`send_to_other_players`] Whether this packet should be sent to players other than the
///                            player attached to the packet (false by default).
///  [`skip_attached_player`] Whether the packet should skip being sent to the player attached
///                            to the packet (false by default).
///
///  Returns void
///
pub fn send_world_collision_override(pid: u16, send_to_other_players: bool, skip_attached_player: bool) {
    unsafe {
        raw::rustSendWorldCollisionOverride(pid, send_to_other_players, skip_attached_player)
    }
}

///
///  Send a WorldRegionAuthority packet establishing a certain player as the only one who
///         should process certain region-specific events (such as weather changes).
///
///  It is always sent to all players.
///
///  [`pid`] The player ID attached to the packet.
///
///  Returns void
///
pub fn send_world_region_authority(pid: u16) {
    unsafe {
        raw::rustSendWorldRegionAuthority(pid)
    }
}

pub fn read_last_worldstate() {
    unsafe {
        raw::rustReadLastWorldstate()
    }
}

pub fn copy_last_worldstate_to_store() {
    unsafe {
        raw::rustCopyLastWorldstateToStore()
    }
}

