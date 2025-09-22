use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::{Arc, Mutex};
use rodio::{Decoder, OutputStream, Sink, Source};
use std::error::Error;

pub struct AudioPlayer {
    _stream: OutputStream,
    music_sink: Arc<Mutex<Sink>>, // Música de fondo (loop)
    sfx_sink: Arc<Mutex<Sink>>,   // Efectos de sonido (una vez)
}

impl AudioPlayer {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        // Crear un nuevo stream de audio
        let (_stream, stream_handle) = OutputStream::try_default()?;
        let music_sink = Sink::try_new(&stream_handle)?;
        let sfx_sink = Sink::try_new(&stream_handle)?;
        
        Ok(Self {
            _stream,
            music_sink: Arc::new(Mutex::new(music_sink)),
            sfx_sink: Arc::new(Mutex::new(sfx_sink)),
        })
    }

    pub fn play_background_music<P: AsRef<Path>>(&self, file_path: P) -> Result<(), Box<dyn Error>> {
        // Detener la música actual si hay alguna reproduciéndose
        self.stop_music();

        // Cargar el archivo de audio
        let file = BufReader::new(File::open(file_path)?);
        let source = Decoder::new(file)?;
        
        // Repetir la música infinitamente
        let source = source.repeat_infinite();
        
        // Reproducir la música
        if let Ok(sink) = self.music_sink.lock() {
            sink.append(source);
            sink.play();
        }
        
        Ok(())
    }

    pub fn set_volume(&self, volume: f32) {
        if let Ok(sink) = self.music_sink.lock() {
            sink.set_volume(volume);
        }
    }

    pub fn pause_music(&self) {
        if let Ok(sink) = self.music_sink.lock() {
            sink.pause();
        }
    }

    pub fn play_music(&self) {
        if let Ok(sink) = self.music_sink.lock() {
            sink.play();
        }
    }

    pub fn stop_music(&self) {
        if let Ok(sink) = self.music_sink.lock() {
            sink.stop();
        }
    }

    pub fn is_music_playing(&self) -> bool {
        if let Ok(sink) = self.music_sink.lock() {
            !sink.empty() && !sink.is_paused()
        } else {
            false
        }
    }

    // --- SFX ---
    pub fn play_sfx_once<P: AsRef<Path>>(&self, file_path: P) -> Result<(), Box<dyn Error>> {
        // Opcional: detener cualquier sfx anterior para evitar solapamientos
        if let Ok(sink) = self.sfx_sink.lock() {
            sink.stop();
        }

        let file = BufReader::new(File::open(file_path)?);
        let source = Decoder::new(file)?; // No repetir

        if let Ok(sink) = self.sfx_sink.lock() {
            sink.append(source);
            sink.play();
        }

        Ok(())
    }
}

// Implementación de Default para facilitar la creación de instancias
impl Default for AudioPlayer {
    fn default() -> Self {
        Self::new().expect("No se pudo crear el reproductor de audio")
    }
}