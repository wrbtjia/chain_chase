use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MistTrackResp<T>{
    pub success: bool,
    pub msg: String,
    pub data: Option<T>
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Labels{
    pub label_list : Vec<String>,
    pub label_type :String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StatusResp{
    pub support_coin : Vec<String>,
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RiskScore {
    pub score :i32,
    pub hacking_event :String,
    pub detail_list : Vec<String>,
    pub risk_detail : Vec<RiskDetail>,

}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RiskDetail {
    pub label :String,
    #[serde(rename = "type")]
    pub types :String,
    pub volume :f64,
    pub address :String,
    pub percent :f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Transactions {
    pub out : Vec<Investigation>,
    #[serde(rename = "in")]
    pub ins  : Vec<Investigation>,
    pub page : i32,
    pub total_pages : i32,
    pub transactions_on_page : i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Investigation{
    pub address :String,
    pub tx_hash_list :Vec<String>,
    pub amount :f64,
    #[serde(rename = "type")]
    pub types :i32,
    pub label :String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OverviewResp{
    pub balance :f64,
    pub txs_count :i32,
    pub first_seen :i32,
    pub last_seen :i32,
    pub total_received :f64,
    pub total_spent :f64,
    pub received_txs_count :i32,
    pub spent_txs_count :i32,

}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SomeRequest{
    pub coin: String,
    pub address: String,
    pub start_timestamp: i32,
    pub end_timestamp: i32,
    #[serde(rename = "type")]
    pub types: String,
    pub page: i32,
}