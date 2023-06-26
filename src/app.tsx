import './app.css'
import { OnOff } from './components/OnOff'
import { RedirectSelect } from './components/RedirectSelect'
import { Textbox } from './components/Textbox'
import { UriList } from './components/UriList'

export function App() {

  return (
    <div id="app">
      <OnOff />

      <RedirectSelect />

      <UriList />

    </div>
  )
}
