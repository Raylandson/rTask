# rTask

rTask is a full-stack Rust project with a backend powered by Axum, Diesel, and SQLite, and a frontend using Leptos. The backend provides a RESTful API for user and task management, while the frontend offers a modern web interface.

## Getting Started

### Backend Setup

1. **Create a `.env` file in the `backend/` directory:**

   This file is required for the backend to connect to the database. Add the following line to your `backend/.env`:

   ```env
   DATABASE_URL=sqlite://db.sqlite
   ```

   You can change the path or use another database if you wish.

2. **Run database migrations:**

   Ensure you have Diesel CLI installed, then run:

   ```bash
   cd backend
   diesel migration run
   ```

3. **Start the backend server:**

   ```bash
   cargo run
   ```

The backend will start and connect to the SQLite database specified in your `.env` file.

## Project Overview

- **Backend:** Rust, Axum, Diesel, SQLite, r2d2 connection pool
- **Frontend:** Rust, Leptos

The backend exposes RESTful endpoints for user and task management. The frontend communicates with the backend API and provides a reactive UI.

---

**Note:** You must create the `.env` file in the `backend/` directory before running the backend server, or the application will not start.

# rTask
