use actix_web::{
    Result, error::ErrorInternalServerError,
    App, Responder, State,
};
use diesel::prelude::*;
use liquid::{ Object, Value };

use ::{
    AppState,
    db::{
        models::User,
        schema::{ user, solution },
    },
    middleware::Template,
};

pub fn configure(app: App<AppState>) -> App<AppState> {
    app.resource("/leaderboard/", |r| r.with(index))
}

fn index(state: State<AppState>) -> Result<impl Responder> {
    // Get the score, it's the number of solutions the user
    // has successfully submitted.
    let num_solutions = solution::table
        .filter(solution::uid.eq(user::id))
        .count()
        .single_value();

    let users = user::table
        .select((user::all_columns, num_solutions))
        .order_by(num_solutions.desc())
        .load::<(User, Option<i64>)>(&state.db)
        .map_err(ErrorInternalServerError)?
        .into_iter()
        .map(|(user, score)| {
            let mut obj = user.to_liquid();
            let score = score.unwrap_or(0) as i32;
            obj.insert("score".into(), Value::scalar(score));
            Value::Object(obj)
        });

    let mut obj = Object::new();
    obj.insert("leaderboard".to_string(), Value::array(users));

    Ok(Template::render("leaderboard.liquid", obj))
}
