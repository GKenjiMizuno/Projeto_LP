use actix_cors::Cors;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Serialize, Deserialize};
use std::sync::Mutex;
use std::collections::HashMap;

#[derive(Serialize, Clone)]  // Adicione Clone aqui
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
}

struct AppState {
    automacao_residencial: AutomacaoResidencial,
    correct_password: String,
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
    web::Json(ResponseData { message })
}

async fn update_data(data: web::Data<Mutex<AppState>>, new_data: web::Json<UpdateData>) -> impl Responder {
    let mut state = data.lock().unwrap();
    state.automacao_residencial.update(new_data.into_inner());
    web::Json(state.automacao_residencial.return_data())
}

async fn login(data: web::Json<LoginRequest>, state: web::Data<Mutex<AppState>>) -> impl Responder {
    let state = state.lock().unwrap();
    if data.password == state.correct_password {
        HttpResponse::Ok().json(LoginResponse {
            message: String::from("Login successful"),
            authenticated: true,
            devices_status: state.automacao_residencial.clone(), // Agora você pode usar clone
        })
    } else {
        HttpResponse::Unauthorized().json(LoginResponse {
            message: String::from("Invalid password"),
            authenticated: false,
            devices_status: AutomacaoResidencial::new(),
        })
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(Mutex::new(AppState {
        automacao_residencial: AutomacaoResidencial::new(),
        correct_password: String::from("1234"),
    }));

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
