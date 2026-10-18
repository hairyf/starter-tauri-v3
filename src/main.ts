import { greet } from './greet'
import './style.css'

const form = document.querySelector<HTMLFormElement>('#greet-form')
const input = document.querySelector<HTMLInputElement>('#name')
const output = document.querySelector<HTMLOutputElement>('#greet-result')

if (!form || !input || !output)
  throw new Error('Missing greet form elements')

form.addEventListener('submit', async (event) => {
  event.preventDefault()
  output.textContent = await greet(input.value)
})
