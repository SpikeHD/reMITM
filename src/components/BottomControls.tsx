import './BottomControls.css'

import TerminalIcon from '../assets/terminal.svg'
import LogIcon from '../assets/text.svg'

interface Props {
  isOn: boolean
}

export function BottomControls(props: Props) {
  return (
    <div id="BottomControls" class={props.isOn ? '' : 'hide'}>
      <span>
        <img src={TerminalIcon} />
      </span>
      <span>
        <img src={LogIcon} />
      </span>
    </div>
  )
}