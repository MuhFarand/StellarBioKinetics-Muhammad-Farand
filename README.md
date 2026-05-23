# Stellar BioKinetics DApp

**Stellar BioKinetics DApp** - Blockchain-Based Bacterial Growth Modeling and Comparative Analysis System

## Project Description
Stellar BioKinetics DApp is a decentralized smart contract solution built on the Stellar blockchain using the Soroban SDK. It provides a secure, immutable, and deterministic platform for storing, managing, and comparing mathematical models of bacterial growth. By leveraging blockchain storage, researchers and bioinformaticians can record critical growth kinetic parameters without relying on centralized database providers.

The system allows users to archive growth profiles—including specific growth rates, lag phase durations, and carrying capacities—and perform trustless comparative analysis directly via smart contract logic on the Stellar network.

## Project Vision
- **Decentralizing Biological Data**: Shifting growth model repositories from siloed servers to a global, distributed ledger.
- **Ensuring Data Provenance**: Providing an immutable, tamper-proof audit trail for microbial growth parameters to prevent unauthorized modifications or data manipulation.
- **Enabling Deterministic Simulation**: Utilizing blockchain runtime environments to guarantee identical, bias-free mathematical comparisons of biological models.
- **Fostering Open Science**: Building a trustless network where phenotypic profiles can be publicly archived, verified, and compared by the global research community.

## Key Features
1. **Growth Profile Archiving**: Register bacterial growth models by inputting core kinetic parameters such as specific growth rate, lag phase duration, and maximum population capacity.
2. **Trustless Comparative Logic**: Compare two bacterial models directly on-chain using deterministic mathematical formulas.
3. **Efficiency Scoring**: Built-in calculation of a relative fitness index using integer math logic: Score = (Growth Rate * Max Population) / (Lag Phase Duration + 1).
4. **Deterministic Fixed-Point Representation**: Uses scaled integers (fixed-point arithmetic) to avoid the non-deterministic computational behavior of floating-point numbers on the blockchain.

## Core Contract Interfaces
- `add_model(name, growth_rate, lag_phase_duration, max_population)` -> Saves a new microbial profile into persistent storage.
- `get_all_models()` -> Fetches the entire collection of registered bacterial models.
- `compare_models(index_a, index_b)` -> Performs an on-chain comparative analysis between two models based on their array index positions.

**Stellar BioKinetics DApp** - Securing Bio-Computational Models on the Blockchain"#;

Id Contract : CD6XNELJMIOMD5HRXF3YYMW75TJNLOGB6CVPA3WXKNYILL3F25FXFO67


