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

pub fn get_data() -> Result<CryptoAPIStruct, serde_json::Error>  {
  let response = reqwest::blocking::get("https://api.coincap.io/v2/assets").unwrap();
  let s = response.text().unwrap();

  serde_json::from_str::<CryptoAPIStruct>(&s)
}

pub fn get_vec_data() -> Vec<Vec<String>> {
  get_data()
    .unwrap().data
    .iter()
    .map(|x| {
      vec![
        x.symbol.clone(),
        x.name.clone(),
        ("$".to_owned() + &x.price_usd[..]).clone(),
        (x.change_percent24_hr[0..6].to_string() + &"%").clone()
      ]
    })
    .collect()
}