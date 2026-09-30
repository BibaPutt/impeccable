// Share of the viewport that raster images cover, measured from rendered boxes.
// Counts <img> (any source, data URIs included), <video>, <input type=image>,
// SVG <image>, and elements or ::before/::after pseudo-elements that paint a
// url() image (background, border, mask, list marker or generated content).
// Gradients are code and do not count. A no-repeat background with an explicit
// pixel size counts at that size; any other background counts at its box. The
// union is taken on a 4px grid so tiled or overlapping pieces add up once.
() => {
  const W = innerWidth, H = innerHeight, cell = 4;
  const cols = Math.ceil(W / cell), rows = Math.ceil(H / cell);
  const grid = new Uint8Array(cols * rows);
  const items = [];
  const url = v => typeof v === 'string' && /url\(/i.test(v);
  const mark = (r, what) => {
    const x0 = Math.max(0, r.left), y0 = Math.max(0, r.top);
    const x1 = Math.min(W, r.right), y1 = Math.min(H, r.bottom);
    if (x1 <= x0 || y1 <= y0) return;
    items.push({ what, box: { x: Math.round(x0), y: Math.round(y0), w: Math.round(x1 - x0), h: Math.round(y1 - y0) }, share: (x1 - x0) * (y1 - y0) / (W * H) });
    for (let y = Math.floor(y0 / cell); y < Math.ceil(y1 / cell); y++)
      for (let x = Math.floor(x0 / cell); x < Math.ceil(x1 / cell); x++) grid[y * cols + x] = 1;
  };
  const visible = s => s.display !== 'none' && s.visibility !== 'hidden' && s.opacity !== '0';
  const name = el => el.tagName.toLowerCase() + (el.id ? '#' + el.id : '') + (el.classList.length ? '.' + [...el.classList].slice(0, 2).join('.') : '');
  const px = v => /^[\d.]+px$/.test(v) ? parseFloat(v) : null;
  const background = (s, r) => {
    const size = (s.backgroundSize || '').split(' ').map(px);
    const norepeat = /^no-repeat( no-repeat)?$/.test(s.backgroundRepeat || '');
    if (norepeat && size.length === 2 && size[0] != null && size[1] != null) {
      const [x, y] = (s.backgroundPosition || '0px 0px').split(' ').map(v => px(v) ?? 0);
      return { left: r.left + x, top: r.top + y, right: r.left + x + size[0], bottom: r.top + y + size[1] };
    }
    return r;
  };
  for (const el of document.querySelectorAll('*')) {
    const s = getComputedStyle(el);
    if (!visible(s)) continue;
    const r = el.getBoundingClientRect();
    const tag = el.tagName.toLowerCase();
    if (tag === 'img' || tag === 'video' || tag === 'image' || (tag === 'input' && el.type === 'image')) mark(r, name(el));
    if (url(s.backgroundImage)) mark(background(s, r), name(el) + ' background');
    if (url(s.borderImageSource) || url(s.maskImage) || url(s.webkitMaskImage)) mark(r, name(el) + ' border/mask image');
    if (url(s.listStyleImage) && s.display === 'list-item') mark(r, name(el) + ' list marker');
    for (const p of ['::before', '::after']) {
      const ps = getComputedStyle(el, p);
      if (!ps || ps.content === 'none' || ps.content === 'normal' || !visible(ps)) continue;
      if (!(url(ps.content) || url(ps.backgroundImage) || url(ps.borderImageSource) || url(ps.maskImage))) continue;
      // A pseudo-element has no client rect. With a used pixel size, count that
      // size (at the viewport origin when fixed, else at the host's corner);
      // without one, count the host box.
      const w = px(ps.width), h = px(ps.height);
      if (w == null || h == null) { mark(r, name(el) + p); continue; }
      const fixed = ps.position === 'fixed';
      const left = fixed ? (px(ps.left) ?? 0) : r.left, top = fixed ? (px(ps.top) ?? 0) : r.top;
      mark({ left, top, right: left + w, bottom: top + h }, name(el) + p);
    }
  }
  const covered = grid.reduce((n, v) => n + v, 0);
  items.sort((a, b) => b.share - a.share);
  return { share: covered / (cols * rows), cell, largest: items.slice(0, 5), measure: 'rendered-box-union-v1' };
}
