import { useEffect, useState } from 'preact/hooks'
import { invoke } from '@tauri-apps/api'
import { listen } from '@tauri-apps/api/event'

import './app.css'
import './logs.css'
import { LogRow } from './components/Logs/LogRow'

export function Logs() {
  const [requests, setRequests] = useState([] as RequestLog[])

  useEffect(() => {
    listen('log_request', ({ payload }) => {
      setRequests((req) => [...req, payload as RequestLog])
      console.log(payload)
    })
  }, [])

  return (
    <>
      <div id="Logs">
        {
          requests.map((request) => (
            <LogRow {...request} key={request.key} />
          ))
        }
      </div>
    </>
  )
}
