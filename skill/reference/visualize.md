# Visualize: Direction Comps & Asset Production

Read this before any comp prompt, decision comps and comp round alike. The MODE RULES block's COMPS section governs with it and wins on conflict (no roll this session: the mode's file named in new-work.md). The world is decided: comps test composition, hierarchy, density, and focal moment, never palette, type voice, material, or motion. A world that cannot carry the concept sends you back to the shortlist.

## Every comp

1. **Frame:** the requested surface at its own viewport, portrait at device size for native or mobile-first, landscape otherwise, edge to edge with no browser, device, or desk around it.
2. **Structure leads the prompt:** the surface's regions in order with their scale relationships. A render that could hang as a poster, or reads as a photograph with text on it, is regenerated with its regions named.
3. **The subject is in it** as the content the regions hold; exclusion lists bind claims, never the subject. If you cannot point at the subject, regenerate.
4. **The job reads:** name the mode from the render alone, or regenerate with the visitor's job as the spine.
5. **Depth, not spread:** one dominant move plus the material, type, and spacing that support it; every other region keeps its content at reading size (body near 16px at 1440 wide) and stops performing. Busy is louder, not bolder.
6. **Real material:** real product name and content, no invented commercial claims.
7. **Established world:** a screenshot of a representative existing page as reference image; chrome, palette, type, and component character carry over, its content does not.
8. **Write and record:** write the prompt in the order the visitor reads it: the headline in its own words and the primary action first, then the one focal move in full detail, then each remaining region in a phrase; the image model fills whatever the prompt describes in detail, so a region you inventory renders as a second focal move. Every comp gets its prompt sidecar (`a.png` gets `a.png.json`); open images by workspace-relative path.

## The comp round

Comps follow `build-phase start`, never precede it. Three comps under `.impeccable/mocks/`: the chosen decision comp, copied unchanged with its sidecar, is the first, never regenerated; generate two more from the direction in words, never from the first comp as input image, varied as the mode's COMPS rules say (a tidier variant adds no choice). No decision comp: render all three. These comps are the build thread's own work, never delegated. A surface round that locked a visualized card has already discharged this round.

## One approval point

Show the three together on the decision page (or inline where the harness renders images; text never counts), ask what carries forward, what feels false, and whether to approve, combine, revise, or reject, then stop. A simulated user is attended. No code before approval or explicit delegation; when the page cannot start, ask through the structured question tool, and only when both fail treat the choice as delegated, recorded like an approval and disclosed in your first reply. On approval, put the comp's path in the surface brief and `"approved": true` in its sidecar, then advance.

## After approval

The comp is a north star for semantic, responsive code, never a license to recompose: same palette over a redrawn topology is a second art direction. Never rasterize core text or controls. A region's medium follows its pixels: figures, objects, shaded or perspectived illustration, and named textures ship as raster plates; text, controls, chrome, countable diagrams, and anything that moves stay semantic. Dropping an image-native region is the user's call at approval, never a silent flattening.

## Provenance

Every raster the artifact references carries its provenance: after any generation, `{{scripts_path}}/impeccable embed-prompt <image> --prompt "<exact prompt>"` (`generate-image` does it itself), or the origin for sourced and pre-existing files; `--scan <dir>` lists what is missing. Late rasters follow the same rule, and abandoned ones are deleted.
