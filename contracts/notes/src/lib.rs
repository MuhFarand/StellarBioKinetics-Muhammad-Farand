#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// 1. Struktur data untuk menyimpan profil model pertumbuhan bakteri
#[contracttype]
#[derive(Clone, Debug)]
pub struct BacteriaModel {
    pub name: String,
    pub growth_rate: u64, // Nilai dikalikan 100 untuk menghindari float (misal 1.50 ditulis 150)
    pub lag_phase_duration: u64, // dalam satuan jam/menit
    pub max_population: u64, // Carrying capacity (K)
}

// 2. Struktur data untuk hasil perbandingan akhir
#[contracttype]
#[derive(Clone, Debug)]
pub struct ComparisonResult {
    pub faster_grower: String,
    pub higher_yield: String,
    pub efficiency_score_diff: i64,
}

const BACTERIA_DATA: Symbol = symbol_short!("BAC_DATA");

#[contract]
pub struct BacteriaComparisonContract;

#[contractimpl]
impl BacteriaComparisonContract {

    // Menyimpan atau menambahkan model bakteri baru ke dalam storage
    pub fn add_model(env: Env, name: String, growth_rate: u64, lag_phase_duration: u64, max_population: u64) -> String {
        let mut models: Vec<BacteriaModel> = env.storage().instance().get(&BACTERIA_DATA).unwrap_or(Vec::new(&env));
        
        let new_model = BacteriaModel {
            name,
            growth_rate,
            lag_phase_duration,
            max_population,
        };
        
        models.push_back(new_model);
        env.storage().instance().set(&BACTERIA_DATA, &models);
        
        String::from_str(&env, "Model bakteri berhasil disimpan.")
    }

    // Mengambil semua list model bakteri yang terdaftar
    pub fn get_all_models(env: Env) -> Vec<BacteriaModel> {
        env.storage().instance().get(&BACTERIA_DATA).unwrap_or(Vec::new(&env))
    }

    // Fungsi inti: Membandingkan dua model bakteri berdasarkan indeksnya di dalam list storage
    pub fn compare_models(env: Env, index_a: u32, index_b: u32) -> ComparisonResult {
        let models: Vec<BacteriaModel> = env.storage().instance().get(&BACTERIA_DATA).unwrap_or(Vec::new(&env));
        
        // Mengambil entitas model bakteri berdasarkan indeks input
        let model_a = models.get(index_a).unwrap();
        let model_b = models.get(index_b).unwrap();

        // Logika penentuan pertumbuhan tercepat (Laju pertumbuhan tinggi & fase lag singkat)
        let mut faster_grower = model_a.name.clone();
        if model_b.growth_rate > model_a.growth_rate && model_b.lag_phase_duration <= model_a.lag_phase_duration {
            faster_grower = model_b.name.clone();
        }

        // Logika penentuan biomassa maksimum tertinggi
        let mut higher_yield = model_a.name.clone();
        if model_b.max_population > model_a.max_population {
            higher_yield = model_b.name.clone();
        }

        // Kalkulasi skor efisiensi sederhana menggunakan matematika integer standar
        // Skor = (Laju Pertumbuhan * Populasi Maksimum) / (Fase Lag + 1)
        let score_a: i64 = ((model_a.growth_rate * model_a.max_population) / (model_a.lag_phase_duration + 1)) as i64;
        let score_b: i64 = ((model_b.growth_rate * model_b.max_population) / (model_b.lag_phase_duration + 1)) as i64;
        let efficiency_score_diff = score_a - score_b;

        ComparisonResult {
            faster_grower,
            higher_yield,
            efficiency_score_diff,
        }
    }
}