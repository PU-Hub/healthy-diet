import 'package:flutter/material.dart';
import 'package:healthy_diet/app.dart';
import 'dart:convert';
import 'package:http/http.dart' as http;
import 'dart:io';
import 'SideMenu.dart';
import 'package:image_picker/image_picker.dart';

void main() {
  WidgetsFlutterBinding.ensureInitialized();

  runApp(const HealthyDiet());
}

//我的

class reHome extends StatefulWidget {
  const reHome({super.key});

  @override
  State<reHome> createState() => _reHomeState();
}

class _reHomeState extends State<reHome> {
  bool _isChatActive = false;

  final TextEditingController _textController = TextEditingController();
  final List<Map<String, String>> _messages = [];

  File? _selectedImage;
  final ImagePicker _picker = ImagePicker();

  Future<void> _pickImage() async {
    final XFile? pickedFile = await _picker.pickImage(
      source: ImageSource.gallery,
      imageQuality: 50,
    );
    if (pickedFile != null) {
      setState(() {
        _selectedImage = File(pickedFile.path);
        _isChatActive = true;
      });
    }
  }

  Future<void> _handleSubmitted(String text) async {
    if (text.trim().isEmpty) return;

    setState(() {
      _messages.add({
        "role": "user",
        "text": text,
        "image": _selectedImage?.path ?? "",
      });
      _isChatActive = true;
    });

    final File? imageToSend = _selectedImage;
    setState(() {
      _selectedImage = null;
      _textController.clear();
    });

    String? base64Image;
    if (imageToSend != null) {
      List<int> imageBytes = await imageToSend.readAsBytes();
      base64Image = base64Encode(imageBytes);
    }

    final url = Uri.parse("http://127.0.0.1:3000/chat");

    try {
      final response = await http.post(
        url,
        headers: {"Content-Type": "application/json"},
        body: jsonEncode({"message": text, "image": base64Image}),
      );
      if (response.statusCode == 200) {
        final data = jsonDecode(response.body);
        final aiReply = data['reply'];

        if (!mounted) return;
        setState(() {
          _messages.add({"role": "ai", "text": aiReply});
        });
      } else {
        print("請求失敗:${response.statusCode}");
        setState(() {
          _messages.add({"role": "ai", "text": "連線錯誤"});
        });
      }
    } catch (e) {
      print("發生錯誤:$e");
      setState(() {
        _messages.add({"role": "ai", "text": "連線錯誤"});
      });
    }
    Future.delayed(const Duration(milliseconds: 1000), () {
      if (!mounted) return;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      drawerScrimColor: Colors.transparent,

      appBar: AppBar(
        backgroundColor: const Color(0xFF6678FF),
        elevation: 0,
        title: Text(
          "首頁",
          style: TextStyle(color: Colors.black, fontWeight: FontWeight.bold),
        ),
        centerTitle: true,

        iconTheme: const IconThemeData(color: Colors.black),

        actions: [
          Container(
            margin: const EdgeInsets.only(right: 16),
            child: CircleAvatar(
              backgroundColor: Colors.grey,
              radius: 18,
              child: const Icon(Icons.person, color: Colors.white),
            ),
          ),
        ],
      ),

      drawer: const SideMenu(),

      body: Container(
        color: Colors.white,
        child: Column(
          children: [
            Expanded(
              child: _isChatActive ? _buildChatContent() : _buildWelcomeContent(),
            ),

            AnimatedContainer(
              duration: const Duration(milliseconds: 500),
              curve: Curves.easeInOutCubic,

              margin: EdgeInsets.only(
                bottom: _isChatActive ? 0 : MediaQuery.of(context).size.height * 0.25,
              ),
              padding: const EdgeInsets.all(16),
              color: Colors.white,
              child: Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  if (_selectedImage != null)
                    Container(
                      margin: const EdgeInsets.only(bottom: 10),
                      decoration: BoxDecoration(
                        border: Border.all(color: Colors.grey),
                        borderRadius: BorderRadius.circular(10),
                      ),
                      child: Stack(
                        children: [
                          Image.file(_selectedImage!),
                          Positioned(
                            right: 0,
                            top: 0,
                            child: IconButton(
                              icon: const Icon(Icons.close, color: Colors.red),
                              onPressed: () => setState(() => _selectedImage = null),
                            ),
                          ),
                        ],
                      ),
                    ),
                  Container(
                    height: 50,
                    decoration: BoxDecoration(
                      color: Colors.grey[300],
                      borderRadius: BorderRadius.circular(25),
                    ),

                    child: Row(
                      children: [
                        const SizedBox(width: 20),
                        Icon(Icons.edit, color: Colors.grey[600], size: 20),
                        const SizedBox(width: 10),

                        IconButton(
                          icon: const Icon(
                            Icons.camera_alt,
                            color: Colors.grey,
                          ),
                          onPressed: _pickImage,
                        ),
                        Expanded(
                          child: TextField(
                            controller: _textController,
                            onSubmitted: _handleSubmitted,

                            onTap: () {
                              setState(() {
                                _isChatActive = true;
                              });
                            },
                            decoration: InputDecoration(
                              hintText: _isChatActive ? "請輸入內容或是傳照片.." : "今天你想吃什麼",
                              hintStyle: TextStyle(color: Colors.grey[600]),
                              border: InputBorder.none,
                            ),
                          ),
                        ),

                        IconButton(
                          icon: const Icon(Icons.send, color: Colors.blue),
                          onPressed: () => _handleSubmitted(_textController.text),
                        ),
                      ],
                    ),
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildWelcomeContent() {
    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          const Text(
            "早安,xxx",
            style: TextStyle(
              fontSize: 32,
              fontWeight: FontWeight.bold,
              letterSpacing: 1.5,
            ),
          ),

          const SizedBox(height: 30),
          Container(
            width: 200,
            height: 15,
            decoration: BoxDecoration(
              color: Colors.grey[200],
              borderRadius: BorderRadius.circular(10),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildChatContent() {
    return ListView.builder(
      padding: const EdgeInsets.all(20),
      itemCount: _messages.length,
      itemBuilder: (context, index) {
        final msg = _messages[index];
        final isUser = msg['role'] == "user";
        final imagePath = msg['image'];

        return Align(
          alignment: isUser ? Alignment.centerRight : Alignment.centerLeft,
          child: Container(
            padding: const EdgeInsets.all(16),
            margin: const EdgeInsets.only(bottom: 20),

            constraints: BoxConstraints(
              maxWidth: MediaQuery.of(context).size.width * 0.7,
            ),
            decoration: BoxDecoration(
              color: isUser ? const Color(0xFF40C4FF) : Colors.grey[300],
              borderRadius: BorderRadius.only(
                topLeft: const Radius.circular(20),
                topRight: const Radius.circular(20),
                bottomLeft: isUser ? const Radius.circular(20) : const Radius.circular(5),
                bottomRight: isUser ? const Radius.circular(5) : const Radius.circular(20),
              ),
            ),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                if (imagePath != null && imagePath.isNotEmpty)
                  Padding(
                    padding: const EdgeInsets.only(bottom: 8.0),
                    child: Image.file(File(imagePath), height: 150),
                  ),

                Text(
                  msg['text'] ?? "",
                  style: TextStyle(
                    color: isUser ? Colors.white : Colors.black,
                    fontSize: 16,
                  ),
                ),
              ],
            ),
          ),
        );
      },
    );
  }
}
