use bitflags::bitflags;


bitflags! {
    pub struct Permissions: i64 {
        const NONE = 0;
        const VIEW_CHANNELS = 1 << 0;
        const SEND_MESSAGES = 1 << 1;
        const MANAGE_MESSAGES = 1 << 2;

        const KICK_MEMBERS = 1 << 3;
        const BAN_MEMBERS = 1 << 4;

        const MANAGE_ROLES = 1 << 5;
        const MANAGE_CHANNELS = 1 << 6;

        const BYPASS_OVERWRITES = 1 << 7;
    }
}

#[derive(Clone, Debug)]
pub struct PermissionsPair {
    pub allow: Permissions,
    pub deny: Permissions
}