# SQS Dashboard

A minimalist and elegant desktop application for managing AWS SQS LocalStack queues. Built with Tauri, Vue.js, and Tailwind CSS.

## Features

- 🎨 **Minimalist UI** - Clean and elegant interface
- 🌓 **Light/Dark Mode** - Toggle between themes with persistent settings
- ⚙️ **SQS Configuration** - Connect to LocalStack or AWS SQS endpoints
- 📋 **Queue Management** - List, create, and manage queues
- 📨 **Message Operations** - Send, receive, and delete messages
- 🚀 **Desktop & Web** - Run as a native desktop app or deploy as a web application

## Prerequisites

- [Node.js](https://nodejs.org/) (v20 or higher)
- [Rust](https://www.rust-lang.org/) (for Tauri desktop app)
- [Docker](https://www.docker.com/) (optional, for LocalStack testing)

## Installation

```bash
# Install dependencies
npm install
```

## Development

### Desktop App

```bash
# Run the Tauri desktop application
npm run tauri dev
```

### Web App

```bash
# Run the web development server
npm run dev
```

## Building

### Desktop App

```bash
# Build the Tauri desktop application
npm run tauri build
```

The built application will be available in `src-tauri/target/release/bundle/`.

### Web App

```bash
# Build for web deployment
npm run build
```

The built files will be in the `dist/` directory.

## Docker Deployment

### Build and Run with Docker

```bash
# Build the Docker image
docker build -t sqs-dashboard .

# Run the container
docker run -p 8080:80 sqs-dashboard
```

Access the application at `http://localhost:8080`

### Run with Docker Compose (includes LocalStack)

```bash
# Start both LocalStack and the dashboard
docker-compose up

# Stop the services
docker-compose down
```

- SQS Dashboard: `http://localhost:8080`
- LocalStack: `http://localhost:4566`

## Usage

1. **Configure SQS Connection**
   - Enter your LocalStack endpoint (default: `http://localhost:4566`)
   - Click "Connect"

2. **Create a Queue**
   - Enter a queue name in the input field
   - Click "Create Queue"

3. **Send Messages**
   - Select a queue from the list
   - Enter your message body (supports JSON or plain text)
   - Click "Send Message"

4. **Receive Messages**
   - Select a queue
   - Click "Receive Messages" to poll for new messages
   - Messages will appear with their ID and body

5. **Delete Messages**
   - Click "Delete" on any received message to remove it from the queue

## Configuration

The application uses LocalStack by default for testing. To connect to AWS SQS:

1. Update the endpoint to your AWS region (e.g., `https://sqs.us-east-1.amazonaws.com`)
2. Ensure proper AWS credentials are configured in your environment

## Tech Stack

- **Frontend**: Vue.js 3 + TypeScript
- **Styling**: Tailwind CSS
- **Desktop**: Tauri (Rust + WebView)
- **Build Tool**: Vite
- **AWS SDK**: aws-sdk-sqs (Rust)

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
