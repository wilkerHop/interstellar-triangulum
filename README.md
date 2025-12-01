# The Digital Artisan: Interstellar Triangulum

**Interstellar Triangulum** is a high-performance, Rust-based video creation engine designed to automate the production of high-quality, data-driven video content. It is built upon the three core pillars of The Digital Artisan philosophy.

![Three Pillars Infographic](file:///Users/wilkerribeiro/.gemini/antigravity/brain/07001e3a-f9ce-41d3-9c23-6a6dc99b73cb/three_pillars_infographic_1764205094252.png)

## The Three Pillars

### 1. Fast (Performance)
**"Speed is the currency of the digital age."**
We leverage the power of **Rust** and **Blender** to create a rendering pipeline that is blazing fast and resource-efficient.
*   **Parallel Rendering**: Utilizes multi-core CPUs to render frames concurrently.
*   **Smart Caching**: SHA256-based caching ensures we never re-render unchanged scenes.
*   **Memory Safety**: Built-in vaults monitor resource usage to prevent crashes.

### 2. Engaging (Narrative)
**"Attention is the scarcest resource."**
We provide tools to ensure your content grabs and holds the viewer's attention.
*   **Narrative Structure**: Automated validation of hook, body, and payoff sequences.
*   **Retention Analysis**: Predictive modeling to identify and fix pacing issues.
*   **Dynamic Visuals**: High-fidelity animations and effects to keep the screen alive.

### 3. Trustworthy (Credibility)
**"Trust is the foundation of influence."**
We integrate verification tools to ensure your content is accurate and reliable.
*   **Claim Classification**: Automatically identifies factual claims in your script.
*   **Source Verification**: Enforces citation requirements for data-driven statements.
*   **Credibility Scoring**: Quantitative analysis of your content's reliability.

## Project Status

| Feature | Status | Description |
| :--- | :--- | :--- |
| **Video Engine** | ✅ Complete | Core Rust engine, JSON parser, Asset loader |
| **Rendering** | ✅ Complete | Blender backend with material system & effects |
| **Audio** | ✅ Complete | Multi-track mixing and synchronization |
| **Narrative Tools** | 🚧 Planned | Structure validation & retention analysis |
| **Credibility Tools** | 🚧 Planned | Claim verification & scoring |

## Getting Started

### Prerequisites
- Rust (latest stable)
- Blender (3.0+)
- FFmpeg

### Running a Demo
```bash
# Build the release binary
cargo build --release

# Run the simple example
./target/release/interstellar-triangulum examples/simple.json --renderer blender
```

## License
MIT
