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
use crate::streams_utils::s_fetch::a_fetch_next_messages;
use crate::streams_utils::s_fetch::s_fetch_next_messages;
use base64::{decode_config, encode_config, URL_SAFE_NO_PAD};
use futures::executor::block_on;
use iota_streams::app::message::GenericMessage;
use iota_streams::app::transport::tangle::client::iota_client::Client as OtherClient;
use iota_streams::app::transport::tangle::client::{Client, SendOptions};
use iota_streams::app::transport::tangle::{AppInst, MsgId, TangleAddress};
use iota_streams::app_channels::api::tangle::Author;
use iota_streams::app_channels::api::tangle::ChannelType;
// use iota_streams::app::transport::tangle::get_hash;
use iota_streams::app_channels::api::tangle::MessageContent;
use iota_streams::app_channels::api::tangle::Subscriber;
use iota_streams::ddml::types::Bytes;
use rxrust::observable;
use rxrust::observable::of::OfEmitter;
use rxrust::observable::ObservableBase;
use shaku::{Component, Interface};
use std::time::Instant;
use std::str::FromStr;

use crate::models::schemas::author_schema::AuthorSchema;

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
   Author::new(&seed,ChannelType::SingleBranch ,client );
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
  let exported = author.export(&password.clone()).await.unwrap();
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
  println!("  msg => <{:x}>", annAddress.to_msg_index());
  let result = json!({
      
      "address": {
          // "msg_link": annAddress.to_msg_index().to_string(),
          "appInst":annAddress.appinst.to_string(),
          "msgId": annAddress.msgid.to_string(),
      },
      "author":{
        "password": password.clone(),
        "state": encodedExported,
        "seed": seed.to_string(),
        
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

    let mut subscriberA = Subscriber::new(&seed, client);
    let get_address = address.appInst.clone()+ &":".to_string() + &address.msgId.clone();
    let announcement_link =
    TangleAddress::from_str(&get_address).unwrap();

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
  let autor = q.author;
  

  let s = String::from_utf8(bytes.to_vec()).unwrap();
  let json: HashMap<String, Value> = serde_json::from_str(&s).unwrap();
  println!("s: {}",s);

  //let client = Client::new_from_url(&std::env::var("NODE").unwrap());

  let send_options: SendOptions = SendOptions {
    url: std::env::var("NODE").unwrap(),
    local_pow: false,
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

  let get_address = address.appInst.clone()+ &":".to_string() + &address.msgId.clone();
  let keyLoadLink =
    // TangleAddress::from_str(&address.appInst.clone(), &address.msgId.clone()).unwrap();
    // TangleAddress::from_str(&"hola".to_string()).unwrap();
   
    // TangleAddress::from_str(&address.msg_link.clone()).unwrap();
    TangleAddress::from_str(&get_address).unwrap();

  let mut author: Author<Client> = Author::import(
    &decode_config(autor.state.clone(), URL_SAFE_NO_PAD).unwrap(),
    &autor.password.clone(),
    client.clone(),
  )
  .await
  .unwrap();

  // let mut author = Author::recover(&author.seed, &keyLoadLink, ChannelType::SingleBranch, client.clone()).await.unwrap();
  
  println!("Author multi branching?: {}", author.is_multi_branching());
  print!("  Author1     : {}", author);
  // let _ = author.fetch_state().unwrap();
  // let _ = author.fetch_all_next_msgs().await;
  // let _ = author.sync_state().await;
  // let _ = author.fetch_next_msgs().await;

  // let prev_msg_link = keyLoadLink;

  // let hash = get_hash(self.appinst.as_ref(), self.msgid.as_ref()).unwrap_or_default();
  // let hasgh = get_hash(tx_address: &[u8], tx_tag: &[u8]);
  // let hash = get_hash(keyLoadLink.appinst.as_ref(), keyLoadLink.msgid.as_ref()).unwrap_or_default();
  // println!("Sent msg0: {}", hash);

  // print!("  Author1     : {}", author);


  // a_fetch_next_messages(&mut author).await;
  // println!("Sent msg1: {}", keyLoadLink);



  // let secuencia = author.store_state_for_all(&keyLoadLink, address.seq_num).unwrap();
  // println!("seq {:?}", secuencia);

  // let state = author.fetch_state().unwrap();
  // let next = author.fetch_next_msgs().await;

    // a_fetch_next_messages(&mut author).await;
  // println!("Sent msg1: {}", keyLoadLink);

  // let public_key = author.get_public_key();

  // let store_status = author.store_state(&public_key,&keyLoadLink);

  // print!("  Author2     : {}", author);
//  let secuencia = author.gen_next_msg_ids(author.is_multi_branching());

//  for (pk, cursor) in &secuencia {
//    println!("{:?} {} ",pk, cursor);
//  }

  // let (msg_link, seq_link) = author
  //   .send_signed_packet(&keyLoadLink, &public, &Bytes::default())
  //   .await.unwrap();
  //   println!("Sent msg2: {} ", msg_link);

    // match seq_link {
    //   Some(v) => println!("seq_link: {}",v.to_string()),
    //   None => println!("Ga")
    // };
    // print!("  Author3     : {}", author);
    // let secuencia = author.store_state_for_all(&keyLoadLink,3).unwrap();

    // println!("sec: {:?}", secuencia);

    // let secuencia = author.receive_sequence(&msg_link).await.unwrap();
    // println!("seq {}", secuencia);

    // let msg_inputs = vec![
    //     s,//"These", "Messages", //"Will", "Be", "Masked", "And", "Sent", "In", "A", "Chain",
    //     // "These", "Messages", "Will", "Be", "Masked", "And", "Sent", "In", "A", "Chain",
    // ];

    let mut prev_msg_link = keyLoadLink;
    // for input in &msg_inputs {
      let now = Instant::now();
        let (msg_link, seq_link) = author.send_signed_packet(
            &prev_msg_link,
            &Bytes::default(),
            // &Bytes("hola".as_bytes().to_vec()),
            &Bytes(payload.as_bytes().to_vec()),
        ).await.unwrap();
        println!("Sent msg: {}, option {:?}", msg_link, Some(seq_link));
        println!("Syncing took: {:.2?}", now.elapsed());
        prev_msg_link = msg_link;
        print!("  Author3     : {}", author);
        match seq_link {
        Some(v) => println!("seq_link: {}",v),
        None => println!("Ga")
    };
    // }

    let exported = author.export(&autor.password.clone()).await.unwrap();
    let encodedExported = encode_config(exported.clone(), URL_SAFE_NO_PAD);

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
    // "appInst":msg_link.appinst.to_string(),
    // "msgId": msg_link.msgid.to_string(),
    // "seq_num": address.seq_num,
    // "msg_link": msg_link.to_string()
    // "ga":"ga",
      
      "address": {
          // "msg_link": annAddress.to_msg_index().to_string(),
          "appInst":msg_link.appinst.to_string(),
          "msgId": msg_link.msgid.to_string(),
      },
      "author":{
        "password": autor.password.clone(),
        "state": encodedExported,
        "seed": autor.seed.to_string(),
        
      },
      
  

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
    Subscriber::new(&randomSeed(64), client.clone());
    let get_address = address.appInst.clone()+ &":".to_string() + &address.msgId.clone();
  let importedLoadLink =
    TangleAddress::from_str(&get_address).unwrap();

  println!("Subscriber1: {}", subscriber);

  subscriber.receive_announcement(&importedLoadLink).await;
  if subscriber.is_registered() {
    println!("subscriber Ok");
  }
  println!("Subscriber2: {}", subscriber);
  // s_fetch_next_messages(&mut subscriber).await;
  let msgs = subscriber.fetch_all_next_msgs().await;

  println!("Subscriber3: {}", subscriber);

  let processed_msgs = msgs
  .iter()
  .map(|msg| {
      let content = &msg.body;
      match content {
          MessageContent::SignedPacket {
              pk: _,
              public_payload: _,
              masked_payload,
          } => String::from_utf8(decode_config(&String::from_utf8(masked_payload.0.to_vec()).unwrap(),URL_SAFE_NO_PAD).unwrap(),).unwrap(),
          // decode_config(&String::from_utf8(bytes.clone().0).unwrap(), URL_SAFE_NO_PAD).unwrap(),
          _ => String::default(),
      }
  })
  .filter(|s| s != &String::default())
  .collect::<Vec<String>>();


let mut my_vec:Vec<Value> = Vec::new();

  print!("Retrieved messages: ");
  for i in 0..processed_msgs.len() {
      print!("{}, ", processed_msgs[i]);
      // assert_eq!(processed_msgs[i], sent_msgs[i])
      let jzx: Value = serde_json::from_str(& processed_msgs[i]).unwrap();
      my_vec.push(jzx);
  }
  println!();

println!("Subscriber4: {}", subscriber);

//   let msgsIds: Vec<(&AppInst, &MsgId)> = msgs
//     .iter()
//     .map(|x: &GenericMessage<TangleAddress, MessageContent>| (&x.link.appinst, &x.link.msgid))
//     .collect();
//   let msgsAddresses: Vec<TangleAddress> = msgsIds
//     .iter()
//     .map(|(x, y)| TangleAddress::from_str(&get_address).unwrap())
//     .collect();

//   let mut signed_packed: HashMap<&TangleAddress, Bytes> = HashMap::new();
// /*
//   msgsAddresses.iter().map(|x: &TangleAddress| async {
//     let (_, pubP, _): (_, Bytes, _) = subscriber.receive_signed_packet(x).await.unwrap();
//     signed_packed.insert(x,pubP);
//   });
// */
//   for e in msgsAddresses.iter(){
//     let (_, pubP, _): (_, Bytes, _) = subscriber.receive_signed_packet(e).await.unwrap();
//     signed_packed.insert(e,pubP);
//   }

//   let msgsPublicPayload: Vec<HashMap<String, Value>> = msgsAddresses
//     .iter()
//     .map(|x: &TangleAddress| {
//       // let (_, pubP, _): (_, Bytes, _) = subscriber.receive_signed_packet(x).await.unwrap();
//       let opt = signed_packed.get(x); // Option<&Bytes>
//       let bytes = opt.unwrap(); /*match opt {
//         Some(v) => v, // &Bytes
//         None => {          
//           // println!("signed packed error");
//          // let p = "F".as_bytes();
//          //let p = vec![0];
//           // Bytes::fr`om(p)
//           "F".as_bytes()
           
//         }
//       };*/
//       return serde_json::from_str(
//         &String::from_utf8(
//           decode_config(&String::from_utf8(bytes.clone().0).unwrap(), URL_SAFE_NO_PAD).unwrap(),
//         )
//         .unwrap(),
//       )
//       .unwrap();
//     })
//     .collect();
  //let array = observable::of(msgsPublicPayload);

  /*
    let j = json!({
      "appInst":retPreviosMsgTag.appinst.to_string(),
      "msgId": retPreviosMsgTag.msgid.to_string()

    });
  */

  
  // HttpResponse::Ok().json(msgsPublicPayload)
  HttpResponse::Ok().json(my_vec)

  // let address = httpController.fetchAll(query.0.clone());
}

// // #[post("/address/fetchAll")]
// // pub async fn addressFetchAll(
// //   httpController: Inject<AppModule, dyn IAppHttpController>,
// //   query: Query<FetchAllQuery>,
// // ) -> impl Responder {
// //   let address = httpController.fetchAll(query.0.clone());
// //   return HttpResponse::Ok().json(pollObservable(address).await);
// // }
