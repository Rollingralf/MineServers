// Runtime API service for Flutter

import 'dart:convert';
import 'package:http/http.dart' as http;
import 'package:web_socket_channel/web_socket_channel.dart';

class RuntimeApi {
  final String baseUrl;
  RuntimeApi({this.baseUrl = 'http://127.0.0.1:7878'});

  Future<Map<String, dynamic>> createServer(String name, String engine, String version) async {
    final uri = Uri.parse('$baseUrl/create');
    final resp = await http.post(uri, body: jsonEncode({'name': name, 'engine': engine, 'version': version}), headers: {'Content-Type': 'application/json'});
    if (resp.statusCode == 200) return jsonDecode(resp.body) as Map<String, dynamic>;
    throw Exception('Failed to create server: ${resp.statusCode} ${resp.body}');
  }

  Future<void> startServer(String id) async {
    final uri = Uri.parse('$baseUrl/start');
    final resp = await http.post(uri, body: jsonEncode({'id': id}), headers: {'Content-Type': 'application/json'});
    if (resp.statusCode != 200) throw Exception('Failed to start server: ${resp.statusCode} ${resp.body}');
  }

  Future<void> stopServer(String id) async {
    final uri = Uri.parse('$baseUrl/stop');
    final resp = await http.post(uri, body: jsonEncode({'id': id}), headers: {'Content-Type': 'application/json'});
    if (resp.statusCode != 200) throw Exception('Failed to stop server: ${resp.statusCode} ${resp.body}');
  }

  Future<Map<String, dynamic>> status(String id) async {
    final uri = Uri.parse('$baseUrl/status?id=$id');
    final resp = await http.get(uri);
    if (resp.statusCode == 200) return jsonDecode(resp.body) as Map<String, dynamic>;
    throw Exception('Failed to get status: ${resp.statusCode} ${resp.body}');
  }

  WebSocketChannel streamLogs(String id) {
    final uri = Uri.parse(baseUrl.replaceFirst('http', 'ws') + '/ws/logs?id=$id');
    return WebSocketChannel.connect(uri);
  }
}
