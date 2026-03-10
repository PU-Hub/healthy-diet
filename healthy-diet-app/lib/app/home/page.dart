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
                    HeadLineText.large('午安，〇〇〇', weight: .bold),
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
                  hintText: '開始聊天...',
                ),
                readOnly: true,
                onTap: () {
                  ChatRoute().push(context);
                },
              ),
            ),
          ),
        ],
      ),
    );
  }
}
