// @generated automatically by Diesel CLI.

diesel::table! {
    pet (id) {
        id -> Integer,
        name -> Text,
        owner_name -> Text,
        owner_phone -> Text,
        age -> Integer,
        pet_type -> Integer,
        vet_id -> Nullable<Integer>,
        created_at -> Timestamp,
        created_by -> Integer,
    }
}

diesel::table! {
    user (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
    }
}

diesel::table! {
    vet (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    visit (id) {
        id -> Integer,
        pet_id -> Integer,
        vet_id -> Integer,
        visit_date -> Date,
        notes -> Nullable<Text>,
    }
}

diesel::joinable!(pet -> user (created_by));
diesel::joinable!(pet -> vet (vet_id));
diesel::joinable!(visit -> pet (pet_id));
diesel::joinable!(visit -> vet (vet_id));

diesel::allow_tables_to_appear_in_same_query!(pet, user, vet, visit,);
