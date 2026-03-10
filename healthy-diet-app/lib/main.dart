import 'package:flutter/material.dart';
import 'package:healthy_diet/app.dart';
import 'package:healthy_diet/app/provider.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();

  await appProvider.load();

  runApp(const HealthyDiet());
}
