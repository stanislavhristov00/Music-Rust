use std::{fs::File, io::{self, BufReader, Seek}};

use rodio::{Decoder, OutputStream, Sink};

use crate::audio::audio_track::AudioTrack;

mod audio;

enum Commands {
    Help,
    PrintPlaylist,
    LoadSong,
    RemoveSong,
    PauseCurrent,
    StartCurrent,
    Loop,
    StopLoop,
    StartOver,
    ShowCurrent,
    SkipTo,
    UNKNOWN
}

fn from_str(command: &str) -> Commands {
    match command {
        "help" => Commands::Help,
        "playlist" => Commands::PrintPlaylist,
        "load" => Commands::LoadSong,
        "remove" => Commands::RemoveSong,
        "pause" => Commands::PauseCurrent,
        "start" => Commands::StartCurrent,
        "current" => Commands::ShowCurrent,
        "loop" => Commands::Loop,
        "stoploop" => Commands::StopLoop,
        "startover" => Commands::StartOver,
        "skip" => Commands::SkipTo,
        _ => Commands::UNKNOWN
    }
}

fn usage() -> String {
    String::from("
    Usage:
        help -> prints this usage
        playlist -> prints the current playlist
        load -> <absolute path> -> loads the audio file at the supplied path
        remove -> <basename> -> removes the audio file that corresponds to basename from the playlist
        pause -> pauses the currently playing song
        start -> resumes the currently playing song
        current -> shows the current song's name
        loop -> loops the current song
        stoploop -> stops the current song's loop
        startover -> starts the current song from the beginning
        skip -> <basename> -> skips to an audio file and continues playing from there
    ")
}

fn create_decoder(file: File) -> Decoder<BufReader<File>> {
    let file = BufReader::new(file);
    let source = Decoder::new(file).unwrap();

    source
}

fn main() {
    let usage = usage();
    let mut playlist: Vec<AudioTrack> = Vec::new();
    let mut current_song: AudioTrack = AudioTrack::default();
    let mut should_loop: bool = false;

    println!("{}", usage);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink_result = Sink::try_new(&stream_handle);

    if let Err(e) = sink_result {
        eprintln!("Failed to create sink: {e}");
        panic!();
    }

    let sink = sink_result.unwrap();

    loop {
        println!("Enter your command: ");
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input).unwrap();

        if current_song.get_base_name() != "" &&
           !should_loop &&
           !playlist.is_empty() &&
           sink.empty() {
            let pos = playlist.iter().position(|x| *x == current_song).unwrap();

            if pos < playlist.len() {
                current_song = playlist.get(pos + 1).unwrap().clone().unwrap();
            } else {
                current_song = playlist.get(0).unwrap().clone().unwrap();
            }

            sink.append(create_decoder(current_song.get_file_handle().unwrap()));
        }

        match from_str(input.trim()) {
            Commands::Help => {
                println!("{}", usage);
                continue;
            },
            Commands::PrintPlaylist => {
                println!("-----------------------------------");
                for song in playlist.iter() {
                    println!("{}", song.get_base_name());
                }
                println!("-----------------------------------");
                continue;
            },
            Commands::LoadSong => {
                println!("Enter the file path: ");
                input.clear();
                let _ = io::stdin().read_line(&mut input).unwrap();

                let track = AudioTrack::new(input.trim());
                match track {
                    Err(e) => {
                        eprintln!("Failed to load {input}: {e}");
                        continue;
                    },
                    Ok(song) => {
                        if !playlist.contains(&song) {
                            if playlist.is_empty() {
                                current_song = song.clone().unwrap();
                            }
                            playlist.push(song);
                        }
                    }
                }
            },
            Commands::RemoveSong => {
                todo!()
            },
            Commands::PauseCurrent => {
                if current_song.get_base_name() != "" {
                    sink.pause();
                }
            },
            Commands::StartCurrent => {
                if current_song.get_base_name() == "" {
                    eprintln!("No songs are loaded in yet");
                    continue;
                }

                if !sink.empty() {
                    sink.play();
                    continue;
                }

                let source = create_decoder(current_song.get_file_handle().unwrap());
                sink.append(source);
            },
            Commands::ShowCurrent => {
                // TODO: Make a function that tracks the progress of a song.
                if current_song.get_base_name() != "" {
                    println!("Currently playing: {}", current_song.get_base_name());
                } else {
                    println!("No song playing currently");
                }
                continue;
            },
            Commands::SkipTo => {
                todo!()
            },
            Commands::Loop => {
                should_loop = true;
            },
            Commands::StopLoop => {
                should_loop = false;
            }
            Commands::StartOver => {
                if current_song.get_base_name() != "" {
                    sink.stop();

                    let mut file = current_song.get_file_handle().unwrap();
                    file.rewind().unwrap();

                    let source = create_decoder(file);
                    sink.append(source);
                } else {
                    eprintln!("No song loaded in yet");
                    continue;
                }
            }
            Commands::UNKNOWN => {
                eprintln!("Unknown command: {input}\n");
                continue;
            }
        }
    }
}