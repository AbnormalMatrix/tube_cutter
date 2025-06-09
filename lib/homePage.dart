import 'package:flutter/material.dart';

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(16.0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            '📏 Tube Cutter',
            style: Theme.of(context).textTheme.titleLarge,
          ),
          const SizedBox(height: 12),
          const Text(
              'Tube Cutter is a tool for generating simple G‑code to cut square tubing on a CNC plasma cutter.'
              ''),
          const SizedBox(height: 8),
          const Text(
            '🛠️ Usage:',
            style: TextStyle(fontWeight: FontWeight.bold),
          ),
          const SizedBox(height: 4),
          const Text('• Define parameters in the setup tab'),
          const Text('• Preview G-code in the preview tab'),
          const Text('• Connect to the machine and run in the run job tab'),
        ],
      ),
    );
    ;
  }
}
