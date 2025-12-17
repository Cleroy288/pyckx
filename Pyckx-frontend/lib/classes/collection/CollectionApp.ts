// == CollectionApp Class - Main app class managing all collection items ==

import { Dvd, type CreateDvdInput, type UpdateDvdInput } from "./items/Dvd";
import { DvdList } from "./lists/DvdList";

// == CollectionApp Class ==
export class CollectionApp {
  private _dvds: DvdList;
  private _loading: boolean;
  private _error: string | null;

  constructor() {
    this._dvds = new DvdList();
    this._loading = false;
    this._error = null;
  }

  // == Accessors ==

  get dvds(): DvdList {
    return this._dvds;
  }

  get loading(): boolean {
    return this._loading;
  }

  get error(): string | null {
    return this._error;
  }

  // == DVD Operations ==

  async loadDvds(): Promise<DvdList> {
    this._loading = true;
    this._error = null;
    
    try {
      this._dvds = await DvdList.fetchAll();
      return this._dvds;
    } catch (err) {
      this._error = err instanceof Error ? err.message : "Failed to load DVDs";
      throw err;
    } finally {
      this._loading = false;
    }
  }

  async addDvd(input: CreateDvdInput): Promise<Dvd> {
    const dvd = await Dvd.create(input);
    this._dvds = this._dvds.add(dvd);
    return dvd;
  }

  async updateDvd(id: string, input: UpdateDvdInput): Promise<Dvd> {
    const existing = this._dvds.findById(id);
    if (!existing) {
      throw new Error(`DVD with id ${id} not found`);
    }
    
    const updated = await existing.save(input);
    this._dvds = this._dvds.update(updated);
    return updated;
  }

  async deleteDvd(id: string): Promise<void> {
    const existing = this._dvds.findById(id);
    if (!existing) {
      throw new Error(`DVD with id ${id} not found`);
    }
    
    await existing.delete();
    this._dvds = this._dvds.remove(id);
  }

  // == Utility ==

  clearError(): void {
    this._error = null;
  }

  // Future: Book operations
  // async loadBooks(): Promise<BookList> { ... }
  // async addBook(input: CreateBookInput): Promise<Book> { ... }
}
