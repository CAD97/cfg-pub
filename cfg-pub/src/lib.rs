extern crate proc_macro;

use {proc_macro::TokenStream, watt::WasmMacro};

static MACRO: WasmMacro = WasmMacro::new(WASM);
static WASM: &[u8] = include_bytes!("macros.wasm");

#[proc_macro_attribute]
pub fn cfg_pub(attr: TokenStream, item: TokenStream) -> TokenStream {
    MACRO.proc_macro_attribute("cfg_pub", attr, item)
}
