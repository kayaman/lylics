import { defineConfig } from 'astro/config';
import db from '@astrojs/db';

import vercel from "@astrojs/vercel/serverless";

// https://astro.build/config
export default defineConfig({
  output: 'server',
  integrations: [db()],
  adapter: vercel()
});