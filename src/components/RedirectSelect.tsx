import './RedirectSelect.css'
import { Textbox } from './Textbox'

export function RedirectSelect() {
  return (
    <div id="RedirectSelect">
      <span>Redirect to:</span>
      <Textbox onUnfocus={value => console.log(value)} onEnter={value => console.log(value)} />
    </div>
  )
}