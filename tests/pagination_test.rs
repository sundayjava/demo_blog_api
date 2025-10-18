use role_base_auth::models::common::{PaginatedResponse, PaginationParams};

#[test]
fn test_pagination_offset() {
    let params = PaginationParams { page: 1, limit: 10 };
    assert_eq!(params.offset(), 0);

    let params = PaginationParams { page: 2, limit: 10 };
    assert_eq!(params.offset(), 10);

    let params = PaginationParams { page: 3, limit: 20 };
    assert_eq!(params.offset(), 40);
}

#[test]
fn test_pagination_validation() {
    let mut params = PaginationParams { page: 0, limit: 10 };
    params.validate();
    assert_eq!(params.page, 1);

    let mut params = PaginationParams { page: 5, limit: 0 };
    params.validate();
    assert_eq!(params.limit, 20);

    let mut params = PaginationParams {
        page: 1,
        limit: 200,
    };
    params.validate();
    assert_eq!(params.limit, 100);
}

#[test]
fn test_paginated_response() {
    let data = vec![1, 2, 3, 4, 5];
    let response = PaginatedResponse::new(data, 1, 10, 50);

    assert_eq!(response.page, 1);
    assert_eq!(response.limit, 10);
    assert_eq!(response.total, 50);
    assert_eq!(response.total_pages, 5);
    assert_eq!(response.data.len(), 5);
}

#[test]
fn test_paginated_response_empty() {
    let data: Vec<i32> = vec![];
    let response = PaginatedResponse::new(data, 1, 10, 0);

    assert_eq!(response.total_pages, 0);
}
