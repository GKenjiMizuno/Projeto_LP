// Importação de bibliotecas necessárias para o projeto.
use actix_cors::Cors;  // Biblioteca para manipulação de Cross-Origin Resource Sharing (CORS) no Actix.
use actix_web::{web, App, HttpResponse, HttpServer, Responder};  // Importa componentes principais do Actix-Web para construir o servidor HTTP.
use serde::{Serialize, Deserialize};  // Bibliotecas para serialização e deserialização de dados (útil para JSON).
use std::{collections::HashMap, env, sync::Mutex, thread, time::Duration};  // Bibliotecas padrão do Rust para manipulação de variáveis de ambiente, sincronização, threads e tempo.
use rand::{self, Rng};  // Biblioteca para geração de números aleatórios.
use serde_json::json;  // Importação da macro json para facilitar a criação de objetos JSON.
extern crate env_logger;  // Biblioteca para configuração de logs baseada em variáveis de ambiente.

// Definição de uma estrutura de dados para representar o estado da automação residencial.
#[derive(Serialize, Clone)]
struct AutomacaoResidencial {
    luz: bool,  // Estado da luz (ligada ou desligada).
    tranca: bool,  // Estado da tranca (trancada ou destrancada).
    alarme: bool,  // Estado do alarme (ativado ou desativado).
    cortinas: bool,  // Estado das cortinas (abertas ou fechadas).
    robo: bool,  // Estado do robô (ativo ou inativo).
    cafeteira: bool,  // Estado da cafeteira (ligada ou desligada).
    ar_condicionado: bool,  // Estado do ar-condicionado (ligado ou desligado).
    aquecedor: bool,  // Estado do aquecedor (ligado ou desligado).
}

// Definição de uma estrutura de dados para representar um relógio simples.
#[derive(Serialize, Clone)]
struct Clock {
    hour: i32,  // Hora atual (formato 24 horas).
}

// Implementação de métodos para a estrutura Clock.
impl Clock {
    // Método construtor para criar uma nova instância de Clock com a hora inicializada em 12.
    fn new() -> Self {
        Self {
            hour: 12,
        }
    }

    // Método para incrementar a hora no relógio.
    fn increment_hour(&mut self) {
        if self.hour < 24 {
            self.hour += 1  // Incrementa a hora enquanto for menor que 24.
        } else {
            self.hour = 0  // Reinicia a hora para 0 após alcançar 24.
        }
    }
}

// Deriva traços para permitir a serialização da estrutura (para enviar dados em formatos como JSON)
// e a clonagem de suas instâncias.
#[derive(Serialize, Clone)]
struct Temperatura {
    temp: f64,  // Armazena o valor da temperatura como um número de ponto flutuante.
}

// Implementação da estrutura Temperatura.
impl Temperatura {
    // Método construtor que inicializa uma nova Temperatura com um valor aleatório entre 10.0 e 30.0 graus.
    fn new() -> Self {
        // Gera um valor inicial de temperatura aleatório entre 10.0 e 30.0 graus.
        let temp_inicial: f64 = rand::thread_rng().gen_range(10.0..30.0);
        Self {
            temp: temp_inicial
        }
    }

    // Método para alterar a temperatura com base na hora atual.
    fn alterar_temp(&mut self, hora_atual: i32) -> f64 {
        // Define a variação máxima de temperatura como 2.0 graus.
        let variacao_max: f64 = 2.0;

        // Calcula a alteração da temperatura. Durante o dia (6h às 18h), a temperatura pode aumentar
        // até 2.0 graus; durante a noite, pode diminuir até 2.0 graus.
        let alteracao: f64 = if (6..18).contains(&hora_atual) {
            rand::thread_rng().gen_range(0.0..variacao_max)  // Aumenta a temperatura durante o dia.
        } else {
            rand::thread_rng().gen_range(-variacao_max..0.0)  // Diminui a temperatura durante a noite.
        };

        // Atualiza a temperatura com a alteração calculada.
        self.temp += alteracao;
        // Retorna o novo valor da temperatura.
        self.temp
    }
}
// Define uma estrutura para deserializar dados recebidos em requisições de atualização.
#[derive(Deserialize)]
struct UpdateData {
    luz: Option<bool>,  // Opcional: estado da luz (true para ligada, false para desligada).
    tranca: Option<bool>,  // Opcional: estado da tranca.
    alarme: Option<bool>,  // Opcional: estado do alarme.
    cortinas: Option<bool>,  // Opcional: estado das cortinas.
    robo: Option<bool>,  // Opcional: estado do robô.
    cafeteira: Option<bool>,  // Opcional: estado da cafeteira.
    ar_condicionado: Option<bool>,  // Opcional: estado do ar-condicionado.
    aquecedor: Option<bool>,  // Opcional: estado do aquecedor.
}

// Define uma estrutura para serializar a resposta a ser enviada ao cliente.
#[derive(Serialize)]
struct ResponseData {
    message: String,  // Mensagem descrevendo o resultado da operação ou status.
    devices_status: HashMap<String, bool>,  // Estado atual dos dispositivos em forma de mapa.
    hora_atual: i32,  // Hora atual no relógio do sistema.
    temp_atual: f64,  // Temperatura atual no sistema.
    authenticated: bool,  // Indica se o usuário está autenticado ou não.
}

// Define uma estrutura para deserializar dados de solicitação de login.
#[derive(Deserialize)]
struct LoginRequest {
    password: String,  // Senha fornecida pelo usuário para tentativa de login.
}

// Define uma estrutura para deserializar dados de solicitação de logout.
#[derive(Deserialize)]
struct LogoutRequest {
    authenticated: bool,  // Indica se o usuário está autenticado ou não no momento do logout.
}

// Define uma estrutura para serializar a resposta de login a ser enviada ao cliente.
#[derive(Serialize)]
struct LoginResponse {
    message: String,  // Mensagem descrevendo o resultado do login.
    authenticated: bool,  // Indica se o login foi bem-sucedido ou não.
    devices_status: AutomacaoResidencial,  // Estado atual dos dispositivos na automação residencial.
    hora_atual: Clock,  // Estado atual do relógio.
    temp_atual: Temperatura,  // Estado atual da temperatura.
}
// Define uma estrutura chamada AppState que contém o estado geral da aplicação.
struct AppState {
    // Campo para armazenar o estado atual dos dispositivos de automação residencial.
    automacao_residencial: AutomacaoResidencial,
    // Campo para armazenar a senha correta necessária para autenticar um usuário.
    correct_password: String,
    // Campo que armazena o estado atual do relógio.
    clock_atual: Clock,
    // Campo que armazena a temperatura atual monitorada pelo sistema.
    temperatura_atual: Temperatura,
    // Campo booleano que indica se um usuário está autenticado ou não.
    authenticated: bool,
}
// Implementação de métodos para a estrutura AutomacaoResidencial.
impl AutomacaoResidencial {
    // Método construtor que inicializa os dispositivos com estados padrão.
    fn new() -> Self {
        Self {
            luz: false,  // Luz inicialmente desligada.
            tranca: true,  // Tranca inicialmente fechada.
            alarme: true,  // Alarme inicialmente ligado.
            cortinas: true,  // Cortinas inicialmente abertas.
            robo: false,  // Robô inicialmente desligado.
            cafeteira: false,  // Cafeteira inicialmente desligada.
            ar_condicionado: false,  // Ar condicionado inicialmente desligado.
            aquecedor: false,  // Aquecedor inicialmente desligado.
        }
    }

    // Método para atualizar o estado dos dispositivos com base em dados recebidos.
    fn update(&mut self, updates: UpdateData) {
        // Atualiza cada dispositivo se um novo valor foi fornecido.
        updates.luz.map(|luz| self.luz = luz);
        updates.tranca.map(|tranca| self.tranca = tranca);
        updates.alarme.map(|alarme| self.alarme = alarme);
        updates.cortinas.map(|cortinas| self.cortinas = cortinas);
        updates.robo.map(|robo| self.robo = robo);
        updates.cafeteira.map(|cafeteira| self.cafeteira = cafeteira);
        updates.ar_condicionado.map(|ar| self.ar_condicionado = ar);
        updates.aquecedor.map(|aquecedor| self.aquecedor = aquecedor);
    }

    // Método para retornar o estado atual dos dispositivos em formato de mapa.
    fn return_data(&self) -> HashMap<String, bool> {
        let mut data = HashMap::new();
        // Insere o estado de cada dispositivo no mapa.
        data.insert("luz".to_string(), self.luz);
        data.insert("tranca".to_string(), self.tranca);
        data.insert("alarme".to_string(), self.alarme);
        data.insert("cortinas".to_string(), self.cortinas);
        data.insert("robo".to_string(), self.robo);
        data.insert("cafeteira".to_string(), self.cafeteira);
        data.insert("ar condicionado".to_string(), self.ar_condicionado);
        data.insert("aquecedor".to_string(), self.aquecedor);
        data
    }

    // Método que ajusta os dispositivos para um estado específico, presumivelmente quando um usuário autenticado acessa a casa.
    fn acesso_garantido(&mut self) {
        self.luz = true;        // Liga a luz, indicando atividade ou presença na casa.
        self.tranca = false;    // Destrava a tranca, permitindo entrada sem obstáculos.
        self.alarme = false;    // Desativa o alarme para evitar disparos acidentais durante a entrada.
    }

    // Método para configurar todos os dispositivos para um estado seguro quando o usuário sai de casa.
    fn fora_de_casa(&mut self) {
        self.luz = false;
        self.tranca = true;
        self.alarme = true;
        self.cortinas = true;
        self.robo = false;
        self.cafeteira = false;
        self.ar_condicionado = false;
        self.aquecedor = false;
    }

    // Método para ajustar os dispositivos com base na temperatura atual.
    fn termostato(&mut self, temperatura: f64) {
        let temperatura_min: f64 = 18.0;
        let temperatura_max: f64 = 25.0;

        // Ajusta os dispositivos de aquecimento e refrigeração com base na temperatura.
        if temperatura < temperatura_min {
            self.ar_condicionado = false;
            self.aquecedor = true;
        } else if temperatura > temperatura_min && temperatura < temperatura_max {
            self.ar_condicionado = false;
            self.aquecedor = false;
        } else {
            self.ar_condicionado = true;
            self.aquecedor = false;
        }
    }

    // Método para configurar os dispositivos com base na hora do dia, especificamente para dormir ou acordar.
    fn dormindo_ou_acordado(&mut self, hora_atual: i32) {
        if hora_atual >= 22 ||   hora_atual < 6 {
            self.luz = false;
            self.cortinas = true;
        } else if hora_atual == 6 {
            self.luz = true;
            self.cortinas = false;
            self.cafeteira = true;
        } else {
            self.cafeteira = false;
        }
    }

    // Método para converter o estado atual dos dispositivos em uma mensagem formatada como string.
    fn to_message(&self) -> String {
        format!(
            "Luz: {}, Tranca: {}, Alarme: {}, Cortinas: {}, Robo: {}, Cafeteira: {}, Ar Condicionado: {}, Aquecedor: {}",
            self.luz, self.tranca, self.alarme, self.cortinas, self.robo, self.cafeteira, self.ar_condicionado, self.aquecedor
        )
    }
}

// Função assíncrona que responde com os dados atuais do estado da automação residencial.
async fn get_data(data: web::Data<Mutex<AppState>>) -> impl Responder {
    // Bloqueia o estado compartilhado da aplicação para leitura.
    let state = data.lock().unwrap();
    // Gera uma mensagem descrevendo o estado atual dos dispositivos.
    let message = state.automacao_residencial.to_message();
    // Retorna os dados como JSON, incluindo a mensagem, o estado atual dos dispositivos,
    // a hora atual, a temperatura atual, e se o usuário está autenticado.
    web::Json(ResponseData { 
        message,
        devices_status: state.automacao_residencial.return_data(),
        hora_atual: state.clock_atual.hour,
        temp_atual: state.temperatura_atual.temp,
        authenticated: state.authenticated,
    })
}

// Função assíncrona para atualizar os dados dos dispositivos na automação residencial.
async fn update_data(state: web::Data<Mutex<AppState>>, new_data: web::Json<UpdateData>) -> impl Responder {
    // Bloqueia o estado para modificação.
    let mut state = state.lock().unwrap();
    // Atualiza o estado dos dispositivos com os novos dados recebidos.
    state.automacao_residencial.update(new_data.into_inner());
    // Retorna o estado atualizado dos dispositivos como JSON.
    web::Json(state.automacao_residencial.return_data())
}

// Função assíncrona para processar uma solicitação de login.
async fn login(data: web::Json<LoginRequest>, state: web::Data<Mutex<AppState>>) -> impl Responder {
    // Bloqueia o estado para modificação segura.
    let mut state = state.lock().unwrap();
    // Verifica se a senha fornecida na solicitação é igual à senha correta armazenada.
    if data.password == state.correct_password {
        // Chama a função para ajustar os dispositivos para um estado de "acesso garantido".
        state.automacao_residencial.acesso_garantido();
        // Marca o usuário como autenticado.
        state.authenticated = true;
        // Retorna uma resposta HTTP positiva com os dados relevantes.
        HttpResponse::Ok().json(ResponseData { 
            message: String::from("Login successful"),
            devices_status: state.automacao_residencial.return_data(),
            hora_atual: state.clock_atual.hour,
            temp_atual: state.temperatura_atual.temp,
            authenticated: state.authenticated,
        })
    } else {
        // Retorna uma resposta HTTP de não autorizado se a senha for incorreta.
        HttpResponse::Unauthorized().json(LoginResponse {
            message: String::from("Invalid password"),
            authenticated: false,
            devices_status: AutomacaoResidencial::new(),
            hora_atual: Clock::new(),
            temp_atual: Temperatura::new(),
        })
    }
}

// Função assíncrona para processar uma solicitação de logout.
async fn logout(request: web::Json<LogoutRequest>, state: web::Data<Mutex<AppState>>) -> impl Responder {
    // Bloqueia o estado para modificação segura.
    let mut state = state.lock().unwrap();

    // Verifica se o usuário está atualmente autenticado antes de proceder.
    if request.authenticated {
        // Chama a função para ajustar os dispositivos para um estado seguro quando ninguém está em casa.
        state.automacao_residencial.fora_de_casa();
        // Marca o usuário como não autenticado.
        state.authenticated = false;
        // Retorna uma resposta HTTP positiva indicando sucesso no logout.
        HttpResponse::Ok().json(json!({"message": "Logout successful and home secured."}))
    } else {
        // Retorna uma resposta HTTP de não autorizado se o usuário não estava autenticado.
        HttpResponse::Unauthorized().json(json!({"message": "Logout failed: user not authenticated."}))
    }
}

// Anotação para indicar que a função `main` deve ser executada em um ambiente assíncrono usando `actix_web`.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Define a variável de ambiente para controlar o nível de log para depuração.
    env::set_var("RUST_LOG", "debug");
    // Inicializa o sistema de log com base nas variáveis de ambiente.
    env_logger::init();

    // Criação e inicialização do estado compartilhado da aplicação usando Mutex para acesso seguro entre threads.
    let state = web::Data::new(Mutex::new(AppState {
        automacao_residencial: AutomacaoResidencial::new(),  // Inicializa a configuração dos dispositivos residenciais.
        correct_password: String::from("1234"),  // Define a senha correta para autenticação.
        clock_atual: Clock::new(),  // Inicializa o relógio.
        temperatura_atual: Temperatura::new(),  // Inicializa a temperatura.
        authenticated: false  // Estado inicial de autenticação é definido como falso.
    }));
    
    // Cria um clone do estado para uso em uma thread separada.
    let state_clone = state.clone();
    // Inicia uma nova thread para atualizar o relógio e a temperatura em intervalos regulares.
    thread::spawn(move || {
        loop {
            // Pausa a thread por 5 segundos.
            thread::sleep(Duration::from_secs(5));
            // Bloqueia o estado para atualização, garantindo que não haja conflitos de acesso.
            let mut state = state_clone.lock().unwrap();
            // Incrementa a hora no relógio.
            state.clock_atual.increment_hour();
            let hora_atual = state.clock_atual.hour;
            // Atualiza a temperatura com base na hora atual.
            state.temperatura_atual.alterar_temp(hora_atual);
            let ultima_temp = state.temperatura_atual.temp;
            // Se autenticado, ajusta o termostato e altera o estado baseado se está dormindo ou acordado.
            if state.authenticated {
                state.automacao_residencial.termostato(ultima_temp);
                state.automacao_residencial.dormindo_ou_acordado(hora_atual);
            }
        }
    });

    // Configuração do servidor HTTP.
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())  // Passa o estado da aplicação para o contexto do Actix.
            .wrap(Cors::permissive())  // Configura CORS de forma permissiva.
            .route("/api/data", web::get().to(get_data))  // Rota para obter dados dos dispositivos.
            .route("/api/update", web::put().to(update_data))  // Rota para atualizar os estados dos dispositivos.
            .route("/api/login", web::post().to(login))  // Rota para login.
            .route("/api/logout", web::post().to(logout))  // Rota para logout.
    })
    .bind("127.0.0.1:8080")?  // Define o endereço e porta onde o servidor deve escutar.
    .run()  // Inicia o servidor para escutar por requisições.
    .await  // Aguarda o servidor terminar de rodar.
}
