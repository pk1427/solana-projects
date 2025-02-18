import { Connection, clusterApiUrl, Keypair, PublicKey } from "@solana/web3.js";
import { bundlrStorage, Metaplex, keypairIdentity } from "@metaplex-foundation/js";
import { createCreateMetadataAccountV3Instruction } from "@metaplex-foundation/mpl-token-metadata";
import dotenv from "dotenv";
import fs from "fs";

dotenv.config();

// Load wallet keypair
const wallet = Keypair.fromSecretKey(
  new Uint8Array(JSON.parse(fs.readFileSync(process.env.KEYPAIR_PATH, "utf-8")))
);

// Initialize Solana connection
const connection = new Connection(clusterApiUrl("devnet"));
const metaplex = Metaplex.make(connection)
  .use(keypairIdentity(wallet))
  .use(bundlrStorage());

// NFT metadata
const metadata = {
  name: "My First Solana NFT",
  symbol: "MFSN",
  uri: "https://arweave.net/YOUR_METADATA_JSON_URL",
  sellerFeeBasisPoints: 500, // 5% royalty
};

// Mint NFT
async function mintNFT() {
  console.log("Minting NFT...");

  const { nft } = await metaplex.nfts().create({
    uri: metadata.uri,
    name: metadata.name,
    symbol: metadata.symbol,
    sellerFeeBasisPoints: metadata.sellerFeeBasisPoints,
  });

  console.log(`NFT Minted: ${nft.address.toBase58()}`);
}

mintNFT().catch(console.error);
