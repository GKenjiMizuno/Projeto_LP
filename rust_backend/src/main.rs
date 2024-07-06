use actix_cors::Cors;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Serialize, Deserialize};
use std::{collections::HashMap, env, sync::Mutex, thread, time::Duration};
use rand::{self, rngs::ThreadRng, Rng};
extern crate env_logger;

#[derive(Serialize, Clone)]
struct AutomacaoResidencial {
    luz: bool,
    tranca: bool,
    alarme: bool,
    cortinas: bool,
    robo: bool,
    cafeteira: bool,
    ar_condicionado: bool,
    aquecedor: bool,
}

#[derive(Serialize, Clone)]
struct Clock {
    hour: i32,
}

impl Clock {
    fn new() -> Self {
        let mut rng: ThreadRng = rand::thread_rng();
        let rand_hour: i32 = rng.gen_range(0..24);
        Self {
            hour: rand_hour,
        }
    }

    fn increment_hour(&mut self) {
        if self.hour < 24 {
            self.hour += 1
        } else {
            self.hour = 0
        }
    }
}

#[derive(Serialize, Clone)]
struct Temperatura {
    temp: f64,
}

impl Temperatura {
    fn new() -> Self {
        let temp_inicial: f64 = rand::thread_rng().gen_range(10.0..30.0);
        Self {
            temp: temp_inicial
        }
    }

    fn alterar_temp(&mut self, hora_atual: i32) -> f64 {
        let variacao_max: f64 = 2.0;

        let alteracao: f64 = if (6..18).contains(&hora_atual) {
            rand::thread_rng().gen_range(0.0..variacao_max)
        } else {
            rand::thread_rng().gen_range(-variacao_max..0.0)
        };

        self.temp += alteracao;
        self.temp
    }
}
#[derive(Deserialize)]
struct UpdateData {
    luz: Option<bool>,
    tranca: Option<bool>,
    alarme: Option<bool>,
    cortinas: Option<bool>,
    robo: Option<bool>,
    cafeteira: Option<bool>,
    ar_condicionado: Option<bool>,
    aquecedor: Option<bool>,
}

#[derive(Serialize)]
struct ResponseData {
    message: String,
    devices_status: HashMap<String, bool>,
    hora_atual: i32,
    temp_atual: f64,
}

#[derive(Deserialize)]
struct LoginRequest {
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    message: String,
    authenticated: bool,
    devices_status: AutomacaoResidencial,
    hora_atual: Clock,
    temp_atual: Temperatura,
}

struct AppState {
    automacao_residencial: AutomacaoResidencial,
    correct_password: String,
    clock_atual: Clock,
    temperatura_atual: Temperatura,
}

impl AutomacaoResidencial {
    fn new() -> Self {
        Self {
            luz: true,
            tranca: false,
            alarme: true,
            cortinas: false,
            robo: false,
            cafeteira: false,
            ar_condicionado: false,
            aquecedor: false,
        }
    }

    fn update(&mut self, updates: UpdateData) {
        if let Some(luz) = updates.luz {
            self.luz = luz;
        }
        if let Some(tranca) = updates.tranca {
            self.tranca = tranca;
        }
        if let Some(alarme) = updates.alarme {
            self.alarme = alarme;
        }
        if let Some(cortinas) = updates.cortinas {
            self.cortinas = cortinas;
        }
        if let Some(robo) = updates.robo {
            self.robo = robo;
        }
        if let Some(cafeteira) = updates.cafeteira {
            self.cafeteira = cafeteira;
        }
        if let Some(ar_condicionado) = updates.ar_condicionado {
            self.ar_condicionado = ar_condicionado;
        }
        if let Some(aquecedor) = updates.aquecedor {
            self.aquecedor = aquecedor;
        }
    }

    fn return_data(&self) -> HashMap<String, bool> {
        let mut data = HashMap::new();
        data.insert(String::from("luz"), self.luz);
        data.insert(String::from("tranca"), self.tranca);
        data.insert(String::from("alarme"), self.alarme);
        data.insert(String::from("cortinas"), self.cortinas);
        data.insert(String::from("robo"), self.robo);
        data.insert(String::from("cafeteira"), self.cafeteira);
        data.insert(String::from("ar condicionado"), self.ar_condicionado);
        data.insert(String::from("aquecedor"), self.aquecedor);
        data
    }

    fn acesso_garantido(&mut self) {
        self.luz = true;
        self.tranca = true;
        self.alarme = false;
    }

    fn termostato (&mut self, temperatura: f64) {
        let temperatura_min:f64 = 18.0;
        let temperatura_max:f64 = 25.0;

        if temperatura < temperatura_min {
            self.ar_condicionado = false;
            self.aquecedor = true;
        } else if temperatura > temperatura_max {
            self.ar_condicionado = true;
            self.aquecedor = false;
        }
    }

    fn to_message(&self) -> String {
        format!(
            "Luz: {}, Tranca: {}, Alarme: {}, Cortinas: {}, Robo: {}, Cafeteira: {}, Ar Condicionado: {}, Aquecedor: {}",
            self.luz, self.tranca, self.alarme, self.cortinas, self.robo, self.cafeteira, self.ar_condicionado, self.aquecedor
        )
    }
}

async fn get_data(data: web::Data<Mutex<AppState>>) -> impl Responder {
    let state = data.lock().unwrap();
    let message = state.automacao_residencial.to_message();
    web::Json(ResponseData { 
        message,
        devices_status: state.automacao_residencial.return_data(),
        hora_atual: state.clock_atual.hour,
        temp_atual: state.temperatura_atual.temp,
    })
}

async fn update_data(data: web::Data<Mutex<AppState>>, new_data: web::Json<UpdateData>) -> impl Responder {
    let mut state = data.lock().unwrap();
    state.automacao_residencial.update(new_data.into_inner());
    web::Json(state.automacao_residencial.return_data())
}

async fn login(data: web::Json<LoginRequest>, state: web::Data<Mutex<AppState>>) -> impl Responder {
    let mut state = state.lock().unwrap();
    if data.password == state.correct_password {
        state.automacao_residencial.acesso_garantido();
        HttpResponse::Ok().json(LoginResponse {
            message: String::from("Login successful"),
            authenticated: true,
            devices_status: state.automacao_residencial.clone(),
            hora_atual: state.clock_atual.clone(),
            temp_atual: state.temperatura_atual.clone(),
        })
    } else {
        HttpResponse::Unauthorized().json(LoginResponse {
            message: String::from("Invalid password"),
            authenticated: false,
            devices_status: AutomacaoResidencial::new(),
            hora_atual: Clock::new(),
            temp_atual: Temperatura::new(),
        })
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let state = web::Data::new(Mutex::new(AppState {
        automacao_residencial: AutomacaoResidencial::new(),
        correct_password: String::from("1234"),
        clock_atual: Clock::new(),
        temperatura_atual: Temperatura::new(),
    }));
    
    // Spawn a background thread to increment the clock every 5 seconds
    let state_clone = state.clone();
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(1));
            let mut state = state_clone.lock().unwrap();
            state.clock_atual.increment_hour();
            let hora_atual = state.clock_atual.hour;
            state.temperatura_atual.alterar_temp(hora_atual);
            let ultima_temp = state.temperatura_atual.temp;
            state.automacao_residencial.termostato(ultima_temp);
            println!("Hora: {}", hora_atual);
            println!("Temperatura: {}", ultima_temp);
            let data = state.automacao_residencial.return_data();
            for (key, value) in &data {
                println!("{}: {}", key, value);
            }
        }
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(Cors::permissive())
            .route("/api/data", web::get().to(get_data))
            .route("/api/update", web::put().to(update_data))
            .route("/api/login", web::post().to(login))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
