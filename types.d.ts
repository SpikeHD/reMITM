interface Config extends PartialConfig {
  urls_to_redirect: string[]
  redirect_to: string
  log_requests: boolean
}

interface PartialConfig {
  launch_at_startup: boolean
  proxy_port: number
  terminal: string
  modify_gsettings: boolean
  use_env_variables: boolean
}
