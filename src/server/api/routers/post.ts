import { type } from 'arktype';

import { createTRPCRouter, protectedProcedure, publicProcedure } from '@/server/api/trpc';
import { posts } from '@/server/db/schema';

export const postRouter = createTRPCRouter({
  create: protectedProcedure
    .input(type({
      name: type('string > 0'),
    }))
    .mutation(async ({ ctx, input }) => {
      await ctx.db.insert(posts).values({
        createdById: ctx.session.user.id,
        name: input.name,
      });
    }),

  getLatest: protectedProcedure.query(async ({ ctx }) => {
    const post = await ctx.db.query.posts.findFirst({
      orderBy: (posts, { desc }) => [desc(posts.createdAt)],
    });

    return post ?? null;
  }),

  getSecretMessage: protectedProcedure.query(() => {
    return 'you can now see this secret message!';
  }),

  hello: publicProcedure
    .input(type({
      text: type('string'),
    }))
    .query(({ input }) => {
      return {
        greeting: `Hello ${input.text}`,
      };
    }),
});
