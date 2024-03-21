import { defineConfig } from 'astro/config';
import db from '@astrojs/db';
import vercel from "@astrojs/vercel/serverless";
import openapiBackend from "@astro-openapi/backend";
import openapiTypegen from "@astro-openapi/typegen";
import openapiBundler from "@astro-openapi/bundler";
import openapiClient from "@astro-openapi/client";
import openapiGuiSwagger from "@astro-openapi/gui-swagger";
import openapiGuiRedoc from "@astro-openapi/gui-redoc";

// https://astro.build/config
export default defineConfig({
  output: 'server',
  integrations: [db(), openapiBackend(), openapiTypegen(), openapiBundler(), openapiClient(), openapiGuiSwagger(), openapiGuiRedoc()],
  adapter: vercel()
});