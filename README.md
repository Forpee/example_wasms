# What each WASM module does?

## 1) **Energy Usage (IoT/Microgrids)** 
**Description:**  
- Models energy usage for microgrids or IoT devices, including line-loss computations, overhead adjustments, and pool health checks.  

 

---

## 2) **DeFi Transaction (Simple Version)**
**Description:**  
- A minimal DeFi swap illustrating constant product market making, fee adjustments, slippage checks, and pool value calculations.  
- The `main` function validates a swap amount, calculates output tokens, fees, slippage, and returns an XOR-combined result.



---

## 3) **DeFi Protocol (Advanced Version)** 
**Description:**  
- A more complex DeFi model: dynamic fees, partial fallback trades, pool state simulation, slippage tolerance, etc.  

---

## 4) **Cryptography Example (Toy RSA)**
**Description:**  
- Demonstrates a toy RSA-like system: prime checks, modulus `n = p*q`, totient calculation, modular exponentiation, partial fallback if prime checks fail.  
- Uses **unsigned** (`u64`) and **signed** (`i64`) arithmetic, plus extended Euclidean GCD logic.



---

## 5) **Data Provenance**  
**Description:**  
- Simulates tracking origin/lifecycle of products in critical industries (pharmaceuticals, agriculture).  


---

## 6) **Regulatory Compliance** 
**Description:**  
- Models a scenario where an energy company needs to prove carbon offset calculations.  


---

## 7) **Game Logic**
**Filename/Context:** `game_virtual_world.rs`  
**Description:**  
- Mimics server-side calculations for a game or virtual world: combat damage, resource forging, XP leveling. 

---

## 8) **Smart Contract Auditing**

**Description:**  
- Emulates a complex on-chain state verification or auditing scenario for a DeFi or general blockchain environment.  




