
use std::error::Error;
use serde_json::Value;
use crate::domain::misttrack_domain::{Labels, MistTrackResp, RiskScore, StatusResp, Transactions};

pub async fn get_address_labels() ->Result<MistTrackResp<Labels>, Box<dyn Error>> {
    let resp = reqwest::get("https://openapi.misttrack.io/v1/address_labels?coin=ETH&address=0x8894E0a0c962CB723c1976a4421c95949bE2D4E3&api_key=0czUhxuDogJebWBfEjFQM34XP2mYS6Rt").await?
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



pub async fn get_risk_score() -> Result<MistTrackResp<RiskScore>, Box<dyn Error>>{
    let resp = reqwest::get("https://openapi.misttrack.io/v1/risk_score?coin=ETH&address=0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045&api_key=0czUhxuDogJebWBfEjFQM34XP2mYS6Rt").await?
        .json::<Value>().await?;
    let my_object: MistTrackResp<RiskScore> = serde_json::from_value(resp)?;
    Ok(my_object)
}

pub async fn get_transactions_investigation() -> Result<MistTrackResp<Transactions>, Box<dyn Error>>{
    let resp = reqwest::get("https://openapi.misttrack.io/v1/transactions_investigation?coin=ETH&address=0xb3065fe2125c413e973829108f23e872e1db9a6b&api_key=YourApiKey").await?
        .json::<Value>().await?;
    let my_object: MistTrackResp<Transactions> = serde_json::from_value(resp)?;
    Ok(my_object)
}