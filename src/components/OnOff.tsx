import { useState } from 'preact/hooks'

import './OnOff.css'
import OnOffSVG from '../assets/onoff.svg'

export function OnOff() {
  const [isOn, setIsOn] = useState(false)

  const toggle = () => {
    setIsOn(!isOn)
  }

  return (
    <div id="OnOffToggle" onClick={toggle}>
      <img src={OnOffSVG} class={isOn ? 'toggled' : ''} />
      <span>{isOn ? 'Connected' : 'Disconnected'}</span>
    </div>
  )
}