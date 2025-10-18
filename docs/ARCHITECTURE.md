# Architecture Overview

## Database Access Pattern

### DbContext - Universal Data Access Layer

The application uses a **single, universal `DbContext`** as the primary data access layer. This is the recommended approach for applications of this size.

#### Key Design Decisions

**Why One DbContext?**

- ✅ Simpler architecture - single entry point to database
- ✅ Easier transaction management (if needed in future)
- ✅ Less boilerplate code
- ✅ Easy to add new tables/models
- ✅ Sufficient scalability for small/medium applications

**When to Consider Separate Contexts:**

- Multiple databases (microservices)
- Very large applications (hundreds of tables)
- Different connection pooling strategies per domain

### Usage Pattern

All server functions should use `DbContext`:

```rust
#[server]
pub async fn get_blogposts() -> Result<Vec<Post>, ServerFnError> {
    let ctx = DbContext::new()?;
    ctx.get_all_posts().await
}
```

### Architecture Layers

```
┌─────────────────────────────────┐
│   Leptos Components (UI)        │
└────────────┬────────────────────┘
             │
             │ #[server] functions
             ▼
┌─────────────────────────────────┐
│   Server Functions              │
│   (src/server/post.rs)          │
└────────────┬────────────────────┘
             │
             │ DbContext
             ▼
┌─────────────────────────────────┐
│   DbContext                     │
│   (src/server/database/)        │
│   - Single access point         │
│   - Error conversion            │
└────────────┬────────────────────┘
             │
             │ Repository trait
             ▼
┌─────────────────────────────────┐
│   Repository Implementations    │
│   (src/server/database/mod.rs)  │
│   - Post                        │
│   - Tag (future)                │
└────────────┬────────────────────┘
             │
             │ SQLx queries
             ▼
┌─────────────────────────────────┐
│   PostgreSQL Database           │
└─────────────────────────────────┘
```

### Adding New Models

To add a new model (e.g., `Comment`):

1. **Create the model** in `src/models/comment.rs`
2. **Implement Repository trait** in `src/server/database/mod.rs`:

   ```rust
   #[async_trait]
   impl Repository for Comment {
       type Entity = Comment;
       // implement get_all, get_by_id, insert, update, delete
   }
   ```

3. **Add methods to DbContext** in `src/server/database/db_context.rs`:

   ```rust
   pub async fn get_all_comments(&self) -> Result<Vec<Comment>, ServerFnError> {
       Comment::get_all(&self.pool).await
           .map_err(|e| ServerFnError::ServerError(format!("Failed: {e}")))
   }
   ```

4. **Create server functions** in `src/server/comment.rs`:

   ```rust
   #[server]
   pub async fn get_comments() -> Result<Vec<Comment>, ServerFnError> {
       let ctx = DbContext::new()?;
       ctx.get_all_comments().await
   }
   ```

### Benefits of This Architecture

1. **Testability**: DbContext can be mocked for unit tests
2. **Centralized error handling**: All SQLx errors converted to ServerFnError in one place
3. **Type safety**: Full type checking at compile time with SQLx macros
4. **Clear separation of concerns**: UI → Server Functions → DbContext → Repository → Database
5. **Easy to extend**: Adding new models follows a clear pattern

### Future Enhancements

Consider these when the application grows:

- **Caching layer** in DbContext for frequently accessed data
- **Transaction support** via `DbContext::transaction()` method
- **Query builders** for complex filtering/pagination
- **Connection pooling tuning** based on load
- **Read replicas** for read-heavy operations
