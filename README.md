# EndPoint

A desktop application for monitoring server endpoints, built with Tauri, SvelteKit, and TypeScript. Easily add servers, check their health status, and view response times in a clean, responsive interface.

## Features

- **Add Servers**: Input server URLs and names to monitor.
- **Health Checks**: Perform HTTP requests to check server status and response times.
- **Status Display**: Visual indicators for online (green), offline (red), or unknown (gray) servers, showing HTTP status codes.
- **Database Storage**: Persistent storage using SQLite for servers and settings.
- **Responsive UI**: Built with SvelteKit and Tailwind CSS for a modern, adaptive design.
- **Cross-Platform**: Runs on Windows, macOS, and Linux via Tauri.

## Tech Stack

- **Frontend**: SvelteKit, TypeScript, Tailwind CSS
- **Backend**: Tauri (Rust), SQLite
- **Build Tool**: Vite
- **Database**: SQLite with migrations

## Prerequisites

- [Node.js](https://nodejs.org/) (v18 or later)
- [Rust](https://rustup.rs/) (latest stable)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites): Install with `npm install -g @tauri-apps/cli`

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/endpoint.git
   cd endpoint
   ```

2. Install frontend dependencies:
   ```bash
   npm install
   ```

3. Install Rust dependencies and build the Tauri app:
   ```bash
   npm run tauri build
   ```

## Development

1. Start the development server:
   ```bash
   npm run tauri dev
   ```
   This launches the app in development mode with hot reloading.

2. For frontend-only development:
   ```bash
   npm run dev
   ```

## Usage

- Launch the app.
- Click "Add Server" to input a server URL and name.
- Use "Check Health" to test connectivity and view status.
- "Clear All Servers" removes all entries.

## Project Structure

```
src/
├── lib/
│   ├── types.ts                    # Shared interfaces
│   ├── services/
│   │   ├── database.ts             # Database operations
│   │   └── healthCheck.ts          # Health check logic
│   └── utils/
│       └── statusHelpers.ts        # Utility functions
├── components/
│   ├── common/
│   │   └── Modal.svelte            # Reusable modal
│   ├── ServerList.svelte           # Server list component
│   └── AddServerForm.svelte        # Add server form
└── routes/
    └── +page.svelte                # Main page
src-tauri/
├── src/
│   ├── main.rs                     # App initialization
│   └── commands.rs                 # Tauri commands
└── migrations/
    ├── 001_init.sql                # Initial schema
    └── 002_add_indexes.sql         # Additional migrations
```

## Contributing

1. Fork the repository.
2. Create a feature branch: `git checkout -b feature-name`.
3. Commit changes: `git commit -am 'Add feature'`.
4. Push to the branch: `git push origin feature-name`.
5. Submit a pull request.

## License

This project is licensed under the MIT License. See LICENSE for details.