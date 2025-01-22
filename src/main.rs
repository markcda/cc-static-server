#![deny(warnings, clippy::todo, clippy::unimplemented)]

mod static_routes;

use cc_server_kit::prelude::*;

#[derive(Default, Clone)]
struct Setup {
  generic_values: GenericValues,
}

impl GenericSetup for Setup {
  fn generic_values(&self) -> &GenericValues {
    &self.generic_values
  }
  fn set_generic_values(&mut self, generic_values: GenericValues) {
    self.generic_values = generic_values;
  }
}

#[tokio::main]
async fn main() {
  let setup = load_generic_config::<Setup>("static-server").await.unwrap();
  let state = load_generic_state(&setup).await.unwrap();
  let router = get_root_router(&state).push(static_routes::frontend_router());
  let (server, _handler) = start(state, &setup, router).await.unwrap();
  server.await
}
