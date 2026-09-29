# Visualize: Direction Comps & Asset Production

Load this from [new-work.md](new-work.md) on a comp-led build, when image generation is available (a harness-native tool or the API fallback `impeccable context` reports). A code-led contract skips this file by design, not by drift; do not load it then. PRODUCT.md and DESIGN.md are preconditions. New-work has already resolved the visual world; this file must not reopen it. A surface-scope structure round that already put three visualized cards before the user (new-work.md, established world) has discharged this round: the locked card's comp is the approved comp, so record the approval and continue at After approval; generate nothing new.

A probe tests composition, narrative, hierarchy, density, focal moment, signature use, and image requirements. It is not a second identity workshop. Keep DESIGN.md's palette, typography direction, material language, component character, imagery stance, and motion grammar fixed.

## Generate three compositional options

The comp round runs inside the build's phase state: `impeccable build-phase start --direction <seed key> --kind <...>` has already run (the roll's output names the command) and its `comps` phase is open before the first comp is generated; a comp rendered before that sits outside the state, and a session resumed from that point has no phases to follow. `impeccable generate-image` refuses to write under `.impeccable/mocks/` until start has run; a harness-native image tool is bound by the same order.

Render three high-fidelity comps of the requested surface and save them under `.impeccable/mocks/`. Comps are the build thread's own work, never delegated: the thread that writes the prompts holds the direction's context and has seen every comp when the build starts. Open every image by its workspace-relative path; sandboxed viewers reject absolute paths. The chosen card's decision comp is comp one. Generate comps two and three from the template in words, never with comp one as an input image: a reference image anchors the layout and the three become one. Send all three to the approval point together. A round with no decision comp (a degraded roll, an identity-mode page, a direction pinned without the decision round) renders all three here. When the user shortlisted several concepts, spread the three across them.

On an established world, pass a screenshot of a representative existing page as a reference image (the harness image tool's input image, or `impeccable generate-image --ref`) and add one line to the prompt: the reference supplies chrome, palette, type and component character; its content, banners and cards stay behind.

### The comp prompt template

Every comp prompt is this template, filled, slots in this order. That covers every decision comp (assigned, pick, challenger, canon) and comps two and three here. Write it as labelled lines; the labels stay in the prompt.

```
FRAME: the page itself, edge to edge, <W>x<H>. No browser window, address bar, device, desk or hands.
MODE: <Persuade | Operate | Read | Experience>
REGIONS: top to bottom, <region: scale>, ..., ending with the top of the section below the first viewport.
REAL CONTENT: per region, the real product name and the actual copy, set as legible text.
PRIMARY CONTROL: <the category's must-have mechanism as a real control, with its visible label>
CONTROLS: <every other control, by name>
ONE FOCAL MOVE: <the one move, and the one region it sits in>
WORLD AS ATTRIBUTES: palette <3-5 hex>; type <description>; ground <treatment>; motif <one>
EXCLUSIONS: <invented claims only>
```

Slot rules:

- **FRAME.** W×H is the surface's own viewport: desktop landscape (1440×900) for web, device portrait (390×844) for a native app or mobile-first surface.
- **MODE.** The surface's mode from the brief, named.
- **REGIONS.** Each region with its scale ("nav bar, 64px tall"; "booking panel, left 40%"). Comp one uses the dealt composition's skeleton. A page with no navigation writes "no navigation".
- **REAL CONTENT.** Nav link labels, headline, body copy, captions, table rows, written out. Body copy near 16px on a 1440px page. No lorem, no "Brand", no "Product Name". Demonstration data is allowed.
- **PRIMARY CONTROL.** The mechanism a visitor of this category expects to find, drawn as a working control with its label: a hotel's dates, guests and Book bar; a dashboard's filter bar and data table; a docs site's search field and sidebar; a shop's price and Add to cart.
- **CONTROLS.** On Operate or Read, standard web components only: buttons, tabs, inputs, selects, checkboxes, tables with sortable headers, link navigation (new-work.md's world-reach rule). On Persuade or Experience, a control may take the world's form, and still shows its label and reads as clickable.
- **ONE FOCAL MOVE.** Exactly one, placed in one region. Every other region holds still in the world's attributes.
- **WORLD AS ATTRIBUTES.** The world enters as colour, type, ground and one motif. Never write the world's object as a noun: the image model paints nouns. Write "ruled 8px grid in #2B4C7E on #F4EFE3, condensed grotesque headings", never "a logbook page". On an established world, these are DESIGN.md's values.
- **EXCLUSIONS.** Prices, customers, logos, metrics and awards the product does not have. Never exclude the subject, its imagery, or a medium the committed imagery stance allows.

Minimum fill: all nine slots carry concrete values, for every card, canon and pick included. A generic value ("modern", "clean layout", "some buttons") counts as empty. Count the slots before sending; a prompt with an empty slot is not sent.

### Check every render

Open each render and write these five lines before anything else:

1. Nav: the link labels you can read, or "no navigation (per REGIONS)".
2. Primary control: what it is, and its label as rendered.
3. Body copy: legible, yes or no.
4. Category: does it read as a website of this category? yes or no.
5. Costume: anything that is a costume of the world rather than a page part (a painted object with no working parts, a poster, a device bezel, ledger paper standing in for a table)? Name it, or "none".

Then either regenerate once, changing the slot that caused the failed line, or write "pass because" and the reason. After one regeneration, keep the better render and name its open line on the approval page. A comp with no written check is not ready for approval.

### Comps two and three

Keep MODE, REAL CONTENT, PRIMARY CONTROL, WORLD AS ATTRIBUTES and EXCLUSIONS from comp one. Change the REGIONS topology: each takes a different one of split, stacked, list-led or media-led that comp one did not use, and moves ONE FOCAL MOVE to fit it. World intensity stays at comp one's level: same palette coverage, same ground, same motif.

Never add a palette artifact, a new type voice or a new motif here. If the committed world cannot fill WORLD AS ATTRIBUTES for this surface, return to the concept shortlist.

Each comp is a direction test, not a screenshot specification. Core UI text, responsive behavior, accessibility, semantics, and interaction states remain implementation responsibilities.

## One approval point

Show the three together on the decision page (`impeccable serve-question`, one option per comp with the comp as its hero), or in the harness only when it renders images inline; a text-only surface does not count as display. Ask what should carry forward, what feels false to the world, and whether the selected concept should be approved, combined, revised, or rejected. Then stop and wait. A structured simulated user counts as attended and receives the same question.

Do not begin code until the user approves a direction or explicitly delegates the choice. If they delegate, choose using the task brief, PRODUCT.md, and DESIGN.md, and state the evidence. Approval refines the task concept; it does not modify DESIGN.md.

This approval point has no substitute and no skip condition. When the structured question tool errors, fall back to the decision page; only after both fail may you treat the choice as delegated, and a delegated pick is recorded exactly as an approval is and disclosed in your first reply, not your last. The finish reviewer treats comp-round comps with no recorded approval as a material finding; decision comps under `.impeccable/mocks/decision/` are the direction round's hand, not comp-round output, and imply no approval on their own.

After approval, record the choice where tools can find it: the approved comp's path goes in the surface brief, and its `.json` prompt sidecar gains `"approved": true` (every comp generated through `impeccable generate-image` has one; create it if a native tool didn't). The sidecar travels with the mocks folder, so the approval survives sessions and machines that never see the brief, and it is what `impeccable build-phase advance` reads to close the comps phase. Summarize the composition and the parts of the comp that must not be literalized, return to new-work.md, record the direction contract from the approved concept, and build.

## After approval: the comp becomes a spec

The approved comp is a north star for translation into semantic, responsive, accessible code, never a license to recompose: keeping the palette and mood while redrawing the topology is a second art direction. Do not rasterize core UI text or controls. Do not substitute a different visual driver after approval without asking.

What the comp shows is measured, not remembered. new-work.md section 6 runs the build as phases (`impeccable build-phase`): the spec phase turns the comp into region boxes with sampled palettes (`impeccable comp-spec`), and the medium of every region follows from what the pixels are, never from what feels buildable: a figure, a product object, machinery, any illustration with perspective, shading, or drawing skill in it, and any texture by name (woven cloth, paper grain, fabric, leather, brushed metal) is a `plate` / `image` / `texture` region and ships as a raster; text, controls, chrome, diagrams with countable elements, flat shape systems, and anything that must move, scale, or respond are semantic. Writing "CSS" for a sculpted panel's finish, or a many-vertex `clip-path` for a torn edge, is the quiet deletion of the approved design; the detector's organic-clip-path and buried-raster rules and the hero gate's region scores catch it. Dropping an image-native region is a scope decision the user makes at the approval point, never a silent flattening after it. Generated imagery is a material, not a claim: evidence rules bind assertions, specs, testimonials, and photographs presented as real, never render fidelity.

## Plates and provenance

Every raster region's plate is produced in the plates phase, before any page code, by the shipped asset producer or in the current thread (use `impeccable comp-spec --crop <id>` and save `impeccable comp-spec --plate-prompt <id>` to a prompt file; pass the crop and prompt to the harness image tool, or use `impeccable generate-image --ref <crop.png> --prompt-file <prompt.txt> --out <plate.png> --size <WxH> --quality high`). For isolated cutouts, add `--background transparent` to both the plate-prompt and API generation commands; use native PNG alpha and preserve white paint and clear gaps. Use `--background opaque` for full-frame imagery. Create output directories first and inspect alpha on light and dark grounds. Generation context is part of the asset: after generating any image with any tool, run `{{scripts_path}}/impeccable embed-prompt <image> --prompt "<prompt>"` with the exact string the tool received (`impeccable generate-image` does this itself), so the intent lives inside the file; `--read` recovers it, `--scan <dir>` lists rasters still missing one. The embedded prompt plus the region's row in the spec is the raster's **provenance**, and every raster the artifact references carries it; a sourced, stock, or pre-existing raster embeds its origin instead. A raster created or replaced later, in a fix batch or a reviewer's rebuild, is produced the same way; a raster a fix abandons is deleted in the same batch.

Convert images with a converter `impeccable context` reported at boot (the IMAGE_TOOLS line); probe only when it reported none, at most once per session, never per image.

Return to [new-work.md](new-work.md) for the direction contract, the phased build, and the finishing pass.
