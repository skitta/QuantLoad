// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(serde::Deserialize)]
struct Primer {
    name: String,
    concentration: f64,
}

#[derive(serde::Deserialize)]
struct Recipe {
    mix: f64,
    primers: f64,
    cDNA: f64,
    water: f64,
}

#[derive(serde::Deserialize)]
struct Samples {
    targets: Vec<String>,
    repeat: usize,
    groups: Vec<String>,
}

#[derive(serde::Deserialize)]
struct PrimersConfig {
    forward: Primer,
    reverse: Primer,
}

#[derive(serde::Deserialize)]
struct QPCRConfig {
    samples: Samples,
    recipe: Recipe,
    primers: PrimersConfig,
}

#[derive(serde::Serialize)]
struct WorkingSolution {
    mix: f64,
    forward_primer: f64,
    reverse_primer: f64,
    water: f64,
    total_volume: f64,
}

#[derive(serde::Serialize)]
struct MasterMix {
    mix: f64,
    forward_primer: f64,
    reverse_primer: f64,
    water: f64,
    total_volume: f64,
}

#[derive(serde::Serialize)]
struct CalculationResult {
    total_reactions: usize,
    working_solutions: std::collections::HashMap<String, WorkingSolution>,
    master_mix: MasterMix,
    total_cdna_volume: f64,
}

#[tauri::command]
fn calculate_qpcr_volumes(config: QPCRConfig) -> CalculationResult {
    let reactions_per_target = config.samples.groups.len() * config.samples.repeat;
    let total_reactions = reactions_per_target * config.samples.targets.len();
    
    let mut working_solutions = std::collections::HashMap::new();
    
    for target in &config.samples.targets {
        let mix_volume = config.recipe.mix * reactions_per_target as f64;
        let forward_primer_volume = config.recipe.primers * reactions_per_target as f64;
        let reverse_primer_volume = config.recipe.primers * reactions_per_target as f64;
        let water_volume = config.recipe.water * reactions_per_target as f64;
        let total_volume = mix_volume + forward_primer_volume + reverse_primer_volume + water_volume;
        
        working_solutions.insert(
            target.clone(),
            WorkingSolution {
                mix: mix_volume,
                forward_primer: forward_primer_volume,
                reverse_primer: reverse_primer_volume,
                water: water_volume,
                total_volume,
            },
        );
    }
    
    let master_mix = MasterMix {
        mix: config.recipe.mix * total_reactions as f64,
        forward_primer: config.recipe.primers * total_reactions as f64,
        reverse_primer: config.recipe.primers * total_reactions as f64,
        water: config.recipe.water * total_reactions as f64,
        total_volume: (config.recipe.mix + config.recipe.primers * 2.0 + config.recipe.water) * total_reactions as f64,
    };
    
    let total_cdna_volume = config.recipe.cDNA * total_reactions as f64;
    
    CalculationResult {
        total_reactions,
        working_solutions,
        master_mix,
        total_cdna_volume,
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, calculate_qpcr_volumes])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
