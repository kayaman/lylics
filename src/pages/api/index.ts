import type { APIRoute } from 'astro'
import { db, sql, Lylic } from 'astro:db'

type Quote = {
	quote: string
	artist: string
}

// GET /
export const GET: APIRoute = async () => {
	const rs = await db.select().from(Lylic).orderBy(sql`random()`).limit(1)
	const {artist, quote} = rs[0]
	return new Response(`${quote} ~~ ${artist}`)
}

export const POST: APIRoute = async ({ request }) => {
	const body = await request.json()
	console.log({ body })
	const { quote, artist } = body
	const lylic = await db.insert(Lylic).values({ quote, artist }).returning()
	return new Response(JSON.stringify(lylic))
}
