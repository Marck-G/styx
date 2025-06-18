use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct TelemetryData {
    pub count: i64,
    pub last_clean: u64, // timestamp UNIX en segundos
}

#[derive(Debug, Clone)]
pub struct Telemetry {
    data: Arc<Mutex<HashMap<i64, TelemetryData>>>,
}

impl Telemetry {
    pub fn new() -> Self {
        Telemetry {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Incrementa contador para route_id dado
    pub fn increment(&self, route_id: i64) {
        let mut map = self.data.lock().unwrap();

        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        map.entry(route_id)
            .and_modify(|entry| entry.count = entry.count.saturating_add(1))
            .or_insert(TelemetryData { count: 1, last_clean: now });
    }

    /// Limpia todos los contadores y actualiza last_clean con timestamp actual
    pub fn clear(&self) {
        let mut map = self.data.lock().unwrap();
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        for (_route_id, telemetry) in map.iter_mut() {
            telemetry.count = 0;
            telemetry.last_clean = now;
        }
    }

    /// Obtener snapshot actual de la telemetría
    pub fn snapshot(&self) -> HashMap<i64, TelemetryData> {
        let map = self.data.lock().unwrap();
        map.clone()
    }
}