use actix_cors::Cors;
use actix_web::{web, App, HttpServer, Responder};
use serde::{Serialize, Deserialize};
use std::{sync::Mutex, collections::HashMap};

#[derive(Serialize)]
struct AutomacaoResidencial {
    luz: bool,
    tranca: bool,
    alarme: bool,
    cortinas: bool,
    robo: bool,
    cafeteira: bool,
    ar_condicionado: bool,
    aquecedor: bool
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

impl AutomacaoResidencial {
    fn new() -> Self {
        Self {
            luz: false,
            tranca: false,
            alarme: true,
            cortinas: false,
            robo: false,
            cafeteira: false,
            ar_condicionado: false,
            aquecedor: false
        }
    }

    fn teclado(&mut self, senha: u32) {
        let senha_correta = 1234;

        if senha == senha_correta {
            self.luz = true;
            self.tranca = true;
            self.alarme = false;
            println!("Senha correta!");
        } else {
            println!("Senha incorreta! Tente novamente.");
        }
    }

    fn termostato (&mut self, temperatura: f32) {
        let temperatura_min:f32 = 15.0;
        let temperatura_max:f32 = 25.0;

        if temperatura < temperatura_min {
            self.ar_condicionado = false;
            self.aquecedor = true;
        } else if temperatura > temperatura_max {
            self.ar_condicionado = true;
            self.aquecedor = false;
        }
    }

    fn return_data(&self) -> HashMap<String, bool>{
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

}

async fn get_data(data: web::Data<Mutex<AutomacaoResidencial>>) -> impl Responder {
    let sistema = data.lock().unwrap();
    let message = sistema.to_message();
    web::Json(ResponseData { message })
}

async fn update_data(data: web::Data<Mutex<AutomacaoResidencial>>, new_data: web::Json<UpdateData>) -> impl Responder {
    let mut sistema = data.lock().unwrap();
    sistema.update(new_data.into_inner());
    web::Json(sistema.return_data())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let automacao_residencial = web::Data::new(Mutex::new(AutomacaoResidencial::new()));
    HttpServer::new(move || {
        App::new()
            .app_data(automacao_residencial.clone()) // Compartilha o estado
            .wrap(Cors::permissive())  // Adiciona middleware CORS
            .route("/api/data", web::get().to(get_data))
            .route("/api/update", web::put().to(update_data))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
