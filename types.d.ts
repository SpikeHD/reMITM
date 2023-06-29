interface Config {
  launch_at_startup: boolean;
  proxy_port: number;
  urls_to_redirect: string[];
  redirect_to: string;
  log_requests: boolean;
  modify_gsettings: boolean;
  use_env_variables: boolean;
  terminal: string;
}