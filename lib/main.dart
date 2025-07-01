import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_tube_cutter/gcodePreviewPage.dart';
import 'package:flutter_tube_cutter/homePage.dart';
import 'package:flutter_tube_cutter/runJobPage.dart';
import 'package:flutter_tube_cutter/settingsPage.dart';
import 'package:flutter_tube_cutter/src/rust/api/gcode.dart';
import 'package:flutter_tube_cutter/src/rust/api/sender.dart';
import 'package:flutter_tube_cutter/src/rust/api/simple.dart';
import 'package:flutter_tube_cutter/src/rust/frb_generated.dart';
import 'package:flutter_tube_cutter/tunePage.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  int selectedIndex = 0;

  // this is the rust gcode struct, in dart you can't interact with the attributes but you can call the methods
  ValueNotifier<Gcode> gcode = ValueNotifier<Gcode>(Gcode());

  num laserOffset = 40.0;

  // this is the rust machine connection struct, used for communicating with the machine
  // it is here to remain alive while the app is open regardless of what page you are on
  MachineConnection machineConnection = MachineConnection();

  // this is the machine's x and y coordinates in a value notifier so it can be subscribed to
  ValueNotifier<MachinePosition> machinePosition =
      ValueNotifier<MachinePosition>(MachinePosition());

  void onNavItemTapped(int index) {
    setState(() {
      selectedIndex = index;
    });
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      theme:
          ThemeData(colorScheme: ColorScheme.fromSeed(seedColor: Colors.green)),
      home: Scaffold(
        appBar: AppBar(
          title: const Text("Tube Cutter"),
          backgroundColor: Colors.green,
        ),
        body: Row(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            MainNavRail(
              selectedIndex: selectedIndex,
              onDestinationSelected: onNavItemTapped,
            ),
            const VerticalDivider(),
            Expanded(
              child: [
                HomePage(),
                SettingsPage(),
                TunePage(gcode: gcode),
                GcodePreviewPage(
                  gcode: gcode,
                ),
                RunJobPage(
                  gcode: gcode,
                  machineConnection: machineConnection,
                  machinePosition: machinePosition,
                ),
              ][selectedIndex],
            ),
          ],
        ),
      ),
    );
  }
}

class MainNavRail extends StatelessWidget {
  final int selectedIndex;
  final ValueChanged<int> onDestinationSelected;

  const MainNavRail({
    super.key,
    required this.selectedIndex,
    required this.onDestinationSelected,
  });

  @override
  Widget build(BuildContext context) {
    return NavigationRail(
      labelType: NavigationRailLabelType.all,
      selectedIndex: selectedIndex,
      onDestinationSelected: onDestinationSelected,
      destinations: const [
        NavigationRailDestination(
          icon: Icon(Icons.home),
          label: Text('Home'),
        ),
        NavigationRailDestination(
          icon: Icon(Icons.settings),
          label: Text('Settings'),
        ),
        NavigationRailDestination(icon: Icon(Icons.tune), label: Text("Setup")),
        NavigationRailDestination(
            icon: Icon(Icons.code), label: Text("Gcode Preview")),
        NavigationRailDestination(
            icon: Icon(Icons.outbond), label: Text("Run Job")),
      ],
    );
  }
}
