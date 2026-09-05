'use strict';

(() => {
  const NS = 'http://www.w3.org/2000/svg';
  const nodes = [
    { id: 'you', label: 'YOU', x: 90, y: 180, endpoint: true },
    { id: 'a', label: 'A', x: 340, y: 70 },
    { id: 'b', label: 'B', x: 660, y: 70 },
    { id: 'c', label: 'C', x: 340, y: 290 },
    { id: 'd', label: 'D', x: 660, y: 290 },
    { id: 'destination', label: 'DESTINATION', x: 910, y: 180, endpoint: true },
  ];
  const edges = [
    { id: 'direct', from: 'you', to: 'destination', cost: 18, label: 'Direct connection' },
    { id: 'ya', from: 'you', to: 'a', cost: 9, label: 'You to A' },
    { id: 'ab', from: 'a', to: 'b', cost: 12, label: 'A to B' },
    { id: 'bd', from: 'b', to: 'destination', cost: 13, label: 'B to Destination' },
    { id: 'yc', from: 'you', to: 'c', cost: 14, label: 'You to C' },
    { id: 'cd', from: 'c', to: 'd', cost: 11, label: 'C to D' },
    { id: 'ddest', from: 'd', to: 'destination', cost: 17, label: 'D to Destination' },
  ];
  const byId = new Map(nodes.map(node => [node.id, node]));
  const state = { blocked: new Set(), maxCost: 60, paused: false };
  const ids = ['surgery-network', 'flow-edges', 'flow-nodes', 'surgery-theatre',
    'surgery-caption', 'surgery-detail', 'surgery-phase', 'surgery-cost',
    'surgery-limit', 'surgery-receipt', 'cut-link', 'tighten-rules',
    'restore-network', 'save-receipt', 'pause-flow'];
  const ui = Object.fromEntries(ids.map(id => [id, document.getElementById(id)]));
  if (ids.some(id => !ui[id])) return;

  const reducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)');
  const compactLayout = window.matchMedia('(max-width: 700px)');
  let receipt = null;

  function solve(blocked, maxCost) {
    const distance = new Map(nodes.map(node => [node.id, Infinity]));
    const previous = new Map();
    const visited = new Set();
    distance.set('you', 0);
    while (visited.size < nodes.length) {
      const next = nodes.filter(node => !visited.has(node.id))
        .sort((left, right) => distance.get(left.id) - distance.get(right.id)
          || left.id.localeCompare(right.id))[0];
      if (!next || !Number.isFinite(distance.get(next.id))) break;
      visited.add(next.id);
      if (next.id === 'destination') break;
      for (const edge of edges) {
        if (blocked.has(edge.id)) continue;
        const neighbor = edge.from === next.id ? edge.to
          : edge.to === next.id ? edge.from : null;
        if (!neighbor || visited.has(neighbor)) continue;
        const candidateCost = distance.get(next.id) + edge.cost;
        if (candidateCost < distance.get(neighbor)) {
          distance.set(neighbor, candidateCost);
          previous.set(neighbor, { node: next.id, edge: edge.id });
        }
      }
    }
    const cost = distance.get('destination');
    if (!Number.isFinite(cost)) {
      return { ok: false, reason: 'disconnected', candidate: null };
    }
    const pathNodes = ['destination'];
    const pathEdges = [];
    let current = 'destination';
    while (current !== 'you') {
      const step = previous.get(current);
      pathEdges.unshift(step.edge);
      pathNodes.unshift(step.node);
      current = step.node;
    }
    return {
      ok: cost <= maxCost,
      reason: cost <= maxCost ? 'route_selected' : 'over_budget',
      candidate: { nodes: pathNodes, edges: pathEdges, cost },
    };
  }

  function svgElement(tag, attributes, text) {
    const element = document.createElementNS(NS, tag);
    for (const [key, value] of Object.entries(attributes || {})) {
      element.setAttribute(key, String(value));
    }
    if (text !== undefined) element.textContent = text;
    return element;
  }

  function toggleEdge(id) {
    if (state.blocked.has(id)) state.blocked.delete(id);
    else state.blocked.add(id);
    render(id);
  }

  function drawNetwork(result, focusEdge) {
    const compact = compactLayout.matches;
    const compactPositions = {
      you: [42, 155], a: [130, 45], b: [270, 45],
      c: [130, 260], d: [270, 260], destination: [358, 155],
    };
    const visibleNodes = nodes.map(node => compact
      ? { ...node, x: compactPositions[node.id][0], y: compactPositions[node.id][1] }
      : node);
    const visibleById = new Map(visibleNodes.map(node => [node.id, node]));
    ui['surgery-network'].setAttribute('viewBox', compact ? '0 0 400 310' : '0 0 1000 360');
    ui['flow-edges'].replaceChildren();
    ui['flow-nodes'].replaceChildren();
    for (const edge of edges) {
      const from = visibleById.get(edge.from);
      const to = visibleById.get(edge.to);
      const blocked = state.blocked.has(edge.id);
      const candidate = Boolean(result.candidate?.edges.includes(edge.id));
      const active = result.ok && candidate;
      const path = `M ${from.x} ${from.y} L ${to.x} ${to.y}`;
      const group = svgElement('g', {
        class: 'flow-edge-group', 'data-edge': edge.id, tabindex: 0, role: 'button',
        'aria-pressed': blocked,
        'aria-label': `${edge.label}, ${edge.cost} toy units, ${blocked ? 'blocked' : 'available'}. Press to ${blocked ? 'restore' : 'cut'} this link.`,
      });
      group.append(svgElement('title', {}, `${edge.label}: ${edge.cost} toy units`));
      group.append(svgElement('path', { d: path, class: 'flow-hit' }));
      group.append(svgElement('path', {
        d: path,
        class: `flow-edge${blocked ? ' blocked' : ''}${active ? ' active' : ''}${candidate && !result.ok ? ' candidate' : ''}`,
        'aria-hidden': true,
      }));
      if (active) group.append(svgElement('path', {
        d: path, class: 'flow-packet', pathLength: 100, 'aria-hidden': true,
      }));
      const bottom = edge.from === 'c' || edge.from === 'd' || edge.to === 'c';
      group.append(svgElement('text', {
        x: (from.x + to.x) / 2,
        y: (from.y + to.y) / 2 + (bottom ? 25 : -17),
        class: `flow-cost-label${blocked ? ' blocked' : ''}`,
        'text-anchor': 'middle', 'aria-hidden': true,
      }, `${blocked ? '× ' : ''}${edge.cost}`));
      group.addEventListener('click', () => toggleEdge(edge.id));
      group.addEventListener('keydown', event => {
        if (event.key === 'Enter' || event.key === ' ') {
          event.preventDefault();
          toggleEdge(edge.id);
        }
      });
      ui['flow-edges'].append(group);
      if (focusEdge === edge.id) group.focus();
    }
    for (const node of visibleNodes) {
      const active = result.ok && result.candidate.nodes.includes(node.id);
      const group = svgElement('g', { class: 'flow-node-group' });
      group.append(svgElement('circle', {
        cx: node.x, cy: node.y, r: compact ? (node.endpoint ? 23 : 16) : (node.endpoint ? 32 : 22),
        class: `flow-node${node.endpoint ? ' endpoint' : ''}${active ? ' active' : ''}`,
      }));
      group.append(svgElement('text', {
        x: node.x, y: node.y + (compact ? (node.endpoint ? 43 : 33) : (node.endpoint ? 55 : 43)),
        class: `flow-node-label${compact ? ' compact' : ''}${compact && node.id === 'destination' ? ' compact-endpoint' : ''}`,
        'text-anchor': 'middle', 'font-size': compact ? (node.id === 'destination' ? 10 : 11) : 13,
      }, node.label));
      ui['flow-nodes'].append(group);
    }
  }

  function render(focusEdge) {
    const result = solve(state.blocked, state.maxCost);
    const direct = result.ok && result.candidate.edges.includes('direct');
    let caption;
    let detail;
    if (result.ok) {
      caption = direct ? 'A little connection. A whole world on the other end.'
        : 'The scenic route is still a route. Hello again, destination.';
      const route = result.candidate.nodes.map(id => byId.get(id).label).join(' → ');
      detail = `${route}. Cheapest available path: ${result.candidate.cost} toy units; policy limit: ${state.maxCost}. Same endpoints, permitted path. All data is invented.`;
    } else if (result.reason === 'over_budget') {
      caption = 'There is a way around. Your rules say: not at that price.';
      detail = `The cheapest available path costs ${result.candidate.cost} toy units, above your limit of ${state.maxCost}. Loosen the rule or restore a cheaper link. No connection is claimed.`;
    } else {
      caption = 'You found the edge of the little internet. Nice work.';
      detail = 'Every path between you and the destination is disconnected. Restore a link or reset the network. Even a very confident arrow cannot invent connectivity.';
    }
    ui['surgery-caption'].textContent = caption;
    ui['surgery-detail'].textContent = detail;
    ui['surgery-phase'].textContent = !result.ok ? '03 / REFUSED'
      : direct ? '01 / DIRECT' : '02 / BYPASS';
    ui['surgery-cost'].textContent = result.candidate
      ? `${result.candidate.cost} toy units${result.ok ? '' : ' / rejected'}` : 'No available path';
    ui['surgery-limit'].textContent = `${state.maxCost} toy units`;
    ui['surgery-theatre'].setAttribute('data-phase', !result.ok ? 'refused' : direct ? 'direct' : 'bypass');
    ui['cut-link'].textContent = state.blocked.has('direct') ? 'Restore direct link' : 'Cut direct link';
    ui['cut-link'].setAttribute('aria-pressed', String(state.blocked.has('direct')));
    ui['tighten-rules'].textContent = state.maxCost === 24 ? 'Loosen the rules' : 'Tighten the rules';
    ui['tighten-rules'].setAttribute('aria-pressed', String(state.maxCost === 24));
    receipt = {
      kind: 'SYNTHETIC_BROWSER_DEMO',
      is_runtime_evidence: false,
      model: 'weighted-undirected-graph-v1',
      units: 'invented toy cost; not latency or a benchmark',
      max_cost: state.maxCost,
      blocked_edges: [...state.blocked].sort(),
      selected_candidate: result.candidate,
      ok: result.ok,
      reason: result.reason,
    };
    ui['surgery-receipt'].textContent = JSON.stringify(receipt, null, 2);
    drawNetwork(result, focusEdge);
  }

  function syncMotion() {
    const paused = state.paused || reducedMotion.matches;
    ui['surgery-theatre'].classList.toggle('motion-paused', paused);
    ui['pause-flow'].setAttribute('aria-pressed', String(paused));
    ui['pause-flow'].disabled = reducedMotion.matches;
    ui['pause-flow'].textContent = reducedMotion.matches ? 'Reduced motion respected'
      : paused ? 'Resume motion' : 'Pause motion';
  }

  ui['cut-link'].addEventListener('click', () => {
    if (state.blocked.has('direct')) state.blocked.delete('direct');
    else state.blocked.add('direct');
    render();
  });
  ui['tighten-rules'].addEventListener('click', () => {
    state.maxCost = state.maxCost === 60 ? 24 : 60;
    render();
  });
  ui['restore-network'].addEventListener('click', () => {
    state.blocked.clear();
    state.maxCost = 60;
    render();
  });
  ui['pause-flow'].addEventListener('click', () => {
    state.paused = !state.paused;
    syncMotion();
  });
  document.querySelector('.receipt-jump')?.addEventListener('click', () => {
    const panel = document.getElementById('receipt');
    if (panel) panel.open = true;
  });
  ui['save-receipt'].addEventListener('click', () => {
    const blob = new Blob([JSON.stringify(receipt, null, 2)], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = url;
    link.download = 'blueshoes-synthetic-flow-receipt.json';
    document.body.append(link);
    link.click();
    link.remove();
    setTimeout(() => URL.revokeObjectURL(url), 1000);
  });
  reducedMotion.addEventListener('change', syncMotion);
  compactLayout.addEventListener('change', () => {
    const focusedEdge = document.activeElement?.getAttribute('data-edge');
    render(focusedEdge || undefined);
  });
  for (const id of ['cut-link', 'tighten-rules', 'restore-network', 'save-receipt', 'pause-flow']) {
    ui[id].disabled = false;
  }
  syncMotion();
  render();
})();
