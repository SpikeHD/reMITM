import { useEffect, useState } from 'preact/hooks'
import { listen } from '@tauri-apps/api/event'

import { LogRow } from './components/Logs/LogRow'

import './app.css'
import './logs.css'
import Arrow from './assets/arrow.svg'
import { Tr } from './components/Translation/Translate'

export function Logs() {
  const [requests, setRequests] = useState([] as RequestLog[])
  const [isOverflowing, setIsOverflowing] = useState(false)
  const [autoScroll, setAutoScroll] = useState(false)
  const [previousScroll, setPreviousScroll] = useState(0)

  useEffect(() => {
    listen('log_request', ({ payload }) => {
      setRequests((req) => [...req, payload as RequestLog])

      if (autoScroll) {
        scrollDown()
      }
    })

    document.addEventListener('scroll', () => {
      setPreviousScroll(window.scrollY)
    })
  }, [])

  const scrollDown = () => {
    // Get the last row
    const logs = document.getElementById('Logs') as HTMLElement
    const elm = logs.lastElementChild as HTMLElement

    elm.scrollIntoView({ behavior: 'smooth' })
  }

  return (
    <>
      <div
        id="Logs"
        ref={(el) => {
          if (el) {
            const root = document.getElementById('app') as HTMLElement
            const isOverflowing = el.scrollHeight >= root.clientHeight
            setIsOverflowing(isOverflowing)

            // If the user has scrolled up, disable auto scroll
            if (window.scrollY < previousScroll) {
              setAutoScroll(false)
            }
          }
        }}
      >
        {requests.length > 0 ? (
          requests.map((request) => <LogRow {...request} key={request.key} />)
        ) : (
          <div className="LogRow">
            <Tr text="logs.no_requests" />
          </div>
        )}
      </div>

      {isOverflowing && (
        <div
          id="ScrollDown"
          onClick={() => {
            scrollDown()
            setAutoScroll(true)
          }}
        >
          <img src={Arrow} />
        </div>
      )}
    </>
  )
}
