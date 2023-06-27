import './RedirectSelect.css'
import { Textbox } from './Textbox'

export function RedirectSelect() {
  return (
    <div id="RedirectSelect">
      <span>Redirect to:</span>
      <Textbox onBlur={value => console.log(value)} onEnter={value => console.log(value)} />
    </div>
  )
}