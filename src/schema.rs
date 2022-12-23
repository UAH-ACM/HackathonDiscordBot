// @generated automatically by Diesel CLI.

diesel::table! {
    team_seaking (id) {
        id -> Int8,
        name -> Varchar,
        discordname -> Text,
        description -> Text,
    }
}

diesel::table! {
    teams (id) {
        id -> Int8,
        team_name -> Varchar,
        description -> Text,
        leader -> Text,
        members -> Text,
        points -> Int8,
        role_id -> Int8,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    team_seaking,
    teams,
);
