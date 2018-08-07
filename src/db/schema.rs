table! {
    language (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    oauth (gid) {
        gid -> Text,
        uid -> Integer,
    }
}

table! {
    problem (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
    }
}

table! {
    solution (id) {
        id -> Integer,
        pid -> Integer,
        uid -> Integer,
        language -> Integer,
        code -> Text,
    }
}

table! {
    user (id) {
        id -> Integer,
        sid -> Text,
        admin -> Bool,
    }
}

joinable!(solution -> language (language));
joinable!(solution -> problem (pid));
joinable!(solution -> user (uid));

allow_tables_to_appear_in_same_query!(
    language,
    oauth,
    problem,
    solution,
    user,
);
