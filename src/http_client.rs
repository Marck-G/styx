use reqwest::{Client, Method, Response};
use std::time::Duration;
use std::collections::HashMap;
use tracing::{info, error};

pub struct HttpClient {
    client: Client,
    timeout_secs: u64,
}

impl HttpClient {
    /// Crea un nuevo cliente con timeout configurable
    pub fn new(timeout_secs: u64) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .build()
            .expect("Error construyendo el cliente HTTP");

        Self { client, timeout_secs }
    }

    /// Realiza una llamada HTTP genérica
    pub async fn request(
        &self,
        method: Method,
        url: &str,
        headers: Option<HashMap<String, String>>,
        // query_params: Option<HashMap<String, String>>,
        body: Option<String>,
    ) -> Result<Response, reqwest::Error> {
        let mut req = self.client.request(method.clone(), url);
        if let Some(hdrs) = headers {
            for (key, value) in hdrs {
                req = req.header(&key, &value);
            }
        }

        // Añadir parámetros de query
        // if let Some(q_params) = query_params {
        //     req = req.query(&q_params); // Usa el método .query() de reqwest
        // }

        if let Some(b) = body {
            req = req.body(b);
        }

        info!("Llamando a {} [{}]", url, method);

        let resp = req.send().await;

        match &resp {
            Ok(response) => info!("Respuesta {} de {}", response.status(), url),
            Err(e) => error!("Error al llamar {}: {:?}", url, e),
        }

        resp
    }
}
