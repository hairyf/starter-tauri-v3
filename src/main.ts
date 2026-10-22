import { getCurrentWindow } from '@tauri-apps/api/window'
import { greet } from './greet'
import './style.css'

const appWindow = getCurrentWindow()

// --- Window chrome -----------------------------------------------------------
// The window is frameless, so the app draws its own title bar. Dragging and
// double-click-to-maximize are handled natively through `app-region: drag`
// (see style.css); the buttons need explicit commands.
document.querySelector('#window-minimize')?.addEventListener('click', () => {
  void appWindow.minimize()
})
document.querySelector('#window-maximize')?.addEventListener('click', () => {
  void appWindow.toggleMaximize()
})
document.querySelector('#window-close')?.addEventListener('click', () => {
  void appWindow.close()
})

// --- Section navigation ------------------------------------------------------
const navItems = document.querySelectorAll<HTMLButtonElement>('.nav__item')
const panels = document.querySelectorAll<HTMLElement>('.panel')

for (const item of navItems) {
  item.addEventListener('click', () => {
    const section = item.dataset.section
    for (const other of navItems) other.classList.toggle('is-active', other === item)
    for (const panel of panels) {
      panel.classList.toggle('is-active', panel.dataset.panel === section)
    }
  })
}

// --- Greet round trip --------------------------------------------------------
const form = document.querySelector<HTMLFormElement>('#greet-form')
const input = document.querySelector<HTMLInputElement>('#name')
const output = document.querySelector<HTMLOutputElement>('#greet-result')

if (form && input && output) {
  form.addEventListener('submit', async (event) => {
    event.preventDefault()
    output.textContent = await greet(input.value)
  })
}

// --- Tray demo ---------------------------------------------------------------
document.querySelector('#tray-hide')?.addEventListener('click', () => {
  void appWindow.hide()
})

// --- Multi window ------------------------------------------------------------
// Each Tauri window is a separate Electron BrowserWindow with its own webview;
// this asks the Rust side to build one from the runtime's `create_window` path.
document.querySelector('#open-window')?.addEventListener('click', async () => {
  const { invoke } = await import('@tauri-apps/api/core')
  await invoke('open_second_window')
})

const runtime = document.querySelector('#fact-runtime')
if (runtime) {
  runtime.textContent = `${navigator.userAgent.includes('Electron') ? 'Electron' : 'WebView2'} · ${location.origin}`
}
