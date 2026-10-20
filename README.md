# URL Shortener

Вебсервіс для скорочення довгих посилань. Зареєстрований користувач створює короткі URL і переглядає статистику переходів; перейти за коротким посиланням може будь-хто без реєстрації.

## Team

- Владислав — frontend, QA, task-tracking
- Данило — backend, database, architecture, DevOps

## Tech stack

- Frontend: React + TypeScript + Vite
- Backend: Rust: Axum + Tokio + SQLx
- Database: PostgreSQL

## Launching backend

```
cd backend
docker compose up -d
cargo run
```