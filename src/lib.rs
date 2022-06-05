mod pb;
mod utils;

// use cosmrs::tx::MsgProto;
// use cosmrs::tx::{MsgProto, Tx};
use substreams::{log, proto};

// use cosmrs::distribution::MsgWithdrawDelegatorReward;
// use cosmrs::proto::cosmos::distribution::v1beta1::MsgWithdrawDelegatorReward as ProtoMsgWithdraw;
// use cosmrs::proto::types::Block as RawBlock;

use cosmrs::proto::tendermint::types::Block as RawBlock;


#[substreams::handlers::map]
fn map_hello_world(block: RawBlock) -> Result<pb::cosmos::Header, substreams::errors::Error>  {


    // substreams::register_panic_hook();

    // let block: pb::cosmos::Block = proto::decode_ptr(block_ptr, block_len).unwrap();

    // let block: RawBlock = proto::decode_ptr(block_ptr, block_len).unwrap();

    // cosmrs::proto::cosmos::
    // We need to update and ask their team to use the definitions that are gneerally defined by the proto buffers.
    // This is the thrift definition of finding and grabbing
    
    // let block = cosmrs::proto::dec

    // let block = cosmrs::tendermint::Block::decode_;
        // let block: cosmrs::tendermint::Block = proto::decode_ptr(block_ptr, block_len).unwrap();

    // let block: cosmrs::tendermint::Block = proto::decode_ptr(block_ptr, block_len).unwrap();

    // None of these blocks have transactions.
    log::println(format!("Found N number of transactions in this block! {:?}", block.data));

    
    // So we dont got shit here. Where can we data to start showing stuff.
    // What can we do now, idk. If we can get some more data...
    // match Tx::from_bytes(tx_response.tx.as_bytes()) {
    // for tx_result in block.transactions {
    //     // Tx::from_bytes(tx_result.tx.as_bytes());

    //     if let Some(x) = tx_result.tx {
    //         if let Some(tx_body) = x.body {
    //             for msg in tx_body.messages {
    //                 log::println(format!("Tx Dataobject {:?}", msg.type_url));
                    
                    
    //                 if msg.type_url == "/cosmos.distribution.v1beta1.MsgWithdrawDelegatorReward" {
    //                     log::println(format!("Tx Dataobject {:?}", msg));
                        
    //                     MsgWithdrawDelegatorReward::try_from(msg);
                        
    //                     // match ProtoMsgWithdraw::try_from(msg) {
    //                     //     Ok(msg_inst_contract) => {
                                
    //                     //         msg_inst_contract;
    //                     //         // return msg_inst_contract.index_message(registry, events);
    //                     //     }
    //                     //     Err(e) => {
    //                     //         // error!(
    //                     //         //     "error parsing MsgInstantiateContract, events: {:?}",
    //                     //         //     events
    //                     //         // );
    //                     //         // return Err(anyhow!(e));
    //                     //     }
    //                     // }
    //                     // match ProtoMsgWithdraw::from_any(msg) {}
    //                     // MsgWithdrawDelegatorReward::try_from(msg);

    //                     // match MsgInstantiateContract::try_from(proto_msg_instantiate_contract) {

    //                     // } 

    //                 }
                                    
    //             }
    //         }

    //         // log::println(format!("Tx Dataobject {:?}", x));
    //     } 
    //     // else {
    //     //     log::println(format!("I'm tired and sad x"));
    //     // }
    //     // No events, RIP.
    //     // if let Some(y) = tx_result.result { 
    //     //     let our_events = y.events;
    //     //     log::println(format!("Events:  {:?}", our_events));
    //     //     log::println(format!("Event Size:  {:?}", our_events.len()));
    //     // } 
    //     // else {
    //     //     log::println(format!("I'm tired and sad y"));
    //     // }
    // }

    // let header: core::option::Option<pb::cosmos::Header> = block.header;
    // match header {
    //     Some(h) => {
    //         // log::println(format!(
    //         //     "Header at height {} with hash {} and chain_id {}",
    //         //     h.height,
    //         //     utils::address_pretty(h.hash.as_slice()), h.chain_id
    //         // ));
    //         substreams::output(h);
    //     }
    //     None => println!("No Header in block."),
    // }

    todo!()
}