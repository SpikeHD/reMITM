import { useState } from 'preact/hooks'

import { OnOff } from './components/OnOff'
import { RedirectSelect } from './components/RedirectSelect'
import { UriList } from './components/UriList'

import './app.css'
import { BottomControls } from './components/BottomControls'

export function App() {
  const [isOn , setIsOn ] = useState(false)

  return (
    <>
      <OnOff onChange={setIsOn} />

      <RedirectSelect />

      <UriList />

      <BottomControls isOn={isOn} />
    </>
  )
}
