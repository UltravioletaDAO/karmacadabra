# 🎯 Karmacadabra: Trustless Agent Economy

> AI agents that autonomously buy/sell data using blockchain-based gasless micropayments

[![Avalanche](https://img.shields.io/badge/Avalanche-Fuji-E84142?logo=avalanche)](https://testnet.snowtrace.io/)
[![ERC-8004](https://img.shields.io/badge/ERC--8004-Trust%20Framework-blue)](https://eips.ethereum.org/EIPS/eip-8004)
[![x402](https://img.shields.io/badge/x402-Payment%20Protocol-green)](https://www.x402.org)
[![Python](https://img.shields.io/badge/Python-3.11+-blue?logo=python)](https://www.python.org/)
[![Rust](https://img.shields.io/badge/Rust-Latest-orange?logo=rust)](https://www.rust-lang.org/)

---

## 🎯 What is Karmacadabra?

**Karmacadabra** is an ecosystem of autonomous AI agents that **buy and sell data** without human intervention using:

- **ERC-8004** for on-chain identity & reputation
- **A2A protocol** (Pydantic AI) for agent-to-agent communication
- **x402 + EIP-3009** for HTTP micropayments (gasless!)
- **CrewAI** for multi-agent orchestration

### The Problem We Solve

**Karma-Hello** has rich Twitch chat logs but no audio context.
**Abracadabra** has stream transcriptions but no chat data.

**Solution**: Agents autonomously negotiate and purchase complementary data, building a complete streaming context. All transactions are verified, on-chain, and gasless.

---

## 🚀 Quick Start (30 minutes)

```bash
# 1. Clone repository
git clone https://github.com/ultravioletadao/karmacadabra.git
cd karmacadabra

# 2. Get testnet AVAX
# Visit: https://faucet.avax.network/

# 3. Deploy contracts
cd erc-20 && ./deploy-fuji.sh && cd ..
cd erc-8004 && ./deploy-fuji.sh && cd ..

# 4. Start facilitator
cd x402-rs && cargo run &

# 5. Run demo
python demo.py
```

**Full guide**: See [QUICKSTART.md](./QUICKSTART.md)

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────┐
│          AVALANCHE FUJI TESTNET (Layer 1)               │
│  ┌──────────────────┐    ┌──────────────────────┐      │
│  │  UVD V2 Token    │    │  ERC-8004 Registries │      │
│  │  (EIP-3009)      │    │  • Identity          │      │
│  │  Gasless txs ✓   │    │  • Reputation        │      │
│  └──────────────────┘    │  • Validation        │      │
│                          └──────────────────────┘      │
└─────────────────────────────────────────────────────────┘
                          ▲
                          │
┌─────────────────────────┴─────────────────────────────┐
│   x402 Facilitator (Layer 2) - Rust                   │
│   • Verifies EIP-712 signatures                       │
│   • Executes transferWithAuthorization()              │
│   • Stateless, no database needed                     │
└─────────────────────────────────────────────────────────┘
            ▲                            ▲
            │                            │
┌───────────┴────────┐      ┌───────────┴────────┐
│ Karma-Hello Agent  │      │ Abracadabra Agent  │
│ • Sells: Chat logs │      │ • Sells: Transcripts│
│ • Buys: Transcripts│      │ • Buys: Chat logs   │
│ • Price: 0.01 UVD  │      │ • Price: 0.02 UVD   │
│ • Data: MongoDB    │      │ • Data: SQLite      │
└────────────────────┘      └─────────────────────┘
            ▲                            ▲
            └────────┬───────────────────┘
                     ▼
         ┌────────────────────┐
         │  Validator Agent   │
         │  • CrewAI crew     │
         │  • Quality score   │
         │  • Fee: 0.001 UVD  │
         └────────────────────┘
```

### 🔗 About Our ERC-8004 Implementation

> **Modified Bi-Directional ERC-8004**: This project uses a custom implementation of ERC-8004 that extends the original specification with bi-directional reputation and validation capabilities. This enhanced version will be deployed on **Avalanche**, home of Ultravioleta DAO, enabling symmetric trust relationships between agents (buyers can validate sellers AND sellers can validate buyers).

**Key Differences from Original ERC-8004:**
- ✅ Bi-directional validation support
- ✅ Optimized for Avalanche's fast finality (2s blocks)
- ✅ Integrated with EIP-3009 for gasless operations
- ✅ Extended reputation metrics for autonomous agents

---

## 💰 What Can Be Monetized?

### Karma-Hello Services (20+ products)
- **Tier 1** (0.01 UVD): Chat logs, user activity
- **Tier 2** (0.10 UVD): ML predictions, sentiment analysis
- **Tier 3** (0.20 UVD): Fraud detection, economic health
- **Enterprise** (up to 200 UVD): White-label, custom models

### Abracadabra Services (30+ products)
- **Tier 1** (0.02 UVD): Raw transcripts, enhanced transcripts
- **Tier 2** (0.15 UVD): Clip generation, blog posts
- **Tier 3** (0.35 UVD): Predictive engine, recommendations
- **Tier 4** (1.50 UVD): Auto video editing, image generation
- **Enterprise** (up to 100 UVD): Custom AI models

**Full catalog**: [MONETIZATION_OPPORTUNITIES.md](./MONETIZATION_OPPORTUNITIES.md)

---

## 📂 Repository Structure

```
karmacadabra/
├── erc-20/                    # UVD V2 Token (EIP-3009)
├── erc-8004/                  # Identity/Reputation/Validation registries
├── x402-rs/                   # Payment facilitator (Rust)
├── validator/                 # Validator agent (Python + CrewAI)
├── karma-hello-agent/         # Chat log seller/buyer agents
├── abracadabra-agent/         # Transcript seller/buyer agents
├── MASTER_PLAN.md            # Complete vision & roadmap
├── ARCHITECTURE.md           # Technical architecture
├── MONETIZATION_OPPORTUNITIES.md
├── QUICKSTART.md             # 30-min setup guide
├── CLAUDE.md                 # Claude Code guidance
└── INDEX.md                  # Documentation index
```

---

## 🛠️ Tech Stack

| Layer | Technology | Purpose |
|-------|-----------|---------|
| **Blockchain** | Avalanche Fuji | EVM testnet for smart contracts |
| **Contracts** | Solidity + Foundry | ERC-8004 registries + UVD token |
| **Facilitator** | Rust (Axum) | x402 payment verification |
| **Agents** | Python 3.11+ | AI agent runtime |
| **AI Framework** | CrewAI | Multi-agent orchestration |
| **LLM** | GPT-4o | Analysis and validation |
| **Web3** | web3.py + ethers-rs | Blockchain interaction |
| **Data** | MongoDB + SQLite + Cognee | Agent data sources |

---

## 🎯 Key Features

✅ **Gasless Micropayments**: Agents don't need ETH/AVAX for gas
✅ **On-Chain Reputation**: ERC-8004 tracks agent reliability
✅ **Trustless Validation**: Independent validators verify data quality
✅ **Agent Discovery**: A2A protocol AgentCards at `/.well-known/agent-card`
✅ **Multi-Agent Workflows**: CrewAI crews for complex tasks
✅ **50+ Monetizable Services**: From $0.01 to $200 UVD per service

---

## 📚 Documentation

| Document | Description | Time |
|----------|-------------|------|
| [QUICKSTART.md](./QUICKSTART.md) | Get running in 30 minutes | 30 min |
| [MASTER_PLAN.md](./MASTER_PLAN.md) | Complete vision & roadmap | 60 min |
| [ARCHITECTURE.md](./ARCHITECTURE.md) | Technical deep dive | 45 min |
| [MONETIZATION_OPPORTUNITIES.md](./MONETIZATION_OPPORTUNITIES.md) | All services & pricing | 30 min |
| [CLAUDE.md](./CLAUDE.md) | Developer guidance | 15 min |
| [INDEX.md](./INDEX.md) | Documentation index | 5 min |

**Component READMEs**: Each folder has detailed setup instructions.

---

## 🧪 Development Status

| Phase | Component | Status |
|-------|-----------|--------|
| **Phase 1** | UVD V2 Token | 🔴 Ready to deploy |
| **Phase 1** | ERC-8004 Registries | 🔴 Ready to deploy |
| **Phase 1** | x402 Facilitator | 🔴 Ready to build |
| **Phase 2** | Validator Agent | 🔴 To implement |
| **Phase 3** | Karma-Hello Agents | 🔴 To implement |
| **Phase 4** | Abracadabra Agents | 🔴 To implement |
| **Phase 5** | End-to-end Testing | 🔴 Pending |

**Timeline**: 6 weeks to production-ready demo

---

## 🔧 Requirements

- **Python** 3.11+
- **Rust** latest stable
- **Foundry** (forge, anvil, cast)
- **Node.js** 18+ (optional, for frontend)
- **AVAX** on Fuji testnet (free from faucet)
- **OpenAI API key** (for CrewAI agents)

---

## 🚦 Getting Started

### 1. Prerequisites
```bash
# Install Foundry
curl -L https://foundry.paradigm.xyz | bash
foundryup

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Python 3.11+
python --version  # Should be 3.11 or higher
```

### 2. Get Testnet AVAX
Visit https://faucet.avax.network/ and request AVAX for your wallet.

### 3. Deploy Infrastructure
```bash
cd erc-20
cp .env.example .env
# Edit .env with your PRIVATE_KEY
./deploy-fuji.sh

cd ../erc-8004
./deploy-fuji.sh

cd ../x402-rs
cargo build --release
cargo run
```

### 4. Run Demo
```bash
python demo.py
```

See [QUICKSTART.md](./QUICKSTART.md) for detailed instructions.

---

## 🤝 Contributing

1. Read [MASTER_PLAN.md](./MASTER_PLAN.md) to understand the vision
2. Check the roadmap for available tasks
3. Implement following the architecture in [ARCHITECTURE.md](./ARCHITECTURE.md)
4. Write tests for all new code
5. Submit PR with documentation

---

## 📖 Learn More

- **ERC-8004**: https://eips.ethereum.org/EIPS/eip-8004
- **A2A Protocol**: https://ai.pydantic.dev/a2a/
- **x402 Protocol**: https://www.x402.org
- **EIP-3009**: https://eips.ethereum.org/EIPS/eip-3009
- **CrewAI**: https://docs.crewai.com/
- **Avalanche**: https://docs.avax.network/

### Trustless Agents Course
https://intensivecolearn.ing/en/programs/trustless-agents

---

## ⚠️ Disclaimer

**TESTNET ONLY**: This project is currently deployed on Avalanche Fuji testnet. Do not use with real funds. Smart contracts have not been audited.

For mainnet deployment:
- [ ] Smart contract audit by reputable firm
- [ ] Bug bounty program
- [ ] Timelock for admin functions
- [ ] Multi-sig for contract ownership

---

## 📄 License

MIT License - See [LICENSE](./LICENSE)

---

## 🌟 Acknowledgments

- **Trustless Agents Course** by Intensive CoLearning
- **ERC-8004 Example** (Bob validator implementation)
- **x402-rs** protocol implementation
- **Pydantic AI** A2A protocol
- **Avalanche** for fast, low-cost testnet

---

## 💬 Contact

- **Project**: Ultravioleta DAO
- **Repo**: https://github.com/ultravioletadao/karmacadabra
- **Docs**: Start with [QUICKSTART.md](./QUICKSTART.md)

---

**Built with ❤️ by Ultravioleta DAO**

*Empowering autonomous AI agents to create a trustless data economy*
