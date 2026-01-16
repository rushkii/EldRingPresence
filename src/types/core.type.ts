export interface GameStatus {
  attached: boolean;
  process_name: string;
}

export interface AttachResult {
  success: boolean;
  message: string;
}
