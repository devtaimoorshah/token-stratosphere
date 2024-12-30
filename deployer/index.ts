const { DirectSecp256k1HdWallet } = require("@cosmjs/proto-signing");
const { SigningCosmWasmClient, CosmWasmClient } = require("@cosmjs/cosmwasm-stargate");
const { coins, GasPrice } = require("@cosmjs/stargate");
const fs = require("fs");
require("dotenv").config();

const mnemonic = process.env.MNEMONIC; // Replace with your mnemonic
const rpcEndpoint = "https://rpc.dukong.mantrachain.io"; // RPC endpoint for your chain
const contractWasmPath = "./stratocoin.wasm"; // Path to your Stratocoin contract(wasm file)

async function deploy() {
  try {
    // Step 1: Set up wallet and client
    const wallet = await DirectSecp256k1HdWallet.fromMnemonic(mnemonic, {
      prefix: "mantra", // Replace with the correct prefix for your chain
    });
    const [account] = await wallet.getAccounts();
    console.log(`Wallet address: ${account.address}`);

    // Step 2: Connect to the blockchain with the correct gasPrice denom
    const client = await SigningCosmWasmClient.connectWithSigner(
      rpcEndpoint,
      wallet,
      { gasPrice: GasPrice.fromString("0.01uom") } // Correct denom here: uom
    );
    console.log("Connected to blockchain");

    // Step 3: Upload contract
    const wasmCode = fs.readFileSync(contractWasmPath); // Using the path to stratocoin.wasm
    const uploadReceipt = await client.upload(
      account.address,
      wasmCode,
      "auto", // Automatically manage gas limit
      "Upload Stratocoin Contract"
    );
    const codeId = uploadReceipt.codeId;
    console.log(`Contract uploaded with Code ID: ${codeId}`);

    // Step 4: Instantiate contract with proper feeAmount
    const initMsg = {
      token_name: "StratoCoin",
      symbol: "STRAT",
      denom: "ustrat",
      decimals: 6,
      total_supply: "1000000000000000",
      initial_circulating_supply: "100000000000000",
      distribution: {
        validators: "400000000000000",
        community_reserve: "300000000000000",
        staking_rewards: "200000000000000",
        liquidity_provision: "50000000000000",
        foundation: "50000000000000",
      },
      inflation: {
        rate: 0.14, // Change from "14%" to 0.14
        decay_rate: 0.5,
        decay_time: "365 days",
        min_inflation: 0.07, // Change from "7%" to 0.07
        capped: true,
      },
      governance: {
        staking_requirement: "1000000 ustrat",
        voting_power: "1 token = 1 vote",
        quorum: 0.334, // Change from "33.4%" to 0.334
        threshold: 0.5, // Change from "50%" to 0.5
      },
      staking: {
        unbonding_time: "14 days",
        max_validators: 150,
        reward_distribution: "pro_rata",
      },
      liquidity_pools: [
        { pair: "STRAT/OSMO", pool_type: "default" },
        { pair: "STRAT/ATOM", pool_type: "default" },
      ],
      ibc_channels: [
        { target_chain: "osmosis-1", transfer_enabled: true },
      ],
    };

    // Explicitly set feeAmount as an array of coin objects
    const feeAmount = [{ denom: "uom", amount: "5000" }];  // Corrected feeAmount format

    const instantiateReceipt = await client.instantiate(
      account.address,
      codeId,
      initMsg,
      "My Stratocoin Contract",
      { gas: "200000", feeAmount }  // Use the correct feeAmount array format
    );

    const contractAddress = instantiateReceipt.contractAddress;
    console.log(`Contract instantiated at address: ${contractAddress}`);
  } catch (error) {
    console.error("Error during deployment:", error);
  }
}

deploy().catch(console.error);
