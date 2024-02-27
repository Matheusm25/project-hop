const { invoke } = window.__TAURI__.tauri;
const events = window.__TAURI__.event;
const process = window.__TAURI__.process;
const shell = window.__TAURI__.shell;

let greetInputEl;
let greetMsgEl;
let sources = {};

window.addEventListener("DOMContentLoaded", async () => {
  const sourcesString = await invoke('get_sources')
  for (const source of sourcesString) {
    sources[source.split(' ')[0]] = source.split(' ')[1];
  }

  document
    .querySelector('#project-input').focus();

  document
    .querySelector('#project-input')
    .addEventListener('keydown', async (e) => {
      if (e.key === 'Enter') {
        const project = document.querySelector('#project-input').value;
        console.log('project', project, sources[project]);
        if (sources[project]) {
          const command = new shell.Command('code', [sources[project]]);
          await command.spawn();
          await sleep(1000);
          process.exit(0);
        }
      }
    });
});

async function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

events.listen('tauri://blur', async () => {
  console.log(sources);
  // process.exit(0);
})