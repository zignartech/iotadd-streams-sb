use crate::models::dtos::create_author_dto::CreateAuthorBody;
use crate::models::dtos::create_author_dto::CreateAuthorQuery;
use crate::models::dtos::fetch_all_dto::FetchAllQuery;
use crate::models::dtos::send_one_dto::SendOneQuery;
use crate::models::schemas::author_schema::Address as IAddress;
use crate::models::schemas::author_schema::Author as IAuthor;
use crate::models::schemas::author_schema::AuthorSchema;
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
use serde_json::Value;
use shaku::{Component, Interface};
use std::collections::HashMap;

//use iota_streams::iota_streams_core::Error;

/*
pub trait IAppService: Interface {
  fn createAuthor(
    &self,
    createAuthorQuery: CreateAuthorQuery,
    createAuthorBody: CreateAuthorBody,
  ) -> ObservableBase<OfEmitter<AuthorSchema>>;
  fn sendOne(
    &self,
    sendOneQuery: SendOneQuery,
    anyBody: HashMap<String, Value>,
    //) -> ObservableBase<OfEmitter<IAddress>>;
  ) -> Result<ObservableBase<OfEmitter<IAddress>>, &str>;
  fn fetchAll(
    &self,
    fetchAllQuery: FetchAllQuery,
  ) -> ObservableBase<OfEmitter<Vec<HashMap<String, Value>>>>;
}

#[derive(Component)]
#[shaku(interface = IAppService)]
pub struct AppService;
impl IAppService for AppService {
  fn createAuthor(
    &self,
    createAuthorQuery: CreateAuthorQuery,
    createAuthorBody: CreateAuthorBody,
  ) -> ObservableBase<OfEmitter<AuthorSchema>> {
    // let client = Client::new_from_url(&std::env::var("NODE").unwrap());

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
     Author::new(&createAuthorBody.seed,"utf-8", 1024, false,client );
    let annAddress: TangleAddress = author.send_announce().await;
    let password = randomSeed(12);
    let exported = author.export(&password.clone()).unwrap();
    let encodedExported = encode_config(exported.clone(), URL_SAFE_NO_PAD);

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
    })
  }
  fn sendOne(
    &self,
    sendOneQuery: SendOneQuery,
    anyBody: HashMap<String, Value>,
  ) -> Result<ObservableBase<OfEmitter<IAddress>>, &str> {
    //let client = Client::new_from_url(&std::env::var("NODE").unwrap());

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

    let payloadStr = serde_json::to_string(&anyBody).unwrap();
    let payload = encode_config(&payloadStr, URL_SAFE_NO_PAD);
    let public = Bytes(payload.as_bytes().to_vec());

    let keyLoadLink = TangleAddress::from_str(
      &sendOneQuery.address.appInst.clone(),
      &sendOneQuery.address.msgId.clone(),
    )
    .unwrap();

    let mut author: Author<Client> = Author::import(
      &decode_config(sendOneQuery.author.state.clone(), URL_SAFE_NO_PAD).unwrap(),
      &sendOneQuery.author.password.clone(),
      client.clone(),
    )
    .unwrap();
    let _ = author.fetch_state().unwrap();
    let _ = author.fetch_all_next_msgs();

    let result = author.send_signed_packet(&keyLoadLink, &public, &Bytes::default());
    //.unwrap();

    let (retPreviosMsgTag, _): (TangleAddress, _) = match result {
      Ok(v) => v,
      Err(e) => return Err("Some error message"),
    };
    Ok(observable::of(IAddress {
      appInst: retPreviosMsgTag.appinst.to_string(),
      msgId: retPreviosMsgTag.msgid.to_string(),
    }))
  }
  fn fetchAll(
    &self,
    fetchAllQuery: FetchAllQuery,
  ) -> ObservableBase<OfEmitter<Vec<HashMap<String, Value>>>> {
    //let client = Client::new_from_url(&std::env::var("NODE").unwrap());

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

    let mut subscriber: Subscriber<Client> =
      Subscriber::new(&randomSeed(64), "utf-8", 1024, client.clone());
    let importedLoadLink = TangleAddress::from_str(
      &fetchAllQuery.address.appInst.clone(),
      &fetchAllQuery.address.msgId.clone(),
    )
    .unwrap();
    subscriber.receive_announcement(&importedLoadLink).unwrap();
    if subscriber.is_registered() {
      println!("subscriber Ok");
    }
    let msgs = subscriber.fetch_all_next_msgs();
    let msgsIds: Vec<(&AppInst, &MsgId)> = msgs
      .iter()
      .map(|x: &GenericMessage<TangleAddress, MessageContent>| (&x.link.appinst, &x.link.msgid))
      .collect();
    let msgsAddresses: Vec<TangleAddress> = msgsIds
      .iter()
      .map(|(x, y)| TangleAddress::from_str(&x.to_string(), &y.to_string()).unwrap())
      .collect();
    let msgsPublicPayload: Vec<HashMap<String, Value>> = msgsAddresses
      .iter()
      .map(|x: &TangleAddress| {
        let (_, pubP, _): (_, Bytes, _) = subscriber.receive_signed_packet(x).unwrap();
        return serde_json::from_str(
          &String::from_utf8(
            decode_config(&String::from_utf8(pubP.0).unwrap(), URL_SAFE_NO_PAD).unwrap(),
          )
          .unwrap(),
        )
        .unwrap();
      })
      .collect();
    observable::of(msgsPublicPayload)
  }
}

*/
