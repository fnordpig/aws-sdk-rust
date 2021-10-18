// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    AccessDeniedException(crate::error::AccessDeniedException),
    InternalException(crate::error::InternalException),
    InvalidInputException(crate::error::InvalidInputException),
    LimitExceededException(crate::error::LimitExceededException),
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::InternalException(inner) => inner.fmt(f),
            Error::InvalidInputException(inner) => inner.fmt(f),
            Error::LimitExceededException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::AssociateMemberAccountError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::AssociateMemberAccountError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::AssociateMemberAccountErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::AssociateMemberAccountErrorKind::InvalidInputException(inner) => {
                    Error::InvalidInputException(inner)
                }
                crate::error::AssociateMemberAccountErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::AssociateMemberAccountErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::AssociateS3ResourcesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::AssociateS3ResourcesError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::AssociateS3ResourcesErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::AssociateS3ResourcesErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::AssociateS3ResourcesErrorKind::InvalidInputException(inner) => {
                    Error::InvalidInputException(inner)
                }
                crate::error::AssociateS3ResourcesErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::AssociateS3ResourcesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DisassociateMemberAccountError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::DisassociateMemberAccountError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DisassociateMemberAccountErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::DisassociateMemberAccountErrorKind::InvalidInputException(inner) => {
                    Error::InvalidInputException(inner)
                }
                crate::error::DisassociateMemberAccountErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DisassociateS3ResourcesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::DisassociateS3ResourcesError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DisassociateS3ResourcesErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::DisassociateS3ResourcesErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::DisassociateS3ResourcesErrorKind::InvalidInputException(inner) => {
                    Error::InvalidInputException(inner)
                }
                crate::error::DisassociateS3ResourcesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ListMemberAccountsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::ListMemberAccountsError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListMemberAccountsErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::ListMemberAccountsErrorKind::InvalidInputException(inner) => {
                    Error::InvalidInputException(inner)
                }
                crate::error::ListMemberAccountsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ListS3ResourcesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::ListS3ResourcesError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListS3ResourcesErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::ListS3ResourcesErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::ListS3ResourcesErrorKind::InvalidInputException(inner) => {
                    Error::InvalidInputException(inner)
                }
                crate::error::ListS3ResourcesErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::UpdateS3ResourcesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::UpdateS3ResourcesError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateS3ResourcesErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::UpdateS3ResourcesErrorKind::InternalException(inner) => {
                    Error::InternalException(inner)
                }
                crate::error::UpdateS3ResourcesErrorKind::InvalidInputException(inner) => {
                    Error::InvalidInputException(inner)
                }
                crate::error::UpdateS3ResourcesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}