import { column, defineDb, defineTable } from 'astro:db'

const Artist = defineTable({
	columns: {
		id: column.number({ primaryKey: true }),
		name: column.text(),
	},
})

const Lylic = defineTable({
	columns: {
		id: column.number({ primaryKey: true }),
		text: column.text(),
		artistId: column.number({ references: () => Artist.columns.id }),
	},
})

// https://astro.build/db/config
export default defineDb({
	tables: {},
})
