use crate::WebSession;
use axum::Router;
use qtest::utils::SoC;

pub trait Platform: SoC + Clone + Send + Sync + 'static {
    /// Add platform specific HTTP routes to the Axum [`Router`].
    ///
    /// By default, this method returns the router unchanged (i.e., no additional routes).
    fn add_routes(router: Router<WebSession<Self>>) -> Router<WebSession<Self>> {
        router
    }
}

// Default empty implementation for platforms that do not need custom routes.
impl Platform for () {}
