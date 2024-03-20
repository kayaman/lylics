import { eq } from '@astrojs/db/dist/runtime/config.js';
import { createRemoteDatabaseClient, asDrizzleTable } from '@astrojs/db/runtime';

const db = await createRemoteDatabaseClient(process.env.ASTRO_STUDIO_APP_TOKEN ?? "f65eda345a8a35890a134d7b0abbdba0c71f3adb:gbaiaa9cxssareb97h7hodpxrr6d", {"BASE_URL": "/", "MODE": "production", "DEV": false, "PROD": true, "SSR": true, "SITE": undefined, "ASSETS_PREFIX": undefined}.ASTRO_STUDIO_REMOTE_DB_URL ?? "https://db.services.astro.build");
const Lylic = asDrizzleTable("Lylic", { "columns": { "id": { "type": "number", "schema": { "unique": false, "deprecated": false, "name": "id", "collection": "Lylic", "primaryKey": true } }, "quote": { "type": "text", "schema": { "unique": false, "deprecated": false, "name": "quote", "collection": "Lylic", "primaryKey": false, "optional": false } }, "artist": { "type": "text", "schema": { "unique": false, "deprecated": false, "name": "artist", "collection": "Lylic", "primaryKey": false, "optional": false } } }, "deprecated": false }, false);

const GET$1 = async ({ params }) => {
  const { id } = params;
  if (!id) {
    return new Response(JSON.stringify({ error: "id is required" }), { status: 400 });
  }
  const lylic = await db.select().from(Lylic).where(eq(Lylic.id, +id));
  return new Response(JSON.stringify(lylic), { status: 200 });
};
const PATCH = async ({ params, request }) => {
  const { id } = params;
  if (!id) {
    return new Response(JSON.stringify({ error: "id is required" }), { status: 400 });
  }
  const body = await request.json();
  console.log({ body });
  const lylic = await db.update(Lylic).set(body).where(eq(Lylic.id, +id)).returning();
  return new Response(JSON.stringify(lylic), { status: 200 });
};
const DELETE = async ({ params }) => {
  const { id } = params;
  if (!id) {
    return new Response(JSON.stringify({ error: "id is required" }), { status: 400 });
  }
  await db.delete(Lylic).where(eq(Lylic.id, +id));
  return new Response(null, { status: 204 });
};

const index$1 = /*#__PURE__*/Object.freeze(/*#__PURE__*/Object.defineProperty({
	__proto__: null,
	DELETE,
	GET: GET$1,
	PATCH
}, Symbol.toStringTag, { value: 'Module' }));

const GET = async () => {
  const lylics = await db.select().from(Lylic).all();
  console.log({ lylics });
  return new Response(JSON.stringify(lylics));
};
const POST = async ({ request }) => {
  const body = await request.json();
  console.log({ body });
  const { quote, artist } = body;
  const lylic = await db.insert(Lylic).values({ quote, artist }).returning();
  return new Response(JSON.stringify(lylic));
};

const index = /*#__PURE__*/Object.freeze(/*#__PURE__*/Object.defineProperty({
	__proto__: null,
	GET,
	POST
}, Symbol.toStringTag, { value: 'Module' }));

export { index as a, index$1 as i };
