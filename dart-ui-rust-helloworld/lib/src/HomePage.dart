import 'dart:async';

import 'package:example_flutter/src/Bluetooth.dart';
import 'package:flutter/material.dart';
import 'package:grpc/grpc.dart';

import 'generated/helloworld.pbgrpc.dart';

class HomePage extends StatefulWidget {
  HomePage({Key key, this.title}) : super(key: key);

  final String title;

  @override
  _HomePageState createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  String _helloDisplay = '';

  void _makeRpcCall() {
    _callRust([]);
  }

  Future<Null> _callRust(List<String> args) async {
    final channel = new ClientChannel('localhost',
        port: 50051,
        options: const ChannelOptions(
            credentials: const ChannelCredentials.insecure()));
    final stub = new GreeterClient(channel);

    final name = args.isNotEmpty ? args[0] : 'world';

    try {
      var response = await stub.sayHello(new HelloRequest()..name = name);
      setState(() {
        _helloDisplay = response.message;
      });
      print('Greeter client received: ${response.message}');
    } catch (e) {
      print('Caught error: $e');
    }
    await channel.shutdown();
  }

  void _pushBluetooth() {
    Navigator.of(context).push(
      MaterialPageRoute<void>(
        builder: (context) => BluetoothPage(title: 'Bluetooth')
      )
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(widget.title),
        actions: <Widget>[
          IconButton(icon: Icon(Icons.list), onPressed: _pushBluetooth),
        ]
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            Text(
              '$_helloDisplay',
              style: Theme.of(context).textTheme.display1,
            ),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: _makeRpcCall,
        tooltip: 'Increment',
        child: Icon(Icons.add),
      ),
    );
  }
}
