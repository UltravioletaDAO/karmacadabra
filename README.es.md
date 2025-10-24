# 🎯 Karmacadabra: Economía de Agentes sin Confianza

> Agentes de IA que compran/venden datos de forma autónoma usando micropagos sin gas basados en blockchain

**🇪🇸 Versión en Español** | **[🇺🇸 English Version](./README.md)**

> **⚡ Importante:** Esto implementa una **versión EXTENDIDA de ERC-8004** con reputación bidireccional (¡NO la especificación base!) desplegada en **Avalanche** - el hogar de **Ultravioleta DAO**. Tanto compradores como vendedores se califican mutuamente después de las transacciones.

[![Avalanche](https://img.shields.io/badge/Avalanche-Fuji-E84142?logo=avalanche)](https://testnet.snowtrace.io/)
[![ERC-8004](https://img.shields.io/badge/ERC--8004%20Extended-Bidirectional%20Rating-blue)](https://eips.ethereum.org/EIPS/eip-8004)
[![x402](https://img.shields.io/badge/x402-Payment%20Protocol-green)](https://www.x402.org)
[![Python](https://img.shields.io/badge/Python-3.11+-blue?logo=python)](https://www.python.org/)
[![Rust](https://img.shields.io/badge/Rust-Latest-orange?logo=rust)](https://www.rust-lang.org/)
[![Desplegado](https://img.shields.io/badge/Desplegado-Fuji%20Testnet-success)](https://testnet.snowtrace.io/)

---

## 🚀 **EN VIVO EN FUJI TESTNET** - Desplegado 22 de Octubre 2025

| Contrato | Dirección | Estado |
|----------|-----------|--------|
| **Token GLUE (EIP-3009)** | [`0x3D19A80b3bD5CC3a4E55D4b5B753bC36d6A44743`](https://testnet.snowtrace.io/address/0x3D19A80b3bD5CC3a4E55D4b5B753bC36d6A44743) | ✅ Verificado |
| **TransactionLogger** | [`0x85ea82dDc0d3dDC4473AAAcc7E7514f4807fF654`](https://testnet.snowtrace.io/address/0x85ea82dDc0d3dDC4473AAAcc7E7514f4807fF654) | ✅ Verificado |
| **Registro de Identidad (ERC-8004)** | [`0xB0a405a7345599267CDC0dD16e8e07BAB1f9B618`](https://testnet.snowtrace.io/address/0xB0a405a7345599267CDC0dD16e8e07BAB1f9B618) | ✅ Verificado |
| **Registro de Reputación (ERC-8004)** | [`0x932d32194C7A47c0fe246C1d61caF244A4804C6a`](https://testnet.snowtrace.io/address/0x932d32194C7A47c0fe246C1d61caF244A4804C6a) | ✅ Verificado |
| **Registro de Validación (ERC-8004)** | [`0x9aF4590035C109859B4163fd8f2224b820d11bc2`](https://testnet.snowtrace.io/address/0x9aF4590035C109859B4163fd8f2224b820d11bc2) | ✅ Verificado |

**Red**: Avalanche Fuji Testnet (Chain ID: 43113)
**Tarifa de Registro**: 0.005 AVAX
**Suministro de Token**: 24,157,817 GLUE (6 decimales)

### Billeteras de Agentes (Financiadas con Tokens GLUE)

| Agente | Dirección de Billetera | Saldo GLUE | Dominio |
|--------|------------------------|------------|---------|
| **Client Agent** | [`0xCf30021812F27132d36dc791E0eC17f34B4eE8BA`](https://testnet.snowtrace.io/address/0xCf30021812F27132d36dc791E0eC17f34B4eE8BA) | 220,000 GLUE | `client.karmacadabra.ultravioletadao.xyz` |
| **Karma-Hello** | [`0x2C3e071df446B25B821F59425152838ae4931E75`](https://testnet.snowtrace.io/address/0x2C3e071df446B25B821F59425152838ae4931E75) | 55,000 GLUE | `karma-hello.karmacadabra.ultravioletadao.xyz` |
| **Abracadabra** | [`0x940DDDf6fB28E611b132FbBedbc4854CC7C22648`](https://testnet.snowtrace.io/address/0x940DDDf6fB28E611b132FbBedbc4854CC7C22648) | 55,000 GLUE | `abracadabra.karmacadabra.ultravioletadao.xyz` |
| **Validator** | [`0x1219eF9484BF7E40E6479141B32634623d37d507`](https://testnet.snowtrace.io/address/0x1219eF9484BF7E40E6479141B32634623d37d507) | 55,000 GLUE | `validator.karmacadabra.ultravioletadao.xyz` |
| **Voice-Extractor** | [`0xDd63D5840090B98D9EB86f2c31974f9d6c270b17`](https://testnet.snowtrace.io/address/0xDd63D5840090B98D9EB86f2c31974f9d6c270b17) | 110,000 GLUE | `voice-extractor.karmacadabra.ultravioletadao.xyz` |
| **Skill-Extractor** | [`0xC1d5f7478350eA6fb4ce68F4c3EA5FFA28C9eaD9`](https://testnet.snowtrace.io/address/0xC1d5f7478350eA6fb4ce68F4c3EA5FFA28C9eaD9) | 55,000 GLUE | `skill-extractor.karmacadabra.ultravioletadao.xyz` |

**Convención de Dominio**: Todos los agentes usan el formato `<nombre-agente>.karmacadabra.ultravioletadao.xyz` (registrado en la blockchain)

**Ver Todos los Contratos**: [Explorador Snowtrace](https://testnet.snowtrace.io/)


---

## 📦 Qué Está Implementado

### ✅ Fase 1: Infraestructura Blockchain (COMPLETA)

**Desplegado en Avalanche Fuji Testnet** - 22 de Octubre, 2025

Todos los contratos inteligentes están en vivo y verificados en Snowtrace. Las wallets de agentes han sido creadas y fondeadas con 55,000 GLUE cada una.

**Estado de Infraestructura:**
- ✅ Token GLUE (EIP-3009) desplegado y verificado
- ✅ Registros ERC-8004 Extendido (Identidad, Reputación, Validación) desplegados
- ✅ Contrato TransactionLogger desplegado
- ✅ 4 wallets de agentes creadas y fondeadas (Validator, Karma-Hello, Abracadabra, Client)
- ✅ AWS Secrets Manager configurado para gestión centralizada de claves
- ⚠️ Facilitador x402 (usando instancia externa)

### ✅ Sprint 1: Fundamentos (COMPLETO - Octubre 2025)

**Utilidades Compartidas en Python** (`shared/`) - **3,100+ líneas** de código listo para producción:

1. **`base_agent.py`** (600+ líneas) - Integración ERC-8004, sistema de reputación, Web3.py, AWS Secrets
2. **`payment_signer.py`** (470+ líneas) - Firma EIP-712, firmas de pago EIP-3009
3. **`x402_client.py`** (530+ líneas) - Cliente de protocolo de pago HTTP x402
4. **`a2a_protocol.py`** (650+ líneas) - Descubrimiento de agentes, AgentCard, Skills
5. **`validation_crew.py`** (550+ líneas) - Patrón de validación CrewAI (Calidad + Fraude + Precio)
6. **`tests/`** (1,200+ líneas) - **26 tests unitarios pasando** + framework de tests de integración

**Testing:** Tests unitarios pasando, tests de integración listos para wallets fondeadas

**Documentación:** 
- Referencia API: [`shared/README.md`](./shared/README.md)
- Guía de Testing: [`shared/tests/README.md`](./shared/tests/README.md)
- Arquitectura: [`ARCHITECTURE.md`](./ARCHITECTURE.md)

### ✅ Fase 2: Desarrollo de Agentes (COMPLETA - Octubre 2025)

**Sprint 2: Agentes del Sistema** - Infraestructura del mercado completa (7/7 hitos)

#### ✅ Sprint 2.1: Agente Validador (COMPLETO)
**Servicio independiente de verificación de calidad de datos** - `validator/` - **1,545+ líneas**

**Qué hace:**
- Validación multi-agente usando CrewAI (9 agentes de IA en 3 crews)
- Análisis de calidad + detección de fraude + revisión de precios
- Envío on-chain de puntuación de validación (validador paga el gas)
- Servidor FastAPI con soporte para protocolo A2A

**Archivos clave:**
1. `main.py` (470+ líneas) - Clase ValidatorAgent, servidor FastAPI
2. `crews/quality_crew.py` (200+ líneas) - Crew de validación de calidad
3. `crews/fraud_crew.py` (200+ líneas) - Crew de detección de fraude
4. `crews/price_crew.py` (200+ líneas) - Crew de equidad de precios
5. `test_validator.py` (280+ líneas) - Suite de pruebas con modos --quick, --live, --crews
6. `README.md` (330+ líneas) - Documentación completa
7. `TESTING.md` (280+ líneas) - Guía de pruebas

**Pruebas:**
```bash
cd validator
python test_validator.py --quick  # Prueba simulada (sin dependencias)
python test_validator.py --live   # Prueba con validador ejecutándose
```

**Despliegue:**
- Wallet: `0x1219eF9484BF7E40E6479141B32634623d37d507`
- Balance: 55,000 GLUE
- Tarifa de validación: 0.001 GLUE por validación
- Puerto: 8001

#### ✅ Sprint 2.2: Agente Cliente (COMPLETO)
**Agente comprador genérico para el mercado** - `client-agent/` - **440+ líneas**

**Qué hace:**
- Descubre vendedores vía protocolo A2A (`/.well-known/agent-card`)
- Solicita validación antes de compras
- Maneja flujo de pago x402 con autorizaciones firmadas
- Guarda datos comprados en estructura de directorios organizada

**Archivos clave:**
1. `main.py` (170+ líneas) - Clase ClientAgent con discover/validate/purchase
2. `.env.example` (40+ líneas) - Plantilla de configuración
3. `README.md` (230+ líneas) - Documentación de uso

**Métodos clave:**
- `discover_seller()` - Descubrimiento por protocolo A2A
- `request_validation()` - Integración con validador
- `save_data()` - Almacenamiento de datos con metadatos

**Despliegue:**
- Wallet: `0xCf30021812F27132d36dc791E0eC17f34B4eE8BA`
- Balance: 55,000 GLUE
- Precio máximo: 1.0 GLUE (configurable)
- Puntuación mínima de validación: 0.7 (configurable)

#### ✅ Sprint 2.3: Integración de Datos (COMPLETO)
**Archivos de datos de muestra para pruebas** - `data/` - **495+ líneas**

**Qué proporciona:**
- Logs de chat realistas del vendedor Karma-Hello
- Transcripciones realistas del vendedor Abracadabra
- Ambos archivos diseñados para pasar validación (puntuaciones 0.8-0.9)

**Archivos clave:**
1. `karma-hello/chat_logs_20251023.json` - 156 mensajes, 23 usuarios, stream de 2 horas
2. `abracadabra/transcription_20251023.json` - 15 segmentos, transcripción de 2 horas
3. `README.md` (300+ líneas) - Documentación de formato de datos

**Estadísticas de datos de muestra:**
- Logs de chat: 156 mensajes de 23 usuarios únicos
- Transcripción: 15 segmentos, 7200 segundos (2 horas)
- Ambos comparten `stream_id: stream_20251023_001` para pruebas de datos complementarios

#### ✅ Sprint 2.4: Agente Karma-Hello (COMPLETO)
**Vendedor + comprador de logs de chat de Twitch** - `karma-hello-agent/` - **720+ líneas**

**Qué hace:**
- **VENDE** logs de chat desde MongoDB vía protocolo x402
- **COMPRA** transcripciones del agente Abracadabra
- Servidor FastAPI con soporte de protocolo A2A
- Respaldo de archivos locales para pruebas
- Precios multi-nivel (0.01-200 GLUE)

**Características clave:**
- Integración con MongoDB para logs de producción
- Capacidades de comprador añadidas (compra transcripciones)
- Almacenamiento: directorio `purchased_transcriptions/`
- Puerto: 8002

**Deployment:**
- Billetera: `0x2C3e071df446B25B821F59425152838ae4931E75`
- Saldo: 55,000 GLUE

#### ✅ Sprint 2.5: Agente Abracadabra (COMPLETO)
**Vendedor + comprador de transcripciones de streams** - `abracadabra-agent/` - **720+ líneas**

**Qué hace:**
- **VENDE** transcripciones desde SQLite+Cognee vía protocolo x402
- **COMPRA** logs de chat del agente Karma-Hello
- Servidor FastAPI con soporte de protocolo A2A
- Precios multi-nivel (0.02-300 GLUE)

**Características clave:**
- Integración con SQLite para almacenamiento de transcripciones
- Capacidades de comprador añadidas (compra logs de chat)
- Almacenamiento: directorio `purchased_chat_logs/`
- Puerto: 8003

**Deployment:**
- Billetera: `0x940DDDf6fB28E611b132FbBedbc4854CC7C22648`
- Saldo: 55,000 GLUE

#### ✅ Sprint 2.6: Agente Voice-Extractor (COMPLETO)
**Perfilador de personalidad lingüística** - `voice-extractor-agent/` - **523+ líneas**

**Qué hace:**
- **COMPRA** logs de chat de Karma-Hello (0.01 GLUE)
- **VENDE** perfiles de personalidad lingüística (0.02-0.40 GLUE)
- Análisis psicolingüístico de 8 categorías
- Precios multi-nivel (basic, standard, complete, enterprise)

**Características clave:**
- Análisis basado en CrewAI (simplificado para MVP)
- Descubrimiento vía protocolo A2A
- Integración con protocolo de pago x402
- Puerto: 8005

**Modelo Económico:**
- Beneficio neto: 0.03 GLUE por extracción (margen del 300%)
- Saldo inicial: 55,000 GLUE (presupuestado)

#### ✅ Sprint 2.7: Agente Skill-Extractor (COMPLETO)
**Perfilador de habilidades y competencias** - `skill-extractor-agent/` - **790+ líneas**

**Qué hace:**
- **COMPRA** logs de chat de Karma-Hello (0.01 GLUE)
- **VENDE** perfiles de habilidades/competencias (0.02-0.50 GLUE)
- Marco de extracción de 5 categorías
- Precios multi-nivel (basic, standard, complete, enterprise)

**Características clave:**
- Extracción de intereses + identificación de habilidades
- Auditoría de herramientas/plataformas + análisis de monetización
- Análisis multi-agente basado en CrewAI
- Puerto: 8085

**5 Categorías de Análisis:**
1. Extracción de Intereses (temas recurrentes, profundidad de engagement)
2. Identificación de Habilidades y Sub-habilidades (jerarquía de 2 niveles)
3. Herramientas y Plataformas (auditoría completa de stack tecnológico)
4. Estilo de Interacción (patrones de colaboración)
5. Potencial de Monetización (análisis de viabilidad comercial)

**Modelo Económico:**
- Beneficio neto: 0.04 GLUE por extracción (margen del 400%)
- Saldo inicial: 55,000 GLUE (presupuestado)

**Progreso:** ✅ **7 de 7 hitos completados (100%)** - SPRINT 2 COMPLETO


---

## 🎯 ¿Qué es Karmacadabra?

**Karmacadabra** es un ecosistema de agentes de IA autónomos que **compran y venden datos** sin intervención humana utilizando:

- **ERC-8004 Extendido** - **¡NO la implementación base!** Esta es una extensión personalizada que habilita **reputación bidireccional** (tanto compradores como vendedores se califican entre sí)
- **Protocolo A2A** (Pydantic AI) para comunicación entre agentes
- **x402 + EIP-3009** para micropagos HTTP (¡sin gas!)
- **CrewAI** para orquestación de múltiples agentes

### 🏔️ Desplegado en Avalanche - Nuestro Hogar

**Karmacadabra vive en Avalanche**, el hogar nativo de blockchain de **Ultravioleta DAO**. Elegimos Avalanche por:

- **Finalidad rápida**: Tiempos de bloque de 2 segundos para transacciones instantáneas de agentes
- **Costos bajos**: Tarifas de gas mínimas hacen que los micropagos sean económicamente viables
- **Compatibilidad EVM**: Soporte completo de Solidity con herramientas de Ethereum
- **Alineación con DAO**: Avalanche es donde Ultravioleta DAO nació y prospera

Actualmente en **testnet Fuji**, con despliegue en mainnet planificado después de auditorías.

### El Problema que Resolvemos

**Karma-Hello** tiene registros ricos de chat de Twitch pero sin contexto de audio.
**Abracadabra** tiene transcripciones de streams pero sin datos de chat.

**Solución**: Los agentes negocian y compran de forma autónoma datos complementarios, construyendo un contexto completo de streaming. Todas las transacciones son verificadas, on-chain, y sin gas.

---

## 🚀 Inicio Rápido (30 minutos)

**✨ ¡Contratos ya desplegados!** Puedes empezar a construir agentes de inmediato.

```bash
# 1. Clonar repositorio
git clone https://github.com/ultravioletadao/karmacadabra.git
cd karmacadabra

# 2. Obtener AVAX de testnet
# Visitar: https://faucet.avax.network/

# 3. Configurar entorno
cd validator
cp .env.example .env
# Agregar tus claves:
# - PRIVATE_KEY (para tu wallet de prueba)
# - OPENAI_API_KEY (para CrewAI)
# - ¡Las direcciones de contratos ya están configuradas!

# 4. Instalar dependencias
pip install -r requirements.txt

# 5. Ejecutar agente validador
python main.py
```

**Contratos Desplegados**: ¡Todos los registros ERC-8004 están en vivo en Fuji!
**Guía completa**: Ver [QUICKSTART.md](./QUICKSTART.md)

---

## 🔐 AWS Secrets Manager (Seguridad)

⚠️ **POLÍTICA DE SEGURIDAD CRÍTICA**: Las claves privadas **NUNCA se almacenan en archivos `.env`**. Todas las claves deben estar en AWS Secrets Manager.

**Por qué AWS Secrets Manager:**
- ✅ Almacenamiento seguro centralizado para todas las claves privadas de agentes
- ✅ Sin claves en repositorios git (incluso archivos de ejemplo tienen `PRIVATE_KEY=` vacío)
- ✅ Soporte de rotación automática vía `rotate-system.py`
- ✅ Registro de auditoría de quién accedió a las claves y cuándo
- ✅ Seguro para transmisiones públicas en vivo (sin exposición accidental de claves)

### Configuración Rápida

```bash
# 1. Configurar AWS CLI (una sola vez)
aws configure
# Ingresar AWS Access Key ID, Secret Access Key, región (us-east-1)

# 2. Almacenar todas las claves en AWS Secrets Manager
python scripts/setup-secrets.py
# Crea el secreto 'karmacadabra' con todas las claves privadas

# 3. Limpiar archivos .env locales (REQUERIDO)
python scripts/clear-env-keys.py
# Establece PRIVATE_KEY= (vacío) en todos los archivos .env

# 4. Probar recuperación
python -m shared.secrets_manager validator-agent
# [AWS Secrets] Retrieved key for 'validator-agent' from AWS
```

**Estructura de Almacenamiento de Claves:**
```json
{
  "erc-20": {"private_key": "0x..."},
  "client-agent": {"private_key": "0x..."},
  "karma-hello-agent": {"private_key": "0x..."},
  "abracadabra-agent": {"private_key": "0x..."},
  "validator-agent": {"private_key": "0x..."},
  "voice-extractor-agent": {"private_key": "0x..."},
  "skill-extractor-agent": {"private_key": "0x..."}
}
```

**Notas Importantes:**
- Clave del deployer ERC-20 almacenada por separado (posee el contrato del token GLUE)
- Rotar clave ERC-20 solo cuando sea necesario: `python rotate-system.py --rotate-erc20`
- Todos los archivos `.env` deben tener `PRIVATE_KEY=` (vacío) - scripts obtienen automáticamente de AWS

**Guía completa**: Ver [shared/AWS_SECRETS_SETUP.md](./shared/AWS_SECRETS_SETUP.md)

---

## 🏗️ Arquitectura

```
┌──────────────────────────────────────────────────────────────────┐
│   AVALANCHE FUJI TESTNET (Nuestro Hogar - Capa 1)               │
│  ┌──────────────────┐    ┌─────────────────────────────────┐    │
│  │  Token GLUE      │    │ ERC-8004 EXTENDIDO              │    │
│  │  (EIP-3009)      │    │  (¡Bidireccional!)              │    │
│  │  Txs sin gas ✓   │    │  • Registro Identidad           │    │
│  └──────────────────┘    │  • Registro Reputación          │    │
│                          │  • Registro Validación          │    │
│                          │    ❗Validador escribe aquí❗   │    │
│                          └─────────────┬───────────────────┘    │
│                                        │ validationResponse()   │
└────────────────────────────────────────┼────────────────────────┘
                          ▲              │ (¡Validador paga gas!)
                          │              ▼
┌─────────────────────────┴─────────┬────────────────────────────┐
│   Facilitador x402 (Rust)         │   Agente Validador (Python)│
│   • Verifica firmas EIP-712       │   • Escucha solicitudes    │
│   • Ejecuta transferWith...()     │   • CrewAI valida datos    │
│   • Sin estado (sin BD)           │   • Paga ~0.01 AVAX gas    │
└───────────┬────────────────────────┴────────────────────────────┘
            │
            ▲                            ▲
┌───────────┴────────┐      ┌───────────┴────────┐
│ Agente Karma-Hello │      │ Agente Abracadabra │
│ • Vende: Logs chat │◄────►│ • Vende: Transcripc│
│ • Compra: Transcr. │      │ • Compra: Logs chat│
│ • Precio: 0.01 GLUE│      │ • Precio: 0.02 GLUE│
│ • Datos: MongoDB   │      │ • Datos: SQLite    │
│ • Gas: 0 (sin gas!)│      │ • Gas: 0 (sin gas!)│
└────────────────────┘      └─────────────────────┘
            ▲                            ▲
            └────────┬───────────────────┘
                     ▼
         ┌────────────────────┐
         │  Agente Validador  │
         │  • Crew de CrewAI  │
         │  • Puntaje calidad │
         │  • Tarifa: 0.001   │
         └────────────────────┘
```

---

## 💰 ¿Qué se Puede Monetizar?

### Servicios de Karma-Hello (20+ productos)
- **Nivel 1** (0.01 GLUE): Logs de chat, actividad de usuarios
- **Nivel 2** (0.10 GLUE): Predicciones ML, análisis de sentimiento
- **Nivel 3** (0.20 GLUE): Detección de fraude, salud económica
- **Empresarial** (hasta 200 GLUE): Marca blanca, modelos personalizados

### Servicios de Abracadabra (30+ productos)
- **Nivel 1** (0.02 GLUE): Transcripciones crudas, mejoradas
- **Nivel 2** (0.15 GLUE): Generación de clips, posts de blog
- **Nivel 3** (0.35 GLUE): Motor predictivo, recomendaciones
- **Nivel 4** (1.50 GLUE): Edición automática de video, generación de imágenes
- **Empresarial** (hasta 100 GLUE): Modelos de IA personalizados

**Catálogo completo**: [MONETIZATION_OPPORTUNITIES.md](./MONETIZATION_OPPORTUNITIES.md)

---

## 📂 Estructura del Repositorio

```
karmacadabra/
├── erc-20/                    # Token GLUE (EIP-3009)
├── erc-8004/                  # ERC-8004 Extendido - Registros de reputación bidireccional
├── x402-rs/                   # Facilitador de pagos (Rust)
├── validator/                 # Agente validador (Python + CrewAI)
├── karma-hello-agent/         # Agentes vendedor/comprador de logs de chat
├── abracadabra-agent/         # Agentes vendedor/comprador de transcripciones
├── client-agent/              # Agente comprador genérico
├── voice-extractor-agent/     # Agente de análisis lingüístico
├── skill-extractor-agent/     # Agente de perfilado de habilidades
├── shared/                    # Utilidades compartidas (AWS Secrets Manager, etc.)
├── scripts/                   # Scripts de configuración y despliegue
├── MASTER_PLAN.md            # Visión completa y hoja de ruta
├── ARCHITECTURE.md           # Arquitectura técnica
├── TRANSACTION_LOGGING.md    # Sistema de logging en blockchain
├── MONETIZATION_OPPORTUNITIES.md
├── QUICKSTART.md             # Guía de configuración de 30 min
├── CLAUDE.md                 # Guía para Claude Code
└── INDEX.md                  # Índice de documentación
```

---

## 🛠️ Stack Tecnológico

| Capa | Tecnología | Propósito |
|-------|-----------|---------|
| **Blockchain** | Avalanche Fuji | Testnet EVM para contratos inteligentes |
| **Contratos** | Solidity + Foundry | Registros ERC-8004 + token GLUE |
| **Facilitador** | Rust (Axum) | Verificación de pagos x402 |
| **Agentes** | Python 3.11+ | Runtime de agentes de IA |
| **Framework IA** | CrewAI | Orquestación multi-agente |
| **LLM** | GPT-4o | Análisis y validación |
| **Web3** | web3.py + ethers-rs | Interacción con blockchain |
| **Datos** | MongoDB + SQLite + Cognee | Fuentes de datos de agentes |

---

## 🎯 Características Clave

✅ **Micropagos sin Gas**: Los agentes no necesitan ETH/AVAX para gas
✅ **Reputación Bidireccional**: Extensión personalizada de ERC-8004 - compradores Y vendedores se califican entre sí (¡no está en la especificación base!)
✅ **Nativo de Avalanche**: Desplegado en nuestra cadena de origen para rendimiento óptimo
✅ **Validación sin Confianza**: Validadores independientes verifican la calidad de los datos
✅ **Descubrimiento de Agentes**: AgentCards del protocolo A2A en `/.well-known/agent-card`
✅ **Flujos Multi-Agente**: Crews de CrewAI para tareas complejas
✅ **Logging en Blockchain**: Todas las transacciones logueadas con mensajes UTF-8 permanentes en Snowtrace
✅ **50+ Servicios Monetizables**: Desde $0.01 hasta $200 GLUE por servicio

---

## 📚 Documentación

| Documento | Descripción | Tiempo |
|----------|-------------|------|
| [QUICKSTART.md](./QUICKSTART.md) | Funcionando en 30 minutos | 30 min |
| [MASTER_PLAN.md](./MASTER_PLAN.md) | Visión completa y hoja de ruta | 60 min |
| [ARCHITECTURE.md](./ARCHITECTURE.md) | Profundización técnica | 45 min |
| [MONETIZATION_OPPORTUNITIES.md](./MONETIZATION_OPPORTUNITIES.md) | Todos los servicios y precios | 30 min |
| [CLAUDE.md](./CLAUDE.md) | Guía para desarrolladores | 15 min |
| [INDEX.md](./INDEX.md) | Índice de documentación | 5 min |

**READMEs de Componentes**: Cada carpeta tiene instrucciones detalladas de configuración.

---

## 🧪 Estado del Desarrollo

| Fase | Componente | Estado |
|-------|-----------|--------|
| **Fase 1** | Registros ERC-8004 Extendidos | ✅ **DESPLEGADO Y VERIFICADO** |
| **Fase 1** | Token GLUE | ✅ **DESPLEGADO Y VERIFICADO** |
| **Fase 1** | TransactionLogger | ✅ **DESPLEGADO Y VERIFICADO** |
| **Fase 1** | Distribución de Tokens | ✅ **COMPLETO** (55,000 GLUE a cada agente) |
| **Fase 1** | Facilitador x402 | ⏸️ Listo (requiere Rust nightly - usando facilitador externo) |
| **Fase 2** | Agente Validador | 🔄 **EN PROGRESO** |
| **Fase 3** | Agentes Karma-Hello | 🔴 Por implementar |
| **Fase 4** | Agentes Abracadabra | 🔴 Por implementar |
| **Fase 5** | Pruebas de Extremo a Extremo | 🔴 Pendiente |

**Fase Actual**: Fase 2 - Implementando agentes Python
**Última Actualización**: 23 de Octubre 2025

---

## 🧰 Caja de Herramientas para Desarrolladores

Scripts de utilidad para gestionar wallets, tokens y despliegues de agentes:

### Generador de Wallets
Genera nuevas wallets compatibles con EVM para agentes:

```bash
# Generar wallet y auto-guardar en .env
python generate-wallet.py client-agent --auto-save

# Generar para múltiples agentes
python generate-wallet.py client-agent-2 --auto-save
python generate-wallet.py validator-2 --auto-save

# Modo interactivo (pregunta antes de guardar)
python generate-wallet.py mi-agente
```

**Características**:
- Crea wallets compatibles con Ethereum (funciona en todas las cadenas EVM)
- Auto-guarda clave privada y dirección en archivo `.env` del agente
- Muestra advertencias de seguridad y mejores prácticas
- Muestra información de testnet Fuji y próximos pasos
- Reutilizable para agentes ilimitados

### Distribuidor de Tokens
Distribuye tokens (GLUE/UVD/etc.) a las wallets de los agentes:

```bash
cd erc-20
python distribute-token.py
```

**Características**:
- Carga automáticamente direcciones de wallet desde archivos `.env` de agentes
- Cantidades configurables (actualmente 55,000 GLUE para todos)
- Soporta cualquier token ERC-20 (configurar via UVD_TOKEN_ADDRESS en .env)
- Muestra saldos antes/después
- Enlaces de transacciones en Snowtrace
- Soporta: validator, karma-hello-agent, abracadabra-agent, client-agent

**Agentes**:
| Agente | Financiado | Saldo |
|--------|------------|-------|
| Validator | ✅ | 55,000 GLUE |
| Karma-Hello | ✅ | 55,000 GLUE |
| Abracadabra | ✅ | 55,000 GLUE |
| Client-Agent | ✅ | 55,000 GLUE |

### Rotación del Sistema (Seguridad)
Rotación completa de infraestructura para escenarios de compromiso de claves:

```bash
# Ejecución en seco (muestra qué pasaría, sin hacer cambios)
python rotate-system.py

# Ejecutar rotación real (requiere confirmación)
python rotate-system.py --confirm

# Rellenar wallets con GLUE solamente (sin rotación)
python rotate-system.py --refill

# Rellenar wallets con GLUE (ejecutar)
python rotate-system.py --refill --confirm

# Rotar wallet del deployer ERC-20 solamente (separado de la rotación de agentes)
python rotate-system.py --rotate-erc20 --confirm
```

**Qué hace**:
1. 🔑 Genera nuevas wallets para TODOS los agentes (validator, karma-hello, abracadabra, client, voice-extractor, skill-extractor)
2. ☁️ Actualiza AWS Secrets Manager con nuevas claves privadas
3. 📜 Redespliega el contrato ERC-20 GLUE token
4. 🏛️ Redespliega registros ERC-8004 (Identity, Reputation, Validation)
5. 📝 Actualiza todos los archivos `.env` de agentes con nuevas direcciones de contratos
6. 💰 Fondea todas las wallets con AVAX de testnet (paso manual)
7. 🎁 Distribuye tokens GLUE a todos los agentes
8. 📄 Actualiza MASTER_PLAN.md con nuevas direcciones de contratos y wallets

**Casos de uso**:
- 🚨 **Compromiso de claves**: Rotar inmediatamente si claves privadas fueron expuestas
- 🔄 **Reinicio limpio**: Comenzar de cero con nueva infraestructura
- 🧪 **Pruebas**: Validar automatización de despliegue
- 🎥 **Post-stream**: Rotar claves después de demostraciones públicas

**Seguridad**:
- ⚠️ Requiere flag `--confirm` para ejecutar (por defecto modo dry-run)
- ⚠️ Requiere escribir 'ROTATE' para confirmar cambios destructivos
- ⚠️ Invalida TODAS las wallets y contratos existentes
- ✅ Modo dry-run seguro muestra exactamente qué pasaría

**Fondeo de Gas**:
- 💰 **¿Necesitas AVAX para gas?** Usa la wallet del deployer ERC-20 (almacenada en AWS Secrets Manager)
- 🔑 Acceso vía `distribute-token.py` (obtiene automáticamente desde AWS)
- ⚠️ **El deployer ERC-20 NO se rota por defecto** (es dueño del contrato GLUE)
- 🔄 Rotar separadamente: `python rotate-system.py --rotate-erc20 --confirm`

**Ejemplo de salida**:
```
STEP 1: Generating New Wallets
✓ validator-agent          -> 0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb9
✓ karma-hello-agent        -> 0x95cED938F7991cd0dFcb48F0a06a40FA1aF46EBC
...

STEP 2: Updating AWS Secrets Manager
✓ Successfully updated AWS Secrets Manager

STEP 3: Deploying ERC-20 GLUE Token
✓ GLUE Token deployed: 0x1234...

... (continúa por los 8 pasos)
```

---

## 🔧 Requisitos

- **Python** 3.11+
- **Rust** última versión estable
- **Foundry** (forge, anvil, cast)
- **Node.js** 18+ (opcional, para frontend)
- **AVAX** en testnet Fuji (gratis desde faucet)
- **Clave API de OpenAI** (para agentes CrewAI)

---

## 🚦 Comenzando

### 1. Prerequisitos
```bash
# Instalar Foundry
curl -L https://foundry.paradigm.xyz | bash
foundryup

# Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Python 3.11+
python --version  # Debe ser 3.11 o superior
```

### 2. Obtener AVAX de Testnet
Visita https://faucet.avax.network/ y solicita AVAX para tu wallet.

### 3. Desplegar Infraestructura
```bash
cd erc-20
cp .env.example .env
# Editar .env con tu PRIVATE_KEY
./deploy-fuji.sh

cd ../erc-8004
./deploy-fuji.sh

cd ../x402-rs
cargo build --release
cargo run
```

### 4. Ejecutar Demo
```bash
python demo.py
```

Ver [QUICKSTART.md](./QUICKSTART.md) para instrucciones detalladas.

---

## 🤝 Contribuir

1. Leer [MASTER_PLAN.md](./MASTER_PLAN.md) para entender la visión
2. Revisar la hoja de ruta para tareas disponibles
3. Implementar siguiendo la arquitectura en [ARCHITECTURE.md](./ARCHITECTURE.md)
4. Escribir pruebas para todo el código nuevo
5. Enviar PR con documentación

---

## 📖 Aprender Más

- **Especificación Base ERC-8004**: https://eips.ethereum.org/EIPS/eip-8004 (¡nosotros extendemos esto con calificaciones bidireccionales!)
- **Protocolo A2A**: https://ai.pydantic.dev/a2a/
- **Protocolo x402**: https://www.x402.org
- **EIP-3009**: https://eips.ethereum.org/EIPS/eip-3009
- **CrewAI**: https://docs.crewai.com/
- **Documentación Avalanche**: https://docs.avax.network/ (¡nuestra cadena de origen!)

### Curso de Agentes sin Confianza
https://intensivecolearn.ing/en/programs/trustless-agents

---

## ⚠️ Descargo de Responsabilidad

**SOLO TESTNET**: Este proyecto está actualmente desplegado en testnet Fuji de Avalanche. No usar con fondos reales. Los contratos inteligentes no han sido auditados.

Para despliegue en mainnet:
- [ ] Auditoría de contratos inteligentes por firma reputada
- [ ] Programa de bug bounty
- [ ] Timelock para funciones de administración
- [ ] Multi-sig para propiedad de contratos

---

## 📄 Licencia

Licencia MIT - Ver [LICENSE](./LICENSE)

---

## 🌟 Agradecimientos

- **[Curso Trustless Agents](https://intensivecolearn.ing/en/programs/trustless-agents)** por Intensive CoLearning
- **Especificación Base ERC-8004** (que extendimos para reputación bidireccional)
- **x402-rs** implementación del protocolo
- **Pydantic AI** protocolo A2A
- **Avalanche** - nuestra blockchain de origen y la fundación de Ultravioleta DAO

---

## 💬 Contacto

- **Proyecto**: Ultravioleta DAO
- **Repositorio**: https://github.com/ultravioletadao/karmacadabra
- **Documentación**: Comenzar con [QUICKSTART.md](./QUICKSTART.md)

---

**Construido con ❤️ por Ultravioleta DAO**

*Empoderando agentes de IA autónomos para crear una economía de datos sin confianza*
