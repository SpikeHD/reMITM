import { h } from 'preact'
import { useState, useEffect } from 'preact/hooks'

import './Textbox.css'
import DeleteButton from '../../assets/close.svg'

interface Props {
  onChange?: (value: string) => void
  onBlur?: (value: string) => void
  onEnter?: (value: string) => void
  onDelete?: () => void
  defaultValue?: string
  value?: string
  placeholder?: string
  readonly?: boolean
  class?: string
}

export function Textbox(props: Props) {
  const [value, setValue] = useState('')

  useEffect(() => {
    setValue(props.defaultValue || '')
  }, [props.defaultValue])

  const handleInputChange = (e: h.JSX.TargetedEvent<HTMLInputElement>) => {
    const newValue = e.currentTarget.value
    setValue(newValue)
    props.onChange?.(newValue)
  }

  const handleBlur = () => {
    props.onBlur?.(value)
  }

  const handleKeyPress = (e: h.JSX.TargetedKeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter') {
      console.log(props.onEnter)
      props.onEnter?.(value)
    }
  }

  return (
    <div className={'TextboxOuter ' + (props?.class || '')}>
      <input
        type="text"
        className="Textbox"
        value={props.value || value}
        onInput={handleInputChange}
        onBlur={handleBlur}
        onKeyPress={handleKeyPress}
        placeholder={props.placeholder}
        ref={(input) => input}
        readOnly={props.readonly !== null && props.readonly}
      />

      {props.onDelete && (
        <div className="TextDelete" onClick={props.onDelete}>
          <img src={DeleteButton} />
        </div>
      )}
    </div>
  )
}
