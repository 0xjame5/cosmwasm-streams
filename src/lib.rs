mod pb;
mod utils;

// use cosmrs::tx::{MsgProto, Tx};
use substreams::{log, proto};

#[no_mangle]
pub extern "C" fn map_hello_world(block_ptr: *mut u8, block_len: usize) {
    substreams::register_panic_hook();

    let block: pb::cosmos::Block = proto::decode_ptr(block_ptr, block_len).unwrap();

    // None of these blocks have transactions.
    log::println(format!("Found N number of transactions in this block! {:?}", block.transactions.len()));

    // So we dont got shit here. Where can we data to start showing stuff.
    // What can we do now, idk. If we can get some more data...
    // match Tx::from_bytes(tx_response.tx.as_bytes()) {
    for tx_result in block.transactions {
        if let Some(x) = tx_result.tx {
            if let Some(tx_body) = x.body {
                for msg in tx_body.messages {
                    // log::println(format!("Tx Dataobject {:?}", msg));
                    let type_url = &msg.type_url;
                    let test = "/cosmwasm.wasm.v1.MsgInstantiateContract".to_string();
                    match type_url {
                        test => {
                            log::println(format!("Tx Dataobject {:?}", test));
                            // todo!();
                        },
                        _ => {
                            log::println(format!("Unmatched"));
                            // todo!();
                        }
                    }
                                    
                }
            }

            // log::println(format!("Tx Dataobject {:?}", x));
        } 
        // else {
        //     log::println(format!("I'm tired and sad x"));
        // }
        // No events, RIP.
        // if let Some(y) = tx_result.result { 
        //     let our_events = y.events;
        //     log::println(format!("Events:  {:?}", our_events));
        //     log::println(format!("Event Size:  {:?}", our_events.len()));
        // } 
        // else {
        //     log::println(format!("I'm tired and sad y"));
        // }
    }

    let header: core::option::Option<pb::cosmos::Header> = block.header;
    match header {
        Some(h) => {
            // log::println(format!(
            //     "Header at height {} with hash {} and chain_id {}",
            //     h.height,
            //     utils::address_pretty(h.hash.as_slice()), h.chain_id
            // ));
            substreams::output(h);
        }
        None => println!("No Header in block."),
    }
}