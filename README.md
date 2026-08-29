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
