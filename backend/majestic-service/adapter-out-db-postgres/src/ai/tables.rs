pub mod ai_entities {
    pub const TABLE: &'static str = "ai_entities";
    pub mod col {
        pub const ID: &'static str = "id";
        pub const NAME: &'static str = "name";
        pub const HEIGHT: &'static str = "height";
        pub const WEIGHT: &'static str = "weight";
        pub const GENDER_ID: &'static str = "gender_id";
        pub const BIRTHDAY: &'static str = "birthday";
        pub const GLB_FILE_URL: &'static str = "glb_file_url";
        pub const CREATED_ON: &'static str = "created_on";
        pub const LAST_MODIFIED_ON: &'static str = "last_modified_on";
    }
}

pub mod personalities {
    pub const TABLE: &'static str = "personalities";
    pub mod col {
        pub const ID: &'static str = "id";
        pub const NAME: &'static str = "name";
        pub const CREATED_ON: &'static str = "created_on";
        pub const LAST_MODIFIED_ON: &'static str = "last_modified_on";
    }
}

pub mod genders {
    pub const TABLE: &'static str = "genders";
    pub mod col {
        pub const ID: &'static str = "id";
        pub const NAME: &'static str = "name";
        pub const CREATED_ON: &'static str = "created_on";
        pub const LAST_MODIFIED_ON: &'static str = "last_modified_on";
    }
}

pub mod ai_entities_personalities {
    pub const TABLE: &'static str = "ai_entities_personalities";
    pub mod col {
        pub const AI_ENTITY_ID: &'static str = "ai_entity_id";
        pub const PERSONALITY_ID: &'static str = "personality_id";
        pub const CREATED_ON: &'static str = "created_on";
        pub const LAST_MODIFIED_ON: &'static str = "last_modified_on";
    }
}