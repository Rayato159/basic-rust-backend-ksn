# 🤖 Development Protocol: Clean Architecture (Rust)

This document defines the strict development pattern for adding new features (referred to as **`[DomainName]`**) to this project. Follow these steps in order.

## 🏗️ 1. Context & Layering
We use a **Clean Architecture** structure with 3 layers (inner to outer):
1. **Repository Layer:** Database interaction (Query only).
2. **UseCase Layer:** Business Logic & Workflow orchestration.
3. **Route/Controller Layer:** HTTP delivery (Axum & Swagger).

---

## 🛠️ 2. Workflow Sequence

### Step 1: Data Preparation
- **Check Structure:** Locate existing modules before writing code.
- **DTO (Data Transfer Object):** Always start by creating the **DTO**. Cross-check with the `migration` package to ensure only necessary database fields are included.
- **Model (Request/Input):** Create the Model for user input. Only include fields the user actually needs to provide.
- **Adapters:** Implement `From` and `To` patterns for seamless conversion between Models and DTOs.

### Step 2: Infrastructure Repository (Implementation)
Implement the database-specific logic using the following pattern. **Strictly NO business logic here.**

pub struct [DomainName]MSSQL {
    db_client: Arc<Mutex<MSSQLClient>>,
}

impl [DomainName]MSSQL {
    pub fn new(db_client: Arc<Mutex<MSSQLClient>>) -> Self {
        Self { db_client }
    }
}

#[async_trait]
impl [DomainName]Repository for [DomainName]MSSQL {
    async fn function_name(&self, ...) -> Result<...> {
        // SQL Queries only to ensure testability/mocking
        // No business logic allowed here.
    }
}

### Step 3: Domain Repository (Trait)
Define the repository interface in the domain layer for mocking and testing.

#[async_trait]
#[automock]
pub trait [DomainName]Repository {
    async fn function_name(&self, ...) -> Result<...>;
}

### Step 4: UseCase Layer (Logic)
Apply the business logic and orchestrate data flow here.

pub struct [DomainName]UseCase<T>
where
    T: [DomainName]Repository + Send + Sync + 'static,
{
    [domain_name]_repository: Arc<T>,
}

impl<T> [DomainName]UseCase<T>
where
    T: [DomainName]Repository + Send + Sync + 'static,
{
    pub fn new([domain_name]_repository: Arc<T>) -> Self {
        Self { [domain_name]_repository }
    }

    pub async fn function_name(&self, ...) -> Result<....> {
        // Business logic goes here
    }
}

### Step 5: Route Layer & Documentation
Expose the feature via HTTP using Axum and update Swagger documentation via Utoipa.

pub fn routes(db_pool: Arc<Mutex<MSSQLClient>>) -> Router {
    let [domain_name]_repository = [DomainName]MSSQL::new(db_pool);
    let [domain_name]_use_case = [DomainName]UseCase::new(Arc::new([domain_name]_repository));

    Router::new()
        .route("/", [http_method](function_name))
        .with_state(Arc::new([domain_name]_use_case))
}

#[utoipa::path(
    [method],
    path = "...",
    request_body = [ModelName],
    responses(
        (status = [SuccessCode], description = "...", body = [ResultName]),
        (status = [ErrorCode], description = "...", body = String)
    ),
    tag = "[DomainName]"
)]
pub async fn function_name(
    State([domain_name]_use_case): State<Arc<[DomainName]UseCase<[DomainName]MSSQL>>>,
    Json([domain_name]_model): Json<[DomainName]Model>,
) -> impl IntoResponse {
    match [domain_name]_use_case.function_name(...).await {
        Ok(result) => (StatusCode::[SuccessType], Json(result)).into_response(),
        Err(e) => (StatusCode::[ErrorType], e.to_string()).into_response(),
    }
}

---

## ✅ 3. Finalization
1. **Update ApiDocs:** Inject the new route into the `ApiDoc` struct and register the sub-router in `http/mod.rs`.
2. **Quality Check:** Run `cargo check`. Resolve all errors immediately. If the issue is critical and unresolvable, stop and report.
