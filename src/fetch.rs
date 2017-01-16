use std::collections::HashMap;
use reqwest::{ Client, Error };

#[derive(Debug, Deserialize)]
pub struct BattleResult {
    number: u32,
    kuski: String,
    time: String,
}

#[derive(Debug, Deserialize)]
pub struct Positions(HashMap<String, BattleResult>);

pub fn get_battle_result(_battle_id: usize) -> Result<(), Error> {
    let client = Client::new()?;
    let mut res = client.get("http://elmaonline.net/API/battle/107044")
                  .send()?;

    let test = res.json::<Positions>()?;
    println!("{:?}", test);
    Ok(())
}
