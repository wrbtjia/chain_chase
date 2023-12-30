
use crate::api::api_result::{ApiOK, Result, ApiErr};
use crate::domain::misttrack_domain::{Labels, StatusResp, RiskScore, Transactions};
use crate::remote;

pub async fn get_labels() -> Result<ApiOK<Labels>>{

    let result = remote::get_address_labels().await;
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

pub async fn get_risk_score() -> Result<ApiOK<RiskScore>>{

    let result = remote::get_risk_score().await;
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

pub async fn get_transactions_investigation() -> Result<ApiOK<Transactions>>{

    let result = remote::get_transactions_investigation().await;
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