use std::collections::HashMap;
use axum::extract::Query;
use serde::Deserialize;
use crate::api::api_result::{ApiOK, Result, ApiErr};
use crate::domain::misttrack_domain::{Labels, StatusResp, RiskScore, Transactions, OverviewResp, SomeRequest};
use crate::remote;

pub async fn get_labels(Query(params): Query<HashMap<String, String>>) -> Result<ApiOK<Labels>>{

/*    for (key, value) in &params {
        println!("key:{},value:{}", key, value);
    }
*/
    let coin =  params.get("coin");
    let address =  params.get("address");

    let coins :String;
    coins = match coin {
        Some(xx) => {
            xx.to_string()
        },
        None => {
            return Err(ApiErr::ErrParams(None));
        }
    };

    let addr :String;
    addr = match address {
        Some(xx) => {
            xx.to_string()
        },
        None => {
            return Err(ApiErr::ErrParams(None));
        }
    };
    let result = remote::get_address_labels(coins,addr).await;
    let  labels = match result {
        Err(_) => {
            return Err(ApiErr::ErrSystem(None));
        },
        Ok(v) => {
            match v.success {
                true => {
                    v.data.unwrap()
                },
                _ => {
                    return Err(ApiErr::ErrSystem(None));
                }
            }
        }
    };

    Ok(ApiOK(Some(labels)))

}


pub async fn get_status() -> Result<ApiOK<StatusResp>>{

    let result = remote::get_status().await;
    let status_resp = match result {
        Ok(s) =>{
            match s.success {
                true => {
                    s.data.unwrap()
                },
                _ => {
                    return Err(ApiErr::ErrSystem(None));
                }
            }
        },
        Err(_) =>{
            return Err(ApiErr::ErrSystem(None));
        }

    };

    Ok(ApiOK(Some(status_resp)))
}


pub async fn get_overview(Query(params): Query<HashMap<String, String>>) -> Result<ApiOK<OverviewResp>>{
    let coin =  params.get("coin");
    let address =  params.get("address");

    let coins :String;
    coins = match coin {
        Some(xx) => {
            xx.to_string()
        },
        None => {
            return Err(ApiErr::ErrParams(None));
        }
    };

    let addr :String;
    addr = match address {
        Some(xx) => {
            xx.to_string()
        },
        None => {
            return Err(ApiErr::ErrParams(None));
        }
    };

    let result = remote::get_overview(coins,addr).await;
    let status_resp = match result {
        Ok(s) =>{
            match s.success {
                true => {
                    s.data.unwrap()
                },
                _ => {
                    return Err(ApiErr::ErrSystem(None));
                }
            }
        },
        Err(_) =>{
            return Err(ApiErr::ErrSystem(None));
        }

    };

    Ok(ApiOK(Some(status_resp)))
}


pub async fn get_risk_score(Query(params): Query<HashMap<String, String>>) -> Result<ApiOK<RiskScore>>{

    let coin =  params.get("coin");
    let address =  params.get("address");
    let txid =  params.get("txid");

    let coins :String;
    coins = match coin {
        Some(xx) => {
            xx.to_string()
        },
        None => {
            return Err(ApiErr::ErrParams(None));
        }
    };

    let addr :String;
    addr = match address {
        Some(xx) => {
            xx.to_string()
        },
        None => {
            "nil".to_string()
        }
    };

    let txids :String;
    txids = match txid {
        Some(xx) => {
            xx.to_string()
        },
        None => {
           "nil".to_string()
        }
    };
    
    if addr.eq("nil") && txids.eq("nil"){
        return Err(ApiErr::ErrParams(None));
    }


    let result = remote::get_risk_score(coins,addr,txids).await;
    let risk_score = match result {
        Ok(s) =>{
            match s.success {
                true => {
                    s.data.unwrap()
                },
                _ => {
                    return Err(ApiErr::ErrSystem(None));
                }
            }
        },
        Err(e) =>{
            println!("error:{}",e);
            return Err(ApiErr::ErrSystem(None));
        }
    };

    Ok(ApiOK(Some(risk_score)))
}





pub async fn get_transactions_investigation(Query(args): Query<SomeRequest>) -> Result<ApiOK<Transactions>>{

    println!("{:?}",args);

    let result = remote::get_transactions_investigation(args).await;
    let transactions = match result {
        Ok(s) =>{
            match s.success {
                true => {
                    s.data.unwrap()
                },
                _ => {
                    return Err(ApiErr::ErrSystem(None));
                }
            }
        },
        Err(e) =>{
            println!("error:{}",e);
            return Err(ApiErr::ErrSystem(None));
        }
    };

    Ok(ApiOK(Some(transactions)))
}