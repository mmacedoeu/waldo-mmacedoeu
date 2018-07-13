use actix::{SyncArbiter, System};
use actors::{Images, SearchExecutor};
use clap::{App, Arg};
use cv::{imgcodecs::ImageReadMode, Mat};
use env_logger::Builder;
use errors::{Error, ErrorKind, Result};
use futures::Future;
use log::LevelFilter;
use std::env;
use tokio;

fn init_logger(pattern: &str) {
    // Always print backtrace on panic.
    env::set_var("RUST_BACKTRACE", "full");
    let mut builder = Builder::new();
    // Disable info logging by default for some modules:
    builder.filter(Some("mio::timer"), LevelFilter::Warn);
    builder.filter(Some("mio::poll"), LevelFilter::Warn);
    builder.filter(Some("tokio_reactor"), LevelFilter::Warn);
    builder.filter(Some("tokio_core"), LevelFilter::Warn);
    builder.filter(Some("tokio_threadpool"), LevelFilter::Warn);

    // Enable info for others.
    builder.filter(None, LevelFilter::Info);

    if let Ok(lvl) = env::var("RUST_LOG") {
        builder.parse(&lvl);
    }

    builder.parse(pattern);
    builder.init();
}

pub fn run<I, T>(args: I) -> Result<()>
where
    I: IntoIterator<Item = T>,
    T: Into<::std::ffi::OsString> + Clone,
{
    let matches = App::new("Subimages")
        .version(crate_version!())
        .author("Marcos Macedo <contato@mmacedo.eu.org>")
        .about("Provided two jpeg images find if one is a cropped image of the other")
        .arg(
            Arg::with_name("log")
                .short("l")
                .long("log")
                .value_name("LOG_PATTERN")
                .help("Sets a custom logging")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("IMAGE1")
                .help("Sets the first image file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("IMAGE2")
                .help("Sets the second image file to use")
                .required(true)
                .index(2),
        )
        .get_matches();

    let log_pattern = matches.value_of("log").unwrap_or("");
    init_logger(log_pattern);

    let image1_path = matches.value_of("IMAGE1").unwrap();
    let image2_path = matches.value_of("IMAGE2").unwrap();

    info!("Loading Image1 \t {:?}", image1_path);
    let mat1 = Mat::from_path(image1_path, ImageReadMode::Grayscale).map_err(|e| {
        error!("error opening IMAGE1: \t {}", e);
        return Error::from_kind(ErrorKind::Failure(::failure::Error::from(e).compat()))
    })?;

    if !mat1.is_valid() {
        error!("error opening IMAGE1: \t {}", image1_path);
        return Err(Error::from_kind(ErrorKind::Msg(String::from("Invalid image1"))))
    }

    info!("Loading Image2 \t {:?}", image2_path);
    let mat2 = Mat::from_path(image2_path, ImageReadMode::Grayscale).map_err(|e| {
        error!("error opening IMAGE2: \t {}", e);
        return Error::from_kind(ErrorKind::Failure(::failure::Error::from(e).compat()))
    })?;

    if !mat2.is_valid() {
        error!("error opening IMAGE1: \t {}", image2_path);
        return Err(Error::from_kind(ErrorKind::Msg(String::from("Invalid image2"))))
    }
    
    System::run(|| {
        // Start executor actors
        let s_addr = SyncArbiter::start(1, move || SearchExecutor {});

        let res = s_addr
        .send(Images {
            img1: mat1,
            img2: mat2,
        });
        // .and_then(|res| {
        //     info!("{:?}", res);
        //     Ok(())
        // });

        // handle() returns tokio handle
        tokio::spawn(
            res.and_then(|res| {
                info!("{:?}", res);

                // stop system and exit
                System::current().stop();
                Ok(())
            }).map_err(|_| ()),
        );
    });

    Ok(())
}
