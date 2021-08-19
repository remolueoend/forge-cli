use crate::errors::AppError;

use super::send_graphql_request;
use eyre::{eyre, Context, ContextCompat, Report};
use graphql_client::{GraphQLQuery, Response};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schemas/gitlab.graphql",
    query_path = "graphql/queries/gitlab_get_mr.graphql",
    response_derives = "Debug"
)]
pub struct GitlabGetMR;

#[derive(Debug)]
pub struct FetchedMergeRequest {
    pub project_path: String,
    pub iid: String,
    pub description: String,
    pub web_url: String,
}

fn parse_mr_response(
    project_path: &String,
    data: Response<gitlab_get_mr::ResponseData>,
) -> Result<FetchedMergeRequest, Report> {
    let project = data
        .data
        .wrap_err("missing data")?
        .project
        .wrap_err("missing project")?;

    let mrs_opt = project
        .merge_requests
        .wrap_err("Missing merge requests")?
        .nodes
        .wrap_err("Missing merge requests")?;
    let first_mr_opt = mrs_opt.first().wrap_err("Missing merge request")?;

    match first_mr_opt {
        None => Err(eyre!("Missing merge request")),
        Some(v) => Ok(FetchedMergeRequest {
            project_path: project_path.clone(),
            iid: v.iid.clone(),
            description: v.description.clone().unwrap_or_default(),
            web_url: v.web_url.clone().unwrap_or_default(),
        }),
    }
}

/**
 * Fetches the merge request of the given project related to the given branch name.
 */
pub async fn get_merge_request(
    token: &String,
    project_path: &String,
    branch_name: &String,
) -> Result<FetchedMergeRequest, Report> {
    let query_body = GitlabGetMR::build_query(gitlab_get_mr::Variables {
        project_path: project_path.clone(),
        branch_name: branch_name.clone(),
    });

    let res = send_graphql_request(token, &query_body)
        .await
        .wrap_err_with(|| format!("Failed to fetch the merge request details from Gitlab."))?;

    let response_body: Response<gitlab_get_mr::ResponseData> = res
        .json()
        .await
        .wrap_err_with(|| format!("Failed to parse the merge request details from Gitlab."))?;

    if let Some(graphql_errs) = response_body.errors {
        Err(AppError::GraphqlError {
            message: String::from(
                "Gitlab returned an error while fetching the merge request details",
            ),
            details: format!("{:?}", graphql_errs[0]),
        })?
    } else {
        parse_mr_response(project_path, response_body)
            .wrap_err("Failed to get merge request details from graphql response")
    }
}
