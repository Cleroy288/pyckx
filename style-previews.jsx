import { useState, useEffect } from "react";

const TABS = ["Actuel++", "Grunge Soft", "Neon Chalk"];

/* ─── Shared fake data ─── */
const stats = [
  { label: "Games", value: "9", accent: true },
  { label: "Conversion", value: "10s", accent: false },
  { label: "Prix", value: "∞ free", accent: true },
];

const collections = [
  { emoji: "🌍", name: "Culture Générale", count: 42, progress: 72 },
  { emoji: "💻", name: "Code & Algo", count: 38, progress: 45 },
  { emoji: "⚖️", name: "Droit Civil L2", count: 56, progress: 88 },
  { emoji: "🧬", name: "Bio Cellulaire", count: 31, progress: 33 },
];

const games = [
  { tag: "QCM", title: "Quiz Culture G", desc: "20 questions sur l'histoire", icon: "🧠", featured: true },
  { tag: "Flashcard", title: "Vocabulaire EN", desc: "200 mots business", icon: "🃏", featured: false },
  { tag: "Vrai/Faux", title: "Bio Cellulaire", desc: "Les bases de la cellule", icon: "🔬", featured: false },
  { tag: "Fill", title: "Code Civil Art.1", desc: "Articles fondamentaux", icon: "⚖️", featured: false },
  { tag: "QCM", title: "Philo Terminale", desc: "Conscience et liberté", icon: "🤔", featured: false },
];

/* ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   VARIANTE A — "Actuel++"
   Ton style actuel poussé plus loin :
   - Split logo conservé
   - Bordures 2px + offset shadows
   - Mais : angles plus cassés, sections qui débordent,
     éléments légèrement tournés, plus de lime, plus de
     mouvement, éléments décoratifs scattered
   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ */
function ActuelPlusPlus() {
  return (
    <div className="min-h-screen" style={{
      background: "#F8F7FF",
      backgroundImage: "radial-gradient(rgba(91,71,214,0.08) 1px, transparent 1px)",
      backgroundSize: "20px 20px",
      fontFamily: "'Outfit', sans-serif",
    }}>
      {/* ── Nav ── */}
      <nav className="sticky top-0 z-50 px-6 h-16 flex items-center justify-between" style={{
        background: "rgba(248,247,255,0.85)",
        backdropFilter: "blur(16px)",
        borderBottom: "2px solid #0D0D1A",
      }}>
        <div className="flex items-center gap-6">
          {/* Split logo */}
          <div className="relative leading-none select-none" style={{ fontSize: "28px", fontFamily: "'Unbounded', sans-serif", fontWeight: 900 }}>
            <span className="relative z-10" style={{ WebkitTextStroke: "2px #0D0D1A", color: "transparent" }}>PYCKX</span>
            <span className="absolute left-[3px] top-[3px]" style={{ color: "#5B47D6" }}>PYCKX</span>
          </div>
          <div className="flex gap-1">
            {["Play", "Explorer", "Stats"].map((t, i) => (
              <button key={i} className="px-3 py-1.5 text-xs font-bold uppercase tracking-wider rounded-sm border-2 transition-all duration-150" style={{
                fontFamily: "'Unbounded', sans-serif",
                fontSize: "10px",
                borderColor: i === 0 ? "#0D0D1A" : "transparent",
                background: i === 0 ? "#A3E635" : "transparent",
                color: "#0D0D1A",
                boxShadow: i === 0 ? "2px 2px 0 #0D0D1A" : "none",
              }}>{t}</button>
            ))}
          </div>
        </div>
        <div className="w-9 h-9 rounded-md border-2 border-[#0D0D1A] bg-[#5B47D6] text-white flex items-center justify-center text-xs font-bold" style={{ fontFamily: "'Unbounded'" }}>CL</div>
      </nav>

      <div className="max-w-6xl mx-auto px-6 py-10">
        {/* ── Hero ── */}
        <div className="relative p-10 mb-12 overflow-hidden" style={{
          background: "white",
          border: "2px solid #0D0D1A",
          boxShadow: "8px 8px 0 #0D0D1A",
        }}>
          {/* Orbs (like your current hero) */}
          <div className="absolute -top-20 -right-20 w-72 h-72 rounded-full opacity-15 blur-3xl" style={{ background: "radial-gradient(circle, #5B47D6, transparent 70%)" }} />
          <div className="absolute -bottom-16 -left-16 w-56 h-56 rounded-full opacity-20 blur-3xl" style={{ background: "radial-gradient(circle, #A3E635, transparent 70%)" }} />

          {/* Eyebrow */}
          <div className="inline-flex items-center gap-2 px-3 py-1 mb-6 rounded-sm border border-[#E4E0F5]" style={{ background: "#EDE9FE" }}>
            <span className="w-2 h-2 rounded-full bg-[#A3E635] animate-pulse" />
            <span className="text-xs font-semibold text-[#5B47D6]" style={{ fontFamily: "'Unbounded'", fontSize: "10px" }}>NOUVEAU</span>
          </div>

          {/* Split hero title — bigger, more offset */}
          <div className="relative mb-4" style={{ fontSize: "52px", fontFamily: "'Unbounded'", fontWeight: 900, lineHeight: 1, letterSpacing: "-3px" }}>
            <span className="relative z-10" style={{ WebkitTextStroke: "2.5px #0D0D1A", color: "transparent" }}>Let's play.</span>
            <span className="absolute left-[4px] top-[4px] z-0" style={{ color: "#5B47D6" }}>Let's play.</span>
          </div>

          <p className="text-[#7C7A8E] text-lg max-w-lg mb-8" style={{ lineHeight: 1.6 }}>
            Apprends en jouant. <strong className="text-[#0D0D1A]">QCM, flashcards, vrai/faux</strong> — 9 types de jeux pour réviser sans t'ennuyer.
          </p>

          {/* CTA buttons */}
          <div className="flex gap-3 mb-8">
            <button className="px-6 py-3 text-sm font-bold uppercase tracking-wider border-2 border-[#0D0D1A] transition-all duration-150 hover:-translate-x-0.5 hover:-translate-y-0.5" style={{
              fontFamily: "'Unbounded'",
              fontSize: "11px",
              background: "#5B47D6",
              color: "white",
              boxShadow: "4px 4px 0 #0D0D1A",
            }}>Commencer →</button>
            <button className="px-6 py-3 text-sm font-bold uppercase tracking-wider border-2 border-[#0D0D1A] bg-transparent text-[#0D0D1A] transition-all duration-150 hover:bg-[#A3E635]" style={{
              fontFamily: "'Unbounded'",
              fontSize: "11px",
              boxShadow: "3px 3px 0 #0D0D1A",
            }}>Explorer</button>
          </div>

          {/* Stats row (like your current) */}
          <div className="flex items-center gap-0 border-2 border-[#0D0D1A] w-fit" style={{ boxShadow: "3px 3px 0 #0D0D1A" }}>
            {stats.map((s, i) => (
              <div key={i} className="px-6 py-3 flex items-center gap-3" style={{ borderRight: i < stats.length - 1 ? "2px solid #0D0D1A" : "none" }}>
                <span className="text-2xl font-black" style={{ fontFamily: "'Unbounded'", color: s.accent ? "#5B47D6" : "#0D0D1A" }}>{s.value}</span>
                <span className="text-xs uppercase tracking-wider text-[#7C7A8E] font-semibold" style={{ fontFamily: "'Unbounded'", fontSize: "9px" }}>{s.label}</span>
              </div>
            ))}
          </div>

          {/* Decorative scattered elements — the "décalé" touch */}
          <div className="absolute top-6 right-8 px-2 py-1 bg-[#A3E635] text-[#0D0D1A] text-xs font-black border-2 border-[#0D0D1A]" style={{ transform: "rotate(12deg)", fontFamily: "'Unbounded'", fontSize: "9px", boxShadow: "2px 2px 0 #0D0D1A" }}>FREE</div>
          <div className="absolute bottom-8 right-16 text-4xl opacity-10" style={{ transform: "rotate(-8deg)" }}>🎲</div>
        </div>

        {/* ── Section divider (lime bar like yours) ── */}
        <div className="mb-8 flex items-center gap-4">
          <div className="h-1 w-12 bg-[#A3E635]" style={{ boxShadow: "2px 2px 0 #0D0D1A" }} />
          <span className="text-xs font-black uppercase tracking-[0.3em] text-[#7C7A8E]" style={{ fontFamily: "'Unbounded'", fontSize: "10px" }}>Continue</span>
          <div className="h-px flex-1 bg-[#E4E0F5]" />
        </div>

        {/* ── Games grid (brutalist cards like yours) ── */}
        <div className="grid grid-cols-3 gap-4 mb-12" style={{ gridAutoRows: "minmax(160px, auto)" }}>
          {games.map((g, i) => {
            const isFeat = i === 0;
            return (
              <div key={i} className={`relative p-6 border-2 border-[#0D0D1A] cursor-pointer transition-all duration-200 group overflow-hidden ${isFeat ? "row-span-2 col-span-1" : ""}`}
                style={{
                  background: isFeat ? "#5B47D6" : "white",
                  color: isFeat ? "white" : "#0D0D1A",
                  boxShadow: "4px 4px 0 #0D0D1A",
                  borderRadius: "0px",
                }}
              >
                {/* Hover: card lifts */}
                <style>{`.group:hover { transform: translate(-2px, -2px); box-shadow: 6px 6px 0 #0D0D1A !important; }`}</style>

                {/* Tag */}
                <div className="inline-block px-2 py-0.5 mb-3 text-xs font-bold uppercase tracking-wider" style={{
                  fontFamily: "'Unbounded'",
                  fontSize: "9px",
                  background: isFeat ? "rgba(255,255,255,0.15)" : "#EDE9FE",
                  color: isFeat ? "white" : "#5B47D6",
                  border: isFeat ? "1px solid rgba(255,255,255,0.2)" : "none",
                }}>{g.tag}</div>

                <div className="text-3xl mb-3">{g.icon}</div>
                <h3 className="text-lg font-bold mb-1" style={{ fontFamily: "'Unbounded'", fontSize: isFeat ? "22px" : "16px", letterSpacing: "-0.5px" }}>{g.title}</h3>
                <p className="text-sm opacity-60">{g.desc}</p>

                {/* Bottom accent bar on hover */}
                <div className="absolute bottom-0 left-0 right-0 h-1 transition-transform duration-300 origin-left scale-x-0 group-hover:scale-x-100" style={{ background: isFeat ? "#A3E635" : "#5B47D6" }} />

                {/* Arrow (slides in) */}
                <div className="absolute bottom-4 right-4 opacity-0 translate-x-2 group-hover:opacity-60 group-hover:translate-x-0 transition-all duration-200 text-lg">→</div>
              </div>
            );
          })}
        </div>

        {/* ── Collections (tilted cards — the "brouillon" touch) ── */}
        <div className="mb-8 flex items-center gap-4">
          <div className="h-1 w-12 bg-[#A3E635]" style={{ boxShadow: "2px 2px 0 #0D0D1A" }} />
          <span className="text-xs font-black uppercase tracking-[0.3em] text-[#7C7A8E]" style={{ fontFamily: "'Unbounded'", fontSize: "10px" }}>Collections</span>
          <div className="h-px flex-1 bg-[#E4E0F5]" />
        </div>

        <div className="grid grid-cols-4 gap-5 mb-12">
          {collections.map((c, i) => {
            const rotations = [-1.5, 0.8, -0.5, 1.2];
            return (
              <div key={i} className="relative p-5 border-2 border-[#0D0D1A] bg-white cursor-pointer transition-all duration-200 group hover:rotate-0" style={{
                transform: `rotate(${rotations[i]}deg)`,
                boxShadow: "3px 3px 0 #0D0D1A",
              }}>
                {/* Sticker-like accent */}
                <div className="absolute -top-2 -right-2 w-6 h-6 rounded-full bg-[#A3E635] border-2 border-[#0D0D1A] flex items-center justify-center text-xs font-black" style={{ fontSize: "8px" }}>{c.progress}%</div>
                <div className="text-2xl mb-2">{c.emoji}</div>
                <h4 className="text-sm font-bold mb-0.5" style={{ fontFamily: "'Unbounded'", fontSize: "12px" }}>{c.name}</h4>
                <div className="text-xs text-[#7C7A8E] mb-3">{c.count} cartes</div>
                <div className="h-1.5 bg-[#EDE9FE] border border-[#0D0D1A]">
                  <div className="h-full bg-[#5B47D6] transition-all" style={{ width: `${c.progress}%` }} />
                </div>
              </div>
            );
          })}
        </div>
      </div>
    </div>
  );
}

/* ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   VARIANTE B — "Grunge Soft"
   Ton style mais : les ombres deviennent floues et
   colorées au lieu de solides, les bordures sont plus
   fines, les cards ont du grain, du texte qui overlap,
   des éléments "scotchés" ou "découpés". Mix entre
   ton brutalisme et du collage organique.
   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ */
function GrungeSoft() {
  return (
    <div className="min-h-screen relative" style={{
      background: "#FAF9F6",
      backgroundImage: `
        radial-gradient(rgba(91,71,214,0.04) 1px, transparent 1px),
        url("data:image/svg+xml,%3Csvg width='300' height='300' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='g'%3E%3CfeTurbulence baseFrequency='0.9' type='fractalNoise' numOctaves='4'/%3E%3C/filter%3E%3Crect width='300' height='300' filter='url(%23g)' opacity='0.025'/%3E%3C/svg%3E")
      `,
      backgroundSize: "20px 20px, 300px 300px",
      fontFamily: "'Outfit', sans-serif",
    }}>
      {/* ── Nav ── */}
      <nav className="sticky top-0 z-50 px-6 h-16 flex items-center justify-between" style={{
        background: "rgba(250,249,246,0.8)",
        backdropFilter: "blur(20px) saturate(1.3)",
        borderBottom: "1.5px solid #E4E0F5",
      }}>
        <div className="flex items-center gap-6">
          <div className="relative" style={{ fontSize: "24px", fontFamily: "'Unbounded'", fontWeight: 900 }}>
            <span style={{ color: "#5B47D6" }}>PY</span><span style={{ color: "#0D0D1A" }}>CKX</span>
            <span className="absolute -bottom-1 left-0 right-0 h-1 bg-[#A3E635] rounded-full" style={{ transform: "rotate(-1deg)" }} />
          </div>
          <div className="flex gap-2">
            {["Play", "Explorer", "Stats"].map((t, i) => (
              <button key={i} className="px-3 py-1.5 text-xs font-semibold uppercase tracking-wider rounded-full transition-all duration-200" style={{
                fontFamily: "'Unbounded'",
                fontSize: "10px",
                background: i === 0 ? "#5B47D6" : "transparent",
                color: i === 0 ? "white" : "#7C7A8E",
                boxShadow: i === 0 ? "0 4px 14px rgba(91,71,214,0.3)" : "none",
              }}>{t}</button>
            ))}
          </div>
        </div>
        <div className="w-9 h-9 rounded-full border-2 border-[#5B47D6] flex items-center justify-center text-xs font-bold text-[#5B47D6]" style={{ fontFamily: "'Unbounded'" }}>CL</div>
      </nav>

      <div className="max-w-6xl mx-auto px-6 py-10">
        {/* ── Hero ── */}
        <div className="relative p-10 mb-14 rounded-2xl overflow-hidden" style={{
          background: "white",
          border: "1.5px solid #E4E0F5",
          boxShadow: "0 8px 40px rgba(91,71,214,0.08), 0 2px 10px rgba(0,0,0,0.03)",
        }}>
          {/* Orbs */}
          <div className="absolute -top-24 -right-24 w-80 h-80 rounded-full opacity-20 blur-3xl" style={{ background: "radial-gradient(circle, #5B47D6, transparent 65%)" }} />
          <div className="absolute -bottom-20 -left-20 w-64 h-64 rounded-full opacity-25 blur-3xl" style={{ background: "radial-gradient(circle, #A3E635, transparent 65%)" }} />

          {/* Eyebrow badge — slightly tilted */}
          <div className="inline-flex items-center gap-2 px-3 py-1.5 mb-6 rounded-full" style={{ background: "#EDE9FE", transform: "rotate(-1deg)" }}>
            <span className="w-2 h-2 rounded-full bg-[#A3E635]" />
            <span className="text-xs font-bold text-[#5B47D6]" style={{ fontFamily: "'Unbounded'", fontSize: "10px" }}>9 TYPES DE JEUX</span>
          </div>

          {/* Title — mixed weights and colors */}
          <h1 className="relative mb-4" style={{ fontSize: "54px", fontFamily: "'Unbounded'", fontWeight: 900, lineHeight: 1.05, letterSpacing: "-3px", color: "#0D0D1A" }}>
            Apprends<br />
            <span className="relative inline-block">
              en jouant
              <span className="absolute -bottom-1 left-0 right-0 h-3 bg-[#A3E635] opacity-40 -z-10 rounded-sm" style={{ transform: "rotate(-0.5deg)" }} />
            </span>
            <span style={{ color: "#5B47D6" }}>.</span>
            {/* Scattered sticker */}
            <span className="absolute -top-2 right-0 px-2 py-1 text-xs font-black bg-[#A3E635] text-[#0D0D1A] rounded-sm" style={{ fontFamily: "'Unbounded'", fontSize: "10px", transform: "rotate(8deg)", boxShadow: "0 3px 10px rgba(163,230,53,0.3)" }}>FREE</span>
          </h1>

          <p className="text-[#7C7A8E] text-lg max-w-xl mb-8" style={{ lineHeight: 1.7 }}>
            QCM, flashcards, vrai/faux, texte à trous — <strong className="text-[#0D0D1A]">révise sans t'ennuyer</strong>.
            Créé pour les étudiants qui veulent apprendre autrement.
          </p>

          {/* CTAs */}
          <div className="flex gap-3 mb-10">
            <button className="px-7 py-3.5 rounded-xl text-sm font-bold text-white transition-all duration-200 hover:-translate-y-0.5" style={{
              fontFamily: "'Unbounded'",
              fontSize: "12px",
              background: "#5B47D6",
              boxShadow: "0 4px 20px rgba(91,71,214,0.35)",
            }}>Commencer →</button>
            <button className="px-7 py-3.5 rounded-xl text-sm font-bold text-[#5B47D6] border-2 border-[#E4E0F5] bg-white transition-all duration-200 hover:border-[#5B47D6]" style={{
              fontFamily: "'Unbounded'",
              fontSize: "12px",
            }}>Explorer</button>
          </div>

          {/* Stats — soft card version */}
          <div className="flex gap-4">
            {stats.map((s, i) => (
              <div key={i} className="px-5 py-3 rounded-xl flex items-center gap-3" style={{
                background: "rgba(237,233,254,0.5)",
                border: "1px solid #E4E0F5",
              }}>
                <span className="text-xl font-black" style={{ fontFamily: "'Unbounded'", color: s.accent ? "#5B47D6" : "#0D0D1A" }}>{s.value}</span>
                <span className="text-xs text-[#7C7A8E] font-semibold uppercase tracking-wider" style={{ fontFamily: "'Unbounded'", fontSize: "9px" }}>{s.label}</span>
              </div>
            ))}
          </div>
        </div>

        {/* ── Games ── */}
        <div className="mb-6 flex items-center gap-4">
          <div className="h-1 w-8 rounded-full bg-[#5B47D6]" />
          <span className="text-xs font-black uppercase tracking-[0.2em] text-[#7C7A8E]" style={{ fontFamily: "'Unbounded'", fontSize: "10px" }}>Continue</span>
          <div className="h-px flex-1 bg-[#E4E0F5]" />
        </div>

        <div className="grid grid-cols-3 gap-4 mb-14" style={{ gridAutoRows: "minmax(160px, auto)" }}>
          {games.map((g, i) => {
            const isFeat = i === 0;
            const rotations = [0, -0.5, 0.3, -0.3, 0.5];
            return (
              <div key={i} className={`relative p-6 rounded-2xl cursor-pointer transition-all duration-300 group overflow-hidden ${isFeat ? "row-span-2" : ""}`} style={{
                background: isFeat ? "#5B47D6" : "white",
                color: isFeat ? "white" : "#0D0D1A",
                border: isFeat ? "none" : "1.5px solid #E4E0F5",
                boxShadow: isFeat ? "0 8px 30px rgba(91,71,214,0.25)" : "0 2px 10px rgba(0,0,0,0.03)",
                transform: `rotate(${rotations[i]}deg)`,
              }}>
                {/* Grain overlay */}
                <div className="absolute inset-0 opacity-[0.03] pointer-events-none rounded-2xl" style={{
                  backgroundImage: `url("data:image/svg+xml,%3Csvg width='200' height='200' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='g'%3E%3CfeTurbulence baseFrequency='0.8' type='fractalNoise'/%3E%3C/filter%3E%3Crect width='200' height='200' filter='url(%23g)'/%3E%3C/svg%3E")`,
                }} />

                {/* Tag */}
                <div className="inline-block px-2.5 py-1 mb-3 rounded-full text-xs font-bold uppercase tracking-wider" style={{
                  fontFamily: "'Unbounded'",
                  fontSize: "9px",
                  background: isFeat ? "rgba(255,255,255,0.15)" : "#EDE9FE",
                  color: isFeat ? "rgba(255,255,255,0.8)" : "#5B47D6",
                }}>{g.tag}</div>

                <div className="text-3xl mb-3">{g.icon}</div>
                <h3 className="font-bold mb-1" style={{ fontFamily: "'Unbounded'", fontSize: isFeat ? "22px" : "15px", letterSpacing: "-0.5px" }}>{g.title}</h3>
                <p className="text-sm opacity-50 leading-relaxed">{g.desc}</p>

                {/* Bottom gradient bar */}
                <div className="absolute bottom-0 left-0 right-0 h-1 origin-left scale-x-0 group-hover:scale-x-100 transition-transform duration-500 rounded-b-2xl" style={{ background: isFeat ? "#A3E635" : "linear-gradient(90deg, #5B47D6, #A3E635)" }} />

                {/* Hover: lift */}
                <div className="absolute inset-0 rounded-2xl opacity-0 group-hover:opacity-100 transition-opacity duration-300 pointer-events-none" style={{ boxShadow: isFeat ? "0 12px 40px rgba(91,71,214,0.35)" : "0 8px 25px rgba(91,71,214,0.12)" }} />
              </div>
            );
          })}
        </div>

        {/* ── Collections ── */}
        <div className="mb-6 flex items-center gap-4">
          <div className="h-1 w-8 rounded-full bg-[#A3E635]" />
          <span className="text-xs font-black uppercase tracking-[0.2em] text-[#7C7A8E]" style={{ fontFamily: "'Unbounded'", fontSize: "10px" }}>Collections</span>
          <div className="h-px flex-1 bg-[#E4E0F5]" />
        </div>

        <div className="grid grid-cols-4 gap-4">
          {collections.map((c, i) => {
            const rotations = [-0.8, 0.5, -0.3, 0.7];
            return (
              <div key={i} className="relative p-5 rounded-xl bg-white cursor-pointer transition-all duration-300 group hover:-translate-y-1 hover:rotate-0" style={{
                border: "1.5px solid #E4E0F5",
                transform: `rotate(${rotations[i]}deg)`,
                boxShadow: "0 2px 10px rgba(0,0,0,0.03)",
              }}>
                <div className="absolute -top-2 -right-2 px-2 py-0.5 rounded-full text-xs font-bold text-white" style={{ background: "#5B47D6", fontFamily: "'Unbounded'", fontSize: "9px", boxShadow: "0 2px 8px rgba(91,71,214,0.3)" }}>{c.progress}%</div>
                <div className="text-2xl mb-2">{c.emoji}</div>
                <h4 className="text-sm font-bold mb-0.5" style={{ fontFamily: "'Unbounded'", fontSize: "12px" }}>{c.name}</h4>
                <div className="text-xs text-[#7C7A8E] mb-3">{c.count} cartes</div>
                <div className="h-1.5 rounded-full bg-[#EDE9FE] overflow-hidden">
                  <div className="h-full rounded-full transition-all" style={{ width: `${c.progress}%`, background: "linear-gradient(90deg, #5B47D6, #A3E635)" }} />
                </div>
              </div>
            );
          })}
        </div>
      </div>
    </div>
  );
}

/* ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   VARIANTE C — "Neon Chalk"
   Fond sombre, accents néon (lime + purple glow),
   bordures qui brillent, typographie Unbounded en gros,
   éléments "griffonnés" (dashed, hand-drawn feel).
   Même ADN brutalist mais version nuit.
   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ */
function NeonChalk() {
  const ink = "#E8E6F0";
  const muted = "#6B6880";
  const bg = "#0F0E17";
  const card = "#1A1828";
  const purple = "#8B7CFF";
  const lime = "#BEFF5A";
  const border = "#2A2838";

  return (
    <div className="min-h-screen" style={{
      background: bg,
      backgroundImage: `radial-gradient(${purple}10 1px, transparent 1px)`,
      backgroundSize: "20px 20px",
      fontFamily: "'Outfit', sans-serif",
      color: ink,
    }}>
      {/* ── Nav ── */}
      <nav className="sticky top-0 z-50 px-6 h-16 flex items-center justify-between" style={{
        background: `${bg}dd`,
        backdropFilter: "blur(16px)",
        borderBottom: `1.5px solid ${border}`,
      }}>
        <div className="flex items-center gap-6">
          <div className="relative" style={{ fontSize: "24px", fontFamily: "'Unbounded'", fontWeight: 900 }}>
            <span style={{ color: purple, textShadow: `0 0 20px ${purple}50` }}>PY</span><span style={{ color: ink }}>CKX</span>
          </div>
          <div className="flex gap-1">
            {["Play", "Explorer", "Stats"].map((t, i) => (
              <button key={i} className="px-3 py-1.5 text-xs font-bold uppercase tracking-wider transition-all duration-150" style={{
                fontFamily: "'Unbounded'",
                fontSize: "10px",
                color: i === 0 ? bg : muted,
                background: i === 0 ? lime : "transparent",
                border: i === 0 ? "none" : `1px solid ${border}`,
                borderRadius: "4px",
                boxShadow: i === 0 ? `0 0 14px ${lime}40` : "none",
              }}>{t}</button>
            ))}
          </div>
        </div>
        <div className="w-9 h-9 rounded-md flex items-center justify-center text-xs font-bold" style={{ fontFamily: "'Unbounded'", border: `1.5px solid ${purple}`, color: purple, boxShadow: `0 0 10px ${purple}30` }}>CL</div>
      </nav>

      <div className="max-w-6xl mx-auto px-6 py-10">
        {/* ── Hero ── */}
        <div className="relative p-10 mb-14 rounded-xl overflow-hidden" style={{
          background: card,
          border: `1.5px solid ${border}`,
          boxShadow: `0 0 40px ${purple}08, 0 8px 30px rgba(0,0,0,0.3)`,
        }}>
          {/* Glowing orbs */}
          <div className="absolute -top-24 -right-24 w-80 h-80 rounded-full opacity-20 blur-3xl" style={{ background: `radial-gradient(circle, ${purple}, transparent 65%)` }} />
          <div className="absolute -bottom-20 -left-20 w-64 h-64 rounded-full opacity-15 blur-3xl" style={{ background: `radial-gradient(circle, ${lime}, transparent 65%)` }} />

          {/* Eyebrow */}
          <div className="inline-flex items-center gap-2 px-3 py-1.5 mb-6 rounded-full" style={{ background: `${purple}15`, border: `1px solid ${purple}30` }}>
            <span className="w-2 h-2 rounded-full" style={{ background: lime, boxShadow: `0 0 6px ${lime}` }} />
            <span className="text-xs font-bold" style={{ fontFamily: "'Unbounded'", fontSize: "10px", color: purple }}>NOUVEAU</span>
          </div>

          {/* Title with glow */}
          <h1 className="relative mb-4" style={{ fontSize: "54px", fontFamily: "'Unbounded'", fontWeight: 900, lineHeight: 1.05, letterSpacing: "-3px" }}>
            Let's{" "}
            <span style={{ color: purple, textShadow: `0 0 30px ${purple}40` }}>play</span>
            <span style={{ color: lime, textShadow: `0 0 20px ${lime}40` }}>.</span>
          </h1>

          <p className="text-lg max-w-xl mb-8" style={{ color: muted, lineHeight: 1.7 }}>
            Apprends en jouant. <strong style={{ color: ink }}>9 types de jeux</strong> pour réviser sans t'ennuyer.
          </p>

          {/* CTAs */}
          <div className="flex gap-3 mb-10">
            <button className="px-7 py-3.5 rounded-lg text-sm font-bold transition-all duration-200 hover:-translate-y-0.5" style={{
              fontFamily: "'Unbounded'",
              fontSize: "12px",
              background: purple,
              color: "white",
              boxShadow: `0 0 20px ${purple}40, 0 4px 15px ${purple}30`,
            }}>Commencer →</button>
            <button className="px-7 py-3.5 rounded-lg text-sm font-bold transition-all duration-200 hover:bg-white/5" style={{
              fontFamily: "'Unbounded'",
              fontSize: "12px",
              color: lime,
              border: `1.5px solid ${lime}40`,
            }}>Explorer</button>
          </div>

          {/* Stats */}
          <div className="flex gap-4">
            {stats.map((s, i) => (
              <div key={i} className="px-5 py-3 rounded-lg flex items-center gap-3" style={{ background: `${purple}08`, border: `1px solid ${border}` }}>
                <span className="text-xl font-black" style={{ fontFamily: "'Unbounded'", color: s.accent ? lime : ink, textShadow: s.accent ? `0 0 10px ${lime}30` : "none" }}>{s.value}</span>
                <span className="text-xs font-semibold uppercase tracking-wider" style={{ fontFamily: "'Unbounded'", fontSize: "9px", color: muted }}>{s.label}</span>
              </div>
            ))}
          </div>

          {/* Neon sticker */}
          <div className="absolute top-6 right-8 px-3 py-1 text-xs font-black rounded-sm" style={{
            fontFamily: "'Unbounded'",
            fontSize: "10px",
            color: bg,
            background: lime,
            boxShadow: `0 0 14px ${lime}50`,
            transform: "rotate(8deg)",
          }}>FREE</div>
        </div>

        {/* ── Games ── */}
        <div className="mb-6 flex items-center gap-4">
          <div className="h-1 w-8 rounded-full" style={{ background: lime, boxShadow: `0 0 8px ${lime}40` }} />
          <span className="text-xs font-black uppercase tracking-[0.2em]" style={{ fontFamily: "'Unbounded'", fontSize: "10px", color: muted }}>Continue</span>
          <div className="h-px flex-1" style={{ background: border }} />
        </div>

        <div className="grid grid-cols-3 gap-4 mb-14" style={{ gridAutoRows: "minmax(160px, auto)" }}>
          {games.map((g, i) => {
            const isFeat = i === 0;
            return (
              <div key={i} className={`relative p-6 rounded-xl cursor-pointer transition-all duration-300 group overflow-hidden ${isFeat ? "row-span-2" : ""}`} style={{
                background: isFeat ? purple : card,
                border: `1.5px solid ${isFeat ? purple : border}`,
                boxShadow: isFeat ? `0 0 30px ${purple}20` : "none",
              }}>
                <div className="inline-block px-2.5 py-1 mb-3 rounded-full text-xs font-bold uppercase tracking-wider" style={{
                  fontFamily: "'Unbounded'",
                  fontSize: "9px",
                  background: isFeat ? "rgba(255,255,255,0.12)" : `${purple}12`,
                  color: isFeat ? "rgba(255,255,255,0.7)" : purple,
                }}>{g.tag}</div>
                <div className="text-3xl mb-3">{g.icon}</div>
                <h3 className="font-bold mb-1" style={{ fontFamily: "'Unbounded'", fontSize: isFeat ? "22px" : "15px", letterSpacing: "-0.5px", color: isFeat ? "white" : ink }}>{g.title}</h3>
                <p className="text-sm leading-relaxed" style={{ color: isFeat ? "rgba(255,255,255,0.5)" : muted }}>{g.desc}</p>

                {/* Neon bar on hover */}
                <div className="absolute bottom-0 left-0 right-0 h-0.5 origin-left scale-x-0 group-hover:scale-x-100 transition-transform duration-500 rounded-b-xl" style={{
                  background: isFeat ? lime : `linear-gradient(90deg, ${purple}, ${lime})`,
                  boxShadow: `0 0 10px ${isFeat ? lime : purple}40`,
                }} />

                {/* Glow on hover */}
                <div className="absolute inset-0 rounded-xl opacity-0 group-hover:opacity-100 transition-opacity duration-300 pointer-events-none" style={{
                  boxShadow: `inset 0 0 30px ${isFeat ? "rgba(255,255,255,0.05)" : `${purple}08`}, 0 0 20px ${isFeat ? `${purple}30` : `${purple}10`}`,
                }} />
              </div>
            );
          })}
        </div>

        {/* ── Collections ── */}
        <div className="mb-6 flex items-center gap-4">
          <div className="h-1 w-8 rounded-full" style={{ background: purple, boxShadow: `0 0 8px ${purple}40` }} />
          <span className="text-xs font-black uppercase tracking-[0.2em]" style={{ fontFamily: "'Unbounded'", fontSize: "10px", color: muted }}>Collections</span>
          <div className="h-px flex-1" style={{ background: border }} />
        </div>

        <div className="grid grid-cols-4 gap-4">
          {collections.map((c, i) => (
            <div key={i} className="relative p-5 rounded-xl cursor-pointer transition-all duration-300 group hover:-translate-y-1" style={{
              background: card,
              border: `1.5px solid ${border}`,
            }}>
              <div className="absolute -top-2 -right-2 px-2 py-0.5 rounded-full text-xs font-bold" style={{
                background: lime,
                color: bg,
                fontFamily: "'Unbounded'",
                fontSize: "9px",
                boxShadow: `0 0 8px ${lime}40`,
              }}>{c.progress}%</div>
              <div className="text-2xl mb-2">{c.emoji}</div>
              <h4 className="text-sm font-bold mb-0.5" style={{ fontFamily: "'Unbounded'", fontSize: "12px" }}>{c.name}</h4>
              <div className="text-xs mb-3" style={{ color: muted }}>{c.count} cartes</div>
              <div className="h-1.5 rounded-full overflow-hidden" style={{ background: `${purple}15` }}>
                <div className="h-full rounded-full transition-all" style={{
                  width: `${c.progress}%`,
                  background: `linear-gradient(90deg, ${purple}, ${lime})`,
                  boxShadow: `0 0 6px ${purple}30`,
                }} />
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}

/* ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   MAIN — Floating tab switcher
   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ */
export default function StylePreviews() {
  const [active, setActive] = useState(0);
  const tabAccents = ["#5B47D6", "#5B47D6", "#8B7CFF"];
  const tabBgs = ["#F8F7FF", "#FAF9F6", "#0F0E17"];

  return (
    <div className="w-full min-h-screen" style={{ background: tabBgs[active] }}>
      {/* Floating tab bar */}
      <div className="fixed bottom-6 left-1/2 -translate-x-1/2 z-[100] flex gap-1 p-1.5 rounded-full" style={{
        background: active === 2 ? "rgba(26,24,40,0.9)" : "rgba(255,255,255,0.9)",
        backdropFilter: "blur(16px)",
        border: `1px solid ${active === 2 ? "rgba(139,124,255,0.2)" : "rgba(0,0,0,0.06)"}`,
        boxShadow: active === 2 ? `0 4px 20px rgba(139,124,255,0.15)` : "0 4px 20px rgba(0,0,0,0.08)",
      }}>
        {TABS.map((tab, i) => (
          <button key={tab} onClick={() => setActive(i)}
            className="px-5 py-2.5 text-xs font-bold uppercase tracking-wider rounded-full transition-all duration-200"
            style={{
              fontFamily: "'Unbounded', sans-serif",
              fontSize: "10px",
              ...(active === i ? {
                background: tabAccents[i],
                color: "white",
                boxShadow: `0 2px 12px ${tabAccents[i]}40`,
              } : {
                color: active === 2 ? "#6B6880" : "#7C7A8E",
                background: "transparent",
              }),
            }}
          >{tab}</button>
        ))}
      </div>

      {active === 0 && <ActuelPlusPlus />}
      {active === 1 && <GrungeSoft />}
      {active === 2 && <NeonChalk />}
    </div>
  );
}
