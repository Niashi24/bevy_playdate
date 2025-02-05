use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn init_app(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_body = &input_fn.block;
    let fn_vis = &input_fn.vis;
    let fn_sig = &input_fn.sig;

    let expanded = quote! {
        use bevy_app::App;
        use core::ptr::NonNull;
        use pd::display::Display;
        use pd::sys::ffi::{PDSystemEvent, PlaydateAPI};
        use pd::sys::EventLoopCtrl;
        use pd::system::System;
        use pd::system::update::UpdateCtrl;
        use crate::game::GamePlugin;

        #[no_mangle]
        fn event_handler(_api: NonNull<PlaydateAPI>, event: PDSystemEvent, _: u32) -> EventLoopCtrl {
            match event {
                PDSystemEvent::kEventInit => {}
                _ => return EventLoopCtrl::Continue,
            }

            let mut app = init_app();

            Display::Default().set_refresh_rate(50.);

            System::Default().set_update_callback_boxed(
                move |_| {
                    app.update();
                    UpdateCtrl::Continue
                },
                (),
            );

            EventLoopCtrl::Continue
        }
        
        #fn_vis #fn_sig #fn_body
    };

    TokenStream::from(expanded)
}