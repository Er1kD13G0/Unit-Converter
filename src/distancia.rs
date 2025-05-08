pub fn milimetro_para_centimetro(mm: f64) -> f64 {
    mm / 10.0
}

pub fn milimetro_para_metro(mm: f64) -> f64 {
    mm / 1000.0
}

pub fn milimetro_para_quilometro(mm: f64) -> f64 {
    mm / 1_000_000.0
}

pub fn centimetro_para_milimetro(cm: f64) -> f64 {
    cm * 10.0
}

pub fn centimetro_para_metro(cm: f64) -> f64 {
    cm / 100.0
}

pub fn centimetro_para_quilometro(cm: f64) -> f64 {
    cm / 100_000.0
}

pub fn metro_para_milimetro(m: f64) -> f64 {
    m * 1000.0
}

pub fn metro_para_centimetro(m: f64) -> f64 {
    m * 100.0
}

pub fn metro_para_quilometro(m: f64) -> f64 {
    m / 1000.0
}

pub fn metro_para_polegada(m: f64) -> f64 {
    m * 39.3701
}

pub fn metro_para_pe(m: f64) -> f64 {
    m * 3.28084
}

pub fn metro_para_jarda(m: f64) -> f64 {
    m * 1.09361
}

pub fn quilometro_para_metro(km: f64) -> f64 {
    km * 1000.0
}

pub fn quilometro_para_milha(km: f64) -> f64 {
    km / 1.60934
}

pub fn polegada_para_metro(inch: f64) -> f64 {
    inch / 39.3701
}

pub fn pe_para_metro(ft: f64) -> f64 {
    ft / 3.28084
}

pub fn jarda_para_metro(yd: f64) -> f64 {
    yd / 1.09361
}

pub fn milha_para_quilometro(mi: f64) -> f64 {
    mi * 1.60934
}
