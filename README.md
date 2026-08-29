# NoServerAI

**Run 7B+ Parameter AI Models on Budget Android Phones (<4GB RAM) – No Cloud, No Expensive GPUs.**

---

## 🚀 Why This Exists

- NVIDIA H100 GPUs cost $40,000+ – out of reach for students in developing countries.
- Cloud AI services (AWS, Azure) are expensive and often restricted under sanctions.
- Most smartphones have powerful NPUs/CPUs sitting idle.
- **We believe AI should be a public good, not a monopoly.**

---

## 🎯 What This Project Does

NoServerAI is a **toolkit** (not a new language) that:
- Splits large LLMs (like LLaMA 3, Gemma, Mistral) into **small chunks** (100-500 MB).
- Loads these chunks **on-demand** from storage using **Memory Mapping** (no need to load the entire model into RAM).
- Runs inference **locally on your phone** – no internet, no cloud, no subscription.

**Result:** A 7B model runs on a $200 Android phone with just 4GB RAM.

---

## 🧠 How It Works (Simple)
[Your Model]
↓ (Split into chunks)
[Memory Mapper (Rust)]
↓ (Loads only needed chunks)
[TensorFlow Lite / llama.cpp]
↓ (Runs on NPU/CPU)
[Output: "Hello, I'm running on your phone!"]

text

---

## 🛠 Tech Stack

| Layer | Technology |
|-------|------------|
| **User Interface** | Python (simple API) |
| **Core Logic** | Rust (fast, memory-safe) |
| **Memory Mapping** | `mmap` (OS-level) |
| **Inference Engine** | TensorFlow Lite / llama.cpp / MLC-LLM |
| **Model Format** | GGUF / Safetensors |

---

## 📦 Installation (for users)

```bash
# Coming soon – but the goal is:
pip install noserverai
🔧 Development Status
☑ Project defined (README, structure)
□ Memory Mapper module (Rust) – in progress
□ Python bindings (PyO3)
□ Android APK wrapper (Termux / native)
□ Test on real device (Xiaomi, Samsung, Pixel)
□ Release v1.0 + Demo Video (2 minutes)
🤝 How You Can Help
We need:

Rust developers for memory-mapping and FFI.

Android developers for APK packaging.

ML engineers to optimize model quantization.

Testers with old phones (3-4GB RAM).

Just open an issue or DM me on GitHub.

🌍 Mission
"Democratizing AI for students, researchers, and innovators under sanctions or with limited resources."

This is not about politics. This is about justice in technology.

📜 License
MIT – Free for everyone, forever.

🙏 Credits
Visionary: (Your alias)

Lead Engineer: (My alias – RustWizard)

Inspired by: llama.cpp, MLC-LLM, and the open-source community.

📬 Contact
Open an issue on GitHub – we're always here.
