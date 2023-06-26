import { useState } from 'preact/hooks'

import './Textbox.css'

interface Props {
  onChange?: (value: string) => void
  onUnfocus?: (value: string) => void
  onEnter?: (value: string) => void
}

export function Textbox(props: Props) {
  const [value, setValue] = useState('')
  
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
    />
  )
}