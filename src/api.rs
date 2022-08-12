use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CryptoAPIData {
  pub id: String,
  pub symbol: String,
  pub name: String,
  pub market_cap_usd: String,
  pub price_usd: String,
  pub change_percent24_hr: String,
}

#[derive(Deserialize, Serialize)]
pub struct CryptoAPIStruct {
  pub data: Vec<CryptoAPIData>,
}

#[derive(Deserialize, Serialize)]
pub struct DataProvider<'a> {
  pub items: Vec<Vec<&'a str>>,
  url: &'static str,
  raw: Vec<Vec<String>>,
}

impl<'a> DataProvider<'a> {
  pub fn new() -> DataProvider<'a> {
    DataProvider {
      items: vec![],
      raw: vec![],
      url: "https://api.coincap.io/v2/assets",
    }
  }

  pub fn get_data(&mut self) -> Result<CryptoAPIStruct, serde_json::Error>  {
    let response = reqwest::blocking::get(self.url).unwrap();
    let s = response.text().unwrap();

    serde_json::from_str::<CryptoAPIStruct>(&s)
  }

  pub fn update_items(&'a mut self) -> Vec<Vec<&'a str>> {
    self.raw = self.get_data()
      .unwrap().data
      .iter()
      .map(|x: &CryptoAPIData| {
        vec![
          x.symbol.to_owned(),
          x.name.to_owned(),
          ("$".to_owned() + &x.price_usd[..]),
          (x.change_percent24_hr[0..6].to_string() + &"%")
        ]
      })
      .collect();

    self.raw.iter().map(|x| {
      x.iter().map(|y| &y[..]).collect()
    }).collect()
  }
}