
table! {
    EventTable (id) {
        id -> Nullable<Integer>,
        name -> Nullable<Text>,
        primary_photo_id -> Nullable<Integer>,
        time_created -> Nullable<Integer>,
        primary_source_id -> Nullable<Text>,
        comment -> Nullable<Text>,
    }
}

table! {
    PhotoTable (id) {
        id -> Integer,
        filename -> Text,
        width -> Nullable<Integer>,
        height -> Nullable<Integer>,
        filesize -> Nullable<Integer>,
        timestamp -> Nullable<Integer>,
        exposure_time -> Nullable<Integer>,
      orientation -> Nullable<Integer>,
        original_orientation -> Nullable<Integer>,
        import_id -> Nullable<Integer>,
        event_id -> Nullable<Integer>,
        transformations -> Nullable<Text>,
        md5 -> Nullable<Text>,
        thumbnail_md5 -> Nullable<Text>,
        
        time_created -> Nullable<Integer>,
        exif_md5 -> Nullable<Text>,
        flags -> Nullable<Integer>,
        rating -> Nullable<Integer>,
        file_format -> Nullable<Integer>,
        title -> Nullable<Text>,
        backlinks -> Nullable<Text>,
        time_reimported -> Nullable<Integer>,
        editable_id -> Nullable<Integer>,
        metadata_dirty -> Nullable<Integer>,
        developer -> Nullable<Text>, 
        develop_shotwell_id -> Nullable<Integer>,
        develop_camera_id -> Nullable<Integer>,
        develop_embedded_id -> Nullable<Integer>,
        comment -> Nullable<Text>,
    }
}

table! {
    SavedSearchDBTable (id) {
        id -> Nullable<Integer>,
        name -> Text,
        operator -> Text,
    }
}

table! {
    SavedSearchDBTable_Date (id) {
        id -> Nullable<Integer>,
        search_id -> Integer,
        search_type -> Text,
        context -> Text,
        date_one -> Nullable<Integer>,
        date_two -> Nullable<Integer>,
    }
}

table! {
    SavedSearchDBTable_Flagged (id) {
        id -> Nullable<Integer>,
        search_id -> Integer,
        search_type -> Text,
        flag_state -> Text,
    }
}

table! {
    SavedSearchDBTable_MediaType (id) {
        id -> Nullable<Integer>,
        search_id -> Integer,
        search_type -> Text,
        context -> Text,
        #[sql_name = "type"]
        type_ -> Nullable<Text>,
    }
}

table! {
    SavedSearchDBTable_Modified (id) {
        id -> Nullable<Integer>,
        search_id -> Integer,
        search_type -> Text,
        context -> Text,
        modified_state -> Text,
    }
}

table! {
    SavedSearchDBTable_Rating (id) {
        id -> Nullable<Integer>,
        search_id -> Integer,
        search_type -> Text,
        rating -> Nullable<Integer>,
        context -> Text,
    }
}

table! {
    SavedSearchDBTable_Text (id) {
        id -> Nullable<Integer>,
        search_id -> Integer,
        search_type -> Text,
        context -> Text,
        text -> Nullable<Text>,
    }
}

table! {
    TagTable (id) {
        id -> Nullable<Integer>,
        name -> Text,
        photo_id_list -> Nullable<Text>,
        time_created -> Nullable<Integer>,
    }
}

table! {
    TombstoneTable (id) {
        id -> Nullable<Integer>,
        filepath -> Text,
        filesize -> Nullable<Integer>,
        md5 -> Nullable<Text>,
        time_created -> Nullable<Integer>,
        reason -> Nullable<Integer>,
    }
}

table! {
    VersionTable (id) {
        id -> Nullable<Integer>,
        schema_version -> Nullable<Integer>,
        app_version -> Nullable<Text>,
        user_data -> Nullable<Text>,
    }
}

table! {
    VideoTable (id) {
        id -> Nullable<Integer>,
        filename -> Text,
        width -> Nullable<Integer>,
        height -> Nullable<Integer>,
        clip_duration -> Nullable<Float>,
        is_interpretable -> Nullable<Integer>,
        filesize -> Nullable<Integer>,
        timestamp -> Nullable<Integer>,
        exposure_time -> Nullable<Integer>,
        import_id -> Nullable<Integer>,
        event_id -> Nullable<Integer>,
        md5 -> Nullable<Text>,
        time_created -> Nullable<Integer>,
        rating -> Nullable<Integer>,
        title -> Nullable<Text>,
        backlinks -> Nullable<Text>,
        time_reimported -> Nullable<Integer>,
        flags -> Nullable<Integer>,
        comment -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    EventTable,
    PhotoTable,
    SavedSearchDBTable,
    SavedSearchDBTable_Date,
    SavedSearchDBTable_Flagged,
    SavedSearchDBTable_MediaType,
    SavedSearchDBTable_Modified,
    SavedSearchDBTable_Rating,
    SavedSearchDBTable_Text,
    TagTable,
    TombstoneTable,
    VersionTable,
    VideoTable,
);
