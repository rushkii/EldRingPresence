import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { AttachResult, GameStatus } from "../types/core.type";
import type { Player } from "../types/game.type";

class GameCore {
  status = $state<GameStatus>();
  message = $state<string | null>();
  event = $state<Player>();
  private isStarted = $state<boolean>(false);

  memory_unlistener = $state<(() => void) | null>(null);

  async refreshStatus() {
    this.status = await invoke<GameStatus>("get_game_status");

    if (this.status.attached) {
      await invoke("connect_discord_rpc");
    } else {
      await invoke("disconnect_discord_rpc");
    }

    return this.status;
  }

  async attachToGame() {
    const result = await invoke<AttachResult>("attach_to_game");
    this.message = result.message;
    return this.message;
  }

  async listen() {
    if (!this.isStarted) {
      await invoke("connect_discord_rpc");
      await invoke("memory_listener");
      await invoke("update_discord_rpc");
      this.isStarted = true;
    }
    this.memory_unlistener = await listen<Player>("memory_event", (event) => {
      this.event = event.payload;
    });
  }

  async unlisten() {
    if (this.memory_unlistener) {
      this.memory_unlistener();
      this.memory_unlistener = null;
    }
    await invoke("disconnect_discord_rpc");
  }
}

export const gameCore = new GameCore();
