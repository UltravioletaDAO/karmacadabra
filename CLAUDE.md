# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

---

## Project Overview

**Karmacadabra** is a trustless agent economy where AI agents autonomously buy/sell data using blockchain-based payments. The system enables:

- **Karma-Hello agents** selling Twitch stream chat logs (0.01-1.00 UVD per service)
- **Abracadabra agents** selling stream transcriptions + AI analysis (0.02-3.00 UVD per service)
- **Validator agents** providing quality verification (0.001 UVD per validation)
- **Gasless micropayments** using EIP-3009 meta-transactions via x402 protocol
- **On-chain reputation** using ERC-8004 registries on Avalanche Fuji testnet

**Key Innovation**: Agents operate autonomously without needing ETH/AVAX for gas fees, using signed payment authorizations settled by a facilitator.

---

## Architecture: Three-Layer System

### Layer 1: Blockchain (Avalanche Fuji Testnet)
- **UVD V2 Token** (`erc-20/`): ERC-20 with EIP-3009 for gasless transfers
- **ERC-8004 Registries** (`erc-8004/`): Identity, Reputation, Validation contracts
- All agents register on-chain and build reputation through transactions

### Layer 2: Payment Facilitator (Rust)
- **x402-rs** (`x402-rs/`): HTTP 402 payment protocol implementation
- Verifies EIP-712 signatures and executes `transferWithAuthorization()` on-chain
- Stateless design - no database, all state on blockchain
- Public endpoint: `facilitator.ultravioletadao.xyz`

### Layer 3: AI Agents (Python + CrewAI)
- **karma-hello-agent/**: Sells stream logs from MongoDB, buys transcripts
- **abracadabra-agent/**: Sells transcripts from SQLite+Cognee, buys logs
- **validator/**: Independent validation service using CrewAI crews
- All agents use A2A protocol (Pydantic AI) for discovery and communication

---

## Data Flow: Payment Transaction

```
1. Buyer discovers Seller via A2A AgentCard (/.well-known/agent-card)
2. Buyer signs EIP-712 payment authorization off-chain
3. Buyer sends HTTP request with X-Payment header to Seller
4. Seller's x402 middleware extracts payment, calls facilitator /verify
5. (Optional) Seller requests validation from Validator agent
6. Validator analyzes data quality using CrewAI crew, submits score on-chain
7. Facilitator executes transferWithAuthorization() - no gas needed by agents
8. Seller returns data, Buyer integrates into knowledge base
```

**Duration**: ~2-3 seconds end-to-end, completely gasless for agents

---

## Component Commands

### Smart Contracts (Foundry)

**Deploy UVD Token to Fuji:**
```bash
cd erc-20
forge build
./deploy-fuji.sh  # or deploy-fuji.ps1 on Windows
```

**Deploy ERC-8004 Registries:**
```bash
cd erc-8004
cd contracts && forge build && cd ..
./deploy-fuji.sh
```

**Run contract tests:**
```bash
cd erc-8004/contracts
forge test -vv
forge test --match-test testValidationFlow  # single test
```

### x402 Facilitator (Rust)

**Build and run:**
```bash
cd x402-rs
cargo build --release
cargo run  # starts on localhost:8080
```

**Run tests:**
```bash
cargo test
cargo test --package x402-axum -- --nocapture  # single crate with output
```

**Health check:**
```bash
curl http://localhost:8080/health
curl http://localhost:8080/supported  # list payment methods
```

### Python Agents

**Setup any agent:**
```bash
cd karma-hello-agent  # or abracadabra-agent or validator
python -m venv venv
source venv/bin/activate  # Windows: venv\Scripts\activate
pip install -r requirements.txt
```

**Register agent on-chain:**
```bash
python scripts/register_seller.py  # or register_validator.py
```

**Run agent:**
```bash
python main.py --mode seller  # for seller agents
python main.py --mode buyer   # for buyer agents
python main.py                # for validator
```

**Run tests:**
```bash
pytest tests/
pytest tests/test_seller.py::test_get_logs -v  # single test
```

---

## Critical Data Locations

### Product Data (What Agents Sell)

**Karma-Hello logs:**
```
karma-hello-agent/logs/YYYYMMDD/
├── full.txt                    # Complete chat for the day
├── {username}.txt              # Per-user message logs
└── ...
```
Source: MongoDB at `z:\ultravioleta\ai\cursor\karma-hello`
Format: `[MM/DD/YYYY HH:MM:SS AM/PM] username: message`

**Abracadabra transcripts:**
```
abracadabra-agent/transcripts/YYYYMMDD/{stream_id}/
├── transcripcion.json          # Main product (3.3MB)
├── ideas_extraidas.json        # 5 ideas with brainstorming (21KB)
├── imagenes_generadas/         # 20 DALL-E 3 images
├── resumen_completo.txt        # Blog post (3.8KB)
├── tweet.txt                   # Generated tweet (318B)
├── segmentos/                  # Detected clips
└── ...
```
Source: SQLite at `z:\ultravioleta\ai\cursor\abracadabra\analytics.db` + Cognee graph (640+ topics)

### Configuration Pattern

All components follow this pattern:
```bash
cp .env.example .env
# Edit .env with your keys and addresses
```

**Critical .env variables:**
- `PRIVATE_KEY`: Wallet private key for signing transactions
- `RPC_URL_FUJI`: Avalanche Fuji RPC endpoint
- `IDENTITY_REGISTRY`: Address from ERC-8004 deployment
- `UVD_TOKEN_ADDRESS`: Address from UVD token deployment
- `FACILITATOR_URL`: x402 facilitator endpoint
- `OPENAI_API_KEY`: For CrewAI agents (GPT-4o)

---

## Agent Implementation Pattern

All agents inherit from `ERC8004BaseAgent` and implement:

```python
class KarmaHelloSeller(ERC8004BaseAgent, A2AServer):
    def __init__(self, config):
        # Register identity on-chain
        self.agent_id = self.register_agent(
            domain="karma-hello-seller.ultravioletadao.xyz"
        )

        # Data source: Local files OR production DB
        if config.USE_LOCAL_FILES:
            self.data_path = "karma-hello-agent/logs"
        else:
            self.db = MongoClient(config.MONGO_URI)["karma_hello"]

        # Publish A2A AgentCard
        self.publish_agent_card()

    @x402_required(price=UVD.amount("0.01"))
    async def get_logs(self, request):
        # CrewAI crew formats/validates data
        crew = Crew(agents=[formatter, validator])
        return crew.kickoff(inputs={"data": raw_logs})
```

**Key pattern**: Agents support both local file testing and production database queries.

---

## Monetization: 50+ Services

Full catalog in `MONETIZATION_OPPORTUNITIES.md`. Quick reference:

**Karma-Hello (6 tiers):**
- Tier 1 (0.01-0.05 UVD): Chat logs, user activity, token economics
- Tier 2 (0.05-0.15 UVD): ML predictions, sentiment analysis
- Tier 3 (0.15-0.30 UVD): Fraud detection, economic health
- Tier 4-6: A/B testing, custom models, enterprise (up to 200 UVD)

**Abracadabra (6 tiers):**
- Tier 1 (0.02-0.08 UVD): Raw/enhanced transcripts, multi-language
- Tier 2 (0.10-0.25 UVD): Clip generation, blog posts, social media
- Tier 3 (0.25-0.50 UVD): Predictive engine, recommendations
- Tier 4-6: Video editing, image generation, enterprise (up to 100 UVD)

**Mapping files to services:**
- `logs/YYYYMMDD/full.txt` → Chat Logs service (0.01 UVD)
- `transcripts/YYYYMMDD/{id}/transcripcion.json` → Raw Transcript (0.02 UVD)
- `transcripts/YYYYMMDD/{id}/ideas_extraidas.json` → Idea Extraction (1.20 UVD)
- `transcripts/YYYYMMDD/{id}/imagenes_generadas/` → Image Generation (0.80 UVD)

---

## Development Workflow

### Phase 1: Blockchain Infrastructure (Current)
```bash
# 1. Deploy contracts
cd erc-20 && ./deploy-fuji.sh && cd ..
cd erc-8004 && ./deploy-fuji.sh && cd ..

# 2. Save addresses to all agent .env files
# Copy IDENTITY_REGISTRY, REPUTATION_REGISTRY, VALIDATION_REGISTRY, UVD_TOKEN

# 3. Start facilitator
cd x402-rs && cargo run &
```

### Phase 2: Agent Development
```bash
# 1. Implement base_agent.py with ERC-8004 integration
# 2. Implement validator_agent.py (based on Bob from erc-8004-example)
# 3. Test registration: python scripts/register_validator.py
```

### Phase 3-4: Service Agents
```bash
# Implement sellers (server agents with x402-axum middleware)
# Implement buyers (client agents with x402-reqwest)
# Test with local data files first, then connect to production DBs
```

### Testing End-to-End
```bash
python demo.py  # runs complete flow simulation
```

---

## Key Technical Decisions

### Why EIP-3009 (not regular ERC-20)?
Agents can't hold ETH/AVAX for gas. EIP-3009 allows off-chain signing + relayer execution, making transactions gasless for agents.

### Why Fuji testnet?
Free, fast (2s blocks), EVM-compatible. Mainnet deployment requires audits.

### Why x402 protocol?
Standard HTTP 402 status code for payments. Stateless, works with existing HTTP clients, no custom payment channels needed.

### Why A2A protocol?
Standard for agent discovery and skill invocation. AgentCard at `/.well-known/agent-card` declares capabilities and pricing.

### Why CrewAI?
Multi-agent workflows for complex tasks (validation requires quality analyst + fraud detector + price reviewer working together).

### Why separate validator?
Trustless verification - buyers don't trust sellers' data quality. Independent validator with on-chain reputation provides credible verification.

---

## Common Issues

**"insufficient funds for gas"**
→ Get AVAX testnet from https://faucet.avax.network/

**"agent not found in registry"**
→ Run `python scripts/register_*.py` to register on-chain first

**"facilitator connection refused"**
→ Ensure x402-rs is running: `curl http://localhost:8080/health`

**"nonce already used"**
→ EIP-3009 uses random nonces - generate a new one (happens on signature replay)

**CrewAI timeouts**
→ Check OPENAI_API_KEY is valid, model is gpt-4o (not gpt-3.5)

---

## Documentation Map

- **MASTER_PLAN.md**: Complete vision, roadmap, all components explained
- **ARCHITECTURE.md**: Technical decisions, stack, data flows
- **MONETIZATION_OPPORTUNITIES.md**: All 50+ services with pricing
- **QUICKSTART.md**: 30-minute setup guide
- **Component READMEs**: Each folder has detailed README with code examples

**Start here**: Read `QUICKSTART.md` (30 min) → `MASTER_PLAN.md` (60 min) → component READMEs

---

## Windows-Specific Notes

This project is developed on Windows (Z: drive paths visible in code).

**PowerShell scripts available:**
- `erc-8004/deploy-fuji.ps1`
- `erc-8004/deploy-fuji.bat`

**Path handling:**
```python
# Use raw strings for Windows paths
logs_path = r"z:\ultravioleta\dao\karmacadabra\karma-hello-agent\logs"
# Or forward slashes work in Python
logs_path = "z:/ultravioleta/dao/karmacadabra/karma-hello-agent/logs"
```

**Virtual environments:**
```bash
python -m venv venv
venv\Scripts\activate  # Windows
source venv/bin/activate  # Linux/Mac
```
