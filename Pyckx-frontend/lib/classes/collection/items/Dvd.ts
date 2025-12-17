// == DVD Class - Represents a single DVD item in the collection ==

import { endpoints } from "@/lib/api/config";

// == Types ==
export interface DvdData {
  id: string;
  name: string;
  year: number;
  realisator: string | null;
  actors: string[];
  genre: string | null;
  created_at: string;
  updated_at: string;
}

export interface CreateDvdInput {
  name: string;
  year: string;
  realisator?: string;
  actors?: string[];
  genre?: string;
}

export interface UpdateDvdInput {
  name?: string;
  year?: string;
  realisator?: string;
  actors?: string[];
  genre?: string;
}

interface DvdSuccessResponse {
  message: string;
  dvd: DvdData;
}

// == Helper ==
function extractErrorMessage(errorBody: unknown, fallback: string): string {
  if (!errorBody || typeof errorBody !== "object") return fallback;
  const err = errorBody as { message?: string };
  return err.message || fallback;
}

// == DVD Class ==
export class Dvd {
  readonly id: string;
  name: string;
  year: number;
  realisator: string | null;
  actors: string[];
  genre: string | null;
  readonly createdAt: Date;
  readonly updatedAt: Date;

  constructor(data: DvdData) {
    this.id = data.id;
    this.name = data.name;
    this.year = data.year;
    this.realisator = data.realisator;
    this.actors = [...data.actors];
    this.genre = data.genre;
    this.createdAt = new Date(data.created_at);
    this.updatedAt = new Date(data.updated_at);
  }

  // == Instance Methods ==
  
  toJSON(): DvdData {
    return {
      id: this.id,
      name: this.name,
      year: this.year,
      realisator: this.realisator,
      actors: [...this.actors],
      genre: this.genre,
      created_at: this.createdAt.toISOString(),
      updated_at: this.updatedAt.toISOString(),
    };
  }

  clone(): Dvd {
    return new Dvd(this.toJSON());
  }

  // == Static Factory ==
  
  static fromAPI(data: DvdData): Dvd {
    return new Dvd(data);
  }

  // == API Methods ==

  static async create(input: CreateDvdInput): Promise<Dvd> {
    const res = await fetch(endpoints.collection.addDvd(), {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      credentials: "include",
      body: JSON.stringify(input),
    });

    if (!res.ok) {
      const errorBody = await res.json().catch(() => null);
      throw new Error(extractErrorMessage(errorBody, `Failed to add DVD (${res.status})`));
    }

    const result: DvdSuccessResponse = await res.json();
    return Dvd.fromAPI(result.dvd);
  }

  async save(input: UpdateDvdInput): Promise<Dvd> {
    const res = await fetch(endpoints.collection.updateDvd(this.id), {
      method: "PUT",
      headers: { "Content-Type": "application/json" },
      credentials: "include",
      body: JSON.stringify(input),
    });

    if (!res.ok) {
      const errorBody = await res.json().catch(() => null);
      throw new Error(extractErrorMessage(errorBody, `Failed to update DVD (${res.status})`));
    }

    const result: DvdSuccessResponse = await res.json();
    return Dvd.fromAPI(result.dvd);
  }

  async delete(): Promise<void> {
    const res = await fetch(endpoints.collection.deleteDvd(this.id), {
      method: "DELETE",
      credentials: "include",
    });

    if (!res.ok) {
      const errorBody = await res.json().catch(() => null);
      throw new Error(extractErrorMessage(errorBody, `Failed to delete DVD (${res.status})`));
    }
  }

  static async getById(id: string): Promise<Dvd> {
    const res = await fetch(endpoints.collection.getDvd(id), {
      method: "GET",
      credentials: "include",
    });

    if (!res.ok) {
      const errorBody = await res.json().catch(() => null);
      throw new Error(extractErrorMessage(errorBody, `Failed to fetch DVD (${res.status})`));
    }

    const data: DvdData = await res.json();
    return Dvd.fromAPI(data);
  }
}
