// Implementation of RpcRequestHandler trait for testing Rpc requests without i/o

use crate::rpc_request::{RpcRequest, RpcRequestHandler};
use serde_json::{json, Number, Value};
use solana_sdk::pubkey::Pubkey;
use std::error;

pub const PUBKEY: &str = "7RoSF9fUmdphVCpabEoefH81WwrW7orsWonXWqTXkKV8";
pub const SIGNATURE: &str =
    "43yNSFC6fYTuPgTNFFhF4axw7AfWxB2BPdurme8yrsWEYwm8299xh8n6TAHjGymiSub1XtyxTNyd9GBfY2hxoBw8";

#[derive(Clone)]
pub struct MockRpcClient {
    pub url: String,
}

impl MockRpcClient {
    pub fn new(url: String) -> Self {
        MockRpcClient { url }
    }

    pub fn retry_get_balance(
        &self,
        pubkey: &Pubkey,
        retries: usize,
    ) -> Result<Option<u64>, Box<dyn error::Error>> {
        let params = json!([format!("{}", pubkey)]);
        let res = self
            .retry_make_rpc_request(&RpcRequest::GetBalance, Some(params), retries)?
            .as_u64();
        Ok(res)
    }

    pub fn retry_make_rpc_request(
        &self,
        request: &RpcRequest,
        params: Option<Value>,
        mut _retries: usize,
    ) -> Result<Value, Box<dyn error::Error>> {
        if self.url == "fails" {
            return Ok(Value::Null);
        }
        let val = match request {
            RpcRequest::ConfirmTransaction => {
                if let Some(Value::Array(param_array)) = params {
                    if let Value::String(param_string) = &param_array[0] {
                        Value::Bool(param_string == SIGNATURE)
                    } else {
                        Value::Null
                    }
                } else {
                    Value::Null
                }
            }
            RpcRequest::GetBalance => {
                let n = if self.url == "airdrop" { 0 } else { 50 };
                Value::Number(Number::from(n))
            }
            RpcRequest::GetRecentBlockhash => Value::String(PUBKEY.to_string()),
            RpcRequest::GetSignatureStatus => {
                let str = if self.url == "account_in_use" {
                    "AccountInUse"
                } else if self.url == "bad_sig_status" {
                    "Nonexistent"
                } else {
                    "Confirmed"
                };
                Value::String(str.to_string())
            }
            RpcRequest::GetTransactionCount => Value::Number(Number::from(1234)),
            RpcRequest::SendTransaction => Value::String(SIGNATURE.to_string()),
            _ => Value::Null,
        };
        Ok(val)
    }
}

impl RpcRequestHandler for MockRpcClient {
    fn make_rpc_request(
        &self,
        request: RpcRequest,
        params: Option<Value>,
    ) -> Result<Value, Box<dyn error::Error>> {
        self.retry_make_rpc_request(&request, params, 0)
    }
}
