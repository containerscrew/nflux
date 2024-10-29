use alloc::string::String;
use aya_ebpf::bindings::xdp_action;

pub fn set_default_action(action: String) {
    let default_action = match action.as_str() {
        "allow" => xdp_action::XDP_PASS,
        "block" => xdp_action::XDP_DROP,
        _ => xdp_action::XDP_DROP,
    };
}
