use cratetorrent::prelude::*;
                                                                             
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // spawn the engine with a default config
    let conf = Conf::new("./var/downloads");
    let (engine, mut alert_rx) = engine::spawn(conf)?;
                                                                             
    // parse torrent metainfo and start the download
    let metainfo = tokio::fs::read("./var/torrents/100MB.bin.torrent").await?;
    let metainfo = Metainfo::from_bytes(&metainfo)?;
    let torrent_id = engine.create_torrent(TorrentParams {
        metainfo,
        // tell the engine to assign a randomly chosen free port
        listen_addr: None,
        // here we could specify peers we knew of that we'd want
        // to connect to
        mode: Mode::Download { seeds: Vec::new() },
        conf: None,
    })?;
                                                                             
    // listen to alerts from the engine
    while let Some(alert) = alert_rx.next().await {
        match alert {
            Alert::TorrentStats { id, stats } => {
                println!("{}: {:#?}", id, stats);
            }
            Alert::TorrentComplete(id) => {
                println!("{} complete, shutting down", id);
                break;
            }
            Alert::Error(e) => {
              // this is where you'd handle recoverable errors
              println!("Engine error: {}", e);
            }
            _ => (),
        }
    }
                                                                             
    // Don't forget to call shutdown on the engine to gracefully stop all
    // entities in the engine. This will wait for announcing the client's
    // leave to all trackers of torrent, finish pending disk and network IO,
    // as well as wait for peer connections to cleanly shut down.
    engine.shutdown().await?;
                                                                             
    Ok(())
}