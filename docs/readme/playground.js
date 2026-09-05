'use strict';
(() => {
  const $ = id => document.getElementById(id);
  const NS = 'http://www.w3.org/2000/svg';
  const names = {you:'You',a:'Edge A',b:'Edge B',x:'Transit A',y:'Transit B',web:'Open web'};
  const links = [
    {id:'direct',a:'you',b:'web',cost:18,label:'Direct path'},
    {id:'ua',a:'you',b:'a',cost:9,label:'You to Edge A'},
    {id:'ax',a:'a',b:'x',cost:12,label:'Edge A to Transit A'},
    {id:'xw',a:'x',b:'web',cost:13,label:'Transit A to Open web'},
    {id:'ub',a:'you',b:'b',cost:14,label:'You to Edge B'},
    {id:'by',a:'b',b:'y',cost:11,label:'Edge B to Transit B'},
    {id:'yw',a:'y',b:'web',cost:17,label:'Transit B to Open web'},
    {id:'tunnel',a:'you',b:'web',cost:28,label:'Operator tunnel',tunnel:true}
  ];
  const state = {blocked:new Set(),budget:60,tunnel:false,paused:false};
  const media = window.matchMedia('(max-width:700px)');
  const reduced = window.matchMedia('(prefers-reduced-motion:reduce)');
  let result = null;
  function svg(tag, attrs, text) { const e=document.createElementNS(NS,tag); for(const [k,v] of Object.entries(attrs||{}))e.setAttribute(k,String(v)); if(text!==undefined)e.textContent=text; return e; }
  function solve() {
    const dist = new Map(Object.keys(names).map(n=>[n,Infinity]));
    const previous = new Map(), visited = new Set();dist.set('you',0);
    while(visited.size<dist.size){
      const next=[...dist].filter(([n])=>!visited.has(n)).sort((a,b)=>a[1]-b[1]||a[0].localeCompare(b[0]))[0];
      if(!next||!Number.isFinite(next[1]))break;
      const [u,d]=next;visited.add(u);if(u==='web')break;
      for(const edge of links){
        if(state.blocked.has(edge.id)||(edge.tunnel&&!state.tunnel))continue;
        const v=edge.a===u?edge.b:edge.b===u?edge.a:null;if(!v||visited.has(v))continue;
        if(d+edge.cost<dist.get(v)){dist.set(v,d+edge.cost);previous.set(v,{node:u,edge:edge.id});}
      }
    }
    const cost=dist.get('web');
    if(!Number.isFinite(cost))return {ok:false,reason:'disconnected',cost:null,nodes:[],edges:[]};
    if(cost>state.budget)return {ok:false,reason:'over_budget',cost,nodes:[],edges:[]};
    const nodes=['web'],edges=[];let n='web';
    while(n!=='you'){const step=previous.get(n);if(!step)throw new Error('Broken predecessor chain');edges.unshift(step.edge);n=step.node;nodes.unshift(n);}
    return {ok:true,reason:'route_selected',cost,nodes,edges};
  }
  function geometry(){
    return media.matches?{width:400,height:455,p:{you:[40,216],a:[120,80],x:[280,80],b:[120,349],y:[280,349],web:[360,216]}}:{width:820,height:410,p:{you:[72,193],a:[282,86],x:[526,86],b:[282,294],y:[526,294],web:[748,193]}};
  }
  function toggle(id){state.blocked.has(id)?state.blocked.delete(id):state.blocked.add(id);render();document.querySelector('[data-edge="'+id+'"]')?.focus();}
  function render(){
    result=solve();const {width,height,p}=geometry();$('network').setAttribute('viewBox',`0 0 ${width} ${height}`);$('edges').replaceChildren();$('nodes').replaceChildren();
    for(const edge of links){
      const [x1,y1]=p[edge.a],[x2,y2]=p[edge.b];const disabled=state.blocked.has(edge.id);const denied=Boolean(edge.tunnel&&!state.tunnel);const selected=result.edges.includes(edge.id);const dy=media.matches?238:230;
      const d=edge.tunnel?`M${x1} ${y1} C${x1} ${y1+dy},${x2} ${y2+dy},${x2} ${y2}`:`M${x1} ${y1} L${x2} ${y2}`;
      const g=svg('g',{'data-edge':edge.id,tabindex:0,role:'button','aria-pressed':disabled,'aria-label':`${edge.label}, cost ${edge.cost} units, ${disabled?'blocked':denied?'excluded by tunnel policy':'available'}. Press to ${disabled?'restore':'block'}.`});
      g.append(svg('title',{},`${edge.label} · ${edge.cost} units`));
      g.append(svg('path',{d,class:'edge-hit'}));
      g.append(svg('path',{d,class:'edge-line'+(disabled?' edge-blocked':denied?' edge-policy':selected?' edge-selected':'')}));
      if(selected)g.append(svg('path',{d,class:'packet','aria-hidden':'true'}));
      let lx=(x1+x2)/2,ly=(y1+y2)/2-12;
      if(edge.id==='direct')ly=y1-14;if(edge.tunnel)ly=y1+dy*.75-10;
      g.append(svg('text',{x:lx,y:ly,'text-anchor':'middle',class:'edge-cost'},edge.tunnel?`TUNNEL / ${edge.cost}`:`${disabled?'× ':''}${edge.cost}`));
      g.addEventListener('click',()=>toggle(edge.id));g.addEventListener('keydown',e=>{if(e.key==='Enter'||e.key===' '){e.preventDefault();toggle(edge.id);}});$('edges').append(g);
    }
    for(const [id,[x,y]] of Object.entries(p)){
      const focus=id==='you'||id==='web';const g=svg('g');g.append(svg('circle',{cx:x,cy:y,r:focus?20:13,class:focus?'node-ring node-focus':'node-ring'}));g.append(svg('circle',{cx:x,cy:y,r:focus?4:3,fill:result.nodes.includes(id)?'#b6f5d2':'#a6b3c9'}));let ly=y+(focus?41:id==='a'||id==='x'?-24:32);g.append(svg('text',{x,y:ly,'text-anchor':'middle',class:'node-label'},names[id]));$('nodes').append(g);
    }
    $('status').textContent=result.ok?'TOY GATE: ALLOW':'TOY GATE: REJECT';$('status').classList.toggle('bad',!result.ok);
    $('cost').replaceChildren(document.createTextNode(result.ok?String(result.cost):'—'));const unit=document.createElement('small');unit.textContent=result.ok?' toy units':' no route';$('cost').append(unit);
    $('route').textContent=result.ok?result.nodes.map(n=>names[n]).join(' → '):result.reason==='disconnected'?'All eligible paths are blocked.':`Cheapest available path costs ${result.cost}; your limit is ${state.budget}.`;
    $('explanation').textContent=result.ok?(result.edges.includes('tunnel')?'Candidate accepted under the toy policy after you explicitly permitted the synthetic tunnel edge. No real tunnel is opened.':'Candidate accepted by the deterministic toy policy gate. No model or real traffic is involved.'):'Candidate rejected. No connection is claimed. Restore a link, relax the toy policy ceiling, or permit the synthetic tunnel edge.';
    $('break').textContent=state.blocked.has('direct')?'Restore direct path':'Break direct path';$('break').setAttribute('aria-pressed',String(state.blocked.has('direct')));$('budgetValue').value=`${state.budget} units`;
  }
  function receipt(){return {kind:'BLUESHOES_SYNTHETIC_DEMO_ONLY',model:'weighted-undirected-graph-v1',created_at:new Date().toISOString(),units:'abstract synthetic cost; not measured',is_runtime_evidence:false,policy:{max_cost:state.budget,operator_tunnel_allowed:state.tunnel},blocked_edges:[...state.blocked].sort(),edges:links,result};}
  function download(){const url=URL.createObjectURL(new Blob([JSON.stringify(receipt(),null,2)],{type:'application/json'}));const a=document.createElement('a');a.href=url;a.download='blueshoes-simulation-not-runtime-evidence.json';document.body.append(a);a.click();a.remove();setTimeout(()=>URL.revokeObjectURL(url),1000);}
  $('break').addEventListener('click',()=>{state.blocked.has('direct')?state.blocked.delete('direct'):state.blocked.add('direct');render();});
  $('isolate').addEventListener('click',()=>{state.blocked=new Set(links.map(e=>e.id));render();});
  $('reset').addEventListener('click',()=>{state.blocked.clear();state.budget=60;state.tunnel=false;$('budget').value='60';$('tunnel').checked=false;render();});
  $('budget').addEventListener('input',e=>{state.budget=Number(e.target.value);render();});
  $('tunnel').addEventListener('change',e=>{state.tunnel=e.target.checked;render();});
  $('pause').addEventListener('click',()=>{state.paused=!state.paused;document.body.classList.toggle('paused',state.paused);$('pause').textContent=state.paused?'Resume motion':'Pause motion';$('pause').setAttribute('aria-pressed',String(state.paused));});
  $('export').addEventListener('click',download);
  $('copy').addEventListener('click',async()=>{try{if(!navigator.clipboard)throw new Error('Clipboard unavailable');await navigator.clipboard.writeText(JSON.stringify(receipt(),null,2));$('copy').textContent='Copied — synthetic scenario only';}catch{$('copy').textContent='Clipboard unavailable — use Save receipt';}setTimeout(()=>$('copy').textContent='Copy this scenario as JSON',3500);});
  function motionPreference(){if(reduced.matches){$('pause').disabled=true;$('pause').textContent='Reduced motion respected';}else{$('pause').disabled=false;$('pause').textContent=state.paused?'Resume motion':'Pause motion';}}
  media.addEventListener('change',render);reduced.addEventListener('change',motionPreference);motionPreference();render();
})();
