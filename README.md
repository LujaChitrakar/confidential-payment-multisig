# Confidential Payment Gateway( with Strata model multisig & KYC Transfers & Swap with JUP )

This project, **confidential payment gateway for banks** is built on Solana using Anchor. While Solana's `confidential transfer` is still under audit, this framework uses a **permissioned, multisig-controlled architecture** for inter-bank settlements, ensuring privacy, compliance, and auditability.

---

## Features

- **Bank Registration & KYC Approval**: Only registered and KYC-approved banks can participate.
- **Multisig-Controlled Transfers**: Bank-to-bank transfers require multiple approvals from authorized signers.
- **Transfer Proposals**: Proposals are created for each transfer; they require multisig approval before execution.
- **Audit & Compliance Ready**: Designed to integrate off-chain auditor access and selective disclosure mechanisms.
- **Placeholder for Future Confidential Transfers**: Integrates easily with token-2022 confidential transfers or zk-proof-based solutions once audited.

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────────────────────┐
│                           Solana Blockchain Network                                 │
├─────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                     │
│  ┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐                  │
│  │   Bank Registry │    │ Multisig Wallet │    │ Transfer Engine │                  │
│  │                 │    │                 │    │                 │                  │
│  │ • Bank Records  │    │ • Threshold     │    │ • Proposals     │                  │
│  │ • KYC Status    │    │ • Signers       │    │ • Executions    │                  │
│  │ • Permissions   │    │ • Authority     │    │ • State Mgmt    │                  │
│  └─────────────────┘    └─────────────────┘    └─────────────────┘                  │
│                                                                                     │
└─────────────────────────────────────────────────────────────────────────────────────┘
┌─────────────────────────────────────────────────────────────────────────────────────┐
│                              Off-Chain Layer                                        │
├─────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                     │
│  ┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐                  │
│  │  Bank Frontend  │    │ Auditor Portal  │    │ Compliance APIs │                  │
│  │                 │    │                 │    │                 │                  │
│  │ • Transaction   │    │ • View Keys     │    │ • Reporting     │                  │
│  │   Interface     │    │ • Audit Trails  │    │ • Monitoring    │                  │
│  │ • Wallet Mgmt   │    │ • Compliance    │    │ • Alerts        │                  │
│  └─────────────────┘    └─────────────────┘    └─────────────────┘                  │
│                                                                                     │
└─────────────────────────────────────────────────────────────────────────────────────┘
**Multisig Control Architecture**


- **Bank**: Holds registration info, KYC status, and associated multisig.
- **Multisig**: Controls bank's outgoing transfers; requires threshold approvals.
```
---

## Multisig Control Architecture
```Multisig Wallet Structure:
┌─────────────────────────┐
│    Multisig Account     │
├─────────────────────────┤
│ • threshold: u8         │
│ • signers: Vec<Pubkey>  │
│ • nonce: u64            │
│ • owner_set_seqno: u32  │
│ • pending_proposals: u8 │
└─────────────────────────┘

Approval Flow:
Bank Operator → Create Transaction → Collect Signatures → Execute
     │              │                    │                 │
     │              ▼                    ▼                 │
     │        ┌──────────────┐    ┌──────────────┐         │
     │        │  Proposal    │    │  Signature   │         │
     │        │  Created     │    │  Collection  │         │
     │        └──────────────┘    └──────────────┘         │
     │                                   │                 │
     └───────────────────────────────────┼─────────────────┘
                                         ▼
                                  ┌──────────────┐
                                  │  Transfer    │
                                  │  Executed    │
                                  └──────────────┘
```
---
## Transfer Execution Flow
```
┌─────────────┐
│  Bank A     │
│  Initiates  │
│  Transfer   │
└──────┬──────┘
       │
       ▼
┌─────────────┐    ┌─────────────────┐
│   Create    │─── │  Validation     │
│  Proposal   │    │   - Bank Status │
│             │    │   - Permissions │
└─────────────┘    │   - Balance     │
       │           └─────────────────┘
       │                     │
       ▼                     │
┌─────────────┐              │
│  Proposal   │◀────────────┘
│  Stored     │
│  On-Chain   │
└──────┬──────┘
       │
       ▼
┌─────────────┐    ┌─────────────────┐
│ Multisig    │─── │  Threshold      │
│ Signing     │    │   Validation    │
│ Process     │    │                 │
└─────────────┘    └─────────────────┘
       │                     │
       │              ┌──────┴──────┐
       │              │             │
       │              ▼             ▼
       │        ┌──────────┐  ┌──────────┐
       │        │Approved  │  │Rejected  │
       │        └─────┬────┘  └──────────┘
       │              │
       ▼              ▼
┌─────────────┐  ┌─────────────┐
│  Execute    │  │   Token     │
│ Transfer    │─ │ Transfer    │
│ Proposal    │  │   (SPL)     │
└─────────────┘  └─────────────┘
       │              │
       ▼              ▼
┌─────────────┐  ┌─────────────┐
│  Update     │  │   Bank B    │
│  Balances   │  │  Receives   │
│             │  │  Payment    │
└─────────────┘  └─────────────┘
```
---
## Future Architecture Enhancements
```
Current (Placeholder):                Future (Confidential):
┌─────────────────┐                  ┌─────────────────┐
│  SPL Token      │                  │ Token-2022      │
│  Transfer       │      ──────      │ Confidential    │
│  (Public)       │                  │ Transfer        │
└─────────────────┘                  └─────────────────┘
         │                                    │
         ▼                                    ▼
┌─────────────────┐                  ┌─────────────────┐
│ Visible         │                  │ Zero-Knowledge  │
│ Amounts         │                  │ Proofs          │
└─────────────────┘                  └─────────────────┘
```
---
## Getting Started

### Prerequisites

- Rust and Anchor installed
- Solana CLI
- Local Solana test validator

### Deploy Program

```bash
anchor build
anchor deploy


### Interact with Program
```bash
anchor test --skip-deploy
```
2. **Approve Bank (KYC)**
3. **Create Transfer Transaction**
4. **Approve Transaction via Multisig**
5. **Execute Transaction**

---

## Security & Compliance Notes

- Multisig prevents a single operator from moving funds.
- Proposal flow ensures controlled execution and prevents double-spending.

---

## Future Work

- Replace placeholder SPL transfer with **confidential token CPI**.
- Integrate **zk-proof verifier** for confidential amounts.
- Implement **auditor view-key system** for selective disclosure.
- Add **freeze/blacklist authority** for compliance.
