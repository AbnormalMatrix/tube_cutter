import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_tube_cutter/src/rust/api/cut.dart';
import 'package:flutter_tube_cutter/src/rust/api/simple.dart';
import 'package:flutter_tube_cutter/src/rust/api/gcode.dart';

class TunePage extends StatefulWidget {
  // this is the rust gcode struct, in dart you can't interact with the attributes but you can call the methods
  final ValueNotifier<Gcode> gcode;

  const TunePage({super.key, required this.gcode});

  @override
  State<TunePage> createState() => _TunePageState();
}

class _TunePageState extends State<TunePage> {
  num tubeWidth = 25.0;
  num cutAngle = 90.0;
  num pierceDelay = 0.5;

  num endX = 0;
  num endY = 0;

  bool shouldRepaintCanvas = false;

  Cut tubeCut = Cut();

  void onEnterWidth(String newNum) {
    if (newNum.isNotEmpty) {
      setState(() {
        tubeWidth = num.parse(newNum);
        // update the cut object
        tubeCut.setTubeWidth(newWidth: tubeWidth.toDouble());
        // update the end positions
        var endPos = tubeCut.getEndPos();
        endX = endPos.$1;
        endY = endPos.$2;
        shouldRepaintCanvas = true;
      });
    }
  }

  void onEnterCutAngle(String newNum) {
    if (newNum.isNotEmpty) {
      setState(() {
        cutAngle = num.parse(newNum);
        // update the cut object
        tubeCut.setCutAngle(newAngle: cutAngle.toDouble());
        // update the end positions
        var endPos = tubeCut.getEndPos();
        endX = endPos.$1;
        endY = endPos.$2;
        shouldRepaintCanvas = true;
      });
    }
  }

  void onEnterPierceDelay(String newNum) {
    setState(() {
      pierceDelay = num.parse(newNum);
      // update the cut object
      tubeCut.setPierceDelay(newDelay: pierceDelay.toDouble());
    });
  }

  void onAddCut() {
    if (tubeCut.isDisposed) {
      tubeCut = Cut();
      tubeCut.setTubeWidth(newWidth: tubeWidth.toDouble());
      tubeCut.setCutAngle(newAngle: cutAngle.toDouble());
      tubeCut.setPierceDelay(newDelay: pierceDelay.toDouble());
    }

    widget.gcode.value.addCut(tubeCut: tubeCut);
  }

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        // Settings column
        Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // tube width
            Tooltip(
              message: "The width of the square tube in milimeters.",
              child: Row(
                children: [
                  const SizedBox(
                    width: 120, // fixed width for labels
                    child: Text(
                      "Tube Width (mm)",
                      textAlign: TextAlign.right,
                    ),
                  ),
                  const VerticalDivider(),
                  SizedBox(
                    width: 250,
                    child: TextField(
                      keyboardType: TextInputType.number,
                      inputFormatters: [
                        FilteringTextInputFormatter.allow(
                            RegExp(r'^[0-9]+\.?[0-9]*'))
                      ],
                      onChanged: onEnterWidth,
                    ),
                  ),
                ],
              ),
            ),

            // cut angle
            Tooltip(
              message: "The angle to cut the tube in degrees.",
              child: Row(
                children: [
                  const SizedBox(
                    width: 120,
                    child: Text(
                      "Cut Angle",
                      textAlign: TextAlign.right,
                    ),
                  ),
                  const VerticalDivider(),
                  SizedBox(
                    width: 250,
                    child: TextField(
                      keyboardType: TextInputType.number,
                      onChanged: onEnterCutAngle,
                      inputFormatters: [
                        FilteringTextInputFormatter.allow(
                            RegExp(r'^[0-9]+\.?[0-9]*'))
                      ],
                    ),
                  ),
                ],
              ),
            ),
            // pierce delay
            Tooltip(
              message:
                  "The amount of time in seconds after the plasma is enabled that the toolhead starts moving",
              child: Row(
                children: [
                  const SizedBox(
                    width: 120,
                    child: Text(
                      "Pierce Delay",
                      textAlign: TextAlign.right,
                    ),
                  ),
                  const VerticalDivider(),
                  SizedBox(
                    width: 250,
                    child: TextField(
                      keyboardType: TextInputType.number,
                      onChanged: onEnterPierceDelay,
                      inputFormatters: [
                        FilteringTextInputFormatter.allow(
                            RegExp(r'^[0-9]+\.?[0-9]*'))
                      ],
                    ),
                  ),
                ],
              ),
            ),
            MainCanvas(
              tubeWidth: tubeWidth,
              shouldRepaintCanvas: shouldRepaintCanvas,
              endX: endX,
              endY: endY,
            ),
          ],
        ),
        VerticalDivider(),
        // add cut column
        Column(
          children: [
            FloatingActionButton.extended(
              onPressed: onAddCut,
              label: Text("Add Cut"),
              icon: Icon(Icons.add),
            )
          ],
        ),
      ],
    );
  }
}

class MainCanvas extends StatelessWidget {
  final num tubeWidth;
  final num endX;
  final num endY;

  final bool shouldRepaintCanvas;
  const MainCanvas(
      {required this.tubeWidth,
      required this.shouldRepaintCanvas,
      required this.endX,
      required this.endY});

  @override
  Widget build(BuildContext context) {
    return CustomPaint(
      size: Size(800, 600),
      painter: MainCanvasPainter(
          shouldRepaintCanvas: shouldRepaintCanvas,
          tubeWidth: tubeWidth,
          endX: endX,
          endY: endY),
    );
  }
}

class MainCanvasPainter extends CustomPainter {
  final num tubeWidth;
  final num endX;
  final num endY;

  final bool shouldRepaintCanvas;

  MainCanvasPainter(
      {required this.shouldRepaintCanvas,
      required this.tubeWidth,
      required this.endX,
      required this.endY});

  double mmToPx(double mm) {
    double scaleFactor = 4.0;
    return mm * scaleFactor;
  }

  @override
  void paint(Canvas canvas, Size size) {
    var paint = Paint()..color = Colors.blue;

    // the tube rect represents the actual tube
    // calculate the offset of the tube based on the size
    double tubeWidthPx = mmToPx(tubeWidth.toDouble());
    double tubeOffset = (size.width / 2) - (tubeWidthPx / 2);

    Rect tubeRect = Rect.fromLTWH(
        tubeOffset, 50, mmToPx(tubeWidth.toDouble()), size.height);
    canvas.drawRect(tubeRect, paint);

    // the cut line represents the cut
    double machineOriginX = (size.width / 2) - (tubeWidthPx / 2);
    double machineOriginY = (size.height + 50) / 2;
    // the start pos
    Offset startPos = Offset(machineOriginX, machineOriginY);
    // the end pos
    double endPosX = machineOriginX + mmToPx(endX.toDouble());
    double endPosY = machineOriginY - mmToPx(endY.toDouble());
    Offset endPos = Offset(endPosX, endPosY);

    // set the color and stroke of the line
    paint.color = Colors.red;
    paint.strokeWidth = 10;
    paint.strokeCap = StrokeCap.round;

    // draw the cut line
    canvas.drawLine(startPos, endPos, paint);
  }

  @override
  bool shouldRepaint(CustomPainter oldDelegate) {
    return shouldRepaintCanvas;
  }
}
