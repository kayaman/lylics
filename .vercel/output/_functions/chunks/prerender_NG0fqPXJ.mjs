import { e as createAstro, f as createComponent, r as renderTemplate, h as addAttribute, m as maybeRenderHead, i as renderHead, j as renderComponent, k as defineScriptVars } from './astro_CKbA4atV.mjs';
import 'kleur/colors';
import 'html-escaper';
/* empty css                                      */
/* empty css                                        */
import 'clsx';
import { s as swaggerDarkModeOverrides } from './gui-swagger-endpoint.02e9ab3b_BstuW1Z-.mjs';
/* empty css                          */

var __freeze$1 = Object.freeze;
var __defProp$1 = Object.defineProperty;
var __template$1 = (cooked, raw) => __freeze$1(__defProp$1(cooked, "raw", { value: __freeze$1(raw || cooked.slice()) }));
var _a$1;
const $$Astro$5 = createAstro();
const $$OpenApiGuiRedoc = createComponent(async ($$result, $$props, $$slots) => {
  const Astro2 = $$result.createAstro($$Astro$5, $$props, $$slots);
  Astro2.self = $$OpenApiGuiRedoc;
  const { schemaUrl = "https://petstore3.swagger.io/api/v3/openapi.json" } = Astro2.props;
  return renderTemplate(_a$1 || (_a$1 = __template$1(['<!-- \n<script src="./node_modules/redoc/bundles/redoc.standalone.js"><\/script> -->', "<redoc", ' data-astro-cid-jhisyjfk></redoc> <script src="https://cdn.redoc.ly/redoc/latest/bundles/redoc.standalone.js"><\/script> <!-- <script src="redoc/bundles/redoc.standalone.js"><\/script> --> <!-- NOT WORKING --> <!-- <script>\n	import \'./node_modules/redoc/bundles/redoc.`standalone`.js\';\n<\/script> -->  '], ['<!-- \n<script src="./node_modules/redoc/bundles/redoc.standalone.js"><\/script> -->', "<redoc", ' data-astro-cid-jhisyjfk></redoc> <script src="https://cdn.redoc.ly/redoc/latest/bundles/redoc.standalone.js"><\/script> <!-- <script src="redoc/bundles/redoc.standalone.js"><\/script> --> <!-- NOT WORKING --> <!-- <script>\n	import \'./node_modules/redoc/bundles/redoc.\\`standalone\\`.js\';\n<\/script> -->  '])), maybeRenderHead(), addAttribute(schemaUrl, "spec-url"));
}, "/home/kayaman/Projects/lylics/node_modules/@astro-openapi/gui-redoc/OpenApiGuiRedoc.astro", void 0);

const $$Astro$4 = createAstro();
const prerender$3 = true;
const $$GuiRedocEndpoint = createComponent(async ($$result, $$props, $$slots) => {
  const Astro2 = $$result.createAstro($$Astro$4, $$props, $$slots);
  Astro2.self = $$GuiRedocEndpoint;
  const schemaUrl = "/api/openapi.json";
  return renderTemplate`<html lang="en"> <head><meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><title>API Reference</title>${renderHead()}</head> <body> ${renderComponent($$result, "OpenApiGuiRedoc", $$OpenApiGuiRedoc, { "schemaUrl": schemaUrl })} </body></html>`;
}, "/home/kayaman/Projects/lylics/node_modules/@astro-openapi/gui-redoc/gui-redoc-endpoint.astro", void 0);

const $$file$3 = "/home/kayaman/Projects/lylics/node_modules/@astro-openapi/gui-redoc/gui-redoc-endpoint.astro";
const $$url$3 = undefined;

const guiRedocEndpoint = /*#__PURE__*/Object.freeze(/*#__PURE__*/Object.defineProperty({
	__proto__: null,
	default: $$GuiRedocEndpoint,
	file: $$file$3,
	prerender: prerender$3,
	url: $$url$3
}, Symbol.toStringTag, { value: 'Module' }));

var __freeze = Object.freeze;
var __defProp = Object.defineProperty;
var __template = (cooked, raw) => __freeze(__defProp(cooked, "raw", { value: __freeze(raw || cooked.slice()) }));
var _a;
const $$Astro$3 = createAstro();
const $$OpenApiGuiSwagger = createComponent(async ($$result, $$props, $$slots) => {
  const Astro2 = $$result.createAstro($$Astro$3, $$props, $$slots);
  Astro2.self = $$OpenApiGuiSwagger;
  const { schemaUrl = "https://petstore3.swagger.io/api/v3/openapi.json" } = Astro2.props;
  return renderTemplate(_a || (_a = __template(["<!-- NOT WORKING --><!-- {schemaUrl && <open-api-gui data-url={schemaUrl} />} --><!--  --><!-- <script>\n	// import './OpenApiGuiSwagger.client.ts';\n\n	console.log('\u2026');\n\n	alert();\n<\/script> -->", '<div id="swagger-ui" data-astro-cid-io5yert4></div> <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@4.5.0/swagger-ui.css"> <link rel="stylesheet"', '> <script src="https://unpkg.com/swagger-ui-dist@4.5.0/swagger-ui-bundle.js" crossorigin><\/script> <script>(function(){', "\n	window.onload = () => {\n		// eslint-disable-next-line no-undef\n		window.ui = SwaggerUIBundle({\n			url: schemaUrl,\n			dom_id: '#swagger-ui',\n		});\n	};\n})();<\/script>  "])), maybeRenderHead(), addAttribute(swaggerDarkModeOverrides, "href"), defineScriptVars({ schemaUrl }));
}, "/home/kayaman/Projects/lylics/node_modules/@astro-openapi/gui-swagger/OpenApiGuiSwagger.astro", void 0);

const $$Astro$2 = createAstro();
const prerender$2 = true;
const $$GuiSwaggerEndpoint = createComponent(async ($$result, $$props, $$slots) => {
  const Astro2 = $$result.createAstro($$Astro$2, $$props, $$slots);
  Astro2.self = $$GuiSwaggerEndpoint;
  const schemaUrl = "/api/openapi.json";
  return renderTemplate`<html lang="en" data-astro-cid-man7looe> <head><meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><title>API Reference</title>${renderHead()}</head> <body data-astro-cid-man7looe> ${renderComponent($$result, "OpenApiGuiSwagger", $$OpenApiGuiSwagger, { "schemaUrl": schemaUrl, "data-astro-cid-man7looe": true })} </body></html>`;
}, "/home/kayaman/Projects/lylics/node_modules/@astro-openapi/gui-swagger/gui-swagger-endpoint.astro", void 0);

const $$file$2 = "/home/kayaman/Projects/lylics/node_modules/@astro-openapi/gui-swagger/gui-swagger-endpoint.astro";
const $$url$2 = undefined;

const guiSwaggerEndpoint = /*#__PURE__*/Object.freeze(/*#__PURE__*/Object.defineProperty({
	__proto__: null,
	default: $$GuiSwaggerEndpoint,
	file: $$file$2,
	prerender: prerender$2,
	url: $$url$2
}, Symbol.toStringTag, { value: 'Module' }));

const $$Astro$1 = createAstro();
function getStaticPaths$1() {
  return [
    //
    "redoc",
    "swagger"
  ].map((gui) => ({
    params: { gui }
  }));
}
const prerender$1 = true;
const $$gui = createComponent(async ($$result, $$props, $$slots) => {
  const Astro2 = $$result.createAstro($$Astro$1, $$props, $$slots);
  Astro2.self = $$gui;
  const SITE_TITLE = "Lylics API Reference";
  const schemaUrl = "http://localhost:4321";
  const { gui } = Astro2.params;
  return renderTemplate`<html lang="en"> <head><meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><title>API Reference — ${SITE_TITLE}</title>${renderHead()}</head> <body${addAttribute(gui, "class")}> ${gui === "swagger" && renderTemplate`${renderComponent($$result, "OpenApiGuiSwagger", $$OpenApiGuiSwagger, { "schemaUrl": schemaUrl })}`} ${gui === "redoc" && renderTemplate`${renderComponent($$result, "OpenApiGuiRedoc", $$OpenApiGuiRedoc, { "schemaUrl": schemaUrl })}`} </body></html>`;
}, "/home/kayaman/Projects/lylics/src/pages/api-docs/[gui].astro", void 0);

const $$file$1 = "/home/kayaman/Projects/lylics/src/pages/api-docs/[gui].astro";
const $$url$1 = "/api-docs/[gui]";

const _gui_ = /*#__PURE__*/Object.freeze(/*#__PURE__*/Object.defineProperty({
	__proto__: null,
	default: $$gui,
	file: $$file$1,
	getStaticPaths: getStaticPaths$1,
	prerender: prerender$1,
	url: $$url$1
}, Symbol.toStringTag, { value: 'Module' }));

const $$Astro = createAstro();
const prerender = true;
function getStaticPaths() {
  return [
    //
    "redoc",
    "swagger"
  ].map((docs) => ({ params: { docs } }));
}
const $$docs = createComponent(async ($$result, $$props, $$slots) => {
  const Astro2 = $$result.createAstro($$Astro, $$props, $$slots);
  Astro2.self = $$docs;
  const { docs } = Astro2.params;
  return renderTemplate`${maybeRenderHead()}<iframe${addAttribute(`/api-docs/${docs}`, "src")} slot="content" allowfullscreen data-astro-cid-p237y4sl></iframe> `;
}, "/home/kayaman/Projects/lylics/src/pages/[docs].astro", void 0);

const $$file = "/home/kayaman/Projects/lylics/src/pages/[docs].astro";
const $$url = "/[docs]";

const _docs_ = /*#__PURE__*/Object.freeze(/*#__PURE__*/Object.defineProperty({
	__proto__: null,
	default: $$docs,
	file: $$file,
	getStaticPaths,
	prerender,
	url: $$url
}, Symbol.toStringTag, { value: 'Module' }));

export { _gui_ as _, guiSwaggerEndpoint as a, _docs_ as b, guiRedocEndpoint as g };
