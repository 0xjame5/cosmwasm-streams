mod pb;
mod utils;

use substreams::{log, proto};

#[no_mangle]
pub extern "C" fn map_hello_world(block_ptr: *mut u8, block_len: usize) {
    substreams::register_panic_hook();

    let block: pb::cosmos::Block = proto::decode_ptr(block_ptr, block_len).unwrap();
    let header: core::option::Option<pb::cosmos::Header> = block.header;
    match header {
        Some(h) => {
            log::println(format!(
                "Header at height {} with hash {} and chain_id {}",
                h.height,
                utils::address_pretty(h.hash.as_slice()), h.chain_id
            ));
            substreams::output(h);
        }
        None => println!("No Header in block."),
    }
}
