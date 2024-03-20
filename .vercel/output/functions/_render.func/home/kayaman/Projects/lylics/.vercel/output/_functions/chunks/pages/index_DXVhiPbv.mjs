import { e as createAstro, f as createComponent, r as renderTemplate } from '../astro_BSX-mMqp.mjs';
import 'kleur/colors';
import 'html-escaper';
import 'clsx';

const $$Astro = createAstro();
const $$Index = createComponent(async ($$result, $$props, $$slots) => {
  const Astro2 = $$result.createAstro($$Astro, $$props, $$slots);
  Astro2.self = $$Index;
  const API = "http://localhost:4321/api/";
  const data = await fetch(API);
  const lylic = await data.text();
  return renderTemplate`${lylic}`;
}, "/home/kayaman/Projects/lylics/src/pages/index.astro", void 0);
const $$file = "/home/kayaman/Projects/lylics/src/pages/index.astro";
const $$url = "";

export { $$Index as default, $$file as file, $$url as url };
