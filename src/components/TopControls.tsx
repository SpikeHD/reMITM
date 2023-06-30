import './TopControls.css'
import SettingsIcon from '../assets/settings.svg'

interface Props {
  onSettingsClick: () => void
}

export function TopControls(props: Props) {
  return (
    <div id="TopControls">
      <span onClick={props.onSettingsClick}>
        <img src={SettingsIcon} />
      </span>
    </div>
  )
}
