// == DvdList Class - Manages a collection of DVDs with filtering/sorting ==

import { endpoints } from "@/lib/api/config";
import { Dvd, type DvdData } from "../items/Dvd";

// == Types ==
export type FilterField = "name" | "year" | "genre";
export type SortOrder = "asc" | "desc";

interface DvdListResponse {
  dvds: DvdData[];
  count: number;
}

// == Helper ==
function extractErrorMessage(errorBody: unknown, fallback: string): string {
  if (!errorBody || typeof errorBody !== "object") return fallback;
  const err = errorBody as { message?: string };
  return err.message || fallback;
}

// == DvdList Class ==
export class DvdList {
  private items: Dvd[];

  constructor(dvds: Dvd[] = []) {
    this.items = [...dvds];
  }

  // == Accessors ==

  get length(): number {
    return this.items.length;
  }

  get all(): Dvd[] {
    return [...this.items];
  }

  get isEmpty(): boolean {
    return this.items.length === 0;
  }

  // == Collection Methods ==

  add(dvd: Dvd): DvdList {
    return new DvdList([...this.items, dvd]);
  }

  remove(id: string): DvdList {
    return new DvdList(this.items.filter(d => d.id !== id));
  }

  update(dvd: Dvd): DvdList {
    return new DvdList(this.items.map(d => d.id === dvd.id ? dvd : d));
  }

  findById(id: string): Dvd | undefined {
    return this.items.find(d => d.id === id);
  }

  // == Filtering & Sorting (returns new DvdList) ==

  filter(query: string, field: FilterField): DvdList {
    if (!query.trim()) return this;

    const q = query.toLowerCase();
    const filtered = this.items.filter(dvd => {
      switch (field) {
        case "name":
          return dvd.name.toLowerCase().includes(q);
        case "year":
          return dvd.year.toString().includes(q);
        case "genre":
          return dvd.genre?.toLowerCase().includes(q) ?? false;
        default:
          return true;
      }
    });

    return new DvdList(filtered);
  }

  sort(field: FilterField, order: SortOrder): DvdList {
    const sorted = [...this.items].sort((a, b) => {
      let comparison = 0;
      switch (field) {
        case "name":
          comparison = a.name.localeCompare(b.name);
          break;
        case "year":
          comparison = a.year - b.year;
          break;
        case "genre":
          comparison = (a.genre ?? "").localeCompare(b.genre ?? "");
          break;
      }
      return order === "asc" ? comparison : -comparison;
    });

    return new DvdList(sorted);
  }

  // == Static API Methods ==

  static async fetchAll(): Promise<DvdList> {
    const res = await fetch(endpoints.collection.dvds(), {
      method: "GET",
      credentials: "include",
    });

    if (!res.ok) {
      const errorBody = await res.json().catch(() => null);
      throw new Error(extractErrorMessage(errorBody, `Failed to fetch DVDs (${res.status})`));
    }

    const data: DvdListResponse = await res.json();
    const dvds = data.dvds.map(d => Dvd.fromAPI(d));
    return new DvdList(dvds);
  }

  // == Iteration ==

  [Symbol.iterator](): Iterator<Dvd> {
    return this.items[Symbol.iterator]();
  }

  map<T>(fn: (dvd: Dvd, index: number) => T): T[] {
    return this.items.map(fn);
  }

  forEach(fn: (dvd: Dvd, index: number) => void): void {
    this.items.forEach(fn);
  }
}
