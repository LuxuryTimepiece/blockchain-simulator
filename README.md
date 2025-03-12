# Blockchain Simulator

A blockchain simulator built with Rust (backend), Vue.js (frontend), and Tailwind CSS for styling. Features a real-time chart of mining times and a scrollable table of blocks.

## Demo

![Blockchain Simulator Screenshot](https://raw.githubusercontent.com/LuxuryTimepiece/blockchain-simulator/refs/heads/main/goodlookingapp.png)

## Features
- Add blocks with custom data via a web form.
- Real-time bar chart of mining times using `vue-chartjs`.
- Scrollable table to view block data (index, data, hash, mining time).
- Validates the blockchain integrity.

## Tech Stack
- **Backend**: Rust with Actix-web
- **Frontend**: Vue.js with Tailwind CSS
- **Charting**: Chart.js via `vue-chartjs`

## Setup

### Prerequisites
- Rust (install via [rustup](https://rustup.rs/))
- Node.js and npm
- Git

### Installation

   1. **Clone the repository**:
      ```bash
      git clone https://github.com/your-username/blockchain-simulator.git
      cd blockchain-simulator
      ```

   2. **Backend Setup**:
      ```bash
      cd blockchain-simulator
      cargo run
      ```

   3. **Frontend Setup**:
      ```bash
      cd blockchain-simulator-frontend
      npm install
      npm run build
      cp -r dist ../
      ```

   4. **Open** `http://localhost:3000` **in your browser**.

   ## Usage
   - Enter block data (e.g., "To the Moon!") in the form and click "Add Block".
   - Watch the table and chart update in real-time.
   - Scroll the table to view all blocks while the chart remains visible.

   ## Contributing
   Feel free to fork this repo and submit pull requests with improvements!

   ## License
   [MIT License](LICENSE)

