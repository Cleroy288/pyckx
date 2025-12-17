// == QcmSetList Class - Manages a collection of QCM sets with filtering/sorting ==

import { QcmSet } from "./QcmSet";
import type { QcmSetData, QcmSetListResponse, Level } from "./types";

// == Types ==
export type QcmSetFilterField = "name" | "level" | "subjects";
export type SortOrder = "asc" | "desc";

// == Helper ==
function extractErrorMessage(errorBody: unknown, fallback: string): string {
  if (!errorBody || typeof errorBody !== "object") return fallback;
  const err = errorBody as { message?: string };
  return err.message || fallback;
}

// == QcmSetList Class ==
export class QcmSetList {
  private items: QcmSet[];

  constructor(sets: QcmSet[] = []) {
    this.items = [...sets];
  }

  // == Accessors ==

  get length(): number {
    return this.items.length;
  }

  get all(): QcmSet[] {
    return [...this.items];
  }

  get isEmpty(): boolean {
    return this.items.length === 0;
  }

  // == Collection Methods ==

  add(set: QcmSet): QcmSetList {
    return new QcmSetList([...this.items, set]);
  }

  remove(id: string): QcmSetList {
    return new QcmSetList(this.items.filter((s) => s.id !== id));
  }

  update(set: QcmSet): QcmSetList {
    return new QcmSetList(this.items.map((s) => (s.id === set.id ? set : s)));
  }

  findById(id: string): QcmSet | undefined {
    return this.items.find((s) => s.id === id);
  }

  // == Filtering & Sorting (returns new QcmSetList) ==

  filter(query: string, field: QcmSetFilterField): QcmSetList {
    if (!query.trim()) return this;

    const q = query.toLowerCase();
    const filtered = this.items.filter((set) => {
      switch (field) {
        case "name":
          return set.name.toLowerCase().includes(q);
        case "level":
          return set.level.toLowerCase().includes(q);
        case "subjects":
          return set.subjects.some((s) => s.toLowerCase().includes(q));
        default:
          return true;
      }
    });

    return new QcmSetList(filtered);
  }

  filterByLevel(level: Level): QcmSetList {
    return new QcmSetList(this.items.filter((s) => s.level === level));
  }

  filterBySubject(subject: string): QcmSetList {
    const q = subject.toLowerCase();
    return new QcmSetList(
      this.items.filter((s) => s.subjects.some((sub) => sub.toLowerCase().includes(q)))
    );
  }

  sort(field: QcmSetFilterField, order: SortOrder): QcmSetList {
    const sorted = [...this.items].sort((a, b) => {
      let comparison = 0;
      switch (field) {
        case "name":
          comparison = a.name.localeCompare(b.name);
          break;
        case "level": {
          const levelOrder = { easy: 1, medium: 2, hard: 3 };
          comparison = levelOrder[a.level] - levelOrder[b.level];
          break;
        }
        case "subjects":
          comparison = a.subjects.join(",").localeCompare(b.subjects.join(","));
          break;
      }
      return order === "asc" ? comparison : -comparison;
    });

    return new QcmSetList(sorted);
  }

  // == Static API Methods ==

  static async fetchAll(): Promise<QcmSetList> {
    const res = await fetch("/app/intello/qcm", {
      method: "GET",
      credentials: "include",
    });

    if (!res.ok) {
      const errorBody = await res.json().catch(() => null);
      throw new Error(
        extractErrorMessage(errorBody, `Failed to fetch QCM sets (${res.status})`)
      );
    }

    const data: QcmSetListResponse = await res.json();
    const sets = data.sets.map((s) => QcmSet.fromAPI(s));
    return new QcmSetList(sets);
  }

  // == Iteration ==

  [Symbol.iterator](): Iterator<QcmSet> {
    return this.items[Symbol.iterator]();
  }

  map<T>(fn: (set: QcmSet, index: number) => T): T[] {
    return this.items.map(fn);
  }

  forEach(fn: (set: QcmSet, index: number) => void): void {
    this.items.forEach(fn);
  }
}
