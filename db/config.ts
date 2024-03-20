import { column, defineDb, defineTable } from 'astro:db'

const Lylic = defineTable({
	columns: {
		id: column.number({ primaryKey: true }),
		quote: column.text(),
		artist: column.text(),
	},
})

// https://astro.build/db/config
export default defineDb({
	tables: { Lylic },
})
