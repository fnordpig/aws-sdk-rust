// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    BadRequestException(crate::error::BadRequestException),
    ConflictException(crate::error::ConflictException),
    ForbiddenException(crate::error::ForbiddenException),
    InternalServerErrorException(crate::error::InternalServerErrorException),
    NotFoundException(crate::error::NotFoundException),
    UnauthorizedException(crate::error::UnauthorizedException),
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BadRequestException(inner) => inner.fmt(f),
            Error::ConflictException(inner) => inner.fmt(f),
            Error::ForbiddenException(inner) => inner.fmt(f),
            Error::InternalServerErrorException(inner) => inner.fmt(f),
            Error::NotFoundException(inner) => inner.fmt(f),
            Error::UnauthorizedException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::CreateBrokerError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::CreateBrokerError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateBrokerErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::CreateBrokerErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::CreateBrokerErrorKind::ForbiddenException(inner) => {
                    Error::ForbiddenException(inner)
                }
                crate::error::CreateBrokerErrorKind::InternalServerErrorException(inner) => {
                    Error::InternalServerErrorException(inner)
                }
                crate::error::CreateBrokerErrorKind::UnauthorizedException(inner) => {
                    Error::UnauthorizedException(inner)
                }
                crate::error::CreateBrokerErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::CreateConfigurationError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::CreateConfigurationError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateConfigurationErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::CreateConfigurationErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::CreateConfigurationErrorKind::ForbiddenException(inner) => {
                    Error::ForbiddenException(inner)
                }
                crate::error::CreateConfigurationErrorKind::InternalServerErrorException(inner) => {
                    Error::InternalServerErrorException(inner)
                }
                crate::error::CreateConfigurationErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::CreateTagsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::CreateTagsError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateTagsErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::CreateTagsErrorKind::ForbiddenException(inner) => {
                    Error::ForbiddenException(inner)
                }
                crate::error::CreateTagsErrorKind::InternalServerErrorException(inner) => {
                    Error::InternalServerErrorException(inner)
                }
                crate::error::CreateTagsErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::CreateTagsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::CreateUserError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::CreateUserError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateUserErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::CreateUserErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::CreateUserErrorKind::ForbiddenException(inner) => {
                    Error::ForbiddenException(inner)
                }
                crate::error::CreateUserErrorKind::InternalServerErrorException(inner) => {
                    Error::InternalServerErrorException(inner)
                }
                crate::error::CreateUserErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::CreateUserErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DeleteBrokerError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteBrokerError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteBrokerErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::DeleteBrokerErrorKind::ForbiddenException(inner) => {
                    Error::ForbiddenException(inner)
                }
                crate::error::DeleteBrokerErrorKind::InternalServerErrorException(inner) => {
                    Error::InternalServerErrorException(inner)
                }
                crate::error::DeleteBrokerErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::DeleteBrokerErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DeleteTagsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteTagsError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteTagsErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::DeleteTagsErrorKind::ForbiddenException(inner) => {
                    Error::ForbiddenException(inner)
                }
                crate::error::DeleteTagsErrorKind::InternalServerErrorException(inner) => {
                    Error::InternalServerErrorException(inner)
                }
                crate::error::DeleteTagsErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::DeleteTagsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DeleteUserError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::DeleteUserError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteUserErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::DeleteUserErrorKind::ForbiddenException(inner) => {
                    Error::ForbiddenException(inner)
                }
                crate::error::DeleteUserErrorKind::InternalServerErrorException(inner) => {
                    Error::InternalServerErrorException(inner)
                }
                crate::error::DeleteUserErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::DeleteUserErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DescribeBrokerError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeBrokerError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeBrokerErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::DescribeBrokerErrorKind::ForbiddenException(inner) => {
                    Error::ForbiddenException(inner)
                }
                crate::error::DescribeBrokerErrorKind::InternalServerErrorException(inner) => {
                    Error::InternalServerErrorException(inner)
                }
                crate::error::DescribeBrokerErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::DescribeBrokerErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DescribeBrokerEngineTypesError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::DescribeBrokerEngineTypesError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeBrokerEngineTypesErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::DescribeBrokerEngineTypesErrorKind::ForbiddenException(inner) => {
                    Error::ForbiddenException(inner)
                }
                crate::error::DescribeBrokerEngineTypesErrorKind::InternalServerErrorException(
                    inner,
                ) => Error::InternalServerErrorException(inner),
                crate::error::DescribeBrokerEngineTypesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DescribeBrokerInstanceOptionsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::DescribeBrokerInstanceOptionsError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DescribeBrokerInstanceOptionsErrorKind::BadRequestException(inner) => Error::BadRequestException(inner),
                crate::error::DescribeBrokerInstanceOptionsErrorKind::ForbiddenException(inner) => Error::ForbiddenException(inner),
                crate::error::DescribeBrokerInstanceOptionsErrorKind::InternalServerErrorException(inner) => Error::InternalServerErrorException(inner),
                crate::error::DescribeBrokerInstanceOptionsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DescribeConfigurationError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::DescribeConfigurationError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeConfigurationErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::DescribeConfigurationErrorKind::ForbiddenException(inner) => {
                    Error::ForbiddenException(inner)
                }
                crate::error::DescribeConfigurationErrorKind::InternalServerErrorException(
                    inner,
                ) => Error::InternalServerErrorException(inner),
                crate::error::DescribeConfigurationErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::DescribeConfigurationErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DescribeConfigurationRevisionError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::DescribeConfigurationRevisionError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DescribeConfigurationRevisionErrorKind::BadRequestException(inner) => Error::BadRequestException(inner),
                crate::error::DescribeConfigurationRevisionErrorKind::ForbiddenException(inner) => Error::ForbiddenException(inner),
                crate::error::DescribeConfigurationRevisionErrorKind::InternalServerErrorException(inner) => Error::InternalServerErrorException(inner),
                crate::error::DescribeConfigurationRevisionErrorKind::NotFoundException(inner) => Error::NotFoundException(inner),
                crate::error::DescribeConfigurationRevisionErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::DescribeUserError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::DescribeUserError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeUserErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::DescribeUserErrorKind::ForbiddenException(inner) => {
                    Error::ForbiddenException(inner)
                }
                crate::error::DescribeUserErrorKind::InternalServerErrorException(inner) => {
                    Error::InternalServerErrorException(inner)
                }
                crate::error::DescribeUserErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::DescribeUserErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ListBrokersError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::ListBrokersError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListBrokersErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::ListBrokersErrorKind::ForbiddenException(inner) => {
                    Error::ForbiddenException(inner)
                }
                crate::error::ListBrokersErrorKind::InternalServerErrorException(inner) => {
                    Error::InternalServerErrorException(inner)
                }
                crate::error::ListBrokersErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ListConfigurationRevisionsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::ListConfigurationRevisionsError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListConfigurationRevisionsErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::ListConfigurationRevisionsErrorKind::ForbiddenException(inner) => {
                    Error::ForbiddenException(inner)
                }
                crate::error::ListConfigurationRevisionsErrorKind::InternalServerErrorException(
                    inner,
                ) => Error::InternalServerErrorException(inner),
                crate::error::ListConfigurationRevisionsErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::ListConfigurationRevisionsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ListConfigurationsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::ListConfigurationsError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListConfigurationsErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::ListConfigurationsErrorKind::ForbiddenException(inner) => {
                    Error::ForbiddenException(inner)
                }
                crate::error::ListConfigurationsErrorKind::InternalServerErrorException(inner) => {
                    Error::InternalServerErrorException(inner)
                }
                crate::error::ListConfigurationsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ListTagsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::ListTagsError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListTagsErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::ListTagsErrorKind::ForbiddenException(inner) => {
                    Error::ForbiddenException(inner)
                }
                crate::error::ListTagsErrorKind::InternalServerErrorException(inner) => {
                    Error::InternalServerErrorException(inner)
                }
                crate::error::ListTagsErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::ListTagsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::ListUsersError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::ListUsersError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListUsersErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::ListUsersErrorKind::ForbiddenException(inner) => {
                    Error::ForbiddenException(inner)
                }
                crate::error::ListUsersErrorKind::InternalServerErrorException(inner) => {
                    Error::InternalServerErrorException(inner)
                }
                crate::error::ListUsersErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::ListUsersErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::RebootBrokerError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::RebootBrokerError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::RebootBrokerErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::RebootBrokerErrorKind::ForbiddenException(inner) => {
                    Error::ForbiddenException(inner)
                }
                crate::error::RebootBrokerErrorKind::InternalServerErrorException(inner) => {
                    Error::InternalServerErrorException(inner)
                }
                crate::error::RebootBrokerErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::RebootBrokerErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::UpdateBrokerError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::UpdateBrokerError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateBrokerErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::UpdateBrokerErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::UpdateBrokerErrorKind::ForbiddenException(inner) => {
                    Error::ForbiddenException(inner)
                }
                crate::error::UpdateBrokerErrorKind::InternalServerErrorException(inner) => {
                    Error::InternalServerErrorException(inner)
                }
                crate::error::UpdateBrokerErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::UpdateBrokerErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::UpdateConfigurationError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::UpdateConfigurationError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateConfigurationErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::UpdateConfigurationErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::UpdateConfigurationErrorKind::ForbiddenException(inner) => {
                    Error::ForbiddenException(inner)
                }
                crate::error::UpdateConfigurationErrorKind::InternalServerErrorException(inner) => {
                    Error::InternalServerErrorException(inner)
                }
                crate::error::UpdateConfigurationErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::UpdateConfigurationErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::UpdateUserError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: smithy_http::result::SdkError<crate::error::UpdateUserError, R>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateUserErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::UpdateUserErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::UpdateUserErrorKind::ForbiddenException(inner) => {
                    Error::ForbiddenException(inner)
                }
                crate::error::UpdateUserErrorKind::InternalServerErrorException(inner) => {
                    Error::InternalServerErrorException(inner)
                }
                crate::error::UpdateUserErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::UpdateUserErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}