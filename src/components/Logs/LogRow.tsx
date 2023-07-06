import './LogRow.css'

export function LogRow(props: RequestLog) {
  return (
    <div className="LogRow">
      <div className="LogRowHeader">
        <div className="LogRowLeft">
          <div className="LogRowMethod">{props.method}</div>
          <div className="LogRowURL">
            {
              props.redirected_to ? (
                props.url + ' -> ' + props.redirected_to
              ) : (
                props.url
              )
            }
          </div>
        </div>
        <div className="LogRowRight">
          <div className="LogRowStatus">{props.response_code}</div>
        </div>
      </div>
      <div className="LogRowBody">
        {props.body}
      </div>
    </div>
  )
}