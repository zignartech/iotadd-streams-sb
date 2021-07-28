use crate::models::dtos::create_author_dto::CreateAuthorBody;
use crate::models::dtos::create_author_dto::CreateAuthorQuery;
use crate::models::dtos::fetch_all_dto::FetchAllQuery;
use crate::models::dtos::send_one_dto::SendOneQuery;
use crate::models::schemas::author_schema::Address;
use crate::models::schemas::author_schema::Author;
use crate::models::schemas::author_schema::AuthorSchema;
use crate::streams_utils::random_seed::randomSeed;
use iota_streams::app::message::GenericMessage;
use iota_streams::app::transport::tangle::{AppInst, MsgId, TangleAddress};
use iota_streams::app_channels::api::tangle::MessageContent;
use iota_streams::app_channels::api::tangle::Subscriber;
use rxrust::observable;
use serde_json::Value;
use std::collections::HashMap;
use base64::{decode_config, encode_config, URL_SAFE_NO_PAD};
use iota_streams::app::transport::tangle::client::Client as SClient;
use iota_streams::app_channels::api::tangle::{Author as SAuthor, ChannelType};
use iota_streams::ddml::types::Bytes;
use rxrust::observable::of::OfEmitter;
use rxrust::observable::ObservableBase;
use shaku::{Component, Interface,};
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
  ) -> ObservableBase<OfEmitter<Address>>;
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
    let client = SClient::new_from_url("https://chrysalis-nodes.iota.cafe:443/");
    let mut author: SAuthor<SClient> =
      SAuthor::new(&createAuthorBody.seed, ChannelType::SingleBranch, client);
    let annAddress: TangleAddress = author.send_announce().unwrap();
    let password = "password".to_string();
    let exported = author.export(&password).unwrap();
    let encodedExported = encode_config(exported.clone(), URL_SAFE_NO_PAD);

    observable::of(AuthorSchema {
      seed: createAuthorQuery.randomSeed.clone().to_string(),
      address: Address {
        appInst: annAddress.appinst.to_string(),
        msgId: annAddress.msgid.to_string(),
      },
      author: Author {
        password: password,
        state: encodedExported,
      },
    })
  }
  fn sendOne(
    &self,
    sendOneQuery: SendOneQuery,
    anyBody: HashMap<String, Value>,
  ) -> ObservableBase<OfEmitter<Address>> {
    let client = SClient::new_from_url("https://chrysalis-nodes.iota.cafe:443/");
    let payloadStr = serde_json::to_string(&anyBody).unwrap();
    let payload = encode_config(&payloadStr, URL_SAFE_NO_PAD);
    let public = Bytes(payload.as_bytes().to_vec());

    let keyLoadLink = TangleAddress::from_str(
      &sendOneQuery.address.appInst.clone(),
      &sendOneQuery.address.msgId.clone(),
    )
    .unwrap();

    let mut author: SAuthor<SClient> = SAuthor::import(
      &decode_config(sendOneQuery.author.state.clone(), URL_SAFE_NO_PAD).unwrap(),
      &sendOneQuery.author.password.clone(),
      client.clone(),
    )
    .unwrap();
    let _ = author.fetch_state();
    let (retPreviosMsgTag, _): (TangleAddress, _) = author
      .send_signed_packet(&keyLoadLink, &public, &Bytes::default())
      .unwrap();
    observable::of(Address {
      appInst: retPreviosMsgTag.appinst.to_string(),
      msgId: retPreviosMsgTag.msgid.to_string(),
    })
  }
  fn fetchAll(
    &self,
    fetchAllQuery: FetchAllQuery,
  ) -> ObservableBase<OfEmitter<Vec<HashMap<String, Value>>>> {
    let client = SClient::new_from_url("https://chrysalis-nodes.iota.cafe:443/");
    let mut subscriber: Subscriber<SClient> = Subscriber::new(&randomSeed(), client.clone());
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
