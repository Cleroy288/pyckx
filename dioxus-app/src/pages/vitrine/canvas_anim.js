(function() {
  var canvas = document.getElementById('vt-canvas');
  var stage = document.getElementById('vt-stage');
  if (!canvas || !stage) return;
  var ctx = canvas.getContext('2d');

  var APPS = [
    {emoji:'\u{1F9E0}',name:'Intello',sub:'AI Study',
     color:'#5B47D6',bg:'#EDE9FE'},
    {emoji:'\u{26A1}',name:'Quick Mode',
     sub:'PDF \u2192 Quiz 10s',
     color:'#D97706',bg:'#FEF3C7'},
    {emoji:'\u{1F0CF}',name:'Flashcards',
     sub:'M\u00E9morisation',
     color:'#059669',bg:'#D1FAE5'},
    {emoji:'\u{1F4E6}',name:'Collection',
     sub:'Catalogue DVD',
     color:'#DB2777',bg:'#FCE7F3'},
    {emoji:'\u{1F4DA}',name:'Cours AI',
     sub:'G\u00E9n\u00E9ration IA',
     color:'#2563EB',bg:'#DBEAFE'},
    {emoji:'\u{2726}',name:'Bient\u00F4t\u2026',
     sub:'En pr\u00E9paration',
     color:'#9CA3AF',bg:'#F3F4F6'},
  ];
  var N = APPS.length;
  var CW = 158, CH = 54;
  var MAX_RX = 320, MAX_RY = 200;
  var dpr = 1, W = 0, H = 0, cx = 0, cy = 0;

  function setupCanvas() {
    dpr = window.devicePixelRatio || 1;
    var rect = stage.getBoundingClientRect();
    W = rect.width; H = rect.height;
    canvas.width = W * dpr;
    canvas.height = H * dpr;
    canvas.style.width = W + 'px';
    canvas.style.height = H + 'px';
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    cx = W / 2; cy = H / 2;
  }
  window.addEventListener('resize', setupCanvas);
  setupCanvas();

  var eio = function(t) {
    return t < 0.5
      ? 4*t*t*t
      : 1 - Math.pow(-2*t + 2, 3) / 2;
  };
  var eob = function(t) {
    return 1
      + 2.70158 * Math.pow(t - 1, 3)
      + 1.70158 * Math.pow(t - 1, 2);
  };
  var eoex = function(t) {
    return t >= 1
      ? 1
      : 1 - Math.pow(2, -10 * t);
  };
  var lerp = function(a, b, t) {
    return a + (b - a) * t;
  };
  var clamp = function(v, lo, hi) {
    return Math.max(lo, Math.min(hi, v));
  };
  var safeR = function(r) {
    return Math.max(0.001, r);
  };

  /* ── Phases (no intro in the loop) ───── */
  var MS_ORBIT = 4000;
  var MS_CONV = 1800;
  var MS_HOLD = 1200;
  var MS_SCATTER = 1800;
  var MS_CYCLE = MS_ORBIT
    + MS_CONV + MS_HOLD + MS_SCATTER;

  /* One-time intro before the loop starts */
  var MS_INTRO = 1600;
  var introPlayed = false;

  var nodes = APPS.map(function(app, i) {
    return {
      app:app, i:i, x:0, y:0,
      alpha:0, scale:0,
      snapX:0, snapY:0,
      burstFired:false, trail:[]
    };
  });

  var sparks = [], rings = [];
  var lastTime = null, elapsed = 0;

  function getOrbit(i, ms) {
    var margin = CW * 0.6;
    var rx = Math.min((W / 2) - margin, MAX_RX);
    var ry = Math.min((H / 2) - margin, MAX_RY);
    var scaleR = i % 2 === 0 ? 1.0 : 0.78;
    var cardRx = rx * scaleR;
    var cardRy = ry * (i % 2 === 0 ? 1.0 : 0.9);
    var speed = (0.00032 + i * 0.00004) / 1000;
    var angle = (i / N) * Math.PI * 2
      + ms * speed;
    return {
      x: cx + Math.cos(angle) * cardRx,
      y: cy + Math.sin(angle) * cardRy
    };
  }

  function spawnSparks(x, y, color) {
    for (var k = 0; k < 14; k++) {
      var a = (k / 14) * Math.PI * 2
        + (Math.random() - 0.5) * 0.35;
      var s = 50 + Math.random() * 90;
      sparks.push({
        x:x, y:y,
        vx:Math.cos(a)*s, vy:Math.sin(a)*s,
        color:color,
        r: 1.5 + Math.random() * 2,
        life:1, decay: 0.8 + Math.random() * 0.5
      });
    }
  }

  function spawnRing(x, y, color) {
    rings.push({
      x:x, y:y, r:6, targetR:80,
      color:color, life:1, decayRate:1.2
    });
  }

  function drawGrid() {
    ctx.save();
    ctx.fillStyle = 'rgba(91,71,214,0.055)';
    var step = 44;
    for (var gx = step * 0.8; gx < W; gx += step) {
      for (var gy = step * 0.8; gy < H; gy += step) {
        ctx.beginPath();
        ctx.arc(gx, gy, 1.1, 0, Math.PI * 2);
        ctx.fill();
      }
    }
    ctx.restore();
  }

  function drawLine(x1, y1, x2, y2, alpha, color) {
    if (alpha < 0.03) return;
    ctx.save();
    ctx.globalAlpha = alpha * 0.09;
    ctx.strokeStyle = color;
    ctx.lineWidth = 6;
    ctx.shadowColor = color;
    ctx.shadowBlur = 12;
    ctx.setLineDash([]);
    ctx.beginPath();
    ctx.moveTo(x1, y1);
    ctx.lineTo(x2, y2);
    ctx.stroke();
    ctx.globalAlpha = alpha * 0.35;
    ctx.lineWidth = 1.2;
    ctx.shadowBlur = 0;
    ctx.setLineDash([6, 4]);
    ctx.lineDashOffset = -(elapsed / 1000) * 30;
    ctx.beginPath();
    ctx.moveTo(x1, y1);
    ctx.lineTo(x2, y2);
    ctx.stroke();
    ctx.restore();
  }

  function drawTrail(trail, color) {
    if (trail.length < 3) return;
    ctx.save();
    ctx.fillStyle = color;
    for (var k = 1; k < trail.length; k++) {
      var t = k / (trail.length - 1);
      ctx.globalAlpha = t * 0.18;
      ctx.beginPath();
      ctx.arc(
        trail[k].x, trail[k].y,
        safeR(t * 2.8), 0, Math.PI * 2
      );
      ctx.fill();
    }
    ctx.restore();
  }

  function drawCard(app, x, y, scale, alpha, tilt) {
    if (alpha < 0.01 || scale < 0.01) return;
    ctx.save();
    ctx.globalAlpha = clamp(alpha, 0, 1);
    ctx.translate(x, y);
    ctx.scale(scale, scale);
    if (tilt) ctx.rotate(tilt);
    var w = CW, h = CH, r = 12;
    var X = -w / 2, Y = -h / 2;
    ctx.shadowColor = app.color + '22';
    ctx.shadowBlur = 16;
    ctx.shadowOffsetY = 4;
    ctx.beginPath();
    ctx.roundRect(X, Y, w, h, r);
    ctx.fillStyle = '#FFFFFF';
    ctx.fill();
    ctx.shadowBlur = 0;
    ctx.shadowOffsetY = 0;
    ctx.beginPath();
    ctx.roundRect(X, Y, 5, h, [r, 0, 0, r]);
    ctx.fillStyle = app.color;
    ctx.fill();
    ctx.beginPath();
    ctx.roundRect(X, Y, w, h, r);
    ctx.strokeStyle = app.color + '25';
    ctx.lineWidth = 1.5;
    ctx.stroke();
    ctx.beginPath();
    ctx.arc(X + 30, 0, 16, 0, Math.PI * 2);
    ctx.fillStyle = app.bg;
    ctx.fill();
    ctx.font = '13px serif';
    ctx.textAlign = 'center';
    ctx.textBaseline = 'middle';
    ctx.fillText(app.emoji, X + 30, 1);
    ctx.font =
      '700 10px "Unbounded",sans-serif';
    ctx.fillStyle = '#0D0D1A';
    ctx.textAlign = 'left';
    ctx.textBaseline = 'alphabetic';
    ctx.fillText(app.name, X + 54, -4);
    ctx.font =
      '600 9px "Mulish",sans-serif';
    ctx.fillStyle = '#7C7A8E';
    ctx.fillText(app.sub, X + 54, 11);
    ctx.restore();
  }

  /* ── One-time intro (fade in from center) ── */
  function updateIntro(n, i, lp) {
    var sfwd = i / N;
    var orb = getOrbit(i, elapsed);
    var d = clamp(
      (lp - sfwd * 0.4) / 0.6, 0, 1
    );
    var e = eob(clamp(d, 0, 1));
    n.alpha = clamp(d * 1.8, 0, 1);
    n.scale = lerp(0.3, 1, e);
    n.x = lerp(cx, orb.x, e);
    n.y = lerp(cy, orb.y, e);
    n.trail.push({x: n.x, y: n.y});
    if (n.trail.length > 18) n.trail.shift();
    n.burstFired = false;
    n.snapX = n.x; n.snapY = n.y;
  }

  /* ── Loop phases ─────────────────────── */
  function updateOrbit(n, i) {
    var orb = getOrbit(i, elapsed);
    n.alpha = 1; n.scale = 1;
    n.x = orb.x; n.y = orb.y;
    n.trail.push({x: n.x, y: n.y});
    if (n.trail.length > 18) n.trail.shift();
    n.snapX = n.x; n.snapY = n.y;
  }

  function updateConverge(n, i, lp) {
    var sfwd = i / N;
    var local = clamp(
      (lp - sfwd * 0.25) / (1 - sfwd * 0.25),
      0, 1
    );
    n.x = lerp(n.snapX, cx, eio(local));
    n.y = lerp(n.snapY, cy, eio(local));
    var fade = clamp(
      (local - 0.65) / 0.35, 0, 1
    );
    n.alpha = lerp(1, 0, eio(fade));
    n.scale = lerp(1, 0, eio(fade));
    n.trail = [];
    if (local >= 0.65 && !n.burstFired) {
      n.burstFired = true;
      spawnSparks(
        lerp(n.snapX, cx, 0.65),
        lerp(n.snapY, cy, 0.65),
        n.app.color
      );
      if (i % 2 === 0) {
        spawnRing(cx, cy, n.app.color);
      }
    }
  }

  function updateHold(n) {
    n.x = cx; n.y = cy;
    n.alpha = 0; n.scale = 0;
    n.burstFired = false; n.trail = [];
    n.snapX = cx; n.snapY = cy;
  }

  function updateScatter(n, i, lp) {
    var srev = (N - 1 - i) / N;
    var orb = getOrbit(i, elapsed);
    var local = clamp(
      (lp - srev * 0.2) / (1 - srev * 0.2),
      0, 1
    );
    n.x = lerp(cx, orb.x, eoex(local));
    n.y = lerp(cy, orb.y, eoex(local));
    n.scale = lerp(
      0, 1, eob(clamp(local * 1.05, 0, 1))
    );
    n.alpha = lerp(0, 1, eoex(local));
    n.trail.push({x: n.x, y: n.y});
    if (n.trail.length > 18) n.trail.shift();
  }

  function getPhase(cycleMs) {
    if (cycleMs < MS_ORBIT) {
      return {
        name: 'orbit',
        lp: cycleMs / MS_ORBIT
      };
    }
    var t = MS_ORBIT;
    if (cycleMs < t + MS_CONV) {
      return {
        name: 'converge',
        lp: (cycleMs - t) / MS_CONV
      };
    }
    t += MS_CONV;
    if (cycleMs < t + MS_HOLD) {
      return {
        name: 'hold',
        lp: (cycleMs - t) / MS_HOLD
      };
    }
    t += MS_HOLD;
    return {
      name: 'scatter',
      lp: (cycleMs - t) / MS_SCATTER
    };
  }

  function drawHoldPulse(lp) {
    var pulse =
      Math.sin(lp * Math.PI * 3) * 0.5 + 0.5;
    var radii = [60, 90, 125];
    for (var k = 0; k < radii.length; k++) {
      ctx.save();
      ctx.globalAlpha = Math.max(
        0, (0.08 - k * 0.025) * pulse
      );
      ctx.strokeStyle = '#5B47D6';
      ctx.lineWidth = k === 0 ? 2 : 1;
      ctx.shadowColor = '#5B47D6';
      ctx.shadowBlur = 10;
      ctx.beginPath();
      ctx.arc(
        cx, cy,
        safeR(radii[k] + pulse * 8),
        0, Math.PI * 2
      );
      ctx.stroke();
      ctx.restore();
    }
  }

  function drawSparksAndRings(dt) {
    rings = rings.filter(function(r) {
      return r.life > 0.01;
    });
    rings.forEach(function(r) {
      r.r += (r.targetR - r.r) * (dt / 1000) * 8;
      r.life -= r.decayRate * dt / 1000;
      var a = clamp(r.life * 0.45, 0, 1);
      ctx.save();
      ctx.globalAlpha = a;
      ctx.strokeStyle = r.color;
      ctx.lineWidth = 1.5;
      ctx.shadowColor = r.color;
      ctx.shadowBlur = 8;
      ctx.beginPath();
      ctx.arc(
        r.x, r.y, safeR(r.r),
        0, Math.PI * 2
      );
      ctx.stroke();
      ctx.restore();
    });
    sparks = sparks.filter(function(s) {
      return s.life > 0.01;
    });
    sparks.forEach(function(s) {
      var dtS = dt / 1000;
      s.x += s.vx * dtS;
      s.y += s.vy * dtS;
      s.vx *= Math.pow(0.88, dtS * 60);
      s.vy *= Math.pow(0.88, dtS * 60);
      s.life -= s.decay * dtS;
      var l = clamp(s.life, 0, 1);
      if (l < 0.01) return;
      ctx.save();
      ctx.globalAlpha = l * 0.8;
      ctx.fillStyle = s.color;
      ctx.shadowColor = s.color;
      ctx.shadowBlur = 6;
      ctx.beginPath();
      ctx.arc(
        s.x, s.y, safeR(s.r * l),
        0, Math.PI * 2
      );
      ctx.fill();
      ctx.restore();
    });
  }

  function tick(ts) {
    if (!document.getElementById('vt-canvas')) {
      return;
    }
    if (!lastTime) lastTime = ts;
    var dt = Math.min(ts - lastTime, 64);
    lastTime = ts;
    elapsed += dt;

    ctx.clearRect(0, 0, W, H);
    drawGrid();

    /* First play: one-time intro from center */
    if (!introPlayed && elapsed < MS_INTRO) {
      var ilp = elapsed / MS_INTRO;
      nodes.forEach(function(n, i) {
        updateIntro(n, i, ilp);
      });
    } else {
      introPlayed = true;
      /* Seamless loop: orbit→converge→hold→scatter */
      var loopTime = introPlayed
        ? elapsed - MS_INTRO
        : elapsed;
      var ph = getPhase(
        ((loopTime % MS_CYCLE) + MS_CYCLE)
          % MS_CYCLE
      );

      nodes.forEach(function(n, i) {
        if (ph.name === 'orbit') {
          updateOrbit(n, i);
        } else if (ph.name === 'converge') {
          updateConverge(n, i, ph.lp);
        } else if (ph.name === 'hold') {
          updateHold(n);
        } else {
          updateScatter(n, i, ph.lp);
        }
      });

      if (ph.name === 'hold') {
        drawHoldPulse(ph.lp);
      }
    }

    /* Draw connections, trails, cards */
    nodes.forEach(function(n) {
      if (n.alpha > 0.04) {
        drawLine(
          n.x, n.y, cx, cy,
          n.alpha * 0.4, n.app.color
        );
      }
    });
    nodes.forEach(function(n) {
      drawTrail(n.trail, n.app.color);
    });

    var sorted = nodes.slice().sort(
      function(a, b) { return a.alpha - b.alpha; }
    );
    sorted.forEach(function(n) {
      drawCard(
        n.app, n.x, n.y,
        n.scale, n.alpha, 0
      );
    });

    drawSparksAndRings(dt);
    requestAnimationFrame(tick);
  }

  requestAnimationFrame(tick);
})();
