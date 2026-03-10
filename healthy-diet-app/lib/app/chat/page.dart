import 'package:flutter/material.dart';
import 'package:healthy_diet/utils/extensions/build_context.dart';
import 'package:healthy_diet/utils/extensions/edge_insets.dart';
import 'package:healthy_diet/widgets/typography.dart';
import 'package:material_symbols_icons/symbols.dart';

class ChatPage extends StatelessWidget {
  const ChatPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: context.padding.onlyBottom,
      child: Column(
        crossAxisAlignment: .stretch,
        children: [
          Expanded(
            child: SingleChildScrollView(
              padding: context.padding.onlyTop + .only(top: kToolbarHeight),
              child: Padding(
                padding: const EdgeInsets.symmetric(horizontal: 32, vertical: 24),
                child: Column(
                  crossAxisAlignment: .start,
                  spacing: 32,
                  children: [
                    Hero(
                      tag: 'welcome_text',
                      child: HeadLineText.large('午安，〇〇〇', weight: .bold),
                    ),
                    Wrap(
                      children: [],
                    ),
                  ],
                ),
              ),
            ),
          ),
          Padding(
            padding: .symmetric(horizontal: 24, vertical: 16),
            child: Hero(
              tag: 'chat_input',
              child: TextField(
                decoration: InputDecoration(
                  filled: true,
                  border: OutlineInputBorder(
                    borderSide: .none,
                    borderRadius: .circular(32),
                  ),
                  suffixIcon: Padding(
                    padding: const .all(4),
                    child: IconButton.filled(
                      style: IconButton.styleFrom(backgroundColor: context.colors.primary),
                      color: context.colors.onPrimary,
                      onPressed: () {},
                      icon: Icon(Symbols.arrow_right_alt_rounded),
                    ),
                  ),
                  hintText: '開始聊天...',
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }
}
