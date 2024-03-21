import { e as createAstro, f as createComponent, r as renderTemplate, m as maybeRenderHead } from "../astro_CKbA4atV.mjs";
import "kleur/colors";
import "html-escaper";
import "clsx";
import { createRemoteDatabaseClient, asDrizzleTable } from "@astrojs/db/runtime";
import "@astrojs/db/dist/runtime/config.js";
let Lylic, index, db, index$1;
let __tla = (async ()=>{
    db = await createRemoteDatabaseClient(process.env.ASTRO_STUDIO_APP_TOKEN ?? "f3af643ebcc043cafa935c993f53ffd7c6d26d95:gbaiaa9cxssareb97h7hodpxrr6d", {
        "BASE_URL": "/",
        "MODE": "production",
        "DEV": false,
        "PROD": true,
        "SSR": true,
        "SITE": undefined,
        "ASSETS_PREFIX": undefined
    }.ASTRO_STUDIO_REMOTE_DB_URL ?? "https://db.services.astro.build");
    Lylic = asDrizzleTable("Lylic", {
        "columns": {
            "id": {
                "type": "number",
                "schema": {
                    "unique": false,
                    "deprecated": false,
                    "name": "id",
                    "collection": "Lylic",
                    "primaryKey": true
                }
            },
            "quote": {
                "type": "text",
                "schema": {
                    "unique": false,
                    "deprecated": false,
                    "name": "quote",
                    "collection": "Lylic",
                    "primaryKey": false,
                    "optional": false
                }
            },
            "artist": {
                "type": "text",
                "schema": {
                    "unique": false,
                    "deprecated": false,
                    "name": "artist",
                    "collection": "Lylic",
                    "primaryKey": false,
                    "optional": false
                }
            }
        },
        "deprecated": false
    }, false);
    const $$Astro$1 = createAstro();
    const $$Index$1 = createComponent(async ($$result, $$props, $$slots)=>{
        const Astro2 = $$result.createAstro($$Astro$1, $$props, $$slots);
        Astro2.self = $$Index$1;
        if (Astro2.request.method === "POST") {
            const data = await Astro2.request.formData();
            const artist = data.get("artist");
            const quote = data.get("quote");
            await db.insert(Lylic).values({
                quote,
                artist
            });
            return Astro2.redirect("/");
        }
        return renderTemplate`${maybeRenderHead()}<form method="POST"> <input type="text" name="quote" placeholder="Quote..." required> <input type="text" name="artist" placeholder="Artist" required> <input type="submit" value="Add Quote"> </form>`;
    }, "/home/kayaman/Projects/lylics/src/pages/new/index.astro", void 0);
    const $$file$1 = "/home/kayaman/Projects/lylics/src/pages/new/index.astro";
    const $$url$1 = "/new";
    index$1 = Object.freeze(Object.defineProperty({
        __proto__: null,
        default: $$Index$1,
        file: $$file$1,
        url: $$url$1
    }, Symbol.toStringTag, {
        value: "Module"
    }));
    const $$Astro = createAstro();
    const $$Index = createComponent(async ($$result, $$props, $$slots)=>{
        const Astro2 = $$result.createAstro($$Astro, $$props, $$slots);
        Astro2.self = $$Index;
        const API = "http://localhost:4321/api/";
        const data = await fetch(API);
        const lylic = await data.text();
        return renderTemplate`${lylic}`;
    }, "/home/kayaman/Projects/lylics/src/pages/index.astro", void 0);
    const $$file = "/home/kayaman/Projects/lylics/src/pages/index.astro";
    const $$url = "";
    index = Object.freeze(Object.defineProperty({
        __proto__: null,
        default: $$Index,
        file: $$file,
        url: $$url
    }, Symbol.toStringTag, {
        value: "Module"
    }));
})();
export { Lylic as L, index as a, db as d, index$1 as i, __tla };
