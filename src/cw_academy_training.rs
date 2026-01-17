// CW Academy Training Data - Sessions 1-10
// Based on CWops Beginner CW Curriculum Rev 4.2.8.1

use rand::seq::SliceRandom;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SessionNumber {
    Session1,
    Session2,
    Session3,
    Session4,
    Session5,
    Session6,
    Session7,
    Session8,
    Session9,
    Session10,
}

impl SessionNumber {
    pub fn as_number(&self) -> u8 {
        match self {
            SessionNumber::Session1 => 1,
            SessionNumber::Session2 => 2,
            SessionNumber::Session3 => 3,
            SessionNumber::Session4 => 4,
            SessionNumber::Session5 => 5,
            SessionNumber::Session6 => 6,
            SessionNumber::Session7 => 7,
            SessionNumber::Session8 => 8,
            SessionNumber::Session9 => 9,
            SessionNumber::Session10 => 10,
        }
    }
    
    pub fn all_values() -> Vec<SessionNumber> {
        vec![
            SessionNumber::Session1,
            SessionNumber::Session2,
            SessionNumber::Session3,
            SessionNumber::Session4,
            SessionNumber::Session5,
            SessionNumber::Session6,
            SessionNumber::Session7,
            SessionNumber::Session8,
            SessionNumber::Session9,
            SessionNumber::Session10,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PracticeType {
    Characters,
    Words,
    Abbreviations,
    Numbers,
    Callsigns,
    Phrases,
}

impl PracticeType {
    pub fn as_str(&self) -> &str {
        match self {
            PracticeType::Characters => "Characters",
            PracticeType::Words => "Words",
            PracticeType::Abbreviations => "CW Abbreviations",
            PracticeType::Numbers => "Numbers",
            PracticeType::Callsigns => "Callsigns",
            PracticeType::Phrases => "Phrases",
        }
    }
}

pub struct TrainingSession {
    characters: Vec<&'static str>,
    words: Vec<&'static str>,
    abbreviations: Vec<&'static str>,
    numbers: Vec<&'static str>,
    callsigns: Vec<&'static str>,
    phrases: Vec<&'static str>,
}

impl TrainingSession {
    pub fn get_practice_items(&self, practice_type: PracticeType) -> Vec<&'static str> {
        match practice_type {
            PracticeType::Characters => self.characters.clone(),
            PracticeType::Words => self.words.clone(),
            PracticeType::Abbreviations => self.abbreviations.clone(),
            PracticeType::Numbers => self.numbers.clone(),
            PracticeType::Callsigns => self.callsigns.clone(),
            PracticeType::Phrases => self.phrases.clone(),
        }
    }
    
    pub fn get_random_item(&self, practice_type: PracticeType) -> Option<&'static str> {
        let items = self.get_practice_items(practice_type);
        if items.is_empty() {
            return None;
        }
        let mut rng = rand::thread_rng();
        items.choose(&mut rng).copied()
    }
}

// Session 1: A E N T
fn session_1() -> TrainingSession {
    TrainingSession {
        characters: vec!["A", "T", "E", "N"],
        words: vec!["TEA", "TEE", "EAT", "ATE", "AT", "TAT", "TEEN", "NEAT", "TEN", "TAN"],
        abbreviations: vec!["AA", "ANT", "NET"],
        numbers: vec![],
        callsigns: vec![],
        phrases: vec!["EAT AT TEN", "AN ANT AT TEE", "A NEAT ANT"],
    }
}

// Session 2: A E N T S I O 1 4
fn session_2() -> TrainingSession {
    TrainingSession {
        characters: vec!["A", "E", "N", "T", "S", "I", "O", "1", "4"],
        words: vec!["TEN", "TON", "TIN", "TIE", "TOE", "NO", "NOT", "NOTE", "IT", "AT", 
                   "ONE", "NEAT", "NET", "NITE", "TOES", "STONE", "TEASE", "NOISE"],
        abbreviations: vec!["ES", "SAE", "SASE", "SN", "STN"],
        numbers: vec!["1441", "4114", "1414"],
        callsigns: vec!["N1AS", "N4ON", "S41T", "NO1S", "AI1E", "IT4O", "EA1ON", "ES4IT"],
        phrases: vec!["NO NET", "STN 1 TO 4", "ITS A TEST", "4 TON STONE"],
    }
}

// Session 3: O I S D H L R 2 5
fn session_3() -> TrainingSession {
    TrainingSession {
        characters: vec!["O", "I", "S", "R", "H", "D", "L", "2", "5"],
        words: vec!["ALL", "TELL", "TALL", "DEAL", "THE", "THEIR", "DOLL", "DELL", 
                   "HALL", "HILL", "HOLE", "LOAD", "LEAD", "LATE", "LATER", "SEAL", 
                   "SELL", "SOLE", "SHE", "HER", "HEAR"],
        abbreviations: vec!["AA", "ES", "SN", "STN", "DE", "DN", "HI", "HR", "LSN", "NR", "RST"],
        numbers: vec!["142", "451", "1425"],
        callsigns: vec!["DL1AT", "HH5H", "HS1TD", "ND2T", "NA4T"],
        phrases: vec!["LSN DN 1", "RST IS 555", "HAIL ES RAIN", "DE NR4DL", "LSN DN 2"],
    }
}

// Session 4: R H D L 1 4 C U  
fn session_4() -> TrainingSession {
    TrainingSession {
        characters: vec!["R", "H", "D", "L", "1", "4", "U", "C"],
        words: vec!["CHAT", "CHAIR", "CHIN", "CHART", "OUCH", "COUCH", "TOUCH", "SUCH", 
                   "TEACH", "REACH", "SUN", "SON", "HOLD", "TOLD", "SAIL", "RAIL", 
                   "TAIL", "NAIL", "OIL", "SOIL", "TOIL", "COIL", "RAIN", "CAUSE", 
                   "SAUCE", "TOSS", "TOLL", "TALL", "TELL", "CELL", "CALL"],
        abbreviations: vec!["ADR", "CL", "CS", "CUL", "DE", "DN", "ES", "NR", "RST", 
                          "SOTA", "STN", "TU", "UR"],
        numbers: vec!["4241", "1452", "5441", "2145"],
        callsigns: vec!["NC5A", "NA2T", "CU1LL", "CO5NO", "NU4R", "CT1AC", "CE1NI"],
        phrases: vec!["UR RST IS", "HR IN UT", "CUL TOD", "LSN DN 1", "SOTA 2514"],
    }
}

// Session 5: U C 2 5 M W 3 6 ?
fn session_5() -> TrainingSession {
    TrainingSession {
        characters: vec!["U", "C", "2", "5", "M", "W", "3", "6", "?"],
        words: vec!["WAIT", "WALL", "WELL", "WILL", "MALL", "MILL", "CHUM", "MOW", 
                   "MUCH", "SUCH", "WATER", "WET", "WHAT", "DEW", "DATE", "ATOM", 
                   "TOW", "TOWER", "WERE", "WHERE", "WAS", "WISH", "WASH", "MAT", 
                   "MATT", "MEL", "HIM", "HER", "HIS", "HW?"],
        abbreviations: vec!["CS", "CUD", "CUL", "DE", "ES", "HR", "HW?", "OM", 
                          "RST", "SHUD", "STN", "TU", "UR", "WL"],
        numbers: vec!["3354", "1432", "6122", "5564", "4321", "2346"],
        callsigns: vec!["W3AA", "N3AM", "DM5RA", "W6AM", "N2AT", "RW5L", "ON4UN"],
        phrases: vec!["NAME?", "UR RST IS 56N", "CU TMW", "DON", "SWR IS 2 TO 1", "CUL TOM"],
    }
}

// Session 6: M W 3 6 F Y ,
fn session_6() -> TrainingSession {
    TrainingSession {
        characters: vec!["M", "W", "3", "6", "F", "Y", ","],
        words: vec!["YOU", "TOY", "FOOT", "TOOTH", "ROOT", "CUTE", "NOISE", "LARRY", 
                   "ROY", "TON", "TEETH", "FEET", "YET", "THEY", "SAY", "RAY", 
                   "HAY", "YRS", "FAIR", "FARE", "FAR", "FUR", "FURRY"],
        abbreviations: vec!["CUL", "CW", "DE", "DWN", "ES", "FER", "HR", "HW?", 
                          "NAME", "RF", "RST", "SWR", "TU", "MI"],
        numbers: vec!["1512", "3316", "4352", "6135"],
        callsigns: vec!["F5IN", "YO1AR", "HH5H", "NO3M", "AA3U", "S52R"],
        phrases: vec!["NAME IS WILL", "UR RST 56N", "HOME IS CODY, WY", "UR NAME?"],
    }
}

// Session 7: F Y 3 6 G P Q 7 9 /
fn session_7() -> TrainingSession {
    TrainingSession {
        characters: vec!["F", "Y", "3", "6", "G", "P", "Q", "7", "9", "/"],
        words: vec!["PAGE", "PAPER", "PEPPER", "GLAD", "GLARE", "LARGE", "LEDGE", 
                   "GEORGE", "GEO", "CHAS", "CHASE", "CHANGE", "PEG", "PUG", "PIG", 
                   "PEN", "PENCIL", "PIPE", "PIT", "GAIN", "GARAGE", "GUARD", "GAS", 
                   "GUS", "CHUG", "YES", "YET", "YONDER", "COY"],
        abbreviations: vec!["AGN?", "CPI", "CUL", "DE", "GM", "GE", "LID", "OM", 
                          "OP", "QTH", "RIG", "RST", "QRM", "TU"],
        numbers: vec!["7423", "14253", "579", "599", "2N222"],
        callsigns: vec!["G4AN/3", "N1AR/5", "W9UCA/9", "W3/PY2AA", "F6/N6AM"],
        phrases: vec!["GD SIG", "RON SOLID CPI", "MAT RIG IS ICOM", "QTH IS TRAPPE, MD", "HW CPI?"],
    }
}

// Session 8: 7 9 / B V <AR>
fn session_8() -> TrainingSession {
    TrainingSession {
        characters: vec!["B", "V", "7", "9", "/", "<AR>"],
        words: vec!["VOTE", "VAT", "VIEW", "WAVE", "PAVE", "SAVE", "VOW", "VALVE", 
                   "SOLVE", "VOLT", "VAULT", "BAD", "BODY", "BORE", "BORN", "BARN", 
                   "BARNEY", "BRAD", "BREAD", "BED", "BETTER", "BEST", "BILL", 
                   "BUILD", "BUILT", "BOLT", "BULB", "BLAME", "BLEND", "BLAND", "BLOW"],
        abbreviations: vec!["73", "AGN", "B4", "BURO", "CS", "CUL", "DE", "DN", 
                          "EFHW", "ES", "FB", "FER", "GA", "GE", "HI", "HW?", 
                          "LID", "OP", "POTA", "PSE", "PWR", "QRM", "RCVR", "RPT", 
                          "SIG", "TU", "UR", "<AR>"],
        numbers: vec!["6146", "5514", "2345", "9765"],
        callsigns: vec!["BV2AA", "BA1RO", "WB2AE", "N6RB/4", "W2/VE1AR", "VE2/W2LE"],
        phrases: vec!["NAME IS BOB", "NAME IS BILL", "SOME QSB", "NAME IS VAL", 
                     "UR RST IS 559", "UR RST IS 459", "BENS BEST BENT WIRE", "GOING QRT"],
    }
}

// Session 9: B V J K 0 8 <BT>
fn session_9() -> TrainingSession {
    TrainingSession {
        characters: vec!["B", "V", "J", "K", "8", "0", "<BT>"],
        words: vec!["JACK", "JAY", "JOHN", "JIM", "JERRY", "BACK", "RACK", "TACK", 
                   "TECH", "TACH", "REACH", "EACH", "TEACH", "HELP", "HIGH", "HILL", 
                   "FACT", "FACE", "FAR", "FEAR", "THEN", "THEIR", "HIM", "HER", 
                   "HIS", "HERS", "THEM", "THEY", "THEIR", "SWITCH", "LINE", "ANT", 
                   "DIPOLE", "VERTICAL", "OHMS", "HOME", "AWAY", "TEST", "ASIA", "AFRICA"],
        abbreviations: vec!["AGN", "BK", "CPI", "CUAGN", "DE", "ES", "FB", "GUD", 
                          "K", "OK", "OP", "R", "RCVR", "RPT", "RST", "SK", "SKED", 
                          "TKS", "TMW", "WKD", "YL"],
        numbers: vec!["807", "7300", "4250A"],
        callsigns: vec!["K1JD", "N1AR", "W2TT", "K2UMU", "N2NW", "VE3NE", "VA3KP", "K4BAI", "N5KO"],
        phrases: vec!["HW CPI?", "NAME IS JOE", "NAME IS JOHN", "NAME IS JIM", 
                     "UR RST IS 579", "UR RST IS 339", "SRI NO CPY", "QRM ON YR SIG"],
    }
}

// Session 10: K J 8 0 Q X Z . <BK> <SK>
fn session_10() -> TrainingSession {
    TrainingSession {
        characters: vec!["K", "J", "8", "0", "X", "Q", "Z", ".", "<BK>", "<SK>"],
        words: vec!["NAME", "MEMPHIS", "NYC", "SF", "DALLAS", "HOUSTON", "TOKYO", 
                   "PARIS", "LONDON", "HAMBURG", "SYDNEY"],
        abbreviations: vec!["BTU", "CUL", "ES", "DE", "DX", "FB", "FER", "GLD", 
                          "HR", "KW", "OP", "PSE", "RPT", "RST", "SRI", "TKS", 
                          "TU", "VY", "WX", "XYL", "YL"],
        numbers: vec!["8044", "7400", "73", "88"],
        callsigns: vec!["ZL2TT", "VK4OM", "JE1TRV", "BA1CW", "KH6LC", "AL2A", "AA3B"],
        phrases: vec!["U HV QSB", "U HV QRM", "NAME?", "QTH?", "QTH IS NY", 
                     "QTH IS PARIS", "PSE QSY TO 7054"],
    }
}

// Public API
pub fn get_session(session: SessionNumber) -> TrainingSession {
    match session {
        SessionNumber::Session1 => session_1(),
        SessionNumber::Session2 => session_2(),
        SessionNumber::Session3 => session_3(),
        SessionNumber::Session4 => session_4(),
        SessionNumber::Session5 => session_5(),
        SessionNumber::Session6 => session_6(),
        SessionNumber::Session7 => session_7(),
        SessionNumber::Session8 => session_8(),
        SessionNumber::Session9 => session_9(),
        SessionNumber::Session10 => session_10(),
    }
}

pub fn get_all_sessions() -> Vec<SessionNumber> {
    vec![
        SessionNumber::Session1,
        SessionNumber::Session2,
        SessionNumber::Session3,
        SessionNumber::Session4,
        SessionNumber::Session5,
        SessionNumber::Session6,
        SessionNumber::Session7,
        SessionNumber::Session8,
        SessionNumber::Session9,
        SessionNumber::Session10,
    ]
}

pub fn get_practice_types() -> Vec<PracticeType> {
    vec![
        PracticeType::Characters,
        PracticeType::Words,
        PracticeType::Abbreviations,
        PracticeType::Numbers,
        PracticeType::Callsigns,
        PracticeType::Phrases,
    ]
}

pub fn get_session_description(session: SessionNumber) -> &'static str {
    match session {
        SessionNumber::Session1 => "Session 1: A E N T (4 chars)",
        SessionNumber::Session2 => "Session 2: +S I O 1 4 (9 chars)",
        SessionNumber::Session3 => "Session 3: +D H L R 2 5 (15 chars)",
        SessionNumber::Session4 => "Session 4: +C U (17 chars)",
        SessionNumber::Session5 => "Session 5: +M W 3 6 ? (22 chars)",
        SessionNumber::Session6 => "Session 6: +F Y , (25 chars)",
        SessionNumber::Session7 => "Session 7: +G P Q 7 9 / (31 chars)",
        SessionNumber::Session8 => "Session 8: +B V <AR> (34 chars)",
        SessionNumber::Session9 => "Session 9: +J K 0 8 <BT> (39 chars)",
        SessionNumber::Session10 => "Session 10: +X Z . <BK> <SK> (44 chars)",
    }
}

// Add random block generator
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BlockSize {
    Fixed2,
    Fixed3,
    Fixed4,
    Fixed5,
    Dynamic,  // Random 1-5 characters
}

impl BlockSize {
    pub fn as_str(&self) -> &str {
        match self {
            BlockSize::Fixed2 => "2 characters",
            BlockSize::Fixed3 => "3 characters",
            BlockSize::Fixed4 => "4 characters",
            BlockSize::Fixed5 => "5 characters",
            BlockSize::Dynamic => "Dynamic (1-5)",
        }
    }
    
    pub fn get_size(&self) -> usize {
        match self {
            BlockSize::Fixed2 => 2,
            BlockSize::Fixed3 => 3,
            BlockSize::Fixed4 => 4,
            BlockSize::Fixed5 => 5,
            BlockSize::Dynamic => {
                let mut rng = rand::thread_rng();
                rng.gen_range(1..=5)
            }
        }
    }
}

pub fn get_block_sizes() -> Vec<BlockSize> {
    vec![
        BlockSize::Fixed2,
        BlockSize::Fixed3,
        BlockSize::Fixed4,
        BlockSize::Fixed5,
        BlockSize::Dynamic,
    ]
}

// Get all characters from a session (letters and numbers only)
pub fn get_session_characters(session: SessionNumber) -> Vec<char> {
    let sess = get_session(session);
    let mut chars = Vec::new();
    
    for item in sess.get_practice_items(PracticeType::Characters) {
        for ch in item.chars() {
            if ch.is_alphanumeric() {
                chars.push(ch);
            }
        }
    }
    
    // Remove duplicates
    chars.sort();
    chars.dedup();
    chars
}

// Get all characters from a range of sessions
pub fn get_characters_from_range(from: SessionNumber, to: SessionNumber) -> Vec<char> {
    let mut all_chars = Vec::new();
    let sessions = get_all_sessions();
    
    let from_idx = from.as_number() as usize - 1;
    let to_idx = to.as_number() as usize - 1;
    
    if from_idx > to_idx || to_idx >= sessions.len() {
        return all_chars;
    }
    
    for idx in from_idx..=to_idx {
        if idx < sessions.len() {
            let chars = get_session_characters(sessions[idx]);
            all_chars.extend(chars);
        }
    }
    
    // Remove duplicates
    all_chars.sort();
    all_chars.dedup();
    all_chars
}

// Generate random block of characters
pub fn generate_random_block(
    from_session: SessionNumber,
    to_session: SessionNumber,
    block_size: BlockSize,
) -> String {
    let chars = get_characters_from_range(from_session, to_session);
    
    if chars.is_empty() {
        return String::from("ERROR");
    }
    
    let size = block_size.get_size();
    let mut rng = rand::thread_rng();
    let mut result = String::new();
    
    for _ in 0..size {
        if let Some(&ch) = chars.choose(&mut rng) {
            result.push(ch);
        }
    }
    
    result
}
