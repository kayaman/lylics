import { Artist, db, Lylic } from 'astro:db'

// https://astro.build/db/seed
export default async function seed() {
	await db.insert(Lylic).values([
		{ id: 1, quote: "My brain's a burger but my heart's the coal", artist: 'Modest Mouse' },
		{ id: 2, quote: 'The years go fast but the days go so slow', artist: 'Modest Mouse' },
		{ id: 3, quote: "The future's uncertain and the end is always near", artist: 'The Doors' },
		{
			id: 4,
			quote: 'Oh, my life is changing everyday, in every possible way',
			artist: 'The Cranberries',
		},
	])
}
