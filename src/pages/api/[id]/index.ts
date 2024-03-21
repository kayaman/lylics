import type { APIRoute } from 'astro';
import { db, eq, Lylic } from 'astro:db';

export const GET: APIRoute = async ({ params }) => {
  const { id } = params;
  if (!id) {
    return new Response(JSON.stringify({ error: 'id is required' }), { status: 400 });
  }

  const lylic = await db.select().from(Lylic).where(eq(Lylic.id, +id));

  return new Response(JSON.stringify(lylic), { status: 200 });
};

export const PATCH: APIRoute = async ({ params, request }) => {
  const { id } = params;
  if (!id) {
    return new Response(JSON.stringify({ error: 'id is required' }), { status: 400 });
  }

  const body = await request.json();
  console.log({ body });

  const lylic = await db.update(Lylic).set(body).where(eq(Lylic.id, +id)).returning();

  return new Response(JSON.stringify(lylic), { status: 200 });
};

export const DELETE: APIRoute = async ({ params }) => {
  const { id } = params;
  if (!id) {
    return new Response(JSON.stringify({ error: 'id is required' }), { status: 400 });
  }
  const lylic = await db.delete(Lylic).where(eq(Lylic.id, +id));
  return new Response(null, { status: 204 });
};
