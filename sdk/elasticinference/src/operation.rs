// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>
/// Describes the locations in which a given accelerator type or set of types is present in a given region.
/// </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeAcceleratorOfferings {
    _private: (),
}
impl DescribeAcceleratorOfferings {
    /// Creates a new builder-style object to manufacture [`DescribeAcceleratorOfferingsInput`](crate::input::DescribeAcceleratorOfferingsInput)
    pub fn builder() -> crate::input::describe_accelerator_offerings_input::Builder {
        crate::input::describe_accelerator_offerings_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeAcceleratorOfferings {
    type Output = std::result::Result<
        crate::output::DescribeAcceleratorOfferingsOutput,
        crate::error::DescribeAcceleratorOfferingsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_accelerator_offerings_error(response)
        } else {
            crate::operation_deser::parse_describe_accelerator_offerings_response(response)
        }
    }
}

/// <p>
/// Describes information over a provided set of accelerators belonging to an account.
/// </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeAccelerators {
    _private: (),
}
impl DescribeAccelerators {
    /// Creates a new builder-style object to manufacture [`DescribeAcceleratorsInput`](crate::input::DescribeAcceleratorsInput)
    pub fn builder() -> crate::input::describe_accelerators_input::Builder {
        crate::input::describe_accelerators_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeAccelerators {
    type Output = std::result::Result<
        crate::output::DescribeAcceleratorsOutput,
        crate::error::DescribeAcceleratorsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_accelerators_error(response)
        } else {
            crate::operation_deser::parse_describe_accelerators_response(response)
        }
    }
}

/// <p>
/// Describes the accelerator types available in a given region, as well as their characteristics, such as memory and throughput.
/// </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeAcceleratorTypes {
    _private: (),
}
impl DescribeAcceleratorTypes {
    /// Creates a new builder-style object to manufacture [`DescribeAcceleratorTypesInput`](crate::input::DescribeAcceleratorTypesInput)
    pub fn builder() -> crate::input::describe_accelerator_types_input::Builder {
        crate::input::describe_accelerator_types_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeAcceleratorTypes {
    type Output = std::result::Result<
        crate::output::DescribeAcceleratorTypesOutput,
        crate::error::DescribeAcceleratorTypesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_accelerator_types_error(response)
        } else {
            crate::operation_deser::parse_describe_accelerator_types_response(response)
        }
    }
}

/// <p>
/// Returns all tags of an Elastic Inference Accelerator.
/// </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListTagsForResource {
    _private: (),
}
impl ListTagsForResource {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceInput`](crate::input::ListTagsForResourceInput)
    pub fn builder() -> crate::input::list_tags_for_resource_input::Builder {
        crate::input::list_tags_for_resource_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListTagsForResource {
    type Output = std::result::Result<
        crate::output::ListTagsForResourceOutput,
        crate::error::ListTagsForResourceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_tags_for_resource_error(response)
        } else {
            crate::operation_deser::parse_list_tags_for_resource_response(response)
        }
    }
}

/// <p>
/// Adds the specified tags to an Elastic Inference Accelerator.
/// </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct TagResource {
    _private: (),
}
impl TagResource {
    /// Creates a new builder-style object to manufacture [`TagResourceInput`](crate::input::TagResourceInput)
    pub fn builder() -> crate::input::tag_resource_input::Builder {
        crate::input::tag_resource_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for TagResource {
    type Output =
        std::result::Result<crate::output::TagResourceOutput, crate::error::TagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_tag_resource_error(response)
        } else {
            crate::operation_deser::parse_tag_resource_response(response)
        }
    }
}

/// <p>
/// Removes the specified tags from an Elastic Inference Accelerator.
/// </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UntagResource {
    _private: (),
}
impl UntagResource {
    /// Creates a new builder-style object to manufacture [`UntagResourceInput`](crate::input::UntagResourceInput)
    pub fn builder() -> crate::input::untag_resource_input::Builder {
        crate::input::untag_resource_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UntagResource {
    type Output =
        std::result::Result<crate::output::UntagResourceOutput, crate::error::UntagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_untag_resource_error(response)
        } else {
            crate::operation_deser::parse_untag_resource_response(response)
        }
    }
}