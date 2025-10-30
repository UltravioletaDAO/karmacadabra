//! Network definitions and known token deployments.
//!
//! This module defines supported networks and their chain IDs,
//! and provides statically known USDC deployments per network.

use crate::types::{MixedAddress, TokenAsset, TokenDeployment, TokenDeploymentEip712};
use alloy::primitives::address;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use std::borrow::Borrow;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::str::FromStr;

/// Supported Ethereum-compatible networks.
///
/// Used to differentiate between testnet and mainnet environments for the x402 protocol.
#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Network {
    /// Base Sepolia testnet (chain ID 84532).
    #[serde(rename = "base-sepolia")]
    BaseSepolia,
    /// Base mainnet (chain ID 8453).
    #[serde(rename = "base")]
    Base,
    /// XDC mainnet (chain ID 50).
    #[serde(rename = "xdc")]
    XdcMainnet,
    /// Avalanche Fuji testnet (chain ID 43113)
    #[serde(rename = "avalanche-fuji")]
    AvalancheFuji,
    /// Avalanche Mainnet (chain ID 43114)
    #[serde(rename = "avalanche")]
    Avalanche,
    /// Solana Mainnet - Live production environment for deployed applications
    #[serde(rename = "solana")]
    Solana,
    /// Solana Devnet - Testing with public accessibility for developers experimenting with their applications
    #[serde(rename = "solana-devnet")]
    SolanaDevnet,
    /// Polygon Amoy testnet (chain ID 80002).
    #[serde(rename = "polygon-amoy")]
    PolygonAmoy,
    /// Polygon mainnet (chain ID 137).
    #[serde(rename = "polygon")]
    Polygon,
    /// Sei mainnet (chain ID 1329).
    #[serde(rename = "sei")]
    Sei,
    /// Sei testnet (chain ID 1328).
    #[serde(rename = "sei-testnet")]
    SeiTestnet,
    /// Celo mainnet (chain ID 42220).
    #[serde(rename = "celo")]
    Celo,
    /// Celo Sepolia testnet (chain ID 44787).
    #[serde(rename = "celo-sepolia")]
    CeloSepolia,
    /// HyperEVM mainnet (chain ID 998).
    #[serde(rename = "hyperevm")]
    HyperEvm,
    /// HyperEVM testnet (chain ID 333).
    #[serde(rename = "hyperevm-testnet")]
    HyperEvmTestnet,
    /// Optimism mainnet (chain ID 10).
    #[serde(rename = "optimism")]
    Optimism,
    /// Optimism Sepolia testnet (chain ID 11155420).
    #[serde(rename = "optimism-sepolia")]
    OptimismSepolia,
}

impl Display for Network {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Network::BaseSepolia => write!(f, "base-sepolia"),
            Network::Base => write!(f, "base"),
            Network::XdcMainnet => write!(f, "xdc"),
            Network::AvalancheFuji => write!(f, "avalanche-fuji"),
            Network::Avalanche => write!(f, "avalanche"),
            Network::Solana => write!(f, "solana"),
            Network::SolanaDevnet => write!(f, "solana-devnet"),
            Network::PolygonAmoy => write!(f, "polygon-amoy"),
            Network::Polygon => write!(f, "polygon"),
            Network::Sei => write!(f, "sei"),
            Network::SeiTestnet => write!(f, "sei-testnet"),
            Network::Celo => write!(f, "celo"),
            Network::CeloSepolia => write!(f, "celo-sepolia"),
            Network::HyperEvm => write!(f, "hyperevm"),
            Network::HyperEvmTestnet => write!(f, "hyperevm-testnet"),
            Network::Optimism => write!(f, "optimism"),
            Network::OptimismSepolia => write!(f, "optimism-sepolia"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum NetworkFamily {
    Evm,
    Solana,
}

impl From<Network> for NetworkFamily {
    fn from(value: Network) -> Self {
        match value {
            Network::BaseSepolia => NetworkFamily::Evm,
            Network::Base => NetworkFamily::Evm,
            Network::XdcMainnet => NetworkFamily::Evm,
            Network::AvalancheFuji => NetworkFamily::Evm,
            Network::Avalanche => NetworkFamily::Evm,
            Network::Solana => NetworkFamily::Solana,
            Network::SolanaDevnet => NetworkFamily::Solana,
            Network::PolygonAmoy => NetworkFamily::Evm,
            Network::Polygon => NetworkFamily::Evm,
            Network::Sei => NetworkFamily::Evm,
            Network::SeiTestnet => NetworkFamily::Evm,
            Network::Celo => NetworkFamily::Evm,
            Network::CeloSepolia => NetworkFamily::Evm,
            Network::HyperEvm => NetworkFamily::Evm,
            Network::HyperEvmTestnet => NetworkFamily::Evm,
            Network::Optimism => NetworkFamily::Evm,
            Network::OptimismSepolia => NetworkFamily::Evm,
        }
    }
}

impl Network {
    /// Return all known [`Network`] variants.
    pub fn variants() -> &'static [Network] {
        &[
            Network::BaseSepolia,
            Network::Base,
            Network::XdcMainnet,
            Network::AvalancheFuji,
            Network::Avalanche,
            Network::Solana,
            Network::SolanaDevnet,
            Network::PolygonAmoy,
            Network::Polygon,
            Network::Sei,
            Network::SeiTestnet,
            Network::Celo,
            Network::CeloSepolia,
            Network::HyperEvm,
            Network::HyperEvmTestnet,
            Network::Optimism,
            Network::OptimismSepolia,
    /// Optimism mainnet (chain ID 10).
    #[serde(rename = "optimism")]
    Optimism,
    /// Optimism Sepolia testnet (chain ID 11155420).
    #[serde(rename = "optimism-sepolia")]
    OptimismSepolia,
        ]
    }
}

/// Lazily initialized known USDC deployment on Base Sepolia as [`USDCDeployment`].
static USDC_BASE_SEPOLIA: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: address!("0x036CbD53842c5426634e7929541eC2318f3dCF7e").into(),
            network: Network::BaseSepolia,
        },
        decimals: 6,
        eip712: Some(TokenDeploymentEip712 {
            name: "USDC".into(),
            version: "2".into(),
        }),
    })
});

/// Lazily initialized known USDC deployment on Base mainnet as [`USDCDeployment`].
static USDC_BASE: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: address!("0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913").into(),
            network: Network::Base,
        },
        decimals: 6,
        eip712: Some(TokenDeploymentEip712 {
            name: "USD Coin".into(),
            version: "2".into(),
        }),
    })
});

/// Lazily initialized known USDC deployment on XDC mainnet as [`USDCDeployment`].
static USDC_XDC: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: address!("0x2A8E898b6242355c290E1f4Fc966b8788729A4D4").into(),
            network: Network::XdcMainnet,
        },
        decimals: 6,
        eip712: Some(TokenDeploymentEip712 {
            name: "Bridged USDC(XDC)".into(),
            version: "2".into(),
        }),
    })
});

/// Lazily initialized known USDC deployment on Avalanche Fuji testnet as [`USDCDeployment`].
static USDC_AVALANCHE_FUJI: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: address!("0x5425890298aed601595a70AB815c96711a31Bc65").into(),
            network: Network::AvalancheFuji,
        },
        decimals: 6,
        eip712: Some(TokenDeploymentEip712 {
            name: "USD Coin".into(),
            version: "2".into(),
        }),
    })
});

/// Lazily initialized known USDC deployment on Avalanche Fuji testnet as [`USDCDeployment`].
static USDC_AVALANCHE: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: address!("0xB97EF9Ef8734C71904D8002F8b6Bc66Dd9c48a6E").into(),
            network: Network::Avalanche,
        },
        decimals: 6,
        eip712: Some(TokenDeploymentEip712 {
            name: "USD Coin".into(),
            version: "2".into(),
        }),
    })
});

/// Lazily initialized known USDC deployment on Solana mainnet as [`USDCDeployment`].
static USDC_SOLANA: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: MixedAddress::Solana(
                Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap(),
            ),
            network: Network::Solana,
        },
        decimals: 6,
        eip712: None,
    })
});

/// Lazily initialized known USDC deployment on Solana mainnet as [`USDCDeployment`].
static USDC_SOLANA_DEVNET: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: MixedAddress::Solana(
                Pubkey::from_str("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU").unwrap(),
            ),
            network: Network::SolanaDevnet,
        },
        decimals: 6,
        eip712: None,
    })
});

/// Lazily initialized known USDC deployment on Polygon Amoy testnet as [`USDCDeployment`].
static USDC_POLYGON_AMOY: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: address!("0x41E94Eb019C0762f9Bfcf9Fb1E58725BfB0e7582").into(),
            network: Network::PolygonAmoy,
        },
        decimals: 6,
        eip712: Some(TokenDeploymentEip712 {
            name: "USDC".into(),
            version: "2".into(),
        }),
    })
});

/// Lazily initialized known USDC deployment on Polygon mainnet as [`USDCDeployment`].
static USDC_POLYGON: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: address!("0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359").into(),
            network: Network::Polygon,
        },
        decimals: 6,
        eip712: Some(TokenDeploymentEip712 {
            name: "USDC".into(),
            version: "2".into(),
        }),
    })
});

static USDC_SEI: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: address!("0xe15fC38F6D8c56aF07bbCBe3BAf5708A2Bf42392").into(),
            network: Network::Sei,
        },
        decimals: 6,
        eip712: Some(TokenDeploymentEip712 {
            name: "USDC".into(),
            version: "2".into(),
        }),
    })
});

static USDC_SEI_TESTNET: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: address!("0x4fCF1784B31630811181f670Aea7A7bEF803eaED").into(),
            network: Network::Sei,
        },
        decimals: 6,
        eip712: Some(TokenDeploymentEip712 {
            name: "USDC".into(),
            version: "2".into(),
        }),
    })
});

static USDC_CELO: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: address!("0xcebA9300f2b948710d2653dD7B07f33A8B32118C").into(),
            network: Network::Celo,
        },
        decimals: 6,
        eip712: Some(TokenDeploymentEip712 {
            name: "USD Coin".into(),
            version: "2".into(),
        }),
    })
});

static USDC_CELO_SEPOLIA: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: address!("0x2F25deB3848C207fc8E0c34035B3Ba7fC157602B").into(),
            network: Network::CeloSepolia,
        },
        decimals: 6,
        eip712: Some(TokenDeploymentEip712 {
            name: "USDC".into(),
            version: "2".into(),
        }),
    })
});

static USDC_HYPEREVM: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: address!("0xC3B4b2C0faE2De7cF7e07c8c84f65d8df61Bf314").into(),
            network: Network::HyperEvm,
        },
        decimals: 6,
        eip712: Some(TokenDeploymentEip712 {
            name: "USDC".into(),
            version: "2".into(),
        }),
    })
});

static USDC_HYPEREVM_TESTNET: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: address!("0xC3B4b2C0faE2De7cF7e07c8c84f65d8df61Bf314").into(),
            network: Network::HyperEvmTestnet,
            Network::Optimism,
            Network::OptimismSepolia,
    /// Optimism mainnet (chain ID 10).
    #[serde(rename = "optimism")]
    Optimism,
    /// Optimism Sepolia testnet (chain ID 11155420).
    #[serde(rename = "optimism-sepolia")]
    OptimismSepolia,
        },
        decimals: 6,
        eip712: Some(TokenDeploymentEip712 {
            name: "USDC".into(),
            version: "2".into(),
        }),
    })
});

/// Lazily initialized UVD V2 deployment on Avalanche Fuji testnet as [`UVDDeployment`].
/// Note: Address must be updated after deploying erc-20/UVD_V2.sol

/// Lazily initialized known USDC deployment on Optimism mainnet as [`USDCDeployment`].
static USDC_OPTIMISM: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: address!("0x0b2C639c533813f4Aa9D7837CAf62653d097Ff85").into(),
            network: Network::Optimism,
        },
        decimals: 6,
        eip712: Some(TokenDeploymentEip712 {
            name: "USD Coin".into(),
            version: "2".into(),
        }),
    })
});

/// Lazily initialized known USDC deployment on Optimism Sepolia testnet as [`USDCDeployment`].
static USDC_OPTIMISM_SEPOLIA: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: address!("0x5fd84259d66Cd46123540766Be93DFE6D43130D7").into(),
            network: Network::OptimismSepolia,
        },
        decimals: 6,
        eip712: Some(TokenDeploymentEip712 {
            name: "USD Coin".into(),
            version: "2".into(),
        }),
    })
});

static UVD_AVALANCHE_FUJI: Lazy<UVDDeployment> = Lazy::new(|| {
    UVDDeployment(TokenDeployment {
        asset: TokenAsset {
            // Reads from GLUE_TOKEN_ADDRESS or GLUE_TOKEN_ADDRESS_AVALANCHE_FUJI environment variable
            // See: erc-20/deployment.json after running ./deploy-fuji.sh
            address: std::env::var("GLUE_TOKEN_ADDRESS_AVALANCHE_FUJI")
                .or_else(|_| std::env::var("GLUE_TOKEN_ADDRESS"))  // Fallback for backwards compatibility
                .ok()
                .and_then(|s| s.parse().ok())
                .map(|addr| MixedAddress::Evm(addr))
                .unwrap_or_else(|| {
                    // Default placeholder address (must be replaced)
                    address!("0x0000000000000000000000000000000000000000").into()
                }),
            network: Network::AvalancheFuji,
        },
        decimals: 6,
        eip712: Some(TokenDeploymentEip712 {
            name: "Gasless Ultravioleta DAO Extended Token".into(),
            version: "2".into(),
        }),
    })
});

/// Lazily initialized WAVAX deployment on Avalanche Fuji testnet as [`WAVAXDeployment`].
static WAVAX_AVALANCHE_FUJI: Lazy<WAVAXDeployment> = Lazy::new(|| {
    WAVAXDeployment(TokenDeployment {
        asset: TokenAsset {
            address: address!("0xd00ae08403B9bbb9124bB305C09058E32C39A48c").into(),
            network: Network::AvalancheFuji,
        },
        decimals: 18,
        eip712: Some(TokenDeploymentEip712 {
            name: "Wrapped AVAX".into(),
            version: "1".into(),
        }),
    })
});

/// A known USDC deployment as a wrapper around [`TokenDeployment`].
#[derive(Clone, Debug)]
pub struct USDCDeployment(pub TokenDeployment);

impl Deref for USDCDeployment {
    type Target = TokenDeployment;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&USDCDeployment> for TokenDeployment {
    fn from(deployment: &USDCDeployment) -> Self {
        deployment.0.clone()
    }
}

impl From<USDCDeployment> for Vec<TokenAsset> {
    fn from(deployment: USDCDeployment) -> Self {
        vec![deployment.asset.clone()]
    }
}

impl From<&USDCDeployment> for Vec<TokenAsset> {
    fn from(deployment: &USDCDeployment) -> Self {
        vec![deployment.asset.clone()]
    }
}

impl USDCDeployment {
    /// Return the known USDC deployment for the given network.
    ///
    /// Panic if the network is unsupported (not expected in practice).
    pub fn by_network<N: Borrow<Network>>(network: N) -> &'static USDCDeployment {
        match network.borrow() {
            Network::BaseSepolia => &USDC_BASE_SEPOLIA,
            Network::Base => &USDC_BASE,
            Network::XdcMainnet => &USDC_XDC,
            Network::AvalancheFuji => &USDC_AVALANCHE_FUJI,
            Network::Avalanche => &USDC_AVALANCHE,
            Network::Solana => &USDC_SOLANA,
            Network::SolanaDevnet => &USDC_SOLANA_DEVNET,
            Network::PolygonAmoy => &USDC_POLYGON_AMOY,
            Network::Polygon => &USDC_POLYGON,
            Network::Sei => &USDC_SEI,
            Network::SeiTestnet => &USDC_SEI_TESTNET,
            Network::Celo => &USDC_CELO,
            Network::CeloSepolia => &USDC_CELO_SEPOLIA,
            Network::HyperEvm => &USDC_HYPEREVM,
            Network::HyperEvmTestnet => &USDC_HYPEREVM_TESTNET,
            Network::Optimism => &USDC_OPTIMISM,
            Network::OptimismSepolia => &USDC_OPTIMISM_SEPOLIA,
        }
    }
}

/// A known UVD (Gasless Ultravioleta DAO Extended Token) deployment as a wrapper around [`TokenDeployment`].
#[derive(Clone, Debug)]
pub struct UVDDeployment(pub TokenDeployment);

impl Deref for UVDDeployment {
    type Target = TokenDeployment;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&UVDDeployment> for TokenDeployment {
    fn from(deployment: &UVDDeployment) -> Self {
        deployment.0.clone()
    }
}

impl From<UVDDeployment> for Vec<TokenAsset> {
    fn from(deployment: UVDDeployment) -> Self {
        vec![deployment.asset.clone()]
    }
}

impl From<&UVDDeployment> for Vec<TokenAsset> {
    fn from(deployment: &UVDDeployment) -> Self {
        vec![deployment.asset.clone()]
    }
}

impl UVDDeployment {
    /// Return the known UVD deployment for the given network.
    ///
    /// Currently only supports Avalanche Fuji testnet.
    pub fn by_network<N: Borrow<Network>>(network: N) -> &'static UVDDeployment {
        match network.borrow() {
            Network::AvalancheFuji => &UVD_AVALANCHE_FUJI,
            _ => panic!("UVD/GLUE token only deployed on Avalanche Fuji testnet"),
        }
    }
}

/// A known WAVAX (Wrapped AVAX) deployment as a wrapper around [`TokenDeployment`].
#[derive(Clone, Debug)]
pub struct WAVAXDeployment(pub TokenDeployment);

impl Deref for WAVAXDeployment {
    type Target = TokenDeployment;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&WAVAXDeployment> for TokenDeployment {
    fn from(deployment: &WAVAXDeployment) -> Self {
        deployment.0.clone()
    }
}

impl From<WAVAXDeployment> for Vec<TokenAsset> {
    fn from(deployment: WAVAXDeployment) -> Self {
        vec![deployment.asset.clone()]
    }
}

impl From<&WAVAXDeployment> for Vec<TokenAsset> {
    fn from(deployment: &WAVAXDeployment) -> Self {
        vec![deployment.asset.clone()]
    }
}

impl WAVAXDeployment {
    /// Return the known WAVAX deployment for Avalanche Fuji.
    pub fn by_network<N: Borrow<Network>>(network: N) -> &'static WAVAXDeployment {
        match network.borrow() {
            Network::AvalancheFuji => &WAVAX_AVALANCHE_FUJI,
            _ => panic!("WAVAX token only deployed on Avalanche Fuji testnet for Karmacadabra"),
        }
    }
}
