use actix_web::dev::{Payload, PayloadStream};
use actix_web::Error;
use actix_web::FromRequest;
use actix_web::HttpRequest;
use shaku::{HasComponent, Interface, ModuleInterface};
use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::Arc;
// mod util;
// use crate::util::get_module_from_state;
// use crate::util::get;
// use crate::actix_handler::util::get_module_from_state;
use crate::actix_handler::util::get_module_from_component_state;

// pub struct Inject<M: ModuleInterface + HasComponent<I>, I: Interface + ?Sized>(
//   Arc<I>,
//   PhantomData<M>,
// );

// impl<M: ModuleInterface + HasComponent<I>, I: Interface + ?Sized> FromRequest for Inject<M, I> {
//   type Config = ();
//   type Error = Error;
//   type Future = futures_util::future::Ready<Result<Self, Error>>;
//   fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
//     let module = match get_module_from_state::<M, I>(&req) {
//       Ok(module) => module,
//       Err(e) => return futures_util::future::err(e),
//     };
//     let component = module.resolve();
//     futures_util::future::ok(Inject(component, PhantomData))
//   }
// }

// impl<M: ModuleInterface + HasComponent<I>, I: Interface + ?Sized> Deref for Inject<M, I> {
//   type Target = I;

//   fn deref(&self) -> &Self::Target {
//     Arc::as_ref(&self.0)
//   }
// }

//

pub struct Inject<M: ModuleInterface + HasComponent<I> + ?Sized, I: Interface + ?Sized>(
  Arc<I>,
  PhantomData<M>,
);

impl<M: ModuleInterface + HasComponent<I> + ?Sized, I: Interface + ?Sized> FromRequest
  for Inject<M, I>
{
  type Error = Error;
  type Future = futures_util::future::Ready<Result<Self, Error>>;
  type Config = ();

  fn from_request(req: &HttpRequest, _: &mut Payload<PayloadStream>) -> Self::Future {
    let module = match get_module_from_component_state::<M, I>(&req) {
      Ok(module) => module,
      Err(e) => return futures_util::future::err(e),
    };
    let component = module.resolve();

    futures_util::future::ok(Inject(component, PhantomData))
  }
}

impl<M: ModuleInterface + HasComponent<I> + ?Sized, I: Interface + ?Sized> Deref for Inject<M, I> {
  type Target = I;

  fn deref(&self) -> &Self::Target {
    Arc::as_ref(&self.0)
  }
}
// let module = req.app_data::<Arc<AppModule>>().map(Arc::as_ref).ok_or_else(|| ErrorInternalServerError("fail"));
// let module = get_module_from_state::<AppModule>(&req).unwarp();
