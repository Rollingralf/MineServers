// GENERATED FILE - minimal placeholder for Flutter project
// Replace with real assets and platform-specific settings when ready.

import 'package:flutter/material.dart';

void main() {
  runApp(const MineServersApp());
}

class MineServersApp extends StatelessWidget {
  const MineServersApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'MineServers',
      theme: ThemeData(
        primarySwatch: Colors.green,
      ),
      initialRoute: '/',
      routes: {
        '/': (_) => const HomePage(),
        '/create': (_) => const CreateServerPage(),
        '/console': (_) => const ConsolePage(),
        '/settings': (_) => const SettingsPage(),
      },
    );
  }
}

class HomePage extends StatelessWidget {
  const HomePage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('MineServers')),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text('Servers', style: TextStyle(fontSize: 20, fontWeight: FontWeight.bold)),
            const SizedBox(height: 12),
            Expanded(
              child: Center(
                child: Text('No servers yet. Tap + to create your first server.', style: TextStyle(color: Colors.grey[700])),
              ),
            ),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () => Navigator.pushNamed(context, '/create'),
        child: const Icon(Icons.add),
      ),
      drawer: Drawer(
        child: ListView(
          children: [
            const DrawerHeader(child: Text('MineServers')),
            ListTile(
              leading: const Icon(Icons.settings),
              title: const Text('Settings'),
              onTap: () => Navigator.pushNamed(context, '/settings'),
            ),
          ],
        ),
      ),
    );
  }
}

class CreateServerPage extends StatelessWidget {
  const CreateServerPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Create Server')),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          children: [
            const TextField(decoration: InputDecoration(labelText: 'Server name')),
            const SizedBox(height: 12),
            DropdownButtonFormField<String>(
              items: const [
                DropdownMenuItem(value: 'paper', child: Text('Java - PaperMC')),
                DropdownMenuItem(value: 'vanilla', child: Text('Java - Vanilla')),
                DropdownMenuItem(value: 'bedrock', child: Text('Bedrock')),
              ],
              onChanged: (_) {},
              decoration: const InputDecoration(labelText: 'Engine'),
            ),
            const SizedBox(height: 12),
            ElevatedButton(
              onPressed: () {
                // TODO: implement server creation flow
                ScaffoldMessenger.of(context).showSnackBar(const SnackBar(content: Text('Server create flow not implemented yet')));
              },
              child: const Text('Create'),
            ),
          ],
        ),
      ),
    );
  }
}

class ConsolePage extends StatelessWidget {
  const ConsolePage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Console')),
      body: const Center(child: Text('Console output will appear here')),
    );
  }
}

class SettingsPage extends StatelessWidget {
  const SettingsPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Settings')),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: ListView(
          children: const [
            ListTile(title: Text('Runtime manager (Rust)')),
            ListTile(title: Text('Server binaries: download on demand')),
            ListTile(title: Text('iOS: dashboard-only')),
          ],
        ),
      ),
    );
  }
}
