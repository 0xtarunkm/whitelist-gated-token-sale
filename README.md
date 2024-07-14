# Whitelist-Gated Token Sale Program

## Overview

This program, developed using Anchor, enables users to participate in a whitelist-gated sale for a new token. The program ensures that:

- **Token Price**: The token price remains static throughout the sale.
- **Purchase Limit**: Each wallet address has a limit on the number of tokens it can purchase.

## Features

### Whitelist-Gated Sale

- **Whitelist Management**: Only users on the whitelist can participate in the sale.
- **Static Pricing**: The price of the token is fixed and does not change during the sale.
- **Purchase Limits**: Each wallet address is restricted to a specific number of tokens.

## Getting Started

### Prerequisites

- Rust
- Anchor framework
- Solana CLI

### Installation

1. **Clone the repository:**

   ```bash
   git clone https://github.com/tarunclub/whitelist-gated-token-sale
   ```

2. **Build the repository**
   ```
   anchor build
   ```
3. **Run tests**
   ```
   anchor test
   ```
