use eframe::egui;
use midir::{MidiInput, MidiInputConnection};
use rodio::{OutputStream, OutputStreamHandle, Sink, Source};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread;

// Morse code decoder
struct MorseDecoder {
    current_sequence: String,
    last_element_time: Option<Instant>,
    wpm: u32,
    dit_length_ms: u32,
    letter_space_ms: u32,
}

impl MorseDecoder {
    fn new(wpm: u32) -> Self {
        let dit_length_ms = 1200 / wpm.max(1);
        let letter_space_ms = dit_length_ms * 5;
        
        MorseDecoder {
            current_sequence: String::new(),
            last_element_time: None,
            wpm,
            dit_length_ms,
            letter_space_ms,
        }
    }
    
    fn update_wpm(&mut self, wpm: u32) {
        self.wpm = wpm.clamp(5, 40);
        self.dit_length_ms = 1200 / self.wpm;
        self.letter_space_ms = self.dit_length_ms * 5;
    }
    
    fn add_element(&mut self, is_dit: bool) {
        if is_dit {
            self.current_sequence.push('.');
        } else {
            self.current_sequence.push('-');
        }
        self.last_element_time = Some(Instant::now());
    }
    
    fn check_timeout(&mut self) -> Option<char> {
        if let Some(last_time) = self.last_element_time {
            let elapsed = last_time.elapsed().as_millis() as u32;
            
            if elapsed > self.letter_space_ms && !self.current_sequence.is_empty() {
                let decoded = self.decode_sequence();
                self.current_sequence.clear();
                self.last_element_time = None;
                return Some(decoded);
            }
        }
        None
    }
    
    fn decode_sequence(&self) -> char {
        match self.current_sequence.as_str() {
            ".-" => 'A', "-..." => 'B', "-.-." => 'C', "-.." => 'D',
            "." => 'E', "..-." => 'F', "--." => 'G', "...." => 'H',
            ".." => 'I', ".---" => 'J', "-.-" => 'K', ".-.." => 'L',
            "--" => 'M', "-." => 'N', "---" => 'O', ".--." => 'P',
            "--.-" => 'Q', ".-." => 'R', "..." => 'S', "-" => 'T',
            "..-" => 'U', "...-" => 'V', ".--" => 'W', "-..-" => 'X',
            "-.--" => 'Y', "--.." => 'Z',
            "-----" => '0', ".----" => '1', "..---" => '2', "...--" => '3',
            "....-" => '4', "....." => '5', "-...." => '6', "--..." => '7',
            "---.." => '8', "----." => '9',
            ".-.-.-" => '.', "--..--" => ',', "..--.." => '?', ".----." => '\'',
            "-.-.--" => '!', "-..-." => '/', "-.--." => '(', "-.--.-" => ')',
            ".-..." => '&', "---..." => ':', "-.-.-." => ';', "-...-" => '=',
            ".-.-." => '+', "-....-" => '-', "..--.-" => '_', ".-..-." => '"',
            "...-..-" => '$', ".--.-." => '@',
            _ => '?',
        }
    }
}

// Audio tone generator
struct ToneGenerator {
    frequency: f32,
    sample_rate: u32,
    phase: f32,
}

impl ToneGenerator {
    fn new(frequency: f32) -> Self {
        ToneGenerator {
            frequency,
            sample_rate: 48000,
            phase: 0.0,
        }
    }
}

impl Iterator for ToneGenerator {
    type Item = f32;
    
    fn next(&mut self) -> Option<Self::Item> {
        let sample = (self.phase * 2.0 * std::f32::consts::PI).sin() * 0.3;
        self.phase += self.frequency / self.sample_rate as f32;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        Some(sample)
    }
}

impl Source for ToneGenerator {
    fn current_frame_len(&self) -> Option<usize> { None }
    fn channels(&self) -> u16 { 1 }
    fn sample_rate(&self) -> u32 { self.sample_rate }
    fn total_duration(&self) -> Option<Duration> { None }
}

// Shared application state
#[derive(Clone)]
struct AppState {
    left_pressed: bool,
    right_pressed: bool,
    decoded_text: String,
    current_sequence: String,
    wpm: u32,
    frequency: u32,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            left_pressed: false,
            right_pressed: false,
            decoded_text: String::new(),
            current_sequence: String::new(),
            wpm: 20,
            frequency: 600,
        }
    }
}

// Main GUI application
struct PaddleDecoderApp {
    state: Arc<Mutex<AppState>>,
    _midi_conn: Option<MidiInputConnection<()>>,
    decoder: Arc<Mutex<MorseDecoder>>,
    _stream: OutputStream,
    _stream_handle: OutputStreamHandle,
}

impl PaddleDecoderApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let (stream, stream_handle) = OutputStream::try_default()
            .expect("Failed to open audio stream");
        
        let sink = Arc::new(Mutex::new(
            Sink::try_new(&stream_handle).expect("Failed to create audio sink")
        ));
        
        let state = Arc::new(Mutex::new(AppState::default()));
        let state_clone = Arc::clone(&state);
        
        let decoder = Arc::new(Mutex::new(MorseDecoder::new(20)));
        let decoder_clone = Arc::clone(&decoder);
        
        // Start automatic keyer thread
        let keyer_state = Arc::clone(&state);
        let keyer_decoder = Arc::clone(&decoder);
        let keyer_sink = Arc::clone(&sink);
        
        thread::spawn(move || {
            automatic_keyer_thread(keyer_state, keyer_decoder, keyer_sink);
        });
        
        let midi_conn = setup_midi(state_clone, decoder_clone);
        
        PaddleDecoderApp {
            state,
            _midi_conn: midi_conn,
            decoder,
            _stream: stream,
            _stream_handle: stream_handle,
        }
    }
}

impl eframe::App for PaddleDecoderApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(ch) = self.decoder.lock().unwrap().check_timeout() {
            self.state.lock().unwrap().decoded_text.push(ch);
        }
        
        {
            let decoder = self.decoder.lock().unwrap();
            let mut state = self.state.lock().unwrap();
            state.current_sequence = decoder.current_sequence.clone();
        }
        
        ctx.request_repaint_after(Duration::from_millis(100));
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                ui.heading("🎹 MORSE CODE PADDLE DECODER 🎹");
                ui.add_space(10.0);
            });
            
            ui.separator();
            ui.add_space(10.0);
            
            let mut state = self.state.lock().unwrap();
            let mut decoder = self.decoder.lock().unwrap();
            
            ui.horizontal(|ui| {
                ui.label("WPM:");
                if ui.add(egui::Slider::new(&mut state.wpm, 5..=40)
                    .text("WPM")).changed() {
                    decoder.update_wpm(state.wpm);
                }
            });
            
            ui.horizontal(|ui| {
                ui.label("Frequency:");
                ui.add(egui::Slider::new(&mut state.frequency, 300..=1000)
                    .step_by(50.0)
                    .text("Hz"));
            });
            
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);
            
            ui.heading("Paddle Status:");
            ui.horizontal(|ui| {
                let left_color = if state.left_pressed {
                    egui::Color32::RED
                } else {
                    egui::Color32::DARK_GRAY
                };
                
                let right_color = if state.right_pressed {
                    egui::Color32::RED
                } else {
                    egui::Color32::DARK_GRAY
                };
                
                ui.label(egui::RichText::new("LEFT (Dit)")
                    .size(20.0)
                    .color(left_color));
                
                ui.add_space(20.0);
                
                ui.label(egui::RichText::new("RIGHT (Dah)")
                    .size(20.0)
                    .color(right_color));
            });
            
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);
            
            ui.heading("Current Sequence:");
            ui.label(egui::RichText::new(&state.current_sequence)
                .size(24.0)
                .monospace()
                .color(egui::Color32::LIGHT_BLUE));
            
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);
            
            ui.heading("Decoded Text:");
            
            egui::ScrollArea::vertical()
                .max_height(150.0)
                .show(ui, |ui| {
                    ui.label(egui::RichText::new(&state.decoded_text)
                        .size(18.0)
                        .monospace()
                        .color(egui::Color32::LIGHT_GREEN));
                });
            
            ui.add_space(10.0);
            
            ui.horizontal(|ui| {
                if ui.button("Add Space").clicked() {
                    state.decoded_text.push(' ');
                }
                
                if ui.button("Clear Text").clicked() {
                    state.decoded_text.clear();
                }
            });
            
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);
            
            ui.heading("Timing:");
            ui.horizontal(|ui| {
                ui.label(format!("Dit: {}ms", decoder.dit_length_ms));
                ui.add_space(10.0);
                ui.label(format!("Dah: {}ms", decoder.dit_length_ms * 3));
                ui.add_space(10.0);
                ui.label(format!("Letter gap: {}ms", decoder.letter_space_ms));
            });
        });
    }
}

// Automatic keyer thread - generates repeated dits/dahs
fn automatic_keyer_thread(
    state: Arc<Mutex<AppState>>,
    decoder: Arc<Mutex<MorseDecoder>>,
    sink: Arc<Mutex<Sink>>,
) {
    loop {
        thread::sleep(Duration::from_millis(10));
        
        let (left_pressed, right_pressed, wpm, frequency) = {
            let s = state.lock().unwrap();
            (s.left_pressed, s.right_pressed, s.wpm, s.frequency)
        };
        
        if left_pressed || right_pressed {
            let dit_ms = 1200 / wpm.max(1);
            let dah_ms = dit_ms * 3;
            
            if left_pressed {
                // Generate DIT
                decoder.lock().unwrap().add_element(true);
                
                // Play DIT tone
                {
                    let snk = sink.lock().unwrap();
                    snk.stop();
                    let tone = ToneGenerator::new(frequency as f32);
                    snk.append(tone);
                    snk.play();
                }
                
                thread::sleep(Duration::from_millis(dit_ms as u64));
                
                // Stop tone
                sink.lock().unwrap().stop();
                
                // Element space (1 dit length pause)
                thread::sleep(Duration::from_millis(dit_ms as u64));
                
            } else if right_pressed {
                // Generate DAH
                decoder.lock().unwrap().add_element(false);
                
                // Play DAH tone
                {
                    let snk = sink.lock().unwrap();
                    snk.stop();
                    let tone = ToneGenerator::new(frequency as f32);
                    snk.append(tone);
                    snk.play();
                }
                
                thread::sleep(Duration::from_millis(dah_ms as u64));
                
                // Stop tone
                sink.lock().unwrap().stop();
                
                // Element space (1 dit length pause)
                thread::sleep(Duration::from_millis(dit_ms as u64));
            }
        }
    }
}

// Setup MIDI connection
fn setup_midi(
    state: Arc<Mutex<AppState>>,
    _decoder: Arc<Mutex<MorseDecoder>>,
) -> Option<MidiInputConnection<()>> {
    let midi_in = match MidiInput::new("paddle-decoder-gui") {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Failed to create MIDI input: {}", e);
            return None;
        }
    };
    
    let ports = midi_in.ports();
    
    if ports.is_empty() {
        eprintln!("No MIDI ports found!");
        return None;
    }
    
    println!("Available MIDI ports:");
    for (i, port) in ports.iter().enumerate() {
        println!("  {}: {}", i, midi_in.port_name(port).unwrap_or_default());
    }
    
    let port = ports.iter()
        .find(|p| {
            let name = midi_in.port_name(p).unwrap_or_default();
            name.contains("MidiStomp") || name.contains("MIDI")
        })
        .or(ports.first())
        .cloned();
    
    let port = match port {
        Some(p) => p,
        None => {
            eprintln!("No suitable MIDI port found");
            return None;
        }
    };
    
    println!("Connecting to: {}", midi_in.port_name(&port).unwrap_or_default());
    
    let conn = midi_in.connect(
        &port,
        "paddle-decoder-gui",
        move |_timestamp, message, _| {
            if message.len() >= 3 {
                let status = message[0];
                let note = message[1];
                let velocity = message[2];
                
                if status == 0x90 {
                    let mut s = state.lock().unwrap();
                    
                    if velocity > 0 {
                        // Paddle pressed
                        match note {
                            1 => s.left_pressed = true,
                            2 => s.right_pressed = true,
                            _ => {}
                        }
                    } else {
                        // Paddle released
                        match note {
                            1 => s.left_pressed = false,
                            2 => s.right_pressed = false,
                            _ => {}
                        }
                    }
                }
            }
        },
        (),
    );
    
    match conn {
        Ok(c) => {
            println!("MIDI connection established!");
            Some(c)
        },
        Err(e) => {
            eprintln!("Failed to connect to MIDI port: {}", e);
            None
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    println!("===========================================");
    println!("    Paddle Decoder with Auto Keyer");
    println!("===========================================\n");
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([600.0, 400.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Paddle Decoder",
        options,
        Box::new(|cc| Box::new(PaddleDecoderApp::new(cc))),
    )
}
