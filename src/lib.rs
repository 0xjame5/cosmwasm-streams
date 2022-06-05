mod pb;
mod utils;

// use cosmrs::tx::MsgProto;
// use cosmrs::tx::{MsgProto, Tx};
use substreams::{log, proto};

// use cosmrs::distribution::MsgWithdrawDelegatorReward;
// use cosmrs::proto::cosmos::distribution::v1beta1::MsgWithdrawDelegatorReward as ProtoMsgWithdraw;
// use cosmrs::proto::types::Block as RawBlock;

// use cosmrs::proto::tendermint::types::Block as RawBlock;


#[substreams::handlers::map]
fn map_hello_world(block: pb::cosmos::Block) -> Result<pb::cosmos::Header, substreams::errors::Error>  {
    // We need to update and ask their team to use the definitions that are gneerally defined by the proto buffers.
    // This is the thrift definition of finding and grabbing
    
    // log::println(format!("Found N number of transactions in this block! {:?}", block.transactions));

    
    

    // So we dont got shit here. Where can we data to start showing stuff.
    // What can we do now, idk. If we can get some more data...
    // We are one of the first using substreams with Graph TECH!!
    

    for tx_result in block.transactions {
        if let Some(x) = tx_result.tx {
            if let Some(tx_body) = x.body {
                for msg in tx_body.messages {
                    log::println(format!("Tx Dataobject {:?}", msg.type_url));
                    // if msg.type_url == "/cosmos.distribution.v1beta1.MsgWithdrawDelegatorReward" {
                        // log::println(format!("Tx Dataobject {:?}", msg));
                    // }
                }
            }
        }
    }

    // for tx_result in block.transactions {
    //         if let Some(x) = tx_result.tx {
    //             if let Some(tx_body) = x.body {
    //                 for msg in tx_body.messages {
    //                     if msg.type_url == "/cosmos.distribution.v1beta1.MsgWithdrawDelegatorReward" {
    //                         log::println(format!("Tx Dataobject {:?}", msg));
    //                     }
    //                 }
    //             }
    //         }
    // }


    let header: core::option::Option<pb::cosmos::Header> = block.header;


    Ok(header.unwrap())

    
}