// == IntelloApp Class - Main app class managing Intello quiz functionality ==

import { QcmSet } from "./QcmSet";
import { QcmSetList } from "./QcmSetList";
import type {
  CreateQcmSetInput,
  UpdateQcmSetInput,
  AvailableGamesResponse,
} from "./types";

// == Helper ==
function extractErrorMessage(errorBody: unknown, fallback: string): string {
  if (!errorBody || typeof errorBody !== "object") return fallback;
  const err = errorBody as { message?: string };
  return err.message || fallback;
}

// == IntelloApp Class ==
export class IntelloApp {
  private _qcmSets: QcmSetList;
  private _loading: boolean;
  private _error: string | null;
  private _availableGames: string[];

  constructor() {
    this._qcmSets = new QcmSetList();
    this._loading = false;
    this._error = null;
    this._availableGames = [];
  }

  // == Accessors ==

  get qcmSets(): QcmSetList {
    return this._qcmSets;
  }

  get loading(): boolean {
    return this._loading;
  }

  get error(): string | null {
    return this._error;
  }

  get availableGames(): string[] {
    return [...this._availableGames];
  }

  // == Games Operations ==

  async loadAvailableGames(): Promise<string[]> {
    this._loading = true;
    this._error = null;

    try {
      const res = await fetch("/app/intello/games", {
        method: "GET",
        credentials: "include",
      });

      if (!res.ok) {
        const errorBody = await res.json().catch(() => null);
        throw new Error(
          extractErrorMessage(errorBody, `Failed to load games (${res.status})`)
        );
      }

      const data: AvailableGamesResponse = await res.json();
      this._availableGames = data.games;
      return this._availableGames;
    } catch (err) {
      this._error = err instanceof Error ? err.message : "Failed to load games";
      throw err;
    } finally {
      this._loading = false;
    }
  }

  // == QCM Set Operations ==

  async loadQcmSets(): Promise<QcmSetList> {
    this._loading = true;
    this._error = null;

    try {
      this._qcmSets = await QcmSetList.fetchAll();
      return this._qcmSets;
    } catch (err) {
      this._error = err instanceof Error ? err.message : "Failed to load QCM sets";
      throw err;
    } finally {
      this._loading = false;
    }
  }

  async addQcmSet(input: CreateQcmSetInput): Promise<QcmSet> {
    const set = await QcmSet.create(input);
    this._qcmSets = this._qcmSets.add(set);
    return set;
  }

  async updateQcmSet(id: string, input: UpdateQcmSetInput): Promise<QcmSet> {
    const existing = this._qcmSets.findById(id);
    if (!existing) {
      throw new Error(`QCM set with id ${id} not found`);
    }

    const updated = await existing.save(input);
    this._qcmSets = this._qcmSets.update(updated);
    return updated;
  }

  async deleteQcmSet(id: string): Promise<void> {
    const existing = this._qcmSets.findById(id);
    if (!existing) {
      throw new Error(`QCM set with id ${id} not found`);
    }

    await existing.delete();
    this._qcmSets = this._qcmSets.remove(id);
  }

  async getQcmSet(id: string): Promise<QcmSet> {
    return QcmSet.getById(id);
  }

  // == Utility ==

  clearError(): void {
    this._error = null;
  }
}
