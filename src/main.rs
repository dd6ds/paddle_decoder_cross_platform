use eframe::egui;
use midir::{MidiInput, MidiInputConnection};
use rodio::{OutputStream, OutputStreamHandle, Sink, Source};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread;

mod cw_academy_training;
use cw_academy_training::{SessionNumber, PracticeType, get_session, get_all_sessions, 
                          get_practice_types, get_session_description, BlockSize, 
                          get_block_sizes, generate_random_block};

mod morse_player;
use morse_player::{MorsePlayer, MorseElement};

#[derive(Debug, Clone, Copy, PartialEq)]
enum TrainingState {
    SelectingItem,
    Playing,
    WaitingForInput,
    CheckingAnswer,
}

#[derive(Debug, Clone)]
struct TrainingData {
    state: TrainingState,
    current_text: String,
    user_input: String,
    attempts: u32,
    feedback_message: String,
    feedback_color: egui::Color32,
    show_answer: bool,
}

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
    
    fn check_timeout(&mut self) -> Option<String> {
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
    
    fn decode_sequence(&self) -> String {
        match self.current_sequence.as_str() {
            // Letters
            ".-" => "A", "-..." => "B", "-.-." => "C", "-.." => "D",
            "." => "E", "..-." => "F", "--." => "G", "...." => "H",
            ".." => "I", ".---" => "J", "-.-" => "K", ".-.." => "L",
            "--" => "M", "-." => "N", "---" => "O", ".--." => "P",
            "--.-" => "Q", ".-." => "R", "..." => "S", "-" => "T",
            "..-" => "U", "...-" => "V", ".--" => "W", "-..-" => "X",
            "-.--" => "Y", "--.." => "Z",
            // Numbers
            "-----" => "0", ".----" => "1", "..---" => "2", "...--" => "3",
            "....-" => "4", "....." => "5", "-...." => "6", "--..." => "7",
            "---.." => "8", "----." => "9",
            // Punctuation
            ".-.-.-" => ".", "--..--" => ",", "..--.." => "?", ".----." => "'",
            "-.-.--" => "!", "-..-." => "/", "-.--." => "(", "-.--.-" => ")",
            ".-..." => "&", "---..." => ":", "-.-.-." => ";", "-...-" => "=",
            ".-.-." => "+", "-....-" => "-", "..--.-" => "_", ".-..-." => "\"",
            "...-..-" => "$", ".--.-." => "@",
            // Prosigns (special multi-character sequences)
            "-..-.-" => "<BK>",  // Break (pause in transmission)
            ".-.-.." => "<AR>",  // End of message
            // NOTE: <BT> uses same pattern as "=" (-...-), so it decodes as "="
            "...-.-" => "<SK>",  // End of contact/silent key
            _ => "?",
        }.to_string()
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

// Function to play morse code sequence
fn play_morse_sequence(
    morse_elements: Vec<MorseElement>,
    sink: Arc<Mutex<Sink>>,
    wpm: u32,
    frequency: u32,
) {
    thread::spawn(move || {
        let dit_ms = 1200 / wpm.max(1);
        
        for element in morse_elements {
            match element {
                MorseElement::Dit => {
                    // Play dit
                    {
                        let snk = sink.lock().unwrap();
                        snk.stop();
                        let tone = ToneGenerator::new(frequency as f32);
                        snk.append(tone);
                        snk.play();
                    }
                    thread::sleep(Duration::from_millis(dit_ms as u64));
                    sink.lock().unwrap().stop();
                    // Element space
                    thread::sleep(Duration::from_millis(dit_ms as u64));
                }
                MorseElement::Dah => {
                    // Play dah
                    {
                        let snk = sink.lock().unwrap();
                        snk.stop();
                        let tone = ToneGenerator::new(frequency as f32);
                        snk.append(tone);
                        snk.play();
                    }
                    thread::sleep(Duration::from_millis((dit_ms * 3) as u64));
                    sink.lock().unwrap().stop();
                    // Element space
                    thread::sleep(Duration::from_millis(dit_ms as u64));
                }
                MorseElement::LetterSpace => {
                    // Extra space between letters (already have 1 dit, add 2 more = 3 total)
                    thread::sleep(Duration::from_millis((dit_ms * 2) as u64));
                }
                MorseElement::WordSpace => {
                    // Space between words (7 dits total, already have 1, add 6 more)
                    thread::sleep(Duration::from_millis((dit_ms * 6) as u64));
                }
            }
        }
    });
}

// Shared application state
#[derive(Clone)]
struct AppState {
    left_pressed: bool,
    right_pressed: bool,
    decoded_text: String,
    current_sequence: String,
    wpm: u32,
    farnsworth_wpm: u32,  // Effective WPM with Farnsworth spacing
    frequency: u32,
    // Training mode
    training_mode: bool,
    current_session: SessionNumber,
    current_practice_type: PracticeType,
    current_training_text: String,
    // Listening mode
    listening_mode: bool,
    correct_answer: String,
    attempt_count: u32,
    show_result: bool,
    result_correct: bool,
    show_answer: bool,
    // Random blocks mode
    random_blocks_mode: bool,
    block_from_session: SessionNumber,
    block_to_session: SessionNumber,
    block_size: BlockSize,
    // Statistics
    correct_count: u32,
    wrong_count: u32,
    // Timeout feature
    timeout_enabled: bool,
    timeout_seconds: u32,
    timeout_start: Option<Instant>,
    // Wrong answers tracking for repeat
    wrong_answers: Vec<String>,
    repeat_wrong_chance: f32, // 0.0 to 1.0, probability to repeat a wrong answer
    // Result display timing
    result_display_start: Option<Instant>,
    result_display_duration: u64, // seconds to show result before moving to next
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            left_pressed: false,
            right_pressed: false,
            decoded_text: String::new(),
            current_sequence: String::new(),
            wpm: 20,
            farnsworth_wpm: 15,  // Default Farnsworth spacing
            frequency: 600,
            training_mode: false,
            current_session: SessionNumber::Session1,
            current_practice_type: PracticeType::Characters,
            current_training_text: String::new(),
            listening_mode: false,
            correct_answer: String::new(),
            attempt_count: 0,
            show_result: false,
            result_correct: false,
            show_answer: false,
            random_blocks_mode: false,
            block_from_session: SessionNumber::Session1,
            block_to_session: SessionNumber::Session5,
            block_size: BlockSize::Fixed3,
            correct_count: 0,
            wrong_count: 0,
            timeout_enabled: true,
            timeout_seconds: 15,
            timeout_start: None,
            wrong_answers: Vec::new(),
            repeat_wrong_chance: 0.3, // 30% chance to repeat wrong answers
            result_display_start: None,
            result_display_duration: 3, // Show result for 3 seconds before moving to next
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
    playback_sink: Arc<Mutex<Sink>>,
}

impl PaddleDecoderApp {
    // Helper method to get next training item with wrong answer repeat logic
    fn get_next_training_item(state: &mut AppState) -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        // Decide if we should repeat a wrong answer
        if !state.wrong_answers.is_empty() && rng.gen::<f32>() < state.repeat_wrong_chance {
            // Select a random wrong answer to repeat
            let index = rng.gen_range(0..state.wrong_answers.len());
            return state.wrong_answers[index].clone();
        }
        
        // Otherwise generate a new item
        if state.random_blocks_mode {
            generate_random_block(
                state.block_from_session,
                state.block_to_session,
                state.block_size
            )
        } else {
            let session = get_session(state.current_session);
            session.get_random_item(state.current_practice_type)
                .map(|item| item.to_string())
                .unwrap_or_default()
        }
    }
    
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let (stream, stream_handle) = OutputStream::try_default()
            .expect("Failed to open audio stream");
        
        let sink = Arc::new(Mutex::new(
            Sink::try_new(&stream_handle).expect("Failed to create audio sink")
        ));
        
        // Create a separate sink for morse playback
        let playback_sink = Arc::new(Mutex::new(
            Sink::try_new(&stream_handle).expect("Failed to create playback sink")
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
            playback_sink,
        }
    }
}

impl eframe::App for PaddleDecoderApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(decoded_str) = self.decoder.lock().unwrap().check_timeout() {
            self.state.lock().unwrap().decoded_text.push_str(&decoded_str);
        }
        
        {
            let decoder = self.decoder.lock().unwrap();
            let mut state = self.state.lock().unwrap();
            state.current_sequence = decoder.current_sequence.clone();
            
            // Check timeout in listening mode
            if state.listening_mode && state.timeout_enabled {
                // First check if we're displaying a result and should move to next
                if let Some(display_start) = state.result_display_start {
                    if display_start.elapsed().as_secs() >= state.result_display_duration {
                        // Time to move to next item
                        let next_item = Self::get_next_training_item(&mut state);
                        state.current_training_text = next_item.clone();
                        state.correct_answer = next_item.clone();
                        state.decoded_text.clear();
                        state.show_result = false;
                        state.show_answer = false;
                        state.attempt_count = 0;
                        state.timeout_start = Some(Instant::now());
                        state.result_display_start = None;
                        
                        // Auto-play the next item
                        let sink_clone = Arc::clone(&self.playback_sink);
                        let wpm = state.wpm;
                        let farnsworth = state.farnsworth_wpm;
                        let freq = state.frequency;
                        let training_text = state.current_training_text.clone();
                        
                        drop(state); // Release lock before spawning thread
                        
                        thread::spawn(move || {
                            let player = MorsePlayer::new_with_farnsworth(freq as f32, wpm, farnsworth);
                            let sink = sink_clone.lock().unwrap();
                            player.play_morse(&sink, &training_text);
                        });
                        return; // Exit early to avoid borrowing issues
                    }
                }
                
                // Check if timeout has expired on current question
                if let Some(start_time) = state.timeout_start {
                    if !state.correct_answer.is_empty() && !state.show_result {
                        let elapsed = start_time.elapsed().as_secs();
                        if elapsed >= state.timeout_seconds as u64 {
                            // Timeout expired - check the answer
                            let decoded = state.decoded_text.trim().to_uppercase();
                            let correct = state.correct_answer.trim().to_uppercase();
                            
                            if decoded == correct {
                                // Answer is CORRECT
                                state.result_correct = true;
                                state.correct_count += 1;
                                // Remove from wrong answers if it was there
                                state.wrong_answers.retain(|item| item.trim().to_uppercase() != correct);
                            } else {
                                // Answer is WRONG
                                state.result_correct = false;
                                state.wrong_count += 1;
                                state.show_answer = true; // Show the correct answer
                                
                                // Add to wrong answers list if not already there
                                let correct_ans = state.correct_answer.clone();
                                if !correct_ans.is_empty() && !state.wrong_answers.contains(&correct_ans) {
                                    state.wrong_answers.push(correct_ans);
                                }
                            }
                            
                            // Show result and start result display timer
                            state.show_result = true;
                            state.timeout_start = None;
                            state.result_display_start = Some(Instant::now());
                        }
                    }
                }
            }
        }
        
        ctx.request_repaint_after(Duration::from_millis(100));
        
        
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
            
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
                if ui.add(egui::Slider::new(&mut state.wpm, 1..=40)
                    .text("WPM")).changed() {
                    decoder.update_wpm(state.wpm);
                    // Ensure Farnsworth WPM doesn't exceed character WPM
                    if state.farnsworth_wpm > state.wpm {
                        state.farnsworth_wpm = state.wpm;
                    }
                }
            });
            
            ui.horizontal(|ui| {
                ui.label("Farnsworth WPM:");
                let max_farnsworth = state.wpm;
                ui.add(egui::Slider::new(&mut state.farnsworth_wpm, 1..=max_farnsworth)
                    .text("Eff. WPM"));
                ui.label("ℹ").on_hover_text(
                    "Effective WPM with extended spacing.\n\
                     Characters sent at full WPM, but with extra spacing.\n\
                     Lower = more time to think between characters."
                );
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
            
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);
            
            // Training Mode Section
            ui.heading("🎓 CW Academy Training Mode");
            
            ui.horizontal(|ui| {
                if ui.button(if state.training_mode { "Disable Training" } else { "Enable Training" }).clicked() {
                    state.training_mode = !state.training_mode;
                    state.listening_mode = false;
                    state.show_result = false;
                    state.show_answer = false;
                    if state.training_mode {
                        let session = get_session(state.current_session);
                        if let Some(item) = session.get_random_item(state.current_practice_type) {
                            state.current_training_text = item.to_string();
                        }
                    }
                }
            });
            
            if state.training_mode {
                ui.add_space(10.0);
                
                // Session Selection
                ui.horizontal(|ui| {
                    ui.label("Session:");
                    egui::ComboBox::from_label("")
                        .selected_text(get_session_description(state.current_session))
                        .show_ui(ui, |ui| {
                            for session in get_all_sessions() {
                                if ui.selectable_value(&mut state.current_session, session, 
                                                       get_session_description(session)).clicked() {
                                    let sess = get_session(state.current_session);
                                    if let Some(item) = sess.get_random_item(state.current_practice_type) {
                                        state.current_training_text = item.to_string();
                                    }
                                    state.listening_mode = false;
                                    state.show_result = false;
                                    state.show_answer = false;
                                }
                            }
                        });
                });
                
                // Practice Type Selection
                ui.horizontal(|ui| {
                    ui.label("Practice Type:");
                    egui::ComboBox::from_label(" ")
                        .selected_text(state.current_practice_type.as_str())
                        .show_ui(ui, |ui| {
                            for practice_type in get_practice_types() {
                                if ui.selectable_value(&mut state.current_practice_type, practice_type,
                                                       practice_type.as_str()).clicked() {
                                    let session = get_session(state.current_session);
                                    if let Some(item) = session.get_random_item(state.current_practice_type) {
                                        state.current_training_text = item.to_string();
                                    }
                                    state.listening_mode = false;
                                    state.show_result = false;
                                    state.show_answer = false;
                                }
                            }
                        });
                });
                
                ui.add_space(10.0);
                
                // Random Blocks Mode Toggle
                ui.horizontal(|ui| {
                    if ui.checkbox(&mut state.random_blocks_mode, "🎲 Random Blocks Mode").changed() {
                        if state.random_blocks_mode {
                            // Generate first random block
                            state.current_training_text = generate_random_block(
                                state.block_from_session,
                                state.block_to_session,
                                state.block_size
                            );
                        }
                    }
                });
                
                if state.random_blocks_mode {
                    ui.add_space(5.0);
                    
                    ui.group(|ui| {
                        ui.label(egui::RichText::new("Random Blocks Settings").strong());
                        
                        ui.horizontal(|ui| {
                            ui.label("From Session:");
                            egui::ComboBox::from_label("")
                                .selected_text(format!("{}", state.block_from_session.as_number()))
                                .show_ui(ui, |ui| {
                                    for session in get_all_sessions() {
                                        if ui.selectable_value(
                                            &mut state.block_from_session,
                                            session,
                                            format!("{}", session.as_number())
                                        ).clicked() {
                                            // Regenerate block
                                            state.current_training_text = generate_random_block(
                                                state.block_from_session,
                                                state.block_to_session,
                                                state.block_size
                                            );
                                        }
                                    }
                                });
                            
                            ui.label("To Session:");
                            egui::ComboBox::from_label(" ")
                                .selected_text(format!("{}", state.block_to_session.as_number()))
                                .show_ui(ui, |ui| {
                                    for session in get_all_sessions() {
                                        if ui.selectable_value(
                                            &mut state.block_to_session,
                                            session,
                                            format!("{}", session.as_number())
                                        ).clicked() {
                                            // Regenerate block
                                            state.current_training_text = generate_random_block(
                                                state.block_from_session,
                                                state.block_to_session,
                                                state.block_size
                                            );
                                        }
                                    }
                                });
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("Block Size:");
                            egui::ComboBox::from_label("  ")
                                .selected_text(state.block_size.as_str())
                                .show_ui(ui, |ui| {
                                    for size in get_block_sizes() {
                                        if ui.selectable_value(
                                            &mut state.block_size,
                                            size,
                                            size.as_str()
                                        ).clicked() {
                                            // Regenerate block
                                            state.current_training_text = generate_random_block(
                                                state.block_from_session,
                                                state.block_to_session,
                                                state.block_size
                                            );
                                        }
                                    }
                                });
                        });
                        
                        ui.add_space(5.0);
                        ui.label(egui::RichText::new(format!(
                            "📚 Training characters from Session {} to {}",
                            state.block_from_session.as_number(),
                            state.block_to_session.as_number()
                        ))
                        .italics()
                        .size(11.0)
                        .color(egui::Color32::LIGHT_BLUE));
                    });
                }
                
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(5.0);
                
                // Mode Toggle: Sending Practice vs Listening Practice
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Training Mode:").strong());
                    if ui.selectable_label(!state.listening_mode, "📝 Sending Practice").clicked() {
                        state.listening_mode = false;
                        state.show_result = false;
                        state.show_answer = false;
                    }
                    if ui.selectable_label(state.listening_mode, "🎧 Listening Practice").clicked() {
                        state.listening_mode = true;
                        state.show_result = false;
                        state.show_answer = false;
                        state.attempt_count = 0;
                        state.decoded_text.clear();
                    }
                });
                
                ui.add_space(10.0);
                
                if !state.listening_mode {
                    // SENDING PRACTICE MODE
                    ui.group(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.heading("Practice This:");
                            ui.add_space(10.0);
                            ui.label(egui::RichText::new(&state.current_training_text)
                                .size(32.0)
                                .monospace()
                                .color(egui::Color32::YELLOW));
                            ui.add_space(10.0);
                            
                            if ui.button("Next Item").clicked() {
                                if state.random_blocks_mode {
                                    state.current_training_text = generate_random_block(
                                        state.block_from_session,
                                        state.block_to_session,
                                        state.block_size
                                    );
                                } else {
                                    let session = get_session(state.current_session);
                                    if let Some(item) = session.get_random_item(state.current_practice_type) {
                                        state.current_training_text = item.to_string();
                                    }
                                }
                            }
                        });
                    });
                    
                    ui.add_space(5.0);
                    ui.label(egui::RichText::new("💡 Send the text above using your paddle, then click 'Next Item'")
                        .italics()
                        .size(12.0)
                        .color(egui::Color32::LIGHT_GRAY));
                        
                } else {
                    // LISTENING PRACTICE MODE
                    ui.group(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.heading("🎧 Listen and Decode");
                            ui.add_space(10.0);
                            
                            // Statistics display
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(format!("✓ Correct: {}", state.correct_count))
                                    .size(16.0)
                                    .color(egui::Color32::from_rgb(0, 255, 0)));
                                ui.add_space(20.0);
                                ui.label(egui::RichText::new(format!("✗ Wrong: {}", state.wrong_count))
                                    .size(16.0)
                                    .color(egui::Color32::from_rgb(255, 100, 100)));
                                ui.add_space(20.0);
                                if state.correct_count + state.wrong_count > 0 {
                                    let accuracy = (state.correct_count as f32 / (state.correct_count + state.wrong_count) as f32) * 100.0;
                                    ui.label(egui::RichText::new(format!("📊 {:.1}%", accuracy))
                                        .size(16.0)
                                        .color(egui::Color32::from_rgb(100, 200, 255)));
                                }
                            });
                            
                            ui.horizontal(|ui| {
                                if ui.button("Reset Statistics").clicked() {
                                    state.correct_count = 0;
                                    state.wrong_count = 0;
                                    state.wrong_answers.clear();
                                }
                            });
                            
                            ui.add_space(10.0);
                            ui.separator();
                            ui.add_space(10.0);
                            
                            // Timeout configuration
                            ui.horizontal(|ui| {
                                ui.checkbox(&mut state.timeout_enabled, "⏱ Enable Auto-Timeout");
                                if state.timeout_enabled {
                                    ui.label("Timeout:");
                                    ui.add(egui::Slider::new(&mut state.timeout_seconds, 10..=120)
                                        .suffix(" sec"));
                                }
                            });
                            
                            // Show time remaining if timeout is active
                            if state.timeout_enabled {
                                if let Some(start_time) = state.timeout_start {
                                    if !state.correct_answer.is_empty() && !state.show_result {
                                        let elapsed = start_time.elapsed().as_secs();
                                        let remaining = state.timeout_seconds.saturating_sub(elapsed as u32);
                                        let color = if remaining <= 5 {
                                            egui::Color32::from_rgb(255, 0, 0)
                                        } else if remaining <= 10 {
                                            egui::Color32::from_rgb(255, 165, 0)
                                        } else {
                                            egui::Color32::from_rgb(100, 200, 255)
                                        };
                                        ui.label(egui::RichText::new(format!("⏱ {}s remaining", remaining))
                                            .size(14.0)
                                            .color(color));
                                    }
                                }
                            }
                            
                            ui.add_space(10.0);
                            
                            let sink_clone = Arc::clone(&self.playback_sink);
                            let wpm = state.wpm;
                            let farnsworth = state.farnsworth_wpm;
                            let freq = state.frequency;
                            let training_text = state.current_training_text.clone();
                            
                            if ui.button(egui::RichText::new("▶ Play Morse Code")
                                .size(20.0)
                                .color(egui::Color32::LIGHT_BLUE)).clicked() {
                                
                                state.correct_answer = training_text.clone();
                                state.decoded_text.clear();
                                state.show_result = false;
                                state.show_answer = false;
                                state.attempt_count = 0;
                                
                                // Start timeout timer if enabled
                                if state.timeout_enabled {
                                    state.timeout_start = Some(Instant::now());
                                }
                                
                                // Play morse in background thread
                                thread::spawn(move || {
                                    let player = MorsePlayer::new_with_farnsworth(freq as f32, wpm, farnsworth);
                                    let sink = sink_clone.lock().unwrap();
                                    player.play_morse(&sink, &training_text);
                                });
                            }
                            
                            ui.add_space(10.0);
                            
                            if !state.correct_answer.is_empty() {
                                ui.label(egui::RichText::new("Now send what you heard using your paddle:")
                                    .size(14.0)
                                    .italics());
                                ui.add_space(10.0);
                                
                                // Show result
                                if state.show_result {
                                    if state.result_correct {
                                        ui.label(egui::RichText::new("✓ RIGHT!")
                                            .size(40.0)
                                            .color(egui::Color32::from_rgb(0, 255, 0)));
                                    } else {
                                        ui.label(egui::RichText::new("✗ FALSE")
                                            .size(40.0)
                                            .color(egui::Color32::from_rgb(255, 0, 0)));
                                    }
                                    
                                    // Show countdown if auto-moving to next (timeout enabled)
                                    if state.timeout_enabled {
                                        if let Some(display_start) = state.result_display_start {
                                            let elapsed = display_start.elapsed().as_secs();
                                            let remaining = state.result_display_duration.saturating_sub(elapsed);
                                            if remaining > 0 {
                                                ui.label(egui::RichText::new(format!("⏱ Next item in {}s...", remaining))
                                                    .size(14.0)
                                                    .color(egui::Color32::from_rgb(150, 150, 150)));
                                            }
                                        }
                                    }
                                    
                                    ui.add_space(10.0);
                                }
                                
                                // Show answer after 2 failed attempts
                                if state.show_answer {
                                    ui.group(|ui| {
                                        ui.label(egui::RichText::new("The correct answer was:")
                                            .size(14.0));
                                        ui.label(egui::RichText::new(&state.correct_answer)
                                            .size(28.0)
                                            .monospace()
                                            .color(egui::Color32::YELLOW));
                                    });
                                    ui.add_space(10.0);
                                }
                                
                                if ui.button(egui::RichText::new("Check Answer")
                                    .size(18.0)).clicked() {
                                    
                                    let decoded = state.decoded_text.trim().to_uppercase();
                                    let correct = state.correct_answer.trim().to_uppercase();
                                    
                                    if decoded == correct {
                                        state.result_correct = true;
                                        state.show_result = true;
                                        state.show_answer = false;
                                        state.correct_count += 1;
                                        state.timeout_start = None; // Stop timeout
                                        state.result_display_start = None; // Manual check, no auto-next
                                        
                                        // Remove from wrong answers if it was there
                                        state.wrong_answers.retain(|item| item.trim().to_uppercase() != correct);
                                    } else {
                                        state.result_correct = false;
                                        state.show_result = true;
                                        state.attempt_count += 1;
                                        
                                        if state.attempt_count >= 2 {
                                            state.show_answer = true;
                                            state.wrong_count += 1;
                                            state.timeout_start = None; // Stop timeout
                                            state.result_display_start = None; // Manual check, no auto-next
                                            
                                            // Add to wrong answers list if not already there
                                            let correct_ans = state.correct_answer.clone();
                                            if !state.wrong_answers.contains(&correct_ans) {
                                                state.wrong_answers.push(correct_ans);
                                            }
                                        } else {
                                            // Play again
                                            let sink_clone = Arc::clone(&self.playback_sink);
                                            let training_text = state.current_training_text.clone();
                                            let wpm_local = wpm;
                                            let farnsworth_local = farnsworth;
                                            let freq_local = freq;
                                            
                                            // Restart timeout timer
                                            if state.timeout_enabled {
                                                state.timeout_start = Some(Instant::now());
                                            }
                                            
                                            thread::spawn(move || {
                                                thread::sleep(Duration::from_millis(1000));
                                                let player = MorsePlayer::new_with_farnsworth(freq_local as f32, wpm_local, farnsworth_local);
                                                let sink = sink_clone.lock().unwrap();
                                                player.play_morse(&sink, &training_text);
                                            });
                                        }
                                    }
                                }
                                
                                ui.add_space(10.0);
                                
                                if ui.button("Next Item").clicked() {
                                    // Generate next item using helper method
                                    let next_item = Self::get_next_training_item(&mut state);
                                    state.current_training_text = next_item.clone();
                                    
                                    state.correct_answer.clear();
                                    state.decoded_text.clear();
                                    state.show_result = false;
                                    state.show_answer = false;
                                    state.attempt_count = 0;
                                    state.timeout_start = None;
                                    state.result_display_start = None;
                                    
                                    // Auto-play if timeout is enabled
                                    if state.timeout_enabled {
                                        state.correct_answer = next_item.clone();
                                        state.timeout_start = Some(Instant::now());
                                        
                                        let sink_clone = Arc::clone(&self.playback_sink);
                                        let wpm_local = state.wpm;
                                        let farnsworth_local = state.farnsworth_wpm;
                                        let freq_local = state.frequency;
                                        
                                        thread::spawn(move || {
                                            let player = MorsePlayer::new_with_farnsworth(freq_local as f32, wpm_local, farnsworth_local);
                                            let sink = sink_clone.lock().unwrap();
                                            player.play_morse(&sink, &next_item);
                                        });
                                    }
                                }
                            }
                        });
                    });
                    
                    ui.add_space(5.0);
                    ui.label(egui::RichText::new("💡 Click 'Play Morse Code', listen carefully, then decode it with your paddle")
                        .italics()
                        .size(12.0)
                        .color(egui::Color32::LIGHT_GRAY));
                }
            }
            }); // Close ScrollArea
        });
    }
}
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
            .with_inner_size([900.0, 950.0])
            .with_min_inner_size([700.0, 600.0])
            .with_resizable(true)
            .with_maximized(false),
        ..Default::default()
    };
    
    eframe::run_native(
        "Paddle Decoder",
        options,
        Box::new(|cc| Box::new(PaddleDecoderApp::new(cc))),
    )
}
