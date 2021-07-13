use graphql_client::GraphQLQuery;

use crate::errors::AppError;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schemas/gitlab.graphql",
    query_path = "graphql/queries/gitlab_get_mr.graphql",
    response_derives = "Debug"
)]
pub struct GitlabGetMR;

/**
 * Fetches the merge request of the given project related to the given branch name.
 */
pub async fn get_merge_request(project_path: String, branch_name: String) -> Result<(), AppError> {
    let request_body = GitlabGetMR::build_query(gitlab_get_mr::Variables {
        project_path,
        branch_name,
    });

    Err(AppError::NotImplemented(String::from("")))
}
