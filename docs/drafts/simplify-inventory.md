# Simplifying new-work.md and visualize.md: inventory, conflicts, checklist

Scratch document for the A/B of two drafts. Not for merge. Base: `origin/main` at 87a6ab0c1. Drafts live on `draft/simplify-moderate` and `draft/simplify-aggressive`; both build and pass `bun run test` against a binary built from the same commit, with no test changes.

## Word counts

| File | main | moderate | aggressive |
|---|---|---|---|
| new-work.md | 9432 | 5021 (-47%) | 2869 (-70%) |
| visualize.md | 2058 | 1238 (-40%) | 597 (-71%) |
| both | 11490 | 6259 (-46%) | 3466 (-70%) |

## Classes and sources

- **(a)** printed or enforced by the engine at the moment it matters.
- **(b)** duplicated in another loaded file.
- **(c)** mode-specific and already in `mode-*.md` (printed by concept-seed as MODE RULES).
- **(d)** needed in the file.
- **(e)** obsolete, or contradicted by a later rule.

Engine source shorthand:

- `SEED:<CONST>`: `crates/context/src/seed_text.rs`, printed by `concept-seed`. Constants used: PROMOTED_DIRECTION, PROMOTED_SURFACE, CHALLENGER_DIRECTION, CHALLENGER_SECTION, RICHNESS, MODE_RULES_BLOCK, SAFER_BLOCK, BOLDER_BLOCK, REROLL_BLOCK, TELEMETRY_BLOCK, MAIN, DEGRADED_BODY.
- `PRES:<CONST>`: the PRESENTATION block from PR #954 (`feat/seed-prints-presentation`, treated as landing): PRESENT_FIRST, PRESENT_REROLL, PRESENT_WAIT, COMPS_DIRECTION(_CODE/_DEGRADED), COMPS_SURFACE(_CODE), BUILD_PATH_RECORDED/NONE, PRESENT_FALLBACK.
- `SQ:<LINE>`: `crates/context/src/serve_question.rs`. ANSWER follow-ups in `print_answer` (CHOSEN CARD, APPROVED COMP, CHOSEN COMP, CANON CHOSEN, REGISTER, FOLLOWUP OPEN, BUILD PATH); `--start` output (QUESTION URL, "Open the URL for the user now...", NEXT read visualize.md, "Then collect the answer with --wait"); `--wait` output (WAITING, PAGE CLOSED, BUILD PATH FLIPPED, COMP SIDECAR MISSING, COMP STALE); start failure (sandbox line); `--schema` SCHEMA_NOTE.
- `CTX:<DIRECTIVE>`: `crates/context/src/context_cli.rs` boot directives: BUILD_PATH_DEFAULT, IMAGE_GEN_AVAILABLE, AUTONOMY_DIRECTIVE_CHECK, SUBAGENT_AUTHORIZATION, MANUAL_DETECTOR_REQUIRED, IMAGE_TOOLS, INCUMBENT_WORLD_UNDOCUMENTED, WORLD_DISCOVERY_REQUIRED.
- `BP:<phase>`: `crates/comp-verbs/src/build_phase.rs` `next_instruction` per phase (comps, spec, plates, hero, sections, motion, responsive, review), `start` (CODE-LED line, "choice ping skipped"), `gate_comps`, hero and responsive three-attempt verdicts, approved-comp refusals (`approved_comp.rs`), `comp_spec.rs` refusals (code region over a quarter of the comp, plate box cutting artwork, painted pixels).

## Inventory: new-work.md (main line numbers)

| ID | L | Instruction | Class | Where / note |
|---|---|---|---|---|
| NW01 | 3 | PRODUCT.md owns truth, DESIGN.md visual decisions, surface brief per route | d | kept, one sentence |
| NW02 | 3 | Complete init first when PRODUCT.md missing; missing DESIGN.md does not route to init | a, b | CTX:PRODUCT_INIT_REQUIRED / BUILD_INIT_REQUIRED; SKILL.src.md routing "Otherwise" and rule skill-new-work-gate |
| NW03 | 5 | region-map.md pointer for a chosen comp | b | repeated at L112; kept once in the spec bullet (moderate), dropped in aggressive (BP:spec + comp-spec refusals) |
| NW04 | 9 | Read DESIGN.md, code, tokens, components, assets | b | SKILL.src.md Setup step 2; kept as the lead-in |
| NW05 | 11-14 | Four authority cases (redesign, established, incomplete, none) | d | kept; redesign half also in SKILL.src.md skill-world-change-semantics (b) |
| NW06 | 12 | Missing DESIGN.md does not erase an incumbent identity | b, a | SKILL.src.md skill-new-work-gate; CTX:INCUMBENT_WORLD_UNDOCUMENTED |
| NW07 | 16 | Local addition inherits the surface | d | kept |
| NW08 | 20 | Ask 2-3 questions via structured tool before implementation; compact confirmation; never skip the confirmation | d | kept; CTX:AUTONOMY_DIRECTIVE_CHECK covers the unattended probe |
| NW09 | 22-25 | Mode-specific question topics | d | not in mode files (they hold Directions and Comps only); kept, compressed in aggressive |
| NW10 | 27 | Success, untouched, what feels wrong; never CSS values or aesthetic lanes | d | kept |
| NW11 | 33 | Extend: inherit, no tournament, no DESIGN.md change unless approved | d | kept |
| NW12 | 37 | Surface round: 5-7 structures, ordered; run concept-seed --scope surface | d | kept |
| NW13 | 41 | Dice deal three; equal cards; THE ROLL kicker; steer; re-roll; user locks | a | SEED:PROMOTED_SURFACE |
| NW14 | 41 | No canon and no pick card at surface scope | a | PRES:COMPS_SURFACE |
| NW15 | 41 | Comp per card under mocks/decision, after serving, in reading order | a | SEED:PROMOTED_SURFACE, PRES:COMPS_SURFACE |
| NW16 | 41 | Anchor each surface comp on a screenshot of an existing page | b, d | visualize.md L11 says the same; kept once in each file (new-work needs it before visualize is read) |
| NW17 | 41 | Without image gen or code-led: wireframe card | a | SEED:PROMOTED_SURFACE, SQ:SCHEMA_NOTE, PRES:COMPS_SURFACE_CODE |
| NW18 | 41 | Locking a card is approval; locked comp skips the three-option round; locked wireframe is code-led | a, d | SEED:PROMOTED_SURFACE says the first half; kept with the `build-phase start --comp` step it implies |
| NW19 | 41 | Never run the seed for a local extension or precise narrow request | d | kept |
| NW20 | 45 | Ground: mechanism, scene, cultural home, proof; rut = category page + opposite; brief's literal picture joins the rut | d | kept |
| NW21 | 46 | Seven candidates from the audience's cultural world incl. graphic and screen traditions | d | kept |
| NW22 | 46 | Operate/Read: never from the tools the audience operates (costume) | c | mode-operate.md and mode-read.md Directions |
| NW23 | 46 | Persuade/Experience: tools count toward the rut, at most one | d | mode-specific but NOT in mode-persuade.md; kept in shared text (candidate to move into mode-persuade.md) |
| NW24 | 46 | Physical object, before the web; near-duplicates once; three material families | d | kept |
| NW25 | 47 | Directions join a visual world to a first-surface experience | a | SEED:PROMOTED_DIRECTION ("decided as one") |
| NW26 | 48 | Run concept-seed --scope direction; no substitute, no skip; contract violation | d | kept verbatim-ish; nothing in the engine can force the call |
| NW27 | 48 | Fuse challengers; two axes; wins/competitive/declined; donation of discipline, never clothes; raise as named line | a | SEED:CHALLENGER_DIRECTION, near verbatim |
| NW28 | 48 | A lifted motif is a costume note, one world owns the page | d | only phrase missing from SEED:CHALLENGER_DIRECTION; kept |
| NW29 | 49 | Present one direction: world, first viewport, visitor path, signature interaction, cross-surface reach, risk | e | conflicts with the card anatomy at L51 and SQ:SCHEMA_NOTE (C1); replaced by "fully committed and raised" |
| NW30 | 49 | Route challengers by verdict; declined demoted, still adoptable | a | SQ:SCHEMA_NOTE (verdict routes rendering, "Adopt anyway"); short mention kept |
| NW31 | 49 | Hand holds at most three full-card challengers; rest wait in re-roll pool | d | not printed; kept |
| NW32 | 49 | Pick card: one, never lead, honest familiarity risk; none when assigned is top | a | SEED:PROMOTED_DIRECTION, SQ:SCHEMA_NOTE |
| NW33 | 49 | Familiar and effective is legitimate | d | kept (moderate), folded (aggressive) |
| NW34 | 49 | Registers plain/safer/bolder; user's steering; rerun with --register | a | SQ:SCHEMA_NOTE reroll, SQ:REGISTER, SEED:SAFER_BLOCK/BOLDER_BLOCK |
| NW35 | 49 | "bolder"/"safer" during an open round means registers, not commands | d, b | bolder.md L3 covers bolder; kept one line |
| NW36 | 49 | Structured-tool option list content; declined fold into assigned option | d | PRES:PRESENT_FALLBACK says "same options" only; kept |
| NW37 | 51 | Canon: never recommend, never weigh, never soften | a, d | SQ:SCHEMA_NOTE "never present it as your own recommendation"; kept one sentence |
| NW38 | 51 | Canon taken: ask 2-3 products, their craft is the bar, full fidelity | a | SQ:CANON CHOSEN |
| NW39 | 51 | Standing preference recorded as brand commitment in PRODUCT.md | a | SEED:SAFER_BLOCK; kept one clause |
| NW40 | 51 | Re-roll eliminates everything shown | a | SEED:REROLL_BLOCK |
| NW41 | 51 | After two consecutive re-rolls ask what is missing | d | kept |
| NW42 | 51 | Self re-roll only on named factual grounds | a | SEED:PROMOTED_DIRECTION; kept one clause (also the guard against taste re-rolls) |
| NW43 | 51 | Pinned direction beats the roll | a | SEED:MAIN, DEGRADED_BODY |
| NW44 | 51 | Field-by-field collision resolution; translate material; look mismatch not grounds | d | kept |
| NW45 | 51 | Payload contents (assigned lead, raised lines, pick, challengers with QUALITY BAR, re-roll, steer, canon, buildPath) | a | SQ:SCHEMA_NOTE, PRES:PRESENT_FIRST |
| NW46 | 51 | Degraded roll still uses the page as one text card | a | SEED:DEGRADED_BODY, PRES:COMPS_DIRECTION_DEGRADED |
| NW47 | 51 | Card anatomy; catalog image as labeled inspiration; canonCard | a | SQ:SCHEMA_NOTE |
| NW48 | 51 | serve-question --start, open URL (in-app, opener, show URL) | a | SQ:start prints the open instruction; PRES:PRESENT_FIRST |
| NW49 | 51 | --wait repeating while exit 3 | a | SQ:WAITING, PRES:PRESENT_WAIT |
| NW50 | 51 | reroll ANSWER: rerun seed with --from/--reroll n, deliver with --update, never second --start | a, d | PRES:PRESENT_REROLL covers --update; nothing prints the --from/--reroll rerun for a plain re-roll (SQ:REGISTER only fires with a register); kept |
| NW51 | 51 | Exit 4: structured tool once, then proceed with assigned direction and state assumptions | a, d | SQ:PAGE CLOSED, PRES:PRESENT_FALLBACK; "proceed with assumptions" kept |
| NW52 | 51 | Harness capability variants (blocking shell, cannot hold --wait, pollable session, exit 2) | a, d | PRES:PRESENT_WAIT/FALLBACK; test-asserted (tests/skill-reference.test.mjs); kept in both drafts |
| NW53 | 53 | Every card declares comp under mocks/decision, canon included | a | PRES:COMPS_DIRECTION |
| NW54 | 53 | Sandboxed shell: start the page through the least-sandboxed path | a | SQ:start failure line ("A sandboxed exec that cannot listen on localhost...") |
| NW55 | 53 | Serve first, then comps; page shimmers | a | SQ:SCHEMA_NOTE, PRES:COMPS_DIRECTION |
| NW56 | 53 | Load visualize.md before the first decision comp prompt | a | SQ:NEXT read visualize.md (start, update, flip) |
| NW57 | 53 | Decision comp content: real page, first viewport plus start of section below, template skeleton, structure-led, real content, no claims, own palette | b, e | visualize.md Every comp + mode COMPS; "start of the section below" conflicts with the hero frame (C2) |
| NW58 | 53 | COMP SIDECAR MISSING / COMP STALE handling | a | SQ:--wait lines carry the instruction |
| NW59 | 53 | Equal fidelity in each card's grammar; aspect follows the surface | a, b | SQ:SCHEMA_NOTE aspect; visualize Frame; fairness clause kept in moderate |
| NW60 | 53 | Reading order: assigned, pick, hand, canon; declined get no comp | a | PRES:COMPS_DIRECTION |
| NW61 | 53 | Parallel subagents: asset producer per card, packet incl. MODE RULES COMPS verbatim, up to four | d | kept; conflicts with visualize "never delegated" (C3), resolved by scoping |
| NW62 | 53 | Regenerate empty slots inline; drop when user answers | d | kept |
| NW63 | 53 | Chosen comp not spent: option one (comp-led) or critique reference (code-led) | a | SQ:CHOSEN COMP |
| NW64 | 53 | Unchosen comps imply no approval | b | visualize.md L30 |
| NW65 | 53 | No image gen: palette chips complete; page demotes catalog art | a | SQ:SCHEMA_NOTE salience parity |
| NW66 | 55 | Build path is a preference, never asked per round | a | PRES:BUILD_PATH_*, CTX:BUILD_PATH_DEFAULT |
| NW67 | 55 | Config resolution: config.local.json over config.json; comp-led default with image gen | a | CTX:BUILD_PATH_DEFAULT, PRES:BUILD_PATH_RECORDED/NONE |
| NW68 | 55 | Payload buildPath with toggle; ANSWER returns buildPath and flipped | a | SQ:SCHEMA_NOTE, SQ:BUILD PATH |
| NW69 | 55 | One-time offer on flip with nothing recorded; yes writes flipped, no writes the other value | a, d | PRES:BUILD_PATH_NONE and SQ:BUILD PATH name the offer; the write semantics are kept |
| NW70 | 55 | User asks in words: update file without asking | d | kept |
| NW71 | 55 | Comp-led: comp is law, finish audits against it | a | SQ:BUILD PATH |
| NW72 | 55 | Code-led: ambition into FIRST VIEWPORT, signature interaction, motion grammar | a, d | SQ:BUILD PATH says "ambition lives in the direction contract"; specifics kept |
| NW73 | 55 | Flip reserve mid-round | a | SQ:BUILD PATH FLIPPED, SQ:SCHEMA_NOTE, PRES:COMPS_*_CODE |
| NW74 | 55 | No image gen: code-led, stated in one line; structured fallback has no toggle | a | PRES:BUILD_PATH_*, PRES:PRESENT_FALLBACK |
| NW75 | 55 | Old two-card execution-contract round retired; followup mechanism | e | history; SQ:FOLLOWUP OPEN covers followup |
| NW76 | 57 | Catalog worlds are working systems; carry palette, type, topology, controls, responsive | a | SEED:RICHNESS |
| NW77 | 57 | How far an interface-language source carries is the mode's call | c | mode-persuade/operate/read Directions |
| NW78 | 57 | Open QUALITY BAR board and hero when the choice lands; sandboxed download | a | SQ:CHOSEN CARD, SEED:CHALLENGER_SECTION |
| NW79 | 57 | Boards set the craft level, never the composition | a | SEED:CHALLENGER_SECTION; kept one clause |
| NW80 | 59 | Every direction viable before the roll; truth failures replaced, not rescued | d | kept |
| NW81 | 59 | Truth binds claims, not demonstrations; label synthetic; uninventable claims | d | kept once; duplicated at L129-130 and visualize L38 (b) |
| NW82 | 61 | MODE RULES govern and win | a | SEED:MODE_RULES_BLOCK header |
| NW83 | 61 | No roll: read the mode's file | d | kept |
| NW84 | 65 | Color strategy menu; page-scale commitment | d | kept |
| NW85 | 65 | "Restrained ... common on screens people operate or read" | c, e | conflicts with mode-operate.md ("a grey screen with one accent colour is the category default", C5); dropped |
| NW86 | 65 | Persuade/Experience may take bolder strategies | d | mode-specific, not in mode files; kept |
| NW87 | 65 | Dark or light from a sentence of physical scene | b | craft-floor.md skill-reflex-theme-by-habit; dropped |
| NW88 | 67 | Faces: reflex list, reason requirement, subject association never a reason | d | kept (ablation-validated family) |
| NW89 | 67 | Operate/Read system stacks; Persuade/Experience faces with POV | d | mode-specific, not in mode files; kept |
| NW90 | 69 | Calibration: four saturated looks, category self-check | d | kept |
| NW91 | 69 | Negative constraints rule out devices, not energy | d | kept |
| NW92 | 69 | Book subject is not cream license | d | kept |
| NW93 | 69 | Pinned world is not its softest rendition | d | kept |
| NW94 | 71-73 | `<claude>` measured rendition prior | d | kept, trimmed |
| NW95 | 77 | Direction contract: six blocks, FINISH verbatim, brief location | d | kept; test-asserted |
| NW96 | 77 | Contract timing: "before code" | e | ordering conflict with visualize L32 (C6); drafts say "right after the direction choice, add the approved comp path later" |
| NW97 | 79 | Never copy contract into source or browser artifacts (nine-item list) | d | kept; test-asserted |
| NW98 | 81 | DESIGN.md written at finish from the build | d | kept |
| NW99 | 83-89 | surface-brief read/write; re-read and verify six blocks and seed key | d | kept; test-asserted |
| NW100 | 91 | Keep the brief small | d | kept |
| NW101 | 93 | Comp-led: visualize before build, three options | a | BP:comps ("read reference/visualize.md, generate three...") |
| NW102 | 93 | Code-led skips by contract, never by drift | d | kept one clause |
| NW103 | 95 | shape returns before persistence | d, b | shape.md L39; kept |
| NW104 | 99 | Build the assigned direction; commit every atom | d | kept |
| NW105 | 103 | Comp is a spatial contract; only the user downgrades it | d | kept |
| NW106 | 103 | Models believe their recreation succeeded; state machine rationale | d | one clause in moderate, cut in aggressive |
| NW107 | 105 | build-phase start --direction --kind "is also the choice ping; the roll's output names the exact command" | e | obsolete: `build-phase start --kind` prints "choice ping skipped"; the roll prints SEED:TELEMETRY_BLOCK (rerun concept-seed --kind); C7 |
| NW108 | 107 | advance after each phase; exit 2 means fix and advance; nothing for later phases while a gate is open | a, d | BP:* NEXT and gate reasons; one sentence kept |
| NW109 | 109 | comps phase: three comps, sidecars, approval | a | BP:comps, gate_comps |
| NW110 | 110 | Frontier-tier note: weaker models say so before the direction round and take code-led | e | contradicts "build path is never a round's question" and the recorded default (C8); dropped in both drafts |
| NW111 | 112 | spec: grid, regions file, kinds, notes, comp-spec --regions | a | BP:spec |
| NW112 | 112 | Text/control narrowing to ink cluster (95%), `snap: false`, explicit box | a, b | comp-spec behavior; region-map.md |
| NW113 | 112 | font-match measure and rank; never write chosen face by hand | a | BP:spec, spec gate refusal ("A face typed into spec.json is the guess this gate exists to refuse") |
| NW114 | 112 | Painted note under a code kind refused | a | comp-spec painted-pixels, BP spec gate |
| NW115 | 112 | Unnamed ink refused; code region over a quarter refused | a | comp_spec.rs (`covers N% of the comp ... is a column`) |
| NW116 | 112 | SVG past icon budget refused at hero; runtime charts fine | a, d | hero gate veto; one clause kept in moderate |
| NW117 | 112 | Callout lines belong to the drawing's plate | b | region-map.md "What varies independently" (spirit); dropped |
| NW118 | 112 | Split by what varies independently | b | region-map.md |
| NW119 | 112 | A crop is never a plate | a | plates gate refusal; approved_comp.rs |
| NW120 | 112 | Plate box holds its whole artwork; bleed | a | comp_spec.rs clipped-artwork refusal |
| NW121 | 112 | Nothing not in the spec; three concessions | d | kept (rule:skill-comp-spec) |
| NW122 | 113 | --inspect-map before producing assets | b | region-map.md steps 3-4; pointer kept in moderate |
| NW123 | 115 | Plates production commands, transparency, verification | a | BP:plates (full command text) |
| NW124 | 115 | Asset producer harness names | d | kept (not printed by BP:plates) |
| NW125 | 115 | Comp is a fixed reference, never edited or regenerated | a, d | approved_comp.rs refusals; one clause kept |
| NW126 | 115 | Single-file deliverable inlines plates | d | kept in moderate |
| NW127 | 115 | --force only for user downgrade quoted in --reason | a | build-phase refuses other reasons |
| NW128 | 115 | Plan and asset review before page code | a, b | BP:plates names component-review; component-review.md |
| NW129 | 117 | hero: scaffold, bind to semantic structure, verbatim words, cap heights, plates first, record hero, then semantic layer | a | BP:hero |
| NW130 | 117 | Fonts served from project files, never hosted stylesheets | a, d | enforced by the first-viewport capture; kept as a clause (a retry costs a round) |
| NW131 | 117 | Gate mechanics: 72%, vetoes, advisories, region verdict meanings, three attempts, acceptance semantics | a, b | gate output, BP hero loop verdict, component-review.md "Acceptance closes human review" |
| NW132 | 117 | Ambition is won or lost here | d | kept |
| NW133 | 118-119 | sections, motion | a | BP:sections, BP:motion |
| NW134 | 120 | responsive: widths 1280-1600, captures, displaced, three attempts | a | BP:responsive, responsive loop verdict |
| NW135 | 124 | Code-led: no comp, ambition in contract, decision comp as critique reference | a, d | SQ:BUILD PATH, BP:start CODE-LED; one sentence kept |
| NW136 | 128 | First viewport is a thesis; memory test | d | kept ("standard hero" reworded to "header-and-cards shell", mode-neutral) |
| NW137 | 129 | Prove, don't claim; demo data | b | truth rule NW81 |
| NW138 | 130 | Author the assets; never substitute chrome | d | kept |
| NW139 | 131 | Build the form's web leverage | d | kept |
| NW140 | 132 | Pace the scroll; spacing rhythm | d, b | spacing half is craft-floor skill-layout-spacing-rhythm; pacing kept |
| NW141 | 133 | Real verified imagery | d | kept |
| NW142 | 134 | Author motion as material | b | craft-floor Motion bullet; dropped |
| NW143 | 136 | Preserve semantics, a11y, performance... | d | kept |
| NW144 | 140 | Batched inspection, user's viewport, two-round ceiling, gates stay open | b, d | SKILL.src.md core principles carry the ceiling; kept compactly |
| NW145 | 140 | Final comp-diff command and reading | d | kept |
| NW146 | 140 | Persuade: first-time visitor gets it in seconds | c | mode-persuade.md Directions; dropped |
| NW147 | 142 | Capture validity | d | kept |
| NW148 | 144 | Polishing over after round two | b | SKILL.src.md principle; one clause |
| NW149 | 144 | Hookless web: run detect --json once | a | CTX:MANUAL_DETECTOR_REQUIRED |
| NW150 | 144 | Native: no detector; reviewer told | d | kept |
| NW151 | 144 | Screenshot file names in .impeccable/review/ | d | kept |
| NW152 | 146 | Finish reviewer spawn, names, packet, fresh context, no definition reads, one long wait, five sections, respawn, degraded path | d | kept; CTX:SUBAGENT_AUTHORIZATION covers authorization and disclosure (b) |
| NW153 | 148 | Four dispositions and their actions; round budget; no second detector | d | kept |
| NW154 | 150 | Late raster provenance and --scan | d | kept |
| NW155 | 152 | Verdict scope; user evidence reopens review | d | kept |
| NW156 | 154 | Documenter spawn, degraded path, design.json, extensions | d | kept |
| NW157 | 156 | build-phase finish disposition; refused ship; re-record after edits; never fix over an open gate | a, d | BP:review and completion NEXT print the re-record rule; kept short |

## Inventory: visualize.md (main line numbers)

| ID | L | Instruction | Class | Where / note |
|---|---|---|---|---|
| VZ01 | 3 | Load on comp-led builds only; code-led must not load it | e | contradicted: SQ:NEXT sends every decision-comp round here, whatever the build path (C4) |
| VZ02 | 3 | PRODUCT.md and DESIGN.md are preconditions | e | on a new world DESIGN.md is written at finish (new-work L81) (C9) |
| VZ03 | 3 | MODE RULES COMPS govern and win | a, d | SEED:MODE_RULES_BLOCK; one sentence kept for the no-roll path |
| VZ04 | 3 | Do not reopen the world | d | kept |
| VZ05 | 3 | Surface round discharges the comp round | a, d | SEED:PROMOTED_SURFACE; kept one sentence |
| VZ06 | 5 | A probe tests composition, not identity; keep DESIGN.md world fixed | d | merged into the opener |
| VZ07 | 9 | Comp round runs after build-phase start; generate-image refuses before | a, d | generate-image guard; kept one clause because native tools are not guarded |
| VZ08 | 11 | Three comps under mocks/, at own viewport | a | BP:comps, gate_comps |
| VZ09 | 11 | Comps never delegated | d | kept, scoped to comp-round comps (C3) |
| VZ10 | 11 | Workspace-relative paths | d | kept one clause |
| VZ11 | 11 | Established world: screenshot reference, what carries and what leaks | d | kept |
| VZ12 | 11 | Three is the number | a | BP:comps; one clause kept |
| VZ13 | 11 | Decision comp is option one; two more from words, not as input image | d, a | SQ:CHOSEN COMP says option one; the copy step into mocks/ was missing (C10); both drafts add it |
| VZ14 | 11 | How comps two and three vary is the mode's call | c | mode COMPS sections |
| VZ15 | 11 | No decision comp: render all three | d | kept |
| VZ16 | 13 | Structure-led prompt; edge to edge; poster self-check | d | kept |
| VZ17 | 14 | Subject must be present; exclusions bind claims | d | kept |
| VZ18 | 15 | Mode readable from render | d | kept |
| VZ19 | 16 | Depth, not spread; content at reading size; one focal move | d | kept |
| VZ20 | 17 | Multiple shortlisted concepts: spread three across them | e | no path produces a multi-concept shortlist anymore (one direction is locked before the comp round) |
| VZ21 | 18 | One committed direction: vary topology, sequence, density... | c, e | superseded by mode COMPS (mode-persuade varies by going further in the same world, C11) |
| VZ22 | 19 | Show enough beyond the opening to prove the concept governs the surface | e | conflicts with comp frame = first viewport the hero gate builds (C2) |
| VZ23 | 20 | No palette artifact, no new type voice or motif; return to shortlist | d | folded into the opener |
| VZ24 | 22 | Comps are direction tests; UI text, responsive, a11y stay implementation work | d | dropped as implied by After approval |
| VZ25 | 26 | Show three on decision page or inline images; text-only not display | d | kept |
| VZ26 | 26 | Ask carry forward / false / approve-combine-revise-reject; stop; simulated user attended | d | kept |
| VZ27 | 28 | No code before approval or delegation; delegated pick from evidence | d | kept |
| VZ28 | 28 | Approval refines the concept, does not modify DESIGN.md | d | dropped (implied) in aggressive |
| VZ29 | 30 | No substitute; structured tool errors then decision page; delegation disclosed first reply | e, d | channel order inverted vs new-work and PRES (C12); drafts put the page first |
| VZ30 | 30 | Unapproved comp-round comps are a finish finding; decision comps imply no approval | d | kept in moderate |
| VZ31 | 32 | Record approval: brief path plus sidecar approved:true | a, d | SQ:APPROVED COMP prints it on a page pick; kept for the structured-tool path |
| VZ32 | 32 | Return to new-work, record the contract, build | e | ordering conflict with new-work section 5 (C6) |
| VZ33 | 36 | North star, no recomposition, no rasterized text, no driver swap | d | kept |
| VZ34 | 38 | Medium follows pixels; plate vs semantic lists | d | kept |
| VZ35 | 38 | CSS for sculpted finish / clip-path torn edge is deletion; detector catches | d, a | detector and hero gate catch; one clause kept in moderate |
| VZ36 | 38 | Dropping an image-native region is the user's decision | d | kept |
| VZ37 | 38 | Generated imagery is a material, not a claim | b | truth rule in new-work |
| VZ38 | 42 | Plate production commands | a | BP:plates |
| VZ39 | 42 | embed-prompt for any image, provenance definition, sourced origin, --scan, late rasters | d | kept |
| VZ40 | 44 | Converters from IMAGE_TOOLS; never probe per image | a | CTX:IMAGE_TOOLS ("never probe again this session") |
| VZ41 | 46 | Return to new-work | e | navigation only |

## Conflicts found

- **C1. Card anatomy, three versions.** new-work L49 lists "world, first viewport, visitor path, signature interaction, cross-surface reach, honest risk"; L51 lists "thesis, palette, materials, first viewport, honest risk, case lines"; SQ:SCHEMA_NOTE lists "thesis, palette, materials, viewport, risk". The schema wins in both drafts.
- **C2. Decision comp frame vs hero frame.** new-work L53 asks each decision comp for "its first viewport and the start of the section below it", and visualize L19 asks for "enough beyond the opening moment". The comp-led build treats the whole comp as the first viewport ("Build only the first viewport, at the comp's own dimensions"; BP:hero `--breakpoint` defaults to the comp's size). A comp that shows the section below gets that band built into the first viewport. Both drafts drop the "section below" and "beyond the opening" asks. Related pull: "full fidelity, committed all the way" and mode-persuade "material coverage is wanted" against visualize's depth rule; mode-persuade already reconciles it ("Coverage is not performance"), so the drafts keep only the depth rule plus the mode block.
- **C3. Delegating comps.** new-work L53 fans decision comps out to asset-producer subagents; visualize L11 says "Comps are the build thread's own work, never delegated". Drafts scope the ban to comp-round comps and keep the decision-round fan-out. Worth an eval axis of its own if fidelity of decision comps drops.
- **C4. When visualize.md applies.** visualize L3 says load it only on comp-led builds and never on code-led; serve-question's NEXT line (and new-work L53) send every decision-comp round to it, comp-led or code-led flip. Drafts open visualize with both entry points.
- **C5. Restrained color on Operate/Read.** new-work L65 calls Restrained (tinted neutrals plus one accent) "common on screens people operate or read"; mode-operate.md says "A grey screen with one accent colour is the category default ... not restraint". Drafts drop the new-work clause.
- **C6. Contract timing.** new-work section 5 records the contract "before code" and the comp round follows it; visualize L32 says "record the direction contract from the approved concept" after comp approval. Drafts: record right after the direction choice, add the approved comp path when the comp round closes.
- **C7. Choice ping.** new-work L105 says `build-phase start --direction --kind` "is also the choice ping; the roll's output names the exact command". The engine's `build-phase start` prints "choice ping skipped" for `--kind` and the roll prints SEED:TELEMETRY_BLOCK (`concept-seed --kind ... --from`). Drafts: send the TELEMETRY ping, then `build-phase start` without `--kind`.
- **C8. Frontier-tier note.** new-work L110 tells smaller models to "say so before the direction round and take the code-led path", against the rule that the build path is a recorded preference no round asks about. Dropped. If the maintainer wants a model-tier fallback, it belongs in `context` (one directive) rather than in prose a weak model must self-diagnose.
- **C9. DESIGN.md as precondition.** visualize L3 makes DESIGN.md a precondition and L5 keeps "DESIGN.md's palette ... fixed"; on a new world new-work L81 writes DESIGN.md at finish. Drafts: the world is fixed "by DESIGN.md or, on a new world, by the chosen direction".
- **C10. Decision comp as option one vs the comps gate.** visualize says the chosen decision comp is the first of the three and SQ:CHOSEN COMP says never regenerate it, but `gate_comps` counts only files directly under `.impeccable/mocks/` and decision comps live in `.impeccable/mocks/decision/`. Nothing says to copy it in. Both drafts add "copy it and its sidecar unchanged into `.impeccable/mocks/`". Engine follow-up worth considering: let the gate count an approved decision comp, or have the CHOSEN COMP line say "copy".
- **C11. How comps two and three vary.** visualize L18 says vary topology, sequence, density, hierarchy; mode-persuade says "go further than the first, never cleaner" in the same world; mode-operate and mode-read vary organisation only. The mode block wins; drafts delete L17-18.
- **C12. Comp-round channel order.** visualize L30: structured tool first, decision page when it errors. new-work and PRES: decision page first, structured tool as the fallback. Drafts follow new-work. (`gate_comps` also names "the structured question tool" as a way to show three images, which a text channel cannot do; engine wording follow-up.)
- **C13. Landing-page vocabulary in mode-neutral text.** new-work L128 "do not trap the concept inside a standard hero"; visualize L11 "a banner, hero, or card lifted verbatim". Drafts reword to "header-and-cards shell" and drop the list. The hero gate and `hero` payload field are engine names and stay.
- **C14. Two-round ceiling vs open gates** (new-work L140, L144, L156) is already reconciled in the text; not a conflict, but stated three times. Drafts state it once.

## Rule markers

All 31 markers in new-work.md (visualize.md has none) survive in both drafts, each still attached to the sentence that carries its rule:

skill-concept-procedure, skill-verdict-and-donation, skill-pick-card-one-only, skill-canon-standing-exit, skill-assigned-plus-reroll, skill-visual-decision-page, skill-decision-comps-full-fidelity, skill-salience-parity, skill-build-path-round, skill-truth-binds-claims, skill-color-strategy, skill-typo-reflex-faces, skill-calibration-saturated-looks, skill-constraints-rule-out-devices-not-energy, skill-book-subject-not-cream-license, skill-pinned-world-not-default-rendition, skill-decide-then-build, skill-design-md-from-the-build, skill-visualize-before-build, skill-commit-every-atom, skill-comp-spec, skill-plates-before-page, skill-human-component-review, skill-hero-gate, skill-author-assets-not-chrome, skill-capture-validity, skill-finish-separate-reviewer, skill-verdict-bounds-the-finish, skill-late-raster-provenance, skill-user-evidence-reopens-review, skill-documenter-records-the-world.

Removed markers: none. Several markers now tag a much shorter rule whose body moved into engine output (skill-concept-procedure, skill-verdict-and-donation, skill-salience-parity, skill-hero-gate, skill-plates-before-page). An ablation that strips by marker now strips less text for those ids; results from the v2.1 matrix for them do not transfer. In `~/code/impeccable-evals` only `skill-color-strategy` (CLAUDE.md, dashboard rules) and `skill-finish-separate-reviewer` (notes) are referenced.

## Test-asserted phrases

- `tests/skill-reference.test.mjs` "keeps direction contracts in development-only surface briefs": the `## 5. Record the decision` ... `## 6.` span must hold "development-only contract", "under `## Direction contract` in the relevant surface brief", "read the brief once more", "all six contract blocks and the seed key", the six `BLOCK:` labels, the nine browser-artifact phrases, and "Never copy the direction contract into implementation source or any browser-delivered artifact"; five retired phrases must stay absent. Kept in both drafts.
- Same file, "routes visual decision fallback through wait capability and start failure": the span from "A harness that can leave a shell blocked" to `<!-- rule:skill-visual-decision-page -->` must hold "cannot hold a blocking `--wait`", "without starting the page", "structured tool", "first reply", "exit code 2 from starting it", "wait check before starting", "session you can poll holds the wait", "rerun `--wait` only after it exits without an answer". Kept in both drafts (compressed in aggressive).
- `tests/skill-behavior/*` and `tests/skill-workflow/*` (opt-in, LLM-backed) assert behavior, not phrases: new-work.md is loaded for greenfield and established-world builds; a question precedes code; the surface brief is written before the page; DESIGN.md comes after the finished build; the degraded `finish-reviewer.md` and `documenter.md` load on a harness without subagents. Every draft keeps the instruction behind each.
- Engine text that names new-work sections: `build-phase start` CODE-LED line cites "reference/new-work.md section 5" and "section 7"; both drafts keep that numbering. `document.md` names the "**Create or replace the visual world**" heading; kept.
- Oracle: no golden reads these two files (mode-rules cases use `tests/fixtures/mode-rules-skill`).

Test changes: none on either branch.

## Behaviour checklist

Line numbers are in the draft files on their branches. "Engine" names what prints at that moment.

| Behaviour | moderate | aggressive | engine |
|---|---|---|---|
| Decision round: ground, seven candidates, roll, no skip | new-work 37-41 | new-work 23-27 | SEED:PROMOTED_DIRECTION |
| Fuse, verdicts, donations, named raises | 41 (pointer + motif clause) | 27 (pointer) | SEED:CHALLENGER_DIRECTION |
| Present: one raised direction, ≤3 full challengers, declined demoted, pick card | 42 | 28 | SEED:PROMOTED_DIRECTION, SQ:SCHEMA_NOTE |
| Decision page: payload, start, open, wait | 48-50 | 32 (pointer + reroll + capability variants) | PRES:PRESENT_FIRST/WAIT, SQ:start |
| Re-roll procedure (--from/--reroll, --update same key, never second --start) | 50 | 32 | PRES:PRESENT_REROLL, SQ:REGISTER |
| Channel fallback (exit 2, exit 4, unholdable wait, pollable session) | 50 | 32 | PRES:PRESENT_FALLBACK, SQ:PAGE CLOSED |
| Comps discipline: decision comps declared, served first, reading order, sidecars | new-work 52; visualize 5-14 | new-work 34; visualize 5-14 | PRES:COMPS_*, SQ:NEXT, SQ:COMP SIDECAR MISSING/STALE |
| Comps discipline: frame, structure-led, subject, mode readable, depth, real content | visualize 7-12 | visualize 7-12 | MODE RULES COMPS |
| Comp-round: after build-phase start, three comps, decision comp copied in as one, never delegated | visualize 18 | visualize 18 | BP:comps, gate_comps |
| Approval point, delegation, sidecar approved:true | visualize 22-28 | visualize 22 | SQ:APPROVED COMP, gate_comps |
| Comp-led phases (start, advance, gates) | new-work 92-101 | new-work 66 | BP:* NEXT, gate reasons, hero/responsive verdicts |
| Spec three concessions, nothing not in spec | new-work 98 | new-work 66 | spec gate refusals |
| Plates before page; asset producer; component review | new-work 99 | new-work 66 | BP:plates, component-review.md |
| Hero: crops in order, route after three failures, self-hosted fonts | new-work 100 | new-work 66 (fonts: engine only) | BP:hero, hero loop verdict |
| Build path handling (default, toggle, one-time offer, write semantics, no-image code-led) | new-work 54 | new-work 36 | PRES:BUILD_PATH_*, CTX:BUILD_PATH_DEFAULT, SQ:BUILD PATH, BP:start CODE-LED |
| Code-led ambition (FIRST VIEWPORT, signature interaction, motion) | 54, 103-105 | 36, 68 | SQ:BUILD PATH |
| Catalog challengers: working systems, QUALITY BAR boards | 42, 56 | 28 | SEED:RICHNESS, SEED:CHALLENGER_SECTION, SQ:CHOSEN CARD |
| Canon exit: never recommend, user's door, preference to PRODUCT.md | 44 | 29 | SQ:CANON CHOSEN, SQ:SCHEMA_NOTE, SEED:SAFER_BLOCK |
| Re-roll rules: eliminate shown, ask after two, factual grounds only, pins win, field-by-field | 46 | 30 | SEED:REROLL_BLOCK, SEED:MAIN |
| Truth rules (viability, claims vs demonstrations) | 58; visualize 12 | 38; visualize 12 | none |
| Direction contract and brief | 74-86 | 52-60 | BP:start CODE-LED cites section 5 |
| Finish: inspect, capture validity, reviewer packet, dispositions, provenance, verdict scope, documenter, build-phase finish | 118-136 | 72-81 | BP:review, completion NEXT, CTX:MANUAL_DETECTOR_REQUIRED, CTX:SUBAGENT_AUTHORIZATION |

Behaviours that live only in engine output under the aggressive draft (and therefore need #954): the decision-page payload shape and serve sequence, the comp declaration and reading order of decision comps, and the recorded build path's name and source.

## Notes for landing either draft

- The in-flight visualize.md write-order sentence (branch `fix/comp-write-order-and-render-check`) belongs in "Every comp" (Record) of either draft; the render-check NEXT line needs no file text.
- If #954 changes its wording, the aggressive draft's pointer sentences ("PRESENTATION prints its procedure", "PRESENTATION names it") need the block name only.
- Candidate follow-ups outside these two files: move NW23 (Persuade tools count toward the rut) and the color/type mode permissions (NW86, NW89) into the mode files; let `gate_comps` count the chosen decision comp (C10); fix the `gate_comps` structured-tool wording (C12).
