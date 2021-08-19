use eyre::Report;
use graphql_client::QueryBody;
use log::debug;
use reqwest::Response;
use serde::Serialize;
use serde_json;

pub mod gitlab_create_issue;
pub mod gitlab_get_mr;
pub mod gitlab_update_mr_desc;

async fn send_graphql_request<'de, V: Serialize>(
    token: &String,
    query_body: &QueryBody<V>,
) -> Result<Response, Report> {
    let client = reqwest::Client::new();

    debug!(
        "Sending query: {}",
        serde_json::to_string(query_body).unwrap()
    );

    let res = client
        .post("https://gitlab.com/api/graphql")
        .header("Authorization", format!("Bearer {}", token))
        .json(query_body)
        .send()
        .await?;

    Ok(res)
}
