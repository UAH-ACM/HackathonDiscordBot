// @generated automatically by Diesel CLI.

diesel::table! {
    team_seaking (id) {
        id -> Int8,
        name -> Varchar,
        discordname -> Text,
        description -> Text,
    }
}
