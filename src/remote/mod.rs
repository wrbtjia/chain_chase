
use std::error::Error;
use axum::extract::Query;
use serde::Deserialize;
use serde_json::Value;
use crate::domain::misttrack_domain::{Labels, MistTrackResp, OverviewResp, RiskScore, SomeRequest, StatusResp, Transactions};

pub async fn get_address_labels(coin:String,address :String) ->Result<MistTrackResp<Labels>, Box<dyn Error>> {

    let str = format!("https://openapi.misttrack.io/v1/address_labels?coin={}&address={}&api_key=hxr2AovkbU7XuPQSJ0p9NWsVt6ZRnBEa", coin, address);
    let resp = reqwest::get(str).await?
        .json::<Value>().await?;

    let my_object: MistTrackResp<Labels> = serde_json::from_value(resp)?;
    Ok(my_object)

}


pub async fn get_status() -> Result<MistTrackResp<StatusResp>, Box<dyn Error>>{
    let resp = reqwest::get("https://openapi.misttrack.io/v1/status").await?
        .json::<Value>().await?;
    let my_object: MistTrackResp<StatusResp> = serde_json::from_value(resp)?;
    Ok(my_object)
}


pub async fn get_overview(coin:String,address :String) -> Result<MistTrackResp<OverviewResp>, Box<dyn Error>>{
    let str = format!("https://openapi.misttrack.io/v1/address_overview?coin={}&address={}&api_key=1H3gjhUQzTO6GpaKwqAZJvoFryLRI9kl", coin, address);
    let resp = reqwest::get(str).await?
        .json::<Value>().await?;
    println!("resp:{:?}",resp);
    let my_object: MistTrackResp<OverviewResp> = serde_json::from_value(resp)?;
    Ok(my_object)
}



pub async fn get_risk_score(coin:String,address :String,txid:String) -> Result<MistTrackResp<RiskScore>, Box<dyn Error>>{
    let mut str = format!("https://openapi.misttrack.io/v1/risk_score?coin={}", coin);
    if !address.eq("nil") {
        str = format!("{}&address={}", str, address);
    }else if !txid.eq("nil"){
        str = format!("{}&txid={}", str, txid);
    }
    str = format!("{}&api_key={}", str, "0czUhxuDogJebWBfEjFQM34XP2mYS6Rt");
    let resp = reqwest::get(str).await?
        .json::<Value>().await?;
    let my_object: MistTrackResp<RiskScore> = serde_json::from_value(resp)?;
    Ok(my_object)
}



pub async fn get_transactions_investigation(req :SomeRequest) -> Result<MistTrackResp<Transactions>, Box<dyn Error>>{

    println!("req:{:?}",req);
    let  str = format!("https://openapi.misttrack.io/v1/transactions_investigation?coin={}&address={}&start_timestamp={}&end_timestamp={}&type={}&page={}&api_key=hxr2AovkbU7XuPQSJ0p9NWsVt6ZRnBEa", req.coin,req.address,req.start_timestamp,req.end_timestamp,req.types,req.page);


    let resp = reqwest::get(str).await?
        .json::<Value>().await?;
    let my_object: MistTrackResp<Transactions> = serde_json::from_value(resp)?;
    Ok(my_object)
}