use eyre::{Context, Report};
use graphql_client::{GraphQLQuery, Response};

use crate::errors::AppError;

use super::send_graphql_request;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schemas/gitlab.graphql",
    query_path = "graphql/queries/gitlab_create_issue.graphql",
    response_derives = "Debug"
)]
pub struct GitlabCreateIssue;

pub struct GitlabCreatedIssue {
    pub iid: String,
    pub web_url: String,
}

pub async fn create_issue(
    token: &String,
    project_path: &String,
    title: &String,
    description: &Option<String>,
) -> Result<GitlabCreatedIssue, Report> {
    let query_body = GitlabCreateIssue::build_query(gitlab_create_issue::Variables {
        project_path: project_path.to_owned(),
        title: title.to_owned(),
        description: description.to_owned(),
    });

    let res = send_graphql_request(token, &query_body)
        .await
        .wrap_err("API request failed")?;

    let response_body: Response<gitlab_create_issue::ResponseData> = res
        .json()
        .await
        .wrap_err("Failed to parse the Graphql response from the API")?;

    if let Some(graphql_errs) = response_body.errors {
        Err(AppError::GraphqlError {
            message: String::from("Gitlab returned an error while creating the issue"),
            details: format!("{:?}", graphql_errs[0]),
        })?
    } else {
        // my god why! is! every! level! optional!
        let issue = response_body
            .data
            .unwrap()
            .create_issue
            .unwrap()
            .issue
            .unwrap();
        Ok(GitlabCreatedIssue {
            iid: issue.id,
            web_url: issue.web_url,
        })
    }
}
