import 'package:flutter/material.dart';
import 'package:flutter_tube_cutter/src/rust/api/gcode.dart';

class GcodePreviewPage extends StatefulWidget {
  final ValueNotifier<Gcode> gcode;
  const GcodePreviewPage({super.key, required this.gcode});

  @override
  State<GcodePreviewPage> createState() => _GcodePreviewPageState();
}

class _GcodePreviewPageState extends State<GcodePreviewPage> {
  late String gcodeValue;

  @override
  void initState() {
    super.initState();
    gcodeValue = widget.gcode.value.getGcodeString();
    widget.gcode.addListener(() {
      setState(() {
        gcodeValue = widget.gcode.value.getGcodeString();
        print("Gcode updated!");
      });
    });
  }

  void onExportPressed() {}

  void resetGcode() {
    setState(() {
      widget.gcode.value = Gcode();
    });
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Text(gcodeValue,
            style: TextStyle(fontFamily: 'monospace', fontSize: 14)),
        FloatingActionButton.extended(
          onPressed: onExportPressed,
          icon: Icon(Icons.save),
          label: Text("Export Gcode"),
        ),
        FloatingActionButton.extended(
          onPressed: resetGcode,
          label: Text("Reset"),
        ),
      ],
    );
  }
}
