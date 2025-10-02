import { env } from '@/env';

import type { Config } from 'drizzle-kit';

export default {
  dbCredentials: {
    url: env.DATABASE_URL,
  },
  dialect: 'postgresql',
  schema: './src/server/db/schema.ts',
  tablesFilter: ['healthy-diet_*'],
} satisfies Config;
