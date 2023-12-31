interface Config extends PartialConfig {
  urls_to_redirect: string[]
  redirect_to: string
  log_requests: boolean
}

interface PartialConfig {
  language: string
  launch_at_startup: boolean
  proxy_port: number
  terminal: string
  modify_gsettings: boolean
  use_env_variables: boolean
}

interface RequestLog {
  url: string
  method: string
  response_code: string
  body: string
  timestamp: string
  redirected_to: string
  key: string
}

interface Lang {
  name: string
  filename: string
}
