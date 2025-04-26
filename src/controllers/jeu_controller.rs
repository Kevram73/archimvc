// src/controllers/jeu_controller.rs

use actix_web::{post, web, HttpResponse, Responder};
use diesel::{prelude::*, QueryDsl, RunQueryDsl};
use diesel::dsl::exists;
use serde::{Deserialize, Serialize};

use crate::db::DbPool;
use crate::models::{Jeu, NewJeu, UpdateJeu};
use crate::schema::jeux;
mod status {
    pub const NOM_VIDE:     i32 = 1;
    pub const NOM_DOUBLON:  i32 = 2;
    pub const ID_VIDE:      i32 = 3;
    pub const ID_NOT_FOUND: i32 = 4;
}

#[derive(Serialize)]
struct ApiResponse<T> {
    status: Vec<i32>,
    get:    T,
}

#[derive(Debug, Deserialize)]
struct FilterRequest {
    id:       Option<i32>,
    pays_id:  Option<i32>,
    actif:    Option<bool>,
}

#[derive(Debug, Deserialize)]
struct UpdateRequest {
    id:      i32,
    updates: UpdateJeu,
}

#[derive(Debug, Deserialize)]
struct ToggleActiveRequest {
    id:    i32,
    actif: bool,
}

/// Vérifie que `raw_nom` n’est ni vide ni déjà utilisé (hors `exclude_id`)
fn validate_nom(
    conn: &mut SqliteConnection,
    raw_nom: &str,
    exclude_id: Option<i32>,
) -> Vec<i32> {
    let mut codes = Vec::new();
    let trimmed = raw_nom.trim();

    if trimmed.is_empty() {
        codes.push(status::NOM_VIDE);
    } else {
        // on sélectionne l'id de tout jeu auquel ce nom appartient
        let mut q = jeux::table
            .select(jeux::id)
            .filter(jeux::nom.eq(trimmed))
            .into_boxed();
        if let Some(eid) = exclude_id {
            q = q.filter(jeux::id.ne(eid));
        }
        if diesel::select(exists(q))
            .get_result::<bool>(conn)
            .unwrap_or(false)
        {
            codes.push(status::NOM_DOUBLON);
        }
    }
    codes
}

/// POST /jeux/liste
#[post("/jeux/liste")]
pub async fn list_jeux(
    pool:    web::Data<DbPool>,
    filters: web::Json<FilterRequest>,
) -> impl Responder {
    let mut conn = pool.get().expect("DB connection failed");
    let mut q = jeux::table.into_boxed();

    // Validation de l'ID si fourni
    if let Some(fid) = filters.id {
        if fid <= 0 {
            return HttpResponse::Ok().json(ApiResponse {
                status: vec![status::ID_VIDE],
                get: Vec::<Jeu>::new(),
            });
        }
        q = q.filter(jeux::id.eq(fid));
    }
    
    if let Some(fp) = filters.pays_id {
        if fp <= 0 {
            return HttpResponse::Ok().json(ApiResponse {
                status: vec![status::ID_VIDE],
                get: Vec::<Jeu>::new(),
            });
        }
        q = q.filter(jeux::pays_id.eq(fp));
    }
    
    if let Some(fa) = filters.actif {
        q = q.filter(jeux::actif.eq(fa));
    }

    match q.select(Jeu::as_select()).load::<Jeu>(&mut conn) {
        Ok(list) => HttpResponse::Ok().json(ApiResponse {
            status: vec![],
            get: list,
        }),
        Err(e) => {
            eprintln!("DB error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// POST /jeux/creer
#[post("/jeux/creer")]
pub async fn create_jeu(
    pool:    web::Data<DbPool>,
    payload: web::Json<NewJeu>,
) -> impl Responder {
    let mut conn = pool.get().expect("DB connection failed");
    let data = payload.into_inner();

    // 1) Validation du nom
    let codes = validate_nom(&mut conn, &data.nom, None);
    if !codes.is_empty() {
        return HttpResponse::Ok().json(ApiResponse {
            status: codes,
            get:    serde_json::json!({ "id": 0 }),
        });
    }

    // 2) Insertion
    if let Err(e) = diesel::insert_into(jeux::table)
        .values(&data)
        .execute(&mut conn)
    {
        eprintln!("Insert error: {}", e);
        return HttpResponse::InternalServerError().finish();
    }

    // 3) Récupération du jeu créé
    let created = jeux::table
        .order(jeux::id.desc())
        .first::<Jeu>(&mut conn)
        .expect("Fetch after insert failed");

    HttpResponse::Created().json(ApiResponse {
        status: vec![],
        get:    created,
    })
}

/// POST /jeux/modifier
#[post("/jeux/modifier")]
pub async fn update_jeu(
    pool: web::Data<DbPool>,
    req:  web::Json<UpdateRequest>,
) -> impl Responder {
    let mut conn = pool.get().expect("DB connection failed");
    let UpdateRequest { id: vid, updates } = req.into_inner();

    // 1) ID valide ?
    if vid <= 0 {
        return HttpResponse::Ok().json(ApiResponse {
            status: vec![status::ID_VIDE],
            get:    serde_json::json!({ "id": 0 }),
        });
    }

    // 2) Existence du jeu
    let exists = diesel::select(exists(jeux::table.find(vid)))
        .get_result::<bool>(&mut conn)
        .unwrap_or(false);
    if !exists {
        return HttpResponse::Ok().json(ApiResponse {
            status: vec![status::ID_NOT_FOUND],
            get:    serde_json::json!({ "id": 0 }),
        });
    }

    // 3) Si on change le nom, re-validation
    if let Some(ref new_nom) = updates.nom {
        let codes = validate_nom(&mut conn, new_nom, Some(vid));
        if !codes.is_empty() {
            return HttpResponse::Ok().json(ApiResponse {
                status: codes,
                get:    serde_json::json!({ "id": 0 }),
            });
        }
    }

    // 4) Mise à jour + récupération
    let upd = updates.prepare();
    if let Err(e) = diesel::update(jeux::table.find(vid))
        .set(&upd)
        .execute(&mut conn)
    {
        eprintln!("Update error: {}", e);
        return HttpResponse::InternalServerError().finish();
    }

    let updated = jeux::table
        .find(vid)
        .first::<Jeu>(&mut conn)
        .expect("Fetch after update failed");

    HttpResponse::Ok().json(ApiResponse {
        status: vec![],
        get:    updated,
    })
}

/// POST /jeux/activer
#[post("/jeux/activer")]
pub async fn toggle_active(
    pool: web::Data<DbPool>,
    req:  web::Json<ToggleActiveRequest>,
) -> impl Responder {
    let mut conn = pool.get().expect("DB connection failed");
    let ToggleActiveRequest { id: vid, actif: flag } = req.into_inner();

    let affected = diesel::update(jeux::table.find(vid))
        .set(jeux::actif.eq(flag))
        .execute(&mut conn)
        .unwrap_or(0);

    let resp = if affected == 0 {
        ApiResponse { status: vec![status::ID_NOT_FOUND], get: serde_json::Value::Null }
    } else {
        ApiResponse { status: vec![],                         get: serde_json::Value::Null }
    };
    HttpResponse::Ok().json(resp)
}
