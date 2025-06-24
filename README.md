# 📝 HoloNotes Backend

A fast and secure backend for **HoloNotes**, a decentralized-inspired note-taking app. Built using **Axum**, **SQLite**, and **SQLx**, this backend supports RESTful APIs, user authentication, and secure data storage.

---

## 🚀 Features

- 🔐 User registration with **Argon2 password hashing**
- 🗒️ Create, read, update, delete notes
- 📦 Modular Axum architecture (`handlers`, `routes`, `models`, `db`)
- ⚡ Built with `tokio`, `sqlx`, and `uuid`
- 🧪 Curl + SQLite CLI compatible
- ✅ Ready for JWT-based authentication (coming next)

---

## 🧱 Tech Stack

| Layer        | Tool              |
|--------------|-------------------|
| Web Framework| Axum              |
| Database     | SQLite            |
| Async Runtime| Tokio             |
| ORM          | SQLx              |
| Auth         | Argon2 + JWT      |
| Env Config   | dotenvy           |
| DateTime     | chrono            |

---

## 📦 Project Structure

holonotes-backend/
├── src/
│ ├── main.rs # Launches Axum server
│ ├── db.rs # DB pool and connection setup
│ ├── models.rs # User and Note data models
│ ├── routes.rs # Routes for users and notes
│ ├── handlers.rs # HTTP logic for endpoints
│ └── auth.rs # Password hashing logic
├── migrations/ # SQLx migrations
├── .env # DATABASE_URL config
├── Cargo.toml
└── README.md

---

## ⚙️ Getting Started

### 1. 📥 Clone the repo

```bash
git clone https://github.com/your-username/holonotes-backend.git
cd holonotes-backend

Create a .env file:

DATABASE_URL=sqlite://holonotes.db

📚 Run migrations:

sqlx migrate run

🚀 Run the server

cargo run
Server will start at: http://localhost:3000

API End points:

POST /register
POST /notes
GET /notes
DELETE /notes/:id
PUT /notes/:id

