use crate::errors::AppError;

use super::send_graphql_request;
use eyre::{Context, Report};
use graphql_client::{GraphQLQuery, Response};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schemas/gitlab.graphql",
    query_path = "graphql/queries/gitlab_update_mr_desc.graphql",
    response_derives = "Debug"
)]
pub struct GitlabUpdateMRDesc;

#[derive(Debug)]
pub struct GetMRResponse {
    pub project_path: String,
    pub iid: String,
    pub description: String,
}

/**
 * Updates the description of a merge request.
 */
pub async fn update_merge_request_desc(
    token: &String,
    project_path: &String,
    iid: &String,
    description: &String,
) -> Result<(), Report> {
    let query_body = GitlabUpdateMRDesc::build_query(gitlab_update_mr_desc::Variables {
        project_path: project_path.clone(),
        iid: iid.clone(),
        desc: Some(description.clone()),
    });

    let res = send_graphql_request(token, &query_body)
        .await
        .wrap_err_with(|| format!("Failed to send the merge request update to the Graphql API."))?;

    let response_body: Response<gitlab_update_mr_desc::ResponseData> =
        res.json().await.wrap_err_with(|| {
            format!("Failed to parse the response of the merge request update from the API.")
        })?;

    if let Some(graphql_errs) = response_body.errors {
        return Err(AppError::GraphqlError {
            message: String::from(
                "Gitlab returned an error while updating the merge request description",
            ),
            details: format!("{:?}", graphql_errs[0]),
        })?;
    }

    Ok(())
}
