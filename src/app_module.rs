// use crate::app_service::HasComponent;
use crate::app_http_controller::AppHttpController;
use crate::app_service::AppService;
use shaku::{module};
// trait MyModule: HasProvider<dyn IAppHttpController +'static> {}

module! {
  pub AppModule {
      components = [AppHttpController,AppService],
      // components = [],
      // providers = [AppService]
      providers = []
      // providers = [AppHttpController,AppService],
  }
}

// let a = AppModule;