# Project Structure

This document describes the organization of the 2HAL9 Demo project after restructuring.

## Directory Layout

```
2hal9-demo/
├── README.md              # Main project documentation (only file in root)
├── LICENSE                # MIT License
├── Cargo.toml            # Workspace configuration
├── Cargo.lock            # Dependency lock file
│
├── crates/               # Rust crates (modular architecture)
│   ├── genius-core/      # Core traits and types
│   ├── genius-engine/    # Game execution engine
│   ├── genius-ai/        # AI provider abstractions
│   ├── genius-games/     # Game implementations
│   ├── genius-server/    # HTTP/WebSocket server
│   └── genius-client/    # Client SDK
│
├── docs/                 # Documentation
│   ├── CONSCIOUSNESS_GAMES_README.md    # Consciousness-themed games documentation
│   ├── NEW_CONSCIOUSNESS_GAMES.md       # New game concepts and plans
│   ├── DEPLOYMENT.md                    # Deployment guide
│   ├── PROJECT_STRUCTURE.md           # This file
│   └── original-readme.md              # Original project README
│
├── docker/               # Docker configuration
│   ├── Dockerfile        # Container image definition
│   └── docker-compose.yml # Multi-container setup
│
├── scripts/              # Utility scripts
│   ├── fix-all-imports.sh       # Fix Rust imports
│   ├── fix-game-imports.sh      # Fix game-specific imports
│   ├── fix-genius-ai.sh         # Fix AI module imports
│   ├── fix-remaining-imports.sh # Fix remaining import issues
│   └── final-fix.sh            # Final cleanup script
│
├── demo/                 # Demo applications and assets
│   ├── *.html           # Web-based game demos
│   ├── *.py             # Python demo scripts
│   └── *.png/gif        # Demo screenshots and animations
│
├── demos/                # Rust demo applications
│   └── emergence_showcase.rs    # Consciousness emergence demonstrations
│
├── examples/             # Example code
│   └── client_demo.py    # Python client example
│
├── k8s/                  # Kubernetes manifests
│   ├── deployment.yaml   # Main deployment
│   ├── service.yaml      # Service definition
│   └── ...              # Other K8s resources
│
├── public/               # Static assets for web server
├── tests/                # Integration tests
└── target/               # Build output (git-ignored)
```

## Key Changes from Previous Structure

1. **Root Directory Cleanup**: Only essential files remain in root (README.md, LICENSE, Cargo files)
2. **Documentation Consolidation**: All documentation moved to `docs/`
3. **Docker Organization**: Docker-related files moved to `docker/`
4. **Scripts Directory**: All shell scripts moved to `scripts/` with relative paths
5. **Clear Separation**: Development files organized by purpose

## Usage Notes

### Docker Commands
```bash
# Build and run with Docker Compose (from project root)
docker-compose -f docker/docker-compose.yml up

# Build Docker image
docker build -t genius-game-server -f docker/Dockerfile .
```

### Running Scripts
All scripts now use relative paths and can be run from any location:
```bash
# From project root
./scripts/fix-all-imports.sh

# From any subdirectory
../scripts/fix-all-imports.sh
```

### Finding Documentation
- Main project overview: `README.md` (root)
- Consciousness games details: `docs/CONSCIOUSNESS_GAMES_README.md`
- New game concepts: `docs/NEW_CONSCIOUSNESS_GAMES.md`
- Deployment guide: `docs/DEPLOYMENT.md`

## Development Workflow

1. **Code**: Work in `crates/` subdirectories
2. **Test**: Run tests with `cargo test --workspace`
3. **Demo**: Use files in `demo/` and `demos/` for demonstrations
4. **Deploy**: Use configurations in `docker/` and `k8s/`
5. **Document**: Add documentation to `docs/`

This structure promotes clarity, maintainability, and follows Rust workspace best practices.