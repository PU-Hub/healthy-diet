import 'package:flutter/material.dart';
import 'package:healthy_diet/router.dart';
import 'package:healthy_diet/utils/extensions/build_context.dart';
import 'package:healthy_diet/utils/extensions/edge_insets.dart';
import 'package:healthy_diet/widgets/typography.dart';
import 'package:material_segmented_list/material_segmented_list.dart';

class HomePage extends StatelessWidget {
  const HomePage({super.key});

  @override
  Widget build(BuildContext context) {
    return SingleChildScrollView(
      padding: context.padding.onlyTop + .symmetric(vertical: 16),
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
            Hero(
              tag: 'chat_input',
              child: TextField(
                decoration: InputDecoration(
                  filled: true,
                  border: OutlineInputBorder(
                    borderSide: .none,
                    borderRadius: .circular(32),
                  ),
                  hintText: '開始聊天...',
                ),
                readOnly: true,
                onTap: () {
                  ChatRoute().push(context);
                },
              ),
            ),
            GridView.count(
              padding: .zero,
              physics: const NeverScrollableScrollPhysics(),
              mainAxisExtent: 96,
              crossAxisCount: 2,
              shrinkWrap: true,
              children: [Card.filled(), Card.filled(), Card.filled(), Card.filled()],
            ),
            SegmentedListSection(
              children: List.generate(
                6,
                (index) => SegmentedListTile(
                  title: Text('聊天歷史 $index'),
                  visualDensity: .compact,
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
