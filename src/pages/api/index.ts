import type { APIRoute } from 'astro'
import { db, Lylic } from 'astro:db'
// GET /
export const GET: APIRoute = async () => {
	const lylics = await db.select().from(Lylic).all()
	console.log({ lylics })
	return new Response(JSON.stringify(lylics))
}

export const POST: APIRoute = async ({ request }) => {
	const body = await request.json()
	console.log({ body })
	const { quote, artist } = body
	const lylic = await db.insert(Lylic).values({ quote, artist }).returning()
	return new Response(JSON.stringify(lylic))
}
