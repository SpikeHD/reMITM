interface Config {
  launch_at_startup: boolean;
  proxy_port: number;
  urls_to_redirect: string[];
  redirect_to: string;
  log_requests: boolean;
}