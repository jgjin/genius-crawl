use std::{
    sync::{
        Arc,
    },
    thread,
};

use crossbeam_channel::{
    self as channel,
    Receiver,
    Sender,
};
use indicatif::{
    ProgressBar,
    ProgressStyle,
};
use num_cpus;
use regex::{
    Regex,
};
use reqwest::{
    Client,
};

use crate::{
    endpoints::{
        search,
    },
    io::{
        lines_from_file,
        read_csv_into_sender,
        write_csv_through_receiver,
    },
    types::{
        ArtistTrack,
        SearchCsv,
        SearchHit,
    },
}; 

lazy_static! {
    static ref NON_WORD: Regex = Regex::new(r"[\W\s]+").unwrap();
}

fn is_match(
    hit: &SearchHit,
    artist: &str,
    track: &str,
) -> bool {
    let hit_artist = NON_WORD.replace_all(&hit.result.primary_artist.name[..], " ").into_owned();
    let hit_track = NON_WORD.replace_all(&hit.result.title[..], " ").into_owned();

    return (
        hit_artist.contains(artist) || artist.contains(&hit_artist[..])
    ) && {
        hit_track.contains(track) || track.contains(&hit_track[..])
    }
}

fn find_match(
    hits: Vec<SearchHit>,
    artist_track: &ArtistTrack,
) -> Option<String> {
    let artist = NON_WORD.replace_all(&artist_track.artist[..], " ").into_owned();
    let track = NON_WORD.replace_all(&artist_track.track[..], " ").into_owned();

    hits.into_iter().find(|hit| {
        // info!("{:?}", hit);
        is_match(hit, &artist[..], &track[..])
    }).map(|hit| {
        hit.result.api_path
    })
}

fn crawl_thread(
    artists_tracks_receiver: Receiver<ArtistTrack>,
    csv_sender: Sender<SearchCsv>,
    client: Arc<Client>,
    progress: Arc<ProgressBar>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        while let Some(artist_track) = artists_tracks_receiver.recv().ok() {
            info!("{:?}", artist_track);
            search(
                &format!(
                    "{} {}",
                    artist_track.artist,
                    artist_track.track,
                )[..],
                client.clone(),
            ).and_then(|resp| {
                let api_path = find_match(
                    resp.response.hits,
                    &artist_track,
                );
                csv_sender.send(SearchCsv::from(
                    artist_track,
                    api_path,
                )).map_err(|err| err.into())
            }).unwrap_or_else(|err| {
                error!("Error crawling: {}", err);
            });

            progress.inc(1);
        }
    })
}


fn crawl(
    artists_tracks_receiver: Receiver<ArtistTrack>,
    csv_sender: Sender<SearchCsv>,
    client: Arc<Client>,
) -> thread::Result<()> {
    let progress = Arc::new(ProgressBar::new(
        (lines_from_file("tracks_final.csv")
         .expect("Error in reading artist tracks")
         .len() - 1) as u64
    ));
    progress.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{wide_bar}] {pos}/{len} ({percent}%)")
    );

    let num_threads = num_cpus::get();
    info!("Using {} threads", num_threads);
    
    let threads: Vec<thread::JoinHandle<()>> = (0..num_threads).map(|_| {
        crawl_thread(
            artists_tracks_receiver.clone(),
            csv_sender.clone(),
            client.clone(),
            progress.clone(),
        )
    }).collect();

    threads.into_iter().map(|join_handle| {
        join_handle.join()
    }).collect::<thread::Result<()>>().and_then(|res| {
        progress.finish_with_message("Done crawling artists tracks");
        Ok(res)
    })
}

pub fn main(
    client: Arc<Client>,
) {
    let (artists_tracks_sender, artists_tracks_receiver) = channel::unbounded();
    let (csv_sender, csv_receiver) = channel::unbounded();

    let reader_thread = thread::spawn(move || {
        read_csv_into_sender(artists_tracks_sender, "tracks_final.csv")
            .expect("Error in reading artists tracks")
    });

    let crawler_thread = thread::spawn(move || {
        crawl(artists_tracks_receiver, csv_sender, client)
            .expect("Error in crawling artists tracks");
    });

    let writer_thread = thread::spawn(move || {
        write_csv_through_receiver(csv_receiver, "search.csv")
            .expect("Error in writing artists tracks");
    });

    reader_thread.join().unwrap_or_else(|err| {
        error!("Error in artists tracks reader thread: {:?}", err);
    });

    crawler_thread.join().unwrap_or_else(|err| {
        error!("Error in artists tracks crawler thread: {:?}", err);
    });

    writer_thread.join().unwrap_or_else(|err| {
        error!("Error in artists tracks writer thread: {:?}", err);
    });

}


