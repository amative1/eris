use crate::aiomas::NewClient;
use crate::config::Config;
use crate::service::{Reconnect, Retry};
use failure::{self, Error, ResultExt};
use serde::{Deserialize, Deserializer};
use serde_derive::Deserialize;
use serde_json::{self, Value};
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct GameId {
    pub id: i32,
    pub is_override: bool,
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct ShowId {
    pub id: i32,
    pub is_override: bool,
}

fn option_bool_as_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Option::<bool>::deserialize(deserializer)?.unwrap_or(false))
}

#[derive(Clone, Debug, Deserialize)]
pub struct HeaderInfo {
    #[serde(deserialize_with = "option_bool_as_bool")]
    pub is_live: bool,
    pub channel: String,
    pub current_game: Option<GameId>,
    pub current_show: Option<ShowId>,
    pub advice: Option<String>,
}

pub struct LRRbot {
    service: Retry,
}

impl LRRbot {
    pub fn new(config: &Config) -> LRRbot {
        #[cfg(unix)]
        let client = NewClient::new(&config.lrrbot_socket);

        #[cfg(not(unix))]
        let client = NewClient::new(&config.lrrbot_port);

        LRRbot {
            service: Retry::new(Reconnect::new(client), 3),
        }
    }

    async fn call(
        &mut self,
        name: String,
        args: Vec<Value>,
        kwargs: HashMap<String, Value>,
    ) -> Result<Value, Error> {
        await!(self.service.call((name, args, kwargs)))?.map_err(failure::err_msg)
    }

    pub async fn get_header_info(&mut self) -> Result<HeaderInfo, Error> {
        let value = await!(self.call("get_header_info".into(), vec![], HashMap::new()))?;
        Ok(serde_json::from_value(value).context("failed to deserialize the response")?)
    }
}
