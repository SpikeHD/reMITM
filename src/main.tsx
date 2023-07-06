import { render } from 'preact'
import { App } from './app.tsx'
import { Logs } from './logs.tsx'
import './index.css'

if (window.location.pathname === '/logs') {
  render(<Logs />, document.getElementById('app') as HTMLElement)
} else {
  render(<App />, document.getElementById('app') as HTMLElement)
}
