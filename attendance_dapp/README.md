# ScholarCheck

**Problem:**
Teachers in large public universities in Southeast Asia struggle to track attendance reliably, leading to manipulated records and unfair scholarship disbursements which rely on strict attendance metrics, costing universities thousands in misallocated funds.

**Solution:**
Teachers record attendance through a Soroban smart contract. The contract immutably logs the attendance on-chain, ensuring verifiable records that cannot be altered without authorization, and making scholarship eligibility transparent and tamper-proof.

**Timeline:**
Hackathon MVP build (3 days)

**Stellar Features Used:**
- Soroban smart contracts
- Trustless state storage

**Vision and Purpose:**
To bring complete transparency to educational funding and credentials, starting with basic verifiable attendance that can plug into DeFi scholarship pools in the future.

**Prerequisites:**
- Rust (`rustc 1.80+`)
- Soroban CLI (`soroban-cli v22.0.0+`)
- `wasm32-unknown-unknown` target

**How to build:**
```bash
soroban contract build
```

**How to test:**
```bash
cargo test
```

**How to deploy to testnet:**
```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/attendance_dapp.wasm \
  --source teacher_wallet \
  --network testnet
```

**Sample CLI invocation:**
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source teacher_wallet \
  --network testnet \
  -- \
  mark_attendance \
  --student GABCD... \
  --date 1718928000 \
  --present true
```

**License:**
MIT
