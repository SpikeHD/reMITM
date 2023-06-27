import { useState, useEffect } from 'preact/hooks'

import './Textbox.css'

interface Props {
  onChange?: (value: string) => void
  onUnfocus?: (value: string) => void
  onEnter?: (value: string) => void
  value?: string
}

export function Textbox(props: Props) {
  const [value, setValue] = useState('')

  useEffect(() => {
    setValue(props.value || '')
  }, [props.value])
  
  return (
    <input
      type="text" 
      class="Textbox"
      onChange={e => {
        // @ts-ignore
        setValue(e.target.value)
        props.onChange && props.onChange(value)
      }}
      onBlur={(() => props.onUnfocus && props.onUnfocus(value))}
      onKeyPress={e => e.key === 'Enter' && props.onEnter && props.onEnter(value)}
      value={value}
    />
  )
}