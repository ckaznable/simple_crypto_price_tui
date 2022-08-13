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

pub fn get_raw_data() -> Result<CryptoAPIStruct, serde_json::Error>  {
  let response = reqwest::blocking::get(String::from("https://api.coincap.io/v2/assets")).unwrap();
  let s = response.text().unwrap();

  serde_json::from_str::<CryptoAPIStruct>(&s)
}

pub fn get_data() -> Vec<Vec<String>> {
  get_raw_data()
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
    .collect()
}