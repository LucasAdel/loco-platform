# Loco Platform 🦀

A professional pharmacy job marketplace built with Rust, Dioxus, and modern web technologies.

## 🚀 Features

- **Rust-Powered Performance**: Built with Rust for memory safety and blazing fast performance
- **Modern UI**: Dioxus frontend with Apple-style design system
- **Interactive Maps**: Mapbox integration for location-based job discovery
- **Real-time Updates**: WebSocket-powered real-time job notifications
- **Mobile-First**: Progressive Web App with offline capabilities
- **Australian Focused**: Designed specifically for the Australian pharmacy market

## 🛠️ Tech Stack

### Frontend
- **Dioxus** - React-like framework for Rust
- **WebAssembly** - Near-native performance in the browser
- **Fermi** - Global state management
- **TailwindCSS** - Utility-first CSS framework

### Backend
- **Axum** - Modern web framework for Rust
- **SeaORM** - Async ORM for database operations
- **PostgreSQL** - Robust relational database
- **Tokio** - Async runtime for Rust

### Integration
- **Mapbox** - Interactive maps and geocoding
- **Supabase** - Authentication and real-time features

## 🏃‍♂️ Quick Start

### Prerequisites

- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- PostgreSQL 14+ ([Install PostgreSQL](https://postgresql.org/download/))
- Dioxus CLI (`cargo install dioxus-cli`)

### Setup

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd loco-platform
   ```

2. **Environment setup**
   ```bash
   cp .env.example .env
   # Edit .env with your configuration
   ```

3. **Database setup**
   ```bash
   # Create database
   createdb loco_platform
   
   # Run migrations (when available)
   cd backend && cargo run --bin migrate
   ```

4. **Install dependencies**
   ```bash
   # All dependencies are managed by Cargo
   cargo check --workspace
   ```

5. **Start development servers**
   
   **Terminal 1 - Backend:**
   ```bash
   cd backend
   cargo run
   # Backend runs on http://localhost:3070
   ```
   
   **Terminal 2 - Frontend:**
   ```bash
   dx serve --hot-reload
   # Frontend runs on http://localhost:8080
   ```

6. **Open your browser**
   ```
   http://localhost:8080
   ```

## 📁 Project Structure

```
loco-platform/
├── frontend/           # Dioxus frontend application
│   ├── src/
│   │   ├── components/ # Reusable UI components
│   │   ├── pages/      # Page components
│   │   ├── hooks/      # Custom Dioxus hooks
│   │   └── services/   # API client services
│   └── assets/         # Static assets and styles
├── backend/            # Axum backend application
│   ├── src/
│   │   ├── handlers/   # HTTP route handlers
│   │   ├── models/     # Database models
│   │   ├── services/   # Business logic
│   │   └── middleware/ # Custom middleware
├── shared/             # Shared types and utilities
│   └── src/
│       ├── types.rs    # Common data structures
│       ├── errors.rs   # Error types
│       └── utils.rs    # Utility functions
├── docs/               # Documentation
├── memory-bank/        # Project memory and context
├── CLAUDE.md          # Development instructions
└── checklist.md       # Implementation roadmap
```

## 🗺️ Key Components

### Job Marketplace
- Job search and filtering
- Location-based discovery
- Real-time job updates
- Application management

### Interactive Maps
- Mapbox integration
- Job clustering
- Commute calculation
- Location search

### User Management
- Role-based access control
- Professional profiles
- Employer dashboards
- Authentication

## 🔧 Development

### Building

```bash
# Development build
cargo build

# Release build
cargo build --release

# Frontend only
dx build

# Backend only
cd backend && cargo build --release
```

### Testing

```bash
# Run all tests
cargo test --workspace

# Frontend tests
cd frontend && cargo test

# Backend tests
cd backend && cargo test
```

### Linting

```bash
# Check code quality
cargo clippy --workspace --all-targets

# Format code
cargo fmt --all
```

## 🌐 API Endpoints

### Jobs
- `GET /api/jobs` - List all jobs
- `POST /api/jobs` - Create new job
- `GET /api/jobs/:id` - Get job details
- `POST /api/jobs/search` - Search jobs with filters

### Authentication
- `POST /api/auth/register` - User registration
- `POST /api/auth/login` - User login
- `POST /api/auth/refresh` - Refresh token

### Health
- `GET /health` - Health check

## 📱 PWA Features

- **Offline Support**: Cached job listings and core functionality
- **Push Notifications**: Real-time job alerts
- **App Install**: Native app-like experience
- **Responsive Design**: Mobile-first responsive layout

## 🔐 Security

- **Type Safety**: Rust's type system prevents common vulnerabilities
- **Authentication**: JWT-based authentication with secure cookies
- **CSRF Protection**: Built-in CSRF protection for forms
- **Input Validation**: Comprehensive input validation and sanitization

## 🚀 Deployment

### Development
```bash
# Start both frontend and backend
./scripts/dev.sh
```

### Production
```bash
# Build for production
./scripts/build.sh

# Deploy with Docker
docker-compose up -d
```

## 📊 Performance

- **Bundle Size**: Optimized WASM bundle < 2MB
- **Load Time**: Sub-second initial load
- **Memory Usage**: Rust's zero-cost abstractions
- **Scalability**: Horizontal scaling with Axum

## 🤝 Contributing

1. **Fork the repository**
2. **Create a feature branch** (`git checkout -b feature/amazing-feature`)
3. **Follow the checklist** (see `checklist.md`)
4. **Run tests** (`cargo test --workspace`)
5. **Submit a pull request**

### Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Use Australian English in all user-facing text
- Write comprehensive tests for new features
- Update documentation as needed

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Dioxus Team** - For the amazing React-like framework
- **Axum Team** - For the powerful web framework
- **Rust Community** - For the incredible ecosystem
- **Australian Pharmacy Community** - For inspiration and feedback

## 📞 Support

- **Documentation**: Check the `/docs` folder
- **Issues**: Create an issue on GitHub
- **Discussions**: Use GitHub Discussions for questions

## 🗺️ Roadmap

See `checklist.md` for the detailed implementation roadmap including:

- ✅ Core job marketplace functionality
- ✅ Interactive maps with Mapbox
- 🔄 AI-powered job matching
- 🔄 Real-time messaging system
- 🔄 Advanced analytics dashboard
- 🔄 Mobile app development

---

**Built with ❤️ in Australia using Rust 🦀**