#[derive(utoipa::OpenApi)]
#[openapi(
    paths(hello),
    components(schemas(HelloResponse)),
    tags(
        (name = "Hello", description = "Hello API endpoints")
    )
)]
struct ApiDoc;