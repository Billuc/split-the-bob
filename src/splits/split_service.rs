use std::collections::HashMap;

use askama::Template;
use axum::{
    Router,
    extract::{self, Query},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
};
use axum_extra::extract::Form;

use crate::splits::split_view::get_split_view;
use crate::splits::{split_repo::SplitRepo, split_view::SplitDetails};
use crate::{
    axum_state::State,
    splits::split_repo::{CreateSplit, UpdateSplit},
    view::index,
};
use crate::{error::Error, splits::redirect_to_split};

pub fn split_service() -> axum::Router<State> {
    Router::new()
        .route("/", get(show_split))
        .route("/", post(update_split))
        .route("/new", post(create_split))
        .route("/details", get(split_details))
}

#[derive(serde::Deserialize)]
pub struct SplitQuery {
    split_id: String,
    errors: Option<String>,
}

async fn show_split(
    extract::State(state): extract::State<State>,
    extract::Query(query): extract::Query<SplitQuery>,
) -> Result<impl IntoResponse, Error> {
    get_split_view(
        &state.db,
        query.split_id,
        query
            .errors
            .unwrap_or(String::new())
            .split('\n')
            .filter(|err| !err.is_empty())
            .collect(),
    )
    .await
}

#[derive(serde::Deserialize)]
pub struct CreateSplitForm {
    description: String,
    #[serde(rename = "participants[]")]
    participants: Vec<String>,
    default_currency: String,
}

pub async fn create_split(
    extract::State(state): extract::State<State>,
    Form(form): Form<CreateSplitForm>,
) -> Result<Response, Error> {
    let new_split = CreateSplit {
        description: form.description,
        participants: distinct_participants(&form.participants),
        default_currency: form.default_currency,
    };

    let response = match SplitRepo::create(&state.db, new_split).await {
        Err(error) => {
            eprintln!("Error creating split: {:?}", error);
            let html: Html<String> = index(vec![error]).await?;
            html.into_response()
        }
        Ok(id) => {
            println!("Created split with id {}", id);
            redirect_to_split(&id, vec![])?
        }
    };

    Ok(response)
}

#[derive(serde::Deserialize)]
pub struct UpdateSplitForm {
    split_id: String,
    description: Option<String>,
    #[serde(rename = "participants[]")]
    participants: Option<Vec<String>>,
    default_currency: Option<String>,
}

pub async fn update_split(
    extract::State(state): extract::State<State>,
    Form(form): Form<UpdateSplitForm>,
) -> Result<Response, Error> {
    let updated_split = UpdateSplit {
        id: form.split_id.clone(),
        description: form.description,
        participants: form.participants.as_ref().map(distinct_participants),
        default_currency: form.default_currency,
    };

    let response = match SplitRepo::update(&state.db, updated_split).await {
        Err(error) => {
            eprintln!("Error updating split: {:?}", error);
            get_split_view(&state.db, form.split_id.clone(), vec![&error.to_string()])
                .await?
                .into_response()
        }
        Ok(_) => {
            println!("Updated split with id {}", form.split_id.clone());
            get_split_view(&state.db, form.split_id.clone(), vec![])
                .await?
                .into_response()
        }
    };

    Ok(response)
}

fn distinct_participants(participants: &Vec<String>) -> Vec<String> {
    let mut map: HashMap<String, &str> = HashMap::new();

    for p in participants {
        let key = p.to_uppercase();
        if map.contains_key(&key) {
            continue;
        }

        map.insert(key, &p);
    }

    map.iter().map(|entry| entry.1.to_string()).collect()
}

#[derive(serde::Deserialize)]
pub struct SplitDetailsQuery {
    split_id: String,
}

pub async fn split_details(
    extract::State(state): extract::State<State>,
    Query(query): Query<SplitDetailsQuery>,
) -> Result<Response, Error> {
    let response = match SplitRepo::get_by_id(&state.db, query.split_id.clone()).await {
        Err(error) => {
            eprintln!(
                "Could not find split with ID {}: {:?}",
                query.split_id.clone(),
                error
            );
            SplitDetails::Failure {
                split_id: query.split_id,
            }
            .render()?
            .into_response()
        }
        Ok(split) => SplitDetails::Success { split }.render()?.into_response(),
    };

    Ok(response)
}
