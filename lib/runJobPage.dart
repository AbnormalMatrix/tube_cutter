import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_tube_cutter/src/rust/api/gcode.dart';
import 'package:flutter_tube_cutter/src/rust/api/sender.dart';
import 'package:flutter_tube_cutter/src/rust/api/simple.dart';

class RunJobPage extends StatefulWidget {
  final ValueNotifier<Gcode> gcode;
  const RunJobPage({super.key, required this.gcode});

  @override
  State<RunJobPage> createState() => _RunJobPageState();
}

class _RunJobPageState extends State<RunJobPage> {
  List<String> serialPorts = [];

  String? selectedSerialPort = null;

  bool isConnected = false;

  JogDist selectedJogDist = JogDist.one;

  // machine connection object (rust)
  MachineConnection machineConnection = MachineConnection();

  void getSerialPortsPressed() {
    setState(() {
      serialPorts = getSerialPorts();
      if (selectedSerialPort != null) {
        selectedSerialPort = serialPorts.first;
      }
    });
  }

  void serialPortSelectedPressed(value) {
    setState(() {
      selectedSerialPort = value;
      machineConnection.setSerialPort(newPort: value);
    });
  }

  void connectToMachine() {
    machineConnection.makeConnection();
    setState(() {
      isConnected = true;
    });
  }

  void selectJogDist(Set<JogDist> newSelection) {
    setState(() {
      selectedJogDist = newSelection.first;
    });
  }

  void jogButtonPressed(x, y) {
    switch (selectedJogDist) {
      case JogDist.pointOne:
        machineConnection.sendStringCommandLowPriority(
            command: jog(xDist: x * 0.1, yDist: y * 0.1));
        break;
      case JogDist.one:
        machineConnection.sendStringCommandLowPriority(
            command: jog(xDist: x, yDist: y));
      case JogDist.ten:
        machineConnection.sendStringCommandLowPriority(
            command: jog(xDist: x * 10.0, yDist: y * 10.0));
      default:
    }
  }

  @override
  Widget build(BuildContext context) {
    return Column(children: [
      // the main row of the top bar
      Row(
        children: [
          // the row for the serial port list and refresh icon button
          Row(
            children: [
              IconButton(
                  onPressed: getSerialPortsPressed, icon: Icon(Icons.refresh)),
              DropdownButton(
                  value: selectedSerialPort,
                  items:
                      serialPorts.map<DropdownMenuItem<String>>((String value) {
                    return DropdownMenuItem<String>(
                      value: value,
                      child: Text(value),
                    );
                  }).toList(),
                  onChanged: serialPortSelectedPressed),
              Padding(
                padding: const EdgeInsets.all(8.0),
                child: ElevatedButton.icon(
                    onPressed: connectToMachine,
                    label: Text("Connect"),
                    icon: Icon(Icons.usb)),
              ),
            ],
          )
        ],
      ),
      Divider(),

      Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // this has the x and y position cards
          ConstrainedBox(
            constraints: BoxConstraints(minWidth: 150, maxWidth: 400),
            child: Column(
              children: [
                AxisInfoCard(
                  axisColor: Colors.red,
                  axisName: "X",
                  axisValue: 0.00,
                ),
                AxisInfoCard(
                  axisColor: Colors.green,
                  axisName: "Y",
                  axisValue: 0.00,
                ),
              ],
            ),
          ),
          SizedBox(
            height: 300,
            width: 300,
            child: Column(
              children: [
                JogArrows(
                  machineConnection: machineConnection,
                ),
                Padding(
                  padding: const EdgeInsets.only(top: 8.0),
                  child: JogDistSelect(
                    selectedDist: selectedJogDist,
                    onSelectionChanged: selectJogDist,
                  ),
                ),
              ],
            ),
          ),
        ],
      ),
      Divider(),
      ElevatedButton.icon(
        onPressed: () {
          machineConnection.sendGcodeCommand(
              command: widget.gcode.value.getGcodeString());
        },
        label: Text("Run Job"),
        icon: Icon(Icons.play_arrow),
      ),
      ElevatedButton.icon(
        onPressed: () {
          machineConnection.home();
        },
        label: Text("Home"),
        icon: Icon(Icons.home),
      )
    ]);
  }
}

class AxisInfoCard extends StatelessWidget {
  final Color axisColor;
  final String axisName;
  final num axisValue;
  const AxisInfoCard(
      {super.key,
      required this.axisColor,
      required this.axisName,
      required this.axisValue});

  @override
  Widget build(BuildContext context) {
    return Card(
      elevation: 4,
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
        child: Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            Text(
              axisName,
              style: TextStyle(
                  fontSize: 18, fontWeight: FontWeight.bold, color: axisColor),
            ),
            Text(
              axisValue.toString(),
              style: TextStyle(fontSize: 18, color: axisColor),
            ),
          ],
        ),
      ),
    );
  }
}

enum JogDist { pointOne, one, ten }

class JogArrows extends StatelessWidget {
  final MachineConnection machineConnection;
  const JogArrows({super.key, required this.machineConnection});

  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        // Up button
        ElevatedButton(
          onPressed: () {
            machineConnection.sendStringCommandLowPriority(
                command: jog(xDist: 0, yDist: 1.0));
          },
          child: Column(
            children: [
              Icon(
                Icons.north,
                size: 24,
                color: Colors.green,
              ),
              Text(
                "Y+",
                style: TextStyle(color: Colors.green, fontSize: 18),
              ),
            ],
          ),
        ),
        // Left, Spacer, Right
        Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            ElevatedButton(
              onPressed: () {
                machineConnection.sendStringCommandLowPriority(
                    command: jog(xDist: -1.0, yDist: 0));
              },
              child: Row(
                children: [
                  Padding(
                    padding: const EdgeInsets.only(right: 8.0),
                    child: Icon(
                      Icons.west,
                      color: Colors.red,
                      size: 24,
                    ),
                  ),
                  Text(
                    "X-",
                    style: TextStyle(fontSize: 18, color: Colors.red),
                  ),
                ],
              ),
            ),
            const SizedBox(width: 20),
            ElevatedButton(
              onPressed: () {
                machineConnection.sendStringCommandLowPriority(
                    command: jog(xDist: 1.0, yDist: 0));
              },
              child: Row(
                children: [
                  Text(
                    "X+",
                    style: TextStyle(fontSize: 18, color: Colors.red),
                  ),
                  Padding(
                    padding: const EdgeInsets.only(left: 8.0),
                    child: Icon(
                      Icons.east,
                      size: 24,
                      color: Colors.red,
                    ),
                  ),
                ],
              ),
            ),
          ],
        ),
        // Down button
        ElevatedButton(
          onPressed: () {
            machineConnection.sendStringCommandLowPriority(
                command: jog(xDist: 0, yDist: -1.0));
          },
          child: Column(
            children: [
              Text(
                "Y-",
                style: TextStyle(fontSize: 18, color: Colors.green),
              ),
              Icon(
                Icons.south,
                size: 24,
                color: Colors.green,
              ),
            ],
          ),
        ),
      ],
    );
  }
}

class JogDistSelect extends StatelessWidget {
  final JogDist selectedDist;
  final void Function(Set<JogDist>) onSelectionChanged;
  const JogDistSelect(
      {super.key,
      required this.selectedDist,
      required this.onSelectionChanged});

  @override
  Widget build(BuildContext context) {
    return SegmentedButton<JogDist>(
      segments: const <ButtonSegment<JogDist>>[
        ButtonSegment<JogDist>(value: JogDist.pointOne, label: Text("0.1")),
        ButtonSegment<JogDist>(value: JogDist.one, label: Text("1.0")),
        ButtonSegment<JogDist>(value: JogDist.ten, label: Text("10.0")),
      ],
      selected: <JogDist>{selectedDist},
      onSelectionChanged: onSelectionChanged,
    );
  }
}
