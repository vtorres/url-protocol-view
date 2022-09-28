export namespace ApiType {
  export interface Registry {
    path: String,
    description: String,
    command: String,
    type: String,
    modified_at: String,
  }
}

export namespace CmdType {
  export interface AppConfig {
    language?: string;
    theme_mode?: "light" | "dark";
    theme_blur?: boolean;
    enable_auto_launch?: boolean;
    enable_silent_start?: boolean;
    theme_setting?: {
      primary_color?: string;
      secondary_color?: string;
      primary_text?: string;
      secondary_text?: string;
      info_color?: string;
      error_color?: string;
      warning_color?: string;
      success_color?: string;
      font_family?: string;
      css_injection?: string;
    };
  }
}
