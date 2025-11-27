# Interstellar Triangulum

[![CI](https://github.com/YOUR_USERNAME/interstellar-triangulum/workflows/CI/badge.svg)](https://github.com/YOUR_USERNAME/interstellar-triangulum/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE)

A Rust-based video creation engine implementing the three pillars from [The Digital Artisan](https://wilkerhop.github.io/the-digital-artisan/):

1. **🦀 Video Engine in Rust** - High-performance rendering pipeline
2. **📈 Viral Narrative** - Retention-optimized script structure
3. **⚖️ Faithful Discourse** - Rhetorical integrity and credibility

## Quick Start

✅ **Phase 1: Foundation (In Progress)**
- ✅ Project setup with Cargo
- ✅ Core data structures (`script.rs`)
- ✅ Script parser with validation (`parser.rs`)
- ✅ Asset loader for images/video/fonts (`assets.rs`)
- 🔄 Testing and verification

## Features

### Current
- ✅ JSON script parsing and validation
- ✅ Image asset loading
- ✅ Font asset loading
- ✅ Video asset stub (placeholder)
- ✅ Support for layers (image, video, text)
- ✅ Transform support (position, scale, rotation, opacity)
- ✅ Effects system (fade in/out, blur, color grading)
- ✅ Transition support (cut, fade, dissolve, wipe)
- ✅ Multi-track audio configuration
- ✅ Asset caching and statistics

### Planned
- ⏳ FFmpeg integration for video decoding
- ⏳ Frame composition and rendering
- ⏳ GPU-accelerated effects (via wgpu)
- ⏳ Audio mixing and synchronization
- ⏳ H.264/H.265 encoding
- ⏳ Narrative structure validation (Hook→Bridge→Payoff)
- ⏳ Retention curve analysis
- ⏳ Credibility scoring tools

## Quick Start

### Build the project
```bash
cargo build
```

### Run the demo
```bash
cargo run
```

### Run tests
```bash
cargo test
```

## Script Format

Scripts are defined in JSON format with the following structure:

```json
{
  "metadata": {
    "title": "My Video",
    "resolution": "1920x1080",
    "fps": 60,
    "duration": 10.0
  },
  "scenes": [
    {
      "id": "scene1",
      "duration": 5.0,
      "layers": [
        {
          "type": "image",
          "source": "background.png"
        },
        {
          "type": "text",
          "content": "Hello World",
          "font": "font.ttf",
          "font_size": 48.0,
          "color": { "r": 255, "g": 255, "b": 255 },
          "position": { "x": 960, "y": 540 }
        }
      ]
    }
  ]
}
```

See `examples/simple.json` for a complete example.

## Architecture

### Core Modules

- **`script.rs`** - Data structures for video scripts
  - `VideoScript`, `Scene`, `Layer`, `Effect`, `Transition`
  - Serde-based JSON deserialization
  
- **`parser.rs`** - Script parsing and validation
  - JSON parsing with error handling
  - Structure validation (duration, scene count, etc.)
  - Script summarization
  
- **`assets.rs`** - Asset loading and management
  - Image loading via `image` crate
  - Video loading (FFmpeg stub)
  - Font loading
  - Asset caching and statistics

### The Three Pillars (Implementation Roadmap)

#### Pillar 1: Video Engine Architecture
**Rendering Pipeline Stages:**
1. Script Parsing ✅
2. Asset Loading ✅ (partial)
3. Frame Composition ⏳
4. Audio Mixing ⏳
5. Encoding & Export ⏳

#### Pillar 2: Viral Narrative Structure
**Script Phases:**
1. **The Hook (0-30s)** - Validate the click, open curiosity gap
2. **The Bridge (Middle)** - Progressive disclosure with false summits
3. **The Payoff (End)** - Deliver on promise, then CTA

Implementation: Validators and analyzers for retention optimization ⏳

#### Pillar 3: Faithful Discourse
**Rhetorical Principles:**
1. **Steelmanning** - Present opposing views fairly
2. **Epistemic Humility** - Separate fact/inference/opinion
3. **Credibility Balance** - Authority + acknowledged limitations

Implementation: Claim classifier and credibility scorer ⏳

## Testing

Unit tests are included for all core modules:

```bash
# Run all tests
cargo test

# Run specific module tests
cargo test script
cargo test parser
cargo test assets

# Run with output
cargo test -- --nocapture
```

## Testing Guidelines

> **RULE**: Every file that exports a function MUST have a corresponding test module.

This rule is enforced by our health check script.

### Running the Health Check

```bash
./scripts/check_test_coverage.sh
```

**What it checks:**
- Every file with functions has a `#[cfg(test)]` module
- All unit tests pass

### Writing Tests

When adding new functions:
1. Add `#[cfg(test)] mod tests { ... }` if not present
2. Write unit tests for all public functions
3. Test edge cases and error conditions
4. Run health check before committing

### Coverage Requirements

- ✅ Unit tests for all public functions (24+ tests currently)
- ✅ Edge case testing
- ✅ Error condition handling
- ✅ Integration tests for main flows

### Current Test Coverage

- **script.rs**: 9 tests (defaults, deserialization, edge cases)
- **parser.rs**: 7 tests (validation, parsing, summaries)
- **assets.rs**: 9 tests (loading, caching, path resolution)
- **Total**: 24tests passing

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed testing guidelines and commit conventions.

## Dependencies

- `serde` - Serialization framework
- `serde_json` - JSON parsing
- `image` - Image processing
- `anyhow` - Error handling
- `thiserror` - Custom errors

### Future Dependencies
- `ffmpeg-next` - Video/audio encoding/decoding
- `wgpu` - GPU compute for effects
- `rayon` - Parallel processing

## Performance Goals

Based on The Digital Artisan benchmarks:
- **Target**: < 10% of real-time for 1080p60 rendering
- **Memory**: < 4GB for typical projects
- **Rust advantage**: 10x+ faster than Python (moviepy)

## Project Structure

```
interstellar-triangulum/
├── .github/
│   └── workflows/
│       └── ci.yml         # CI/CD pipeline
├── src/
│   ├── lib.rs             # Library exports
│   ├── main.rs            # Demo application
│   ├── script.rs          # Data structures + tests
│   ├── parser.rs          # JSON parsing + tests
│   └── assets.rs          # Asset loading + tests
├── examples/
│   ├── simple.json        # Example video script
│   └── assets/            # Example assets
├── scripts/
│   └── check_test_coverage.sh  # Health check script
├── Cargo.toml             # Dependencies
├── LICENSE                # MIT License
├── README.md              # This file
└── CONTRIBUTING.md        # Contribution guidelines
```

---

**Built following The Digital Artisan's principles** 🚀
