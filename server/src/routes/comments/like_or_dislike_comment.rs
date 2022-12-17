use crate::{extractors::AuthUser, io::error::AppError, repos::CommentsRepository};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn like_or_dislike_comment(
    auth_service: AuthUser,
    State(conn): State<PgPool>,
    Path(path): Path<(Uuid, Uuid)>,
) -> Result<impl IntoResponse, AppError> {
    let comments_repository = CommentsRepository { connection: &conn };

    let comment_id = &path.1;

    let user_id = auth_service.id;

    let comment = comments_repository.find_one(comment_id).await?;

    if comment.is_none() {
        return Err(AppError::NotFound);
    }

    let comment_like = comments_repository
        .find_one_like(comment_id, &user_id)
        .await?;

    match comment_like {
        Some(_) => {
            comments_repository.delete_like(comment_id).await?;

            Ok(())
        }
        None => {
            comments_repository
                .insert_like(comment_id, &user_id)
                .await?;

            Ok(())
        }
    }
}
