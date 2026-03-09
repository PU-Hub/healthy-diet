//這個是之後比較方便連頁面的地方，可以先空白

import 'package:flutter/material.dart';
import 'package:healthy_diet/app.dart';
import 'main.dart';
import 'side_menu.dart';
import 'package:flutter/material.dart';

class ClockPage extends StatelessWidget {
  const ClockPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text("自訂定時提醒"), centerTitle: true),
    );
  }
}
