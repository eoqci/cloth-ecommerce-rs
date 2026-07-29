<h1 align="center">Cloth Ecommerce</h1>
<p align="center">
  <em>A blazing-fast REST API for selling clothes, built with Rust & Axum</em>
</p>

<div align="center">
  <img src="https://img.shields.io/badge/Rust-CE422B?style=for-the-badge&logo=rust&logoColor=white&labelColor=8B2500" />
  <img src="https://img.shields.io/badge/Axum-CE422B?style=for-the-badge&logo=rust&logoColor=white&labelColor=8B2500" />
  <img src="https://img.shields.io/badge/SQLx-CE422B?style=for-the-badge&logo=rust&logoColor=white&labelColor=8B2500" />
  <img src="https://img.shields.io/badge/PostgreSQL-4169E1?style=for-the-badge&logo=postgresql&logoColor=white&labelColor=1a3a6b" />
  <img src="https://img.shields.io/badge/Redis-FF4438?style=for-the-badge&logo=redis&logoColor=white&labelColor=a81a0e" />
  <img src="https://img.shields.io/badge/R2-F38020?style=for-the-badge&logo=cloudflare&logoColor=white&labelColor=b35a00" />
  <img src="https://img.shields.io/badge/AWS-FF9900?style=for-the-badge&logo=amazonapigateway&logoColor=white&labelColor=c47500" />
  <img src="https://img.shields.io/badge/REST API-FF9900?style=for-the-badge&logo=amazonapigateway&logoColor=white&labelColor=c47500" />
</div>

<br/>

<div align="center">
  <img src="https://img.shields.io/badge/Status-In%20Development-yellow?style=flat-square" />
  <img src="https://img.shields.io/badge/License-MIT-green?style=flat-square" />
  <img src="https://img.shields.io/badge/Architecture-Modular%20%2B%20Layered-blue?style=flat-square" />
</div>

## ABOUT
**Cloth Ecommerce** is a REST API for an online clothing store, built with **Rust + Axum**. The project prioritizes **zero-GC memory efficiency** — no Java, no C#, no Go. Just raw Rust performance.

Built on top of **Tokio** (async runtime) and **Tower** (middleware/service abstractions), this project uses a **Modular + Layered Architecture** — clean enough for a solo dev, scalable enough for a small team. No microservices, no overengineering. Just a solid monorepo that gets the job done.

> Why Rust over Golang? Simple — no garbage collector means predictable, low memory consumption at scale. This project is also a personal challenge to push deeper into systems programming with a production-grade use case., only mono repo. I challenge myself with a brandnew and hard development framework (actix harder, I know), to learn how a programming language that not use GC, to see how far I can go.

I may need refactor the whole codebase soon. So as i see, its quite outdated with what i got. the codebase may more complicated than it used to be, but easier to maintain. The ideally is im gonna make state into deep module root, each module has it own state, problems is you might see the codebase is too hard to able to read anyway. so i will change it soon.
