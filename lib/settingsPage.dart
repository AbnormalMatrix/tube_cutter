import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_tube_cutter/src/rust/api/settings.dart';

class SettingsPage extends StatefulWidget {
  final CutterSettings cutterSettings;

  const SettingsPage({super.key, required this.cutterSettings});

  @override
  State<SettingsPage> createState() => _SettingsPageState();
}

class _SettingsPageState extends State<SettingsPage> {
  void onEditLaserX(String newValue) {
    if (newValue.isNotEmpty) {
      var newValueDouble = double.parse(newValue);
      widget.cutterSettings.laserOffsetX = newValueDouble;
      widget.cutterSettings.save();
    }
  }

  void onEditLaserY(String newValue) {
    if (newValue.isNotEmpty) {
      var newValueDouble = double.parse(newValue);
      widget.cutterSettings.laserOffsetY = newValueDouble;
      widget.cutterSettings.save();
    }
  }

  void onToggleLaser(bool newValue) {
    setState(() {
      widget.cutterSettings.useLaser = newValue;
      widget.cutterSettings.save();
    });
  }

  void onToggleHomeAfterCut(bool newValue) {
    setState(() {
      widget.cutterSettings.homeAfterCut = newValue;
      widget.cutterSettings.save();
    });
  }

  void onEditJogSpeed(String newValue) {
    widget.cutterSettings.jogSpeed = double.parse(newValue);
    widget.cutterSettings.save();
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        SettingsToggleItem(
          settingName: "Home After Cut",
          settingValue: widget.cutterSettings.homeAfterCut,
          onToggle: onToggleHomeAfterCut,
        ),
        SettingsItem(
          settingName: "Jog Speed",
          settingValue: widget.cutterSettings.jogSpeed,
          onEditFunc: onEditJogSpeed,
        ),
        SettingsToggleItem(
          settingName: "Use Laser",
          settingValue: widget.cutterSettings.useLaser,
          onToggle: onToggleLaser,
        ),
        SettingsItem(
          settingName: "Laser Offset X",
          settingValue: widget.cutterSettings.laserOffsetX,
          onEditFunc: onEditLaserX,
        ),
        SettingsItem(
          settingName: "Laser Offset Y",
          settingValue: widget.cutterSettings.laserOffsetY,
          onEditFunc: onEditLaserY,
        ),
      ],
    );
  }
}

class SettingsItem extends StatefulWidget {
  final String settingName;
  final double settingValue;
  final Function(String) onEditFunc;

  const SettingsItem({
    super.key,
    required this.settingName,
    required this.settingValue,
    required this.onEditFunc,
  });

  @override
  State<SettingsItem> createState() => _SettingsItemState();
}

class _SettingsItemState extends State<SettingsItem> {
  late TextEditingController _controller;

  @override
  void initState() {
    super.initState();
    _controller = TextEditingController(text: widget.settingValue.toString());
  }

  @override
  void didUpdateWidget(covariant SettingsItem oldWidget) {
    super.didUpdateWidget(oldWidget);
    // If settingValue changes from parent, update the controller text
    if (oldWidget.settingValue != widget.settingValue) {
      _controller.text = widget.settingValue.toString();
    }
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return ConstrainedBox(
      constraints: const BoxConstraints(maxWidth: 500),
      child: Card(
        margin: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
        clipBehavior: Clip.hardEdge,
        child: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
          child: Row(
            children: [
              Expanded(
                child: Text(
                  widget.settingName,
                  style: const TextStyle(fontSize: 18),
                ),
              ),
              SizedBox(
                width: 80,
                child: TextField(
                  onSubmitted: widget.onEditFunc,
                  controller: _controller,
                  inputFormatters: [
                    FilteringTextInputFormatter.allow(
                        RegExp(r'^[0-9]*\.?[0-9]*'))
                  ],
                  onChanged: widget.onEditFunc,
                  keyboardType:
                      const TextInputType.numberWithOptions(decimal: true),
                  textAlign: TextAlign.right,
                  decoration: const InputDecoration(
                    isDense: true,
                    contentPadding: EdgeInsets.zero,
                    border: InputBorder.none,
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class SettingsToggleItem extends StatelessWidget {
  final String settingName;
  final bool settingValue;
  final ValueChanged<bool> onToggle;

  const SettingsToggleItem({
    super.key,
    required this.settingName,
    required this.settingValue,
    required this.onToggle,
  });

  @override
  Widget build(BuildContext context) {
    return ConstrainedBox(
      constraints: const BoxConstraints(maxWidth: 500),
      child: Card(
        margin: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
        clipBehavior: Clip.hardEdge,
        child: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
          child: Row(
            children: [
              Expanded(
                child: Text(
                  settingName,
                  style: const TextStyle(fontSize: 18),
                ),
              ),
              Switch(
                value: settingValue,
                onChanged: onToggle,
              ),
            ],
          ),
        ),
      ),
    );
  }
}
