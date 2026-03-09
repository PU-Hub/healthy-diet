//這個先是第一版
import 'package:flutter/material.dart';
import 'package:healthy_diet/app.dart';
import 'main.dart';
import 'clock_page.dart';

class SideMenu extends StatelessWidget {
  const SideMenu({super.key});

  @override
  Widget build(BuildContext context) {
    return Drawer(
      backgroundColor: const Color(0xCCFF6F00),
      width: 300,
      child: SafeArea(
        child: Padding(
          padding: const EdgeInsetsGeometry.all(24.0),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Align(
                alignment: Alignment.centerRight,
                child: IconButton(
                  icon: const Icon(
                    Icons.arrow_back_ios,
                    color: Colors.black,
                    size: 30,
                  ),
                  onPressed: () => Navigator.pop(context),
                ),
              ),

              const SizedBox(height: 20),

              const Text(
                "歷史對話",
                style: TextStyle(fontSize: 20, fontWeight: FontWeight.bold),
              ),
              const Divider(color: Colors.black, thickness: 2),
              const SizedBox(height: 15),

              _buildHistoryItem(),
              _buildHistoryItem(),
              _buildHistoryItem(),

              const Spacer(),

              const Text(
                "其他功能",
                style: TextStyle(fontSize: 20, fontWeight: FontWeight.bold),
              ),

              const Divider(color: Colors.black, thickness: 2),
              const SizedBox(height: 15),

              _buildFunctionButton(
                context,
                "自訂定時提醒",
                const Color(0xFFFF3333),
                () {
                  Navigator.pop(context);
                  Navigator.push(
                    context,
                    MaterialPageRoute(builder: (context) => const ClockPage()),
                  );
                },
              ),

              _buildFunctionButton(
                context,
                "最近飲食資料",
                const Color(0xFF199400),
                () {
                  Navigator.pop(context);
                  Navigator.push(
                    context,
                    MaterialPageRoute(builder: (context) => const ClockPage()),
                    //這個是要之後方便連接"最近飲食資料"的頁面
                  );
                },
              ),

              _buildFunctionButton(
                context,
                "編輯個人資料",
                const Color(0xFFAB67AD),
                () {
                  Navigator.pop(context);
                  Navigator.push(
                    context,
                    MaterialPageRoute(builder: (context) => const ClockPage()),
                    //這個是要之後方便連接"編輯個人資料"的頁面
                  );
                },
              ),

              const SizedBox(height: 20),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildHistoryItem() {
    return Container(
      margin: const EdgeInsets.only(bottom: 15),
      height: 45,
      decoration: BoxDecoration(
        color: const Color(0xFFE0E0E0),
        borderRadius: BorderRadius.circular(25),
      ),
    );
  }

  Widget _buildFunctionButton(
    BuildContext context,
    String text,
    Color color,
    VoidCallback onTap,
  ) {
    return Container(
      margin: const EdgeInsets.only(bottom: 15),
      width: double.infinity,
      height: 50,
      child: ElevatedButton(
        style: ElevatedButton.styleFrom(
          backgroundColor: color,
          foregroundColor: Colors.black,
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadiusGeometry.circular(25),
          ),
          elevation: 0,
        ),
        onPressed: onTap,
        child: Text(
          text,
          style: const TextStyle(fontSize: 16, fontWeight: FontWeight.bold),
        ),
      ),
    );
  }
}
