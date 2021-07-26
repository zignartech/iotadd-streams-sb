use std::cell::RefCell;
use std::rc::Rc;
use rxrust::prelude::of::OfEmitter;
use rxrust::prelude::*;
use crate::actix_utils::callback_to_future::FutureWithReturn;
pub fn pollObservable<T>(o: ObservableBase<OfEmitter<T>>) -> FutureWithReturn<T> {
  let item = Rc::new(RefCell::new(None));
  {
    let item = item.clone();
    o.subscribe(move |x| {
      *item.borrow_mut() = Some(x)
    });
  }
  FutureWithReturn { item }
}