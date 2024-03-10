use actix_web::{HttpResponse};

pub async fn catalogo() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .body(r#"[
            "resumo": {"qtd":3, "Livraria Porco Fiel"},
            "livros":[
            {"id": "101", "Doce de amendoin"},
            {"id": "102", "Porco entre o rabo"},
            {"id": "103", "Vaca prenha"},
            ]"#)
}



