mod auth;
mod category;
mod posts;
mod user;
mod comments;

use std::sync::Arc;

use axum::Router;
use tokio::net::TcpListener;

use crate::state::AppState;

pub use self::auth::auth_routes;
pub use self::category::category_routes;
pub use self::comments::comment_routes;
pub use self::posts::post_routes;
pub use self::user::user_routes;

pub struct AppRouter;

impl AppRouter {
    pub async fn serve(port: u16, app_state: AppState) -> Result<(), Box<dyn std::error::Error>> {
       
        let shared_state = Arc::new(app_state);

        
        let router = Router::new()
            .merge(auth_routes(shared_state.clone()))
            .merge(category_routes(shared_state.clone()))
            .merge(comment_routes(shared_state.clone()))
            .merge(post_routes(shared_state.clone()))
            .merge(user_routes(shared_state.clone()));
            

       
        let addr = format!("0.0.0.0:{}", port);
        let listener = TcpListener::bind(addr).await?;
        println!("Server running on http://{}", listener.local_addr()?);

        axum::serve(listener, router).await.unwrap();
        Ok(())
    }
}