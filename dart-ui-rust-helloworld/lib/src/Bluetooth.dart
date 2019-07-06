import 'dart:async';

import 'package:example_flutter/src/generated/bluetooth.pbgrpc.dart';
import 'package:flutter/material.dart';
import 'package:grpc/grpc.dart';

class BluetoothPage extends StatefulWidget {
  BluetoothPage({Key key, this.title}) : super(key: key);

  final String title;

  @override
  _BluetoothPageState createState() => _BluetoothPageState();
}

class _BluetoothPageState extends State<BluetoothPage> {
  List _devices = [];
  // void _makeRpcCall() {
  //   _callRust([]);
  // }

  // Future<Null> _callRust(List<String> args) async {
  //   final channel = new ClientChannel('localhost',
  //       port: 50051,
  //       options: const ChannelOptions(
  //           credentials: const ChannelCredentials.insecure()));
  //   final stub = new GreeterClient(channel);

  //   final name = args.isNotEmpty ? args[0] : 'world';

  //   try {
  //     var response = await stub.sayHello(new HelloRequest()..name = name);
  //     setState(() {
  //       _helloDisplay = response.message;
  //     });
  //     print('Greeter client received: ${response.message}');
  //   } catch (e) {
  //     print('Caught error: $e');
  //   }
  //   await channel.shutdown();
  // }
  Future<Null> _onSearchPressed() async {
    print('Searched Pressed');
    final channel = new ClientChannel(
      'localhost',
      port: 50051,
      options: const ChannelOptions(
        credentials: const ChannelCredentials.insecure(),
      ),
    );

    final bluetoothClient = new BluetoothRpcClient(channel);

    try {
      var response = await bluetoothClient.startScan(new StartScanRequest());
      print('Start Scan response: $response');
    } catch (e) {
      print('Error: $e');
    }

    await channel.shutdown();
  }

  Future<Null> _onListDevices() async {
    print('List Pressed');
    final channel = new ClientChannel(
      'localhost',
      port: 50051,
      options: const ChannelOptions(
        credentials: const ChannelCredentials.insecure(),
      ),
    );

    final bluetoothClient = new BluetoothRpcClient(channel);

    try {
      var response = await bluetoothClient.listFoundDevices(new ListFoundDevicesRequest());
      print('List Devices response: $response');
      setState(() {
        _devices = response.devices;
      });
    } catch (e) {
      print('Error: $e');
    }

    await channel.shutdown();
  }

  @override
  Widget build(BuildContext context) {
    var bluetoothBody = <Widget>[];
    _devices.forEach((device) {
      bluetoothBody.add(Text(
        device.name,
      ));
    });

    bluetoothBody.addAll([
        MaterialButton(
          onPressed: _onSearchPressed,
          child: Text(
            'Search for bluetooth devices',
          )
        ),
        MaterialButton(
          onPressed: _onListDevices,
          child: Text(
            'List bluetooth devices',
          )
        )
      ]);

    return Scaffold(
      appBar: AppBar(
        title: Text(widget.title),
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: bluetoothBody,
        ),
      ),
    );
  }
}
