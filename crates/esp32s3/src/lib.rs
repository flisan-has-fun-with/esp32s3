pub mod board;
pub mod image;
pub mod rect_ext;

#[macro_export]
macro_rules! main {
    ($( $body:tt )*) => {
        fn main() {
            // https://github.com/aedm/esp32-s3-rust-axum-example/tree/main
        
            esp_idf_svc::sys::link_patches();
            esp_idf_svc::log::EspLogger::initialize_default();
            esp_idf_svc::io::vfs::initialize_eventfd(1).expect("Failed to initialize eventfd");
        
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Failed to build Tokio runtime");
        
            rt.block_on(async { _main().await });
        }
        
        async fn _main() {
            $( $body )*
        }
    }
}
