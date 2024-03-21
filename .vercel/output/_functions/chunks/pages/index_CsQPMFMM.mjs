import { d as db, L as Lylic, __tla as __tla_0 } from "./index_CThLp5Fz.mjs";
import { eq, sql } from "@astrojs/db/dist/runtime/config.js";
let index, index$1;
let __tla = Promise.all([
    (()=>{
        try {
            return __tla_0;
        } catch  {}
    })()
]).then(async ()=>{
    const GET$1 = async ({ params })=>{
        const { id } = params;
        if (!id) {
            return new Response(JSON.stringify({
                error: "id is required"
            }), {
                status: 400
            });
        }
        const lylic = await db.select().from(Lylic).where(eq(Lylic.id, +id));
        return new Response(JSON.stringify(lylic), {
            status: 200
        });
    };
    const PATCH = async ({ params, request })=>{
        const { id } = params;
        if (!id) {
            return new Response(JSON.stringify({
                error: "id is required"
            }), {
                status: 400
            });
        }
        const body = await request.json();
        console.log({
            body
        });
        const lylic = await db.update(Lylic).set(body).where(eq(Lylic.id, +id)).returning();
        return new Response(JSON.stringify(lylic), {
            status: 200
        });
    };
    const DELETE = async ({ params })=>{
        const { id } = params;
        if (!id) {
            return new Response(JSON.stringify({
                error: "id is required"
            }), {
                status: 400
            });
        }
        await db.delete(Lylic).where(eq(Lylic.id, +id));
        return new Response(null, {
            status: 204
        });
    };
    index$1 = Object.freeze(Object.defineProperty({
        __proto__: null,
        DELETE,
        GET: GET$1,
        PATCH
    }, Symbol.toStringTag, {
        value: "Module"
    }));
    const GET = async ()=>{
        const rs = await db.select().from(Lylic).orderBy(sql`random()`).limit(1);
        const { artist, quote } = rs[0];
        return new Response(`${quote} ~~ ${artist}`);
    };
    const POST = async ({ request })=>{
        const body = await request.json();
        console.log({
            body
        });
        const { quote, artist } = body;
        const lylic = await db.insert(Lylic).values({
            quote,
            artist
        }).returning();
        return new Response(JSON.stringify(lylic));
    };
    index = Object.freeze(Object.defineProperty({
        __proto__: null,
        GET,
        POST
    }, Symbol.toStringTag, {
        value: "Module"
    }));
});
export { index as a, index$1 as i, __tla };
