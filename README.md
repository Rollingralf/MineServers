# MineServers

This branch (feat/flutter-skeleton) contains a Flutter multi-platform skeleton for MineServers. The goal is a single app that provides a dashboard and runtime manager to create and manage local Minecraft servers on Windows, macOS, and as a dashboard on iOS.

Important: iOS apps cannot host a full Minecraft server in an App Store compliant way. The iOS app in this project will be a dashboard/remote-control client for servers running on other devices.

What's included in this branch:

- Flutter project skeleton (lib/main.dart) with placeholder screens: Server List, Create Server, Console, Settings
- pubspec.yaml with basic dependencies
- runtime/README.md describing the runtime manager scaffold
- GitHub Actions workflow (flutter CI)

Quickstart (local)

1. Install Flutter SDK: https://flutter.dev
2. Clone this repository and checkout branch: feat/flutter-skeleton

   git clone https://github.com/Rollingralf/MineServers.git
   cd MineServers
   git checkout feat/flutter-skeleton

3. Get dependencies:
   flutter pub get

4. Run on desktop (example):
   flutter run -d windows
   flutter run -d macos

5. Run on iOS (dashboard-only):
   flutter run -d ios (requires Xcode and macOS)

Next steps

- Implement the runtime manager (recommended: Rust) that spawns server processes, streams logs, and exposes a local HTTP/WebSocket API.
- Wire up Flutter to the runtime manager to start/stop servers, stream console output, manage worlds/plugins, and handle backups.
- Add on-demand server downloads (PaperMC, Bedrock dedicated server) and integrity checks.

If you want me to continue, I can implement the runtime manager prototype for desktop (start/stop and console streaming) and wire it to the Flutter UI.
