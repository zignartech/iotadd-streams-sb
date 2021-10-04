// use crate::actix_handler::inject_component::Inject;
// use crate::app_http_controller::IAppHttpController;
// use crate::app_module::AppModule;
use crate::models::dtos::create_author_dto::{CreateAuthorBody, CreateAuthorQuery};
use crate::models::dtos::fetch_all_dto::FetchAllQuery;
use crate::models::dtos::create_subscribe_dto::CreateSubscriberQuery;
use crate::models::dtos::send_one_dto::SendOneQuery;
// use crate::rx_utils::poll_observable::pollObservable;
use actix_web::HttpResponse;
use actix_web::{post, web, web::Json, web::Query, Responder};
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;

use iota_streams::app::transport::tangle::PAYLOAD_BYTES;

// use crate::models::schemas::author_schema::Address as IAddress;
// use crate::models::schemas::author_schema::Author as IAuthor;
// use crate::models::schemas::author_schema::AuthorSchema;
use crate::streams_utils::random_seed::randomSeed;
use base64::{decode_config, encode_config, URL_SAFE_NO_PAD};
use futures::executor::block_on;
use iota_streams::app::message::GenericMessage;
use iota_streams::app::transport::tangle::client::iota_client::Client as OtherClient;
use iota_streams::app::transport::tangle::client::{Client, SendOptions};
use iota_streams::app::transport::tangle::{AppInst, MsgId, TangleAddress};
use iota_streams::app_channels::api::tangle::Author;
use iota_streams::app_channels::api::tangle::MessageContent;
use iota_streams::app_channels::api::tangle::Subscriber;
use iota_streams::ddml::types::Bytes;
use rxrust::observable;
use rxrust::observable::of::OfEmitter;
use rxrust::observable::ObservableBase;
use shaku::{Component, Interface};


#[post("/author")]
pub async fn index(
  //httpController: Inject<AppModule, dyn IAppHttpController>,
  query: Query<CreateAuthorQuery>,
  body: Json<CreateAuthorBody>,
) -> HttpResponse {
  //let author = httpController.createAuthor(query.0.clone(),body.0.clone());

  // from postman
  let rs = query.into_inner().randomSeed;
  let seed = body.into_inner().seed;

  //
  let send_options: SendOptions = SendOptions {
    url: std::env::var("NODE").unwrap(),
    local_pow: false,
    depth: 3,
    threads: 2,
  };

  let iota_client = block_on(
    OtherClient::builder()
      .with_node(&std::env::var("NODE").unwrap())
      .unwrap()
      .with_local_pow(false)
      .finish(),
  )
  .unwrap();

  let client = Client::new(send_options, iota_client);

  let mut author: Author<Client> =
   // Author::new(&createAuthorBody.seed,"utf-8", ChannelType::SingleBranch, client);
   Author::new(&seed,"utf-8", PAYLOAD_BYTES, false,client );
  let j = author.send_announce().await; //let annAddress: TangleAddress

  let annAddress: TangleAddress = match j {
    Ok(v) => v,
    _ => {
      return HttpResponse::Ok()
        .content_type("application/json")
        .json("author error");
    }
  };

  let password = randomSeed(12);
  let exported = author.export(&password.clone()).unwrap();
  let encodedExported = encode_config(exported.clone(), URL_SAFE_NO_PAD);

  // result
  /*
  observable::of(AuthorSchema {
    seed: createAuthorQuery.randomSeed.clone().to_string(),
    address: IAddress {
      appInst: annAddress.appinst.to_string(),
      msgId: annAddress.msgid.to_string(),
    },
    author: IAuthor {
      password: password.clone(),
      state: encodedExported,
    },
  });*/

  let result = json!({
      "seed": rs.to_string(),
      "address": {
          "appInst":annAddress.appinst.to_string(),
          "msgId": annAddress.msgid.to_string(),
      },
      "author":{
        "password": password.clone(),
        "state": encodedExported,
      },
  });

  HttpResponse::Ok().json(result)
}

#[post("/create_subscriber")]
pub async fn createSubscriber(
  query: Query<CreateSubscriberQuery>,
  body: Json<CreateAuthorBody>,
) -> HttpResponse {

    // from postman
    // let rs = query.into_inner().randomSeed;
    let seed = body.into_inner().seed;

    let q = query.into_inner();
    let address = q.address;
    // let author = q.author;
    //
    let send_options: SendOptions = SendOptions {
      url: std::env::var("NODE").unwrap(),
      local_pow: false,
      depth: 3,
      threads: 2,
    };
  
    let iota_client = block_on(
      OtherClient::builder()
        .with_node(&std::env::var("NODE").unwrap())
        .unwrap()
        .with_local_pow(false)
        .finish(),
    )
    .unwrap();
  
    let client = Client::new(send_options, iota_client);

    let mut subscriberA = Subscriber::new(&seed, "utf-8", PAYLOAD_BYTES, client);

    let announcement_link =
    TangleAddress::from_str(&address.appInst.clone(), &address.msgId.clone()).unwrap();

    let result = subscriberA.receive_announcement(&announcement_link).await;
        print!("  SubscriberA: {}", subscriberA);

    match result {
      Ok(v) => println!("receive_announcement ok"),
      _ => println!("receive_announcement error"),
    };
        // if author.channel_address() == subscriberA.channel_address(){
        //   println!("Channel address matched {}")
        // }
    if 
    subscriberA
    .channel_address()
    .map_or(false, |appinst| appinst == &announcement_link.appinst){
      println!("Appinst matched");
    } else{
      println!("Appinst Miss matched");
    }

    let subscribeA_link = {
      let msg = subscriberA.send_subscribe(&announcement_link).await.expect("Send subscribe error");
      println!("  msg => <{}> {:?}", msg.msgid, msg);
      print!("  SubscriberA: {}", subscriberA);
      msg
  };
    // author.receive_subscribe(&subscribeA_link)?;
    // print!("  Author     : {}", author); 

    // subscriberA.receive_keyload(&previous_msg_link)?;
    // print!("  SubscriberA: {}", subscriberA);
  
    // let (_signer_pk, unwrapped_public, unwrapped_masked) = subscriberA.receive_signed_packet(&previous_msg_link)?;
    // print!("  SubscriberA: {}", subscriberA);
    // try_or!(
    //     public_payload == unwrapped_public,
    //     PublicPayloadMismatch(public_payload.to_string(), unwrapped_public.to_string())
    // )?;
    // try_or!(
    //     masked_payload == unwrapped_masked,
    //     PublicPayloadMismatch(masked_payload.to_string(), unwrapped_masked.to_string())
    // )?;

    // println!("\nSubscriber A fetching transactions...");
    // utils::s_fetch_next_messages(&mut subscriberA);

    // println!("\nTagged packet 1 - SubscriberA");
    // let previous_msg_link = {
    //     let (msg, seq) = subscriberA.send_tagged_packet(&previous_msg_link, &public_payload, &masked_payload)?;
    //     println!("  msg => <{}> {:?}", msg.msgid, msg);
    //     panic_if_not(seq.is_none());
    //     print!("  SubscriberA: {}", subscriberA);
    //     msg
    // };

  //   let result = json!({
  //     "seed": rs.to_string(),
  //     "address": {
  //         "appInst":annAddress.appinst.to_string(),
  //         "msgId": annAddress.msgid.to_string(),
  //     },
  //     "author":{
  //       "password": password.clone(),
  //       "state": encodedExported,
  //     },
  // });

  HttpResponse::Ok().json("Subscribe ok")

}

#[post("/address/sendOne")]
pub async fn addressSendOne(
  //httpController: Inject<AppModule, dyn IAppHttpController>,
  query: Query<SendOneQuery>,
  bytes: web::Bytes,
) -> HttpResponse {
  let q = query.into_inner();
  let address = q.address;
  let author = q.author;

  let s = String::from_utf8(bytes.to_vec()).unwrap();
  let json: HashMap<String, Value> = serde_json::from_str(&s).unwrap();

  //let client = Client::new_from_url(&std::env::var("NODE").unwrap());

  let send_options: SendOptions = SendOptions {
    url: std::env::var("NODE").unwrap(),
    local_pow: false,
    depth: 3,
    threads: 1,
  };

  let iota_client = block_on(
    OtherClient::builder()
      .with_node(&std::env::var("NODE").unwrap())
      .unwrap()
      .with_local_pow(false)
      .finish(),
  )
  .unwrap();
  // send_keyload_for_everyone()
  let client = Client::new(send_options, iota_client);

  let payloadStr = serde_json::to_string(&json).unwrap();
  let payload = encode_config(&payloadStr, URL_SAFE_NO_PAD);
  let public = Bytes(payload.as_bytes().to_vec());

  let keyLoadLink =
    TangleAddress::from_str(&address.appInst.clone(), &address.msgId.clone()).unwrap();

  let mut author: Author<Client> = Author::import(
    &decode_config(author.state.clone(), URL_SAFE_NO_PAD).unwrap(),
    &author.password.clone(),
    client.clone(),
  )
  .unwrap();
  let _ = author.fetch_state().unwrap();
  let _ = author.fetch_all_next_msgs().await;

  let (msg_link, _seq_link) = author
    .send_signed_packet(&keyLoadLink, &public, &Bytes::default())
    .await.unwrap();
  //.unwrap();

  // let (retPreviosMsgTag, _): (TangleAddress, _) = match result {
  //   Ok(v) => v,
  //   _ => {
  //     return HttpResponse::Ok()
  //       .content_type("application/json")
  //       .json("sendone error");
  //   }
  // };
  /*
    let address = Ok(observable::of(IAddress {
      appInst: retPreviosMsgTag.appinst.to_string(),
      msgId: retPreviosMsgTag.msgid.to_string(),
    }));
  */

  let j = json!({
    "appInst":msg_link.appinst.to_string(),
    "msgId": msg_link.msgid.to_string()

  });

  HttpResponse::Ok().json(j)
  /*
  match address {
    Ok(v) => return HttpResponse::Ok().json(pollObservable(v).await),
    _ => return HttpResponse::Ok().json("chingtmd"),
  }*/

  //return HttpResponse::Ok().json(pollObservable(address).await);
}

#[post("/address/fetchAll")]
pub async fn addressFetchAll(
  //httpController: Inject<AppModule, dyn IAppHttpController>,
  query: Query<FetchAllQuery>,
) -> HttpResponse {
  let address = query.into_inner().address;

  //let client = Client::new_from_url(&std::env::var("NODE").unwrap());

  let send_options: SendOptions = SendOptions {
    url: std::env::var("NODE").unwrap(),
    local_pow: false,
    depth: 3,
    threads: 3,
  };

  let iota_client = block_on(
    OtherClient::builder()
      .with_node(&std::env::var("NODE").unwrap())
      .unwrap()
      .with_local_pow(false)
      .finish(),
  )
  .unwrap();

  let client = Client::new(send_options, iota_client);

  let mut subscriber: Subscriber<Client> =
    Subscriber::new(&randomSeed(64), "utf-8", PAYLOAD_BYTES, client.clone());
  let importedLoadLink =
    TangleAddress::from_str(&address.appInst.clone(), &address.msgId.clone()).unwrap();
  subscriber.receive_announcement(&importedLoadLink).await;
  if subscriber.is_registered() {
    println!("subscriber Ok");
  }
  let msgs = subscriber.fetch_all_next_msgs().await;
  let msgsIds: Vec<(&AppInst, &MsgId)> = msgs
    .iter()
    .map(|x: &GenericMessage<TangleAddress, MessageContent>| (&x.link.appinst, &x.link.msgid))
    .collect();
  let msgsAddresses: Vec<TangleAddress> = msgsIds
    .iter()
    .map(|(x, y)| TangleAddress::from_str(&x.to_string(), &y.to_string()).unwrap())
    .collect();

  let mut signed_packed: HashMap<&TangleAddress, Bytes> = HashMap::new();
/*
  msgsAddresses.iter().map(|x: &TangleAddress| async {
    let (_, pubP, _): (_, Bytes, _) = subscriber.receive_signed_packet(x).await.unwrap();
    signed_packed.insert(x,pubP);
  });
*/
  for e in msgsAddresses.iter(){
    let (_, pubP, _): (_, Bytes, _) = subscriber.receive_signed_packet(e).await.unwrap();
    signed_packed.insert(e,pubP);
  }

  let msgsPublicPayload: Vec<HashMap<String, Value>> = msgsAddresses
    .iter()
    .map(|x: &TangleAddress| {
      // let (_, pubP, _): (_, Bytes, _) = subscriber.receive_signed_packet(x).await.unwrap();
      let opt = signed_packed.get(x); // Option<&Bytes>
      let bytes = opt.unwrap(); /*match opt {
        Some(v) => v, // &Bytes
        None => {          
          // println!("signed packed error");
         // let p = "F".as_bytes();
         //let p = vec![0];
          // Bytes::fr`om(p)
          "F".as_bytes()
           
        }
      };*/
      return serde_json::from_str(
        &String::from_utf8(
          decode_config(&String::from_utf8(bytes.clone().0).unwrap(), URL_SAFE_NO_PAD).unwrap(),
        )
        .unwrap(),
      )
      .unwrap();
    })
    .collect();
  //let array = observable::of(msgsPublicPayload);

  /*
    let j = json!({
      "appInst":retPreviosMsgTag.appinst.to_string(),
      "msgId": retPreviosMsgTag.msgid.to_string()

    });
  */
  HttpResponse::Ok().json(msgsPublicPayload)

  // let address = httpController.fetchAll(query.0.clone());
}

// #[post("/address/fetchAll")]
// pub async fn addressFetchAll(
//   httpController: Inject<AppModule, dyn IAppHttpController>,
//   query: Query<FetchAllQuery>,
// ) -> impl Responder {
//   let address = httpController.fetchAll(query.0.clone());
//   return HttpResponse::Ok().json(pollObservable(address).await);
// }
