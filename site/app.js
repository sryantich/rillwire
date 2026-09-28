"use strict";
const seeds = {
  general: [["Ari", "teal", "A small client should still feel like a complete place to talk."], ["Mika", "purple", "Agreed. Keep the conversation comfortable, then measure everything underneath."], ["Jules", "orange", "The RAM can go back to the game. Eventually. Benchmarks first."]],
  "build-log": [["Jules", "orange", "Native scaffold, bounded cache, and fixture replay are in place."], ["Ari", "teal", "Next: a real Windows baseline. The targets are hypotheses until we measure."]],
  "native-ui": [["Mika", "purple", "The desktop uses Rust and egui. This web preview explores the visual direction."], ["Ari", "teal", "Keyboard navigation, IME, and screen readers belong in the first release gate."]],
  "transport-lab": [["Ari", "teal", "Discord chooses the protocols its servers speak. Local optimizations still matter."], ["Jules", "orange", "QUIC experiments will use controlled peers. Voice needs DAVE interoperability."]]
};
let current = "general";
const history = Object.fromEntries(Object.entries(seeds).map(([key, rows]) => [key, rows.map(row => [...row])]));
const messages = document.getElementById("messages");
function render() {
  messages.replaceChildren();
  for (const [name, color, content] of history[current]) {
    const row = document.createElement("div"); row.className = "message";
    const avatar = document.createElement("div"); avatar.className = `avatar ${color}`; avatar.textContent = name[0];
    const body = document.createElement("div"); body.className = "body";
    const author = document.createElement("div"); author.className = "name"; author.textContent = name;
    const stamp = document.createElement("small"); stamp.textContent = name === "You" ? "local echo" : "synthetic message";
    const text = document.createElement("p"); text.textContent = content;
    author.append(stamp); body.append(author, text); row.append(avatar, body); messages.append(row);
  }
  messages.scrollTop = messages.scrollHeight;
}
document.querySelectorAll("[data-channel]").forEach(button => button.addEventListener("click", () => {
  current = button.dataset.channel;
  document.getElementById("channel-title").textContent = current;
  document.getElementById("welcome-title").textContent = current === "general" ? "Good conversations start here." : `Welcome to #${current}.`;
  document.querySelectorAll("[data-channel]").forEach(item => { const active = item === button; item.classList.toggle("active", active); item.setAttribute("aria-pressed", String(active)); });
  render();
}));
document.getElementById("composer").addEventListener("submit", event => {
  event.preventDefault(); const input = document.getElementById("message"); const content = input.value.trim();
  if (!content) return;
  history[current].push(["You", "own", content.slice(0, 2000)]);
  if (history[current].length > 100) history[current].shift();
  input.value = ""; render(); input.focus();
});
render();
