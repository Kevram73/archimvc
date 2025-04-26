// @generated automatically by Diesel CLI.

diesel::table! {
    agent_gains (id) {
        id -> Integer,
        gain_id -> Integer,
        agent_id -> Integer,
        facteur -> Integer,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    agents (id) {
        id -> Integer,
        nom -> Nullable<Text>,
        phone -> Nullable<Text>,
        code -> Text,
        parent_id -> Nullable<Integer>,
        profil_id -> Nullable<Integer>,
        taux_commission -> Integer,
        password -> Text,
        actif -> Bool,
        membre -> Bool,
        pwd_changed -> Bool,
        fils_parent -> Bool,
        code_a -> Text,
        code_b -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    gains (id) {
        id -> Integer,
        nb_boules -> Nullable<Integer>,
        nb_pos_fixe -> Nullable<Integer>,
        facteur -> Nullable<Integer>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    jeux (id) {
        id -> Integer,
        nom -> Nullable<Text>,
        pays_id -> Integer,
        actif -> Bool,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    profiles (id) {
        id -> Integer,
        nom -> Nullable<Text>,
        fils_parent -> Bool,
        taux_commission -> Integer,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    tirages (id) {
        id -> Integer,
        jeu_id -> Nullable<Integer>,
        numero -> Nullable<Integer>,
        total -> Integer,
        la_date -> Nullable<Date>,
        hr_lance -> Nullable<Timestamp>,
        hr_seuil -> Nullable<Time>,
        hr_clot -> Nullable<Time>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        email -> Text,
        created_at -> Timestamp,
    }
}

diesel::joinable!(agent_gains -> agents (agent_id));
diesel::joinable!(agent_gains -> gains (gain_id));
diesel::joinable!(agents -> profiles (profil_id));

diesel::allow_tables_to_appear_in_same_query!(
    agent_gains,
    agents,
    gains,
    jeux,
    profiles,
    tirages,
    users,
);
