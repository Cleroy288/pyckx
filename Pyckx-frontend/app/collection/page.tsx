"use client"

import { useState, useEffect, useMemo, useRef } from "react"
import { Library, Plus, Search, Pencil, Trash2, Loader2, ArrowUpDown, X } from "lucide-react"
import { useRouter } from "next/navigation"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@/components/ui/dialog"
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import { GridBackground } from "@/components/grid-background"
import { TopNavigation } from "@/components/top-navigation"
import { AuthGuard } from "@/components/auth-guard"
import { useUserApps } from "@/lib/user-apps-context"

// Import from classes
import { 
  CollectionApp, 
  DvdList, 
  Dvd,
  type FilterField, 
  type SortOrder,
  type CreateDvdInput,
  type UpdateDvdInput 
} from "@/lib/classes/collection"

// Pre-configured genres
const GENRES = [
  "Action", "Adventure", "Animation", "Biography", "Comedy", "Crime",
  "Documentary", "Drama", "Family", "Fantasy", "Film-Noir", "History",
  "Horror", "Musical", "Mystery", "Romance", "Sci-Fi", "Sport",
  "Thriller", "War", "Western",
] as const

// Generate years from current year down to 1900
const currentYear = new Date().getFullYear()
const YEARS = Array.from({ length: currentYear - 1899 }, (_, i) => currentYear - i)

export default function CollectionPage() {
  // Collection app instance (persists across renders)
  const collectionRef = useRef<CollectionApp>(new CollectionApp())
  const collection = collectionRef.current
  const router = useRouter()
  const { currentPage } = useUserApps()
  
  // State
  const [dvds, setDvds] = useState<DvdList>(new DvdList())
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  
  // Search and sort
  const [searchQuery, setSearchQuery] = useState("")
  const [filterField, setFilterField] = useState<FilterField>("name")
  const [sortOrder, setSortOrder] = useState<SortOrder>("asc")
  
  // Dialog states
  const [showAddDialog, setShowAddDialog] = useState(false)
  const [showEditDialog, setShowEditDialog] = useState(false)
  const [showDeleteDialog, setShowDeleteDialog] = useState(false)
  const [selectedDvd, setSelectedDvd] = useState<Dvd | null>(null)
  
  // Form states
  const [formLoading, setFormLoading] = useState(false)
  const [formError, setFormError] = useState<string | null>(null)
  
  // Add form states
  const [addActors, setAddActors] = useState<string[]>([])
  const [addActorInput, setAddActorInput] = useState("")
  const [addGenre, setAddGenre] = useState<string>("")
  const [addYear, setAddYear] = useState<string>("")
  
  // Edit form states
  const [editActors, setEditActors] = useState<string[]>([])
  const [editActorInput, setEditActorInput] = useState("")
  const [editGenre, setEditGenre] = useState<string>("")
  const [editYear, setEditYear] = useState<string>("")

  // Handle navigation back to dashboard
  useEffect(() => {
    if (currentPage.type === "dashboard") {
      router.push("/")
    }
  }, [currentPage, router])

  // Load DVDs on mount
  useEffect(() => {
    loadDvds()
  }, [])

  const loadDvds = async () => {
    setLoading(true)
    setError(null)
    try {
      const list = await collection.loadDvds()
      setDvds(list)
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to load DVDs")
    } finally {
      setLoading(false)
    }
  }

  // Filter and sort DVDs using DvdList methods
  const filteredDvds = useMemo(() => {
    return dvds
      .filter(searchQuery, filterField)
      .sort(filterField, sortOrder)
  }, [dvds, searchQuery, filterField, sortOrder])

  const resetAddForm = () => {
    setAddActors([])
    setAddActorInput("")
    setAddGenre("")
    setAddYear("")
    setFormError(null)
  }

  const handleAddActor = (
    actors: string[], 
    setActors: (a: string[]) => void, 
    input: string, 
    setInput: (s: string) => void
  ) => {
    const trimmed = input.trim()
    if (trimmed && !actors.includes(trimmed)) {
      setActors([...actors, trimmed])
      setInput("")
    }
  }

  const handleRemoveActor = (
    actors: string[], 
    setActors: (a: string[]) => void, 
    actor: string
  ) => {
    setActors(actors.filter(a => a !== actor))
  }

  const handleAddDvd = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    setFormLoading(true)
    setFormError(null)
    
    const formData = new FormData(e.currentTarget)
    const input: CreateDvdInput = {
      name: formData.get("name") as string,
      year: addYear,
      realisator: formData.get("realisator") as string || undefined,
      actors: addActors,
      genre: addGenre || undefined,
    }
    
    try {
      await collection.addDvd(input)
      setDvds(collection.dvds)
      setShowAddDialog(false)
      resetAddForm()
    } catch (err) {
      setFormError(err instanceof Error ? err.message : "Failed to add DVD")
    } finally {
      setFormLoading(false)
    }
  }

  const handleEditDvd = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    if (!selectedDvd) return
    
    setFormLoading(true)
    setFormError(null)
    
    const formData = new FormData(e.currentTarget)
    const input: UpdateDvdInput = {
      name: formData.get("name") as string || undefined,
      year: editYear || undefined,
      realisator: formData.get("realisator") as string || undefined,
      actors: editActors,
      genre: editGenre || undefined,
    }
    
    try {
      await collection.updateDvd(selectedDvd.id, input)
      setDvds(collection.dvds)
      setShowEditDialog(false)
      setSelectedDvd(null)
    } catch (err) {
      setFormError(err instanceof Error ? err.message : "Failed to update DVD")
    } finally {
      setFormLoading(false)
    }
  }

  const handleDeleteDvd = async () => {
    if (!selectedDvd) return
    
    setFormLoading(true)
    setFormError(null)
    
    try {
      await collection.deleteDvd(selectedDvd.id)
      setDvds(collection.dvds)
      setShowDeleteDialog(false)
      setSelectedDvd(null)
    } catch (err) {
      setFormError(err instanceof Error ? err.message : "Failed to delete DVD")
    } finally {
      setFormLoading(false)
    }
  }

  const openEditDialog = (dvd: Dvd) => {
    setSelectedDvd(dvd)
    setEditActors([...dvd.actors])
    setEditActorInput("")
    setEditGenre(dvd.genre ?? "")
    setEditYear(dvd.year.toString())
    setFormError(null)
    setShowEditDialog(true)
  }

  const openDeleteDialog = (dvd: Dvd) => {
    setSelectedDvd(dvd)
    setFormError(null)
    setShowDeleteDialog(true)
  }

  const toggleSortOrder = () => {
    setSortOrder(prev => prev === "asc" ? "desc" : "asc")
  }

  return (
    <AuthGuard>
      <div className="relative min-h-screen overflow-hidden selection:bg-primary/30">
        <GridBackground />
        <div className="relative z-10">
          <TopNavigation />
          <main className="container mx-auto px-6 py-6">
            <div className="space-y-6">
              {/* Header */}
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-3">
                  <div className="flex h-12 w-12 items-center justify-center rounded-xl bg-primary/10 border border-primary/20">
                    <Library className="h-6 w-6 text-primary" />
                  </div>
                  <div>
                    <h1 className="text-2xl font-bold tracking-tight text-foreground">DVD Collection</h1>
                    <p className="text-sm text-muted-foreground">{dvds.length} DVDs in your collection</p>
                  </div>
                </div>
                <Button 
                  onClick={() => { setFormError(null); setShowAddDialog(true) }}
                  className="gap-2 bg-primary text-primary-foreground hover:bg-primary/90"
                >
                  <Plus className="h-4 w-4" />
                  Add DVD
                </Button>
              </div>

              {/* Search and Filter */}
              <div className="flex items-center gap-4">
                <Select value={filterField} onValueChange={(v) => setFilterField(v as FilterField)}>
                  <SelectTrigger className="w-[120px] bg-card/60 border-border/50">
                    <SelectValue placeholder="Filter by" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="name">Name</SelectItem>
                    <SelectItem value="year">Year</SelectItem>
                    <SelectItem value="genre">Genre</SelectItem>
                  </SelectContent>
                </Select>
                <div className="relative flex-1">
                  <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
                  <Input 
                    placeholder={`Search by ${filterField}...`}
                    className="pl-10 bg-card/60 border-border/50"
                    value={searchQuery}
                    onChange={(e) => setSearchQuery(e.target.value)}
                  />
                </div>
                <Button 
                  variant="outline" 
                  size="icon" 
                  onClick={toggleSortOrder}
                  className="bg-card/60 border-border/50"
                  title={`Sort ${sortOrder === "asc" ? "ascending" : "descending"}`}
                >
                  <ArrowUpDown className={`h-4 w-4 ${sortOrder === "desc" ? "rotate-180" : ""} transition-transform`} />
                </Button>
              </div>

              {/* Error state */}
              {error && (
                <div className="rounded-lg bg-destructive/10 border border-destructive/20 p-4">
                  <p className="text-sm text-destructive">{error}</p>
                  <Button variant="link" onClick={loadDvds} className="text-destructive p-0 h-auto mt-2">
                    Try again
                  </Button>
                </div>
              )}

              {/* Loading state */}
              {loading && (
                <div className="flex items-center justify-center py-16">
                  <Loader2 className="h-8 w-8 animate-spin text-primary" />
                </div>
              )}

              {/* Empty state */}
              {!loading && !error && dvds.isEmpty && (
                <div className="flex flex-col items-center justify-center rounded-2xl border border-dashed border-border/50 bg-card/20 py-16">
                  <div className="flex h-16 w-16 items-center justify-center rounded-2xl bg-primary/10 border border-primary/20 mb-4">
                    <Library className="h-8 w-8 text-primary" />
                  </div>
                  <h3 className="text-lg font-semibold text-foreground mb-2">No DVDs yet</h3>
                  <p className="text-sm text-muted-foreground mb-6 text-center max-w-sm">
                    Start building your DVD collection by adding your first movie.
                  </p>
                  <Button 
                    onClick={() => setShowAddDialog(true)}
                    className="gap-2 bg-primary text-primary-foreground hover:bg-primary/90"
                  >
                    <Plus className="h-4 w-4" />
                    Add your first DVD
                  </Button>
                </div>
              )}

              {/* No results */}
              {!loading && !error && !dvds.isEmpty && filteredDvds.isEmpty && (
                <div className="flex flex-col items-center justify-center rounded-2xl border border-dashed border-border/50 bg-card/20 py-12">
                  <Search className="h-8 w-8 text-muted-foreground mb-4" />
                  <h3 className="text-lg font-semibold text-foreground mb-2">No results found</h3>
                  <p className="text-sm text-muted-foreground">Try adjusting your search query</p>
                </div>
              )}

              {/* DVD Grid */}
              {!loading && !error && !filteredDvds.isEmpty && (
                <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
                  {filteredDvds.map((dvd) => (
                    <div 
                      key={dvd.id}
                      className="group rounded-xl border border-accent/30 bg-accent/10 p-5 backdrop-blur-sm transition-all hover:border-accent/50 hover:bg-accent/20"
                    >
                      <div className="flex items-start justify-between mb-3">
                        <h3 className="font-semibold text-foreground line-clamp-1">{dvd.name}</h3>
                        <span className="text-sm text-muted-foreground shrink-0 ml-2">{dvd.year}</span>
                      </div>
                      
                      {dvd.realisator && (
                        <p className="text-sm text-muted-foreground mb-1">
                          <span className="text-foreground/70">Director:</span> {dvd.realisator}
                        </p>
                      )}
                      
                      {dvd.actors.length > 0 && (
                        <p className="text-sm text-muted-foreground mb-1 line-clamp-1">
                          <span className="text-foreground/70">Actors:</span> {dvd.actors.join(", ")}
                        </p>
                      )}
                      
                      {dvd.genre && (
                        <span className="inline-block mt-2 px-2 py-0.5 text-xs rounded-full bg-primary/10 text-primary border border-primary/20">
                          {dvd.genre}
                        </span>
                      )}
                      
                      <div className="flex items-center gap-2 mt-4 pt-3 border-t border-border/30">
                        <Button 
                          variant="ghost" 
                          size="sm" 
                          onClick={() => openEditDialog(dvd)}
                          className="flex-1 h-8 text-muted-foreground hover:text-primary hover:bg-primary/10"
                        >
                          <Pencil className="h-3.5 w-3.5 mr-1.5" />
                          Edit
                        </Button>
                        <Button 
                          variant="ghost" 
                          size="sm" 
                          onClick={() => openDeleteDialog(dvd)}
                          className="flex-1 h-8 text-muted-foreground hover:text-destructive hover:bg-destructive/10"
                        >
                          <Trash2 className="h-3.5 w-3.5 mr-1.5" />
                          Delete
                        </Button>
                      </div>
                    </div>
                  ))}
                </div>
              )}

              {/* Add DVD Dialog */}
              <Dialog open={showAddDialog} onOpenChange={(open) => { setShowAddDialog(open); if (!open) resetAddForm(); }}>
                <DialogContent className="sm:max-w-md max-h-[90vh] overflow-y-auto">
                  <DialogHeader>
                    <DialogTitle>Add New DVD</DialogTitle>
                    <DialogDescription>Add a new DVD to your collection.</DialogDescription>
                  </DialogHeader>
                  <form onSubmit={handleAddDvd} className="space-y-4">
                    <div className="space-y-2">
                      <Label htmlFor="add-name">Name *</Label>
                      <Input id="add-name" name="name" required placeholder="Movie title" />
                    </div>
                    <div className="space-y-2">
                      <Label>Year *</Label>
                      <Select value={addYear} onValueChange={setAddYear} required>
                        <SelectTrigger className="w-full">
                          <SelectValue placeholder="Select year" />
                        </SelectTrigger>
                        <SelectContent className="max-h-[200px]">
                          {YEARS.map(year => (
                            <SelectItem key={year} value={year.toString()}>{year}</SelectItem>
                          ))}
                        </SelectContent>
                      </Select>
                    </div>
                    <div className="space-y-2">
                      <Label htmlFor="add-realisator">Director</Label>
                      <Input id="add-realisator" name="realisator" placeholder="Director name" />
                    </div>
                    <div className="space-y-2">
                      <Label>Actors</Label>
                      <div className="flex gap-2">
                        <Input 
                          value={addActorInput}
                          onChange={(e) => setAddActorInput(e.target.value)}
                          placeholder="Actor name"
                          onKeyDown={(e) => {
                            if (e.key === "Enter") {
                              e.preventDefault()
                              handleAddActor(addActors, setAddActors, addActorInput, setAddActorInput)
                            }
                          }}
                        />
                        <Button 
                          type="button" 
                          variant="outline" 
                          size="icon"
                          onClick={() => handleAddActor(addActors, setAddActors, addActorInput, setAddActorInput)}
                        >
                          <Plus className="h-4 w-4" />
                        </Button>
                      </div>
                      {addActors.length > 0 && (
                        <div className="flex flex-wrap gap-2 mt-2">
                          {addActors.map((actor, i) => (
                            <span 
                              key={i} 
                              className="inline-flex items-center gap-1 px-2 py-1 text-xs rounded-full bg-primary/10 text-primary border border-primary/20"
                            >
                              {actor}
                              <button 
                                type="button" 
                                onClick={() => handleRemoveActor(addActors, setAddActors, actor)}
                                className="hover:text-destructive"
                              >
                                <X className="h-3 w-3" />
                              </button>
                            </span>
                          ))}
                        </div>
                      )}
                    </div>
                    <div className="space-y-2">
                      <Label>Genre</Label>
                      <Select value={addGenre} onValueChange={setAddGenre}>
                        <SelectTrigger className="w-full">
                          <SelectValue placeholder="Select genre" />
                        </SelectTrigger>
                        <SelectContent>
                          {GENRES.map(genre => (
                            <SelectItem key={genre} value={genre}>{genre}</SelectItem>
                          ))}
                        </SelectContent>
                      </Select>
                    </div>
                    {formError && (
                      <div className="rounded-lg bg-destructive/10 border border-destructive/20 p-3">
                        <p className="text-sm text-destructive">{formError}</p>
                      </div>
                    )}
                    <DialogFooter>
                      <Button type="button" variant="outline" onClick={() => { setShowAddDialog(false); resetAddForm(); }}>
                        Cancel
                      </Button>
                      <Button type="submit" disabled={formLoading || !addYear}>
                        {formLoading ? <Loader2 className="h-4 w-4 animate-spin mr-2" /> : null}
                        Add DVD
                      </Button>
                    </DialogFooter>
                  </form>
                </DialogContent>
              </Dialog>

              {/* Edit DVD Dialog */}
              <Dialog open={showEditDialog} onOpenChange={setShowEditDialog}>
                <DialogContent className="sm:max-w-md max-h-[90vh] overflow-y-auto">
                  <DialogHeader>
                    <DialogTitle>Edit DVD</DialogTitle>
                    <DialogDescription>Update the DVD information.</DialogDescription>
                  </DialogHeader>
                  <form onSubmit={handleEditDvd} className="space-y-4">
                    <div className="space-y-2">
                      <Label htmlFor="edit-name">Name</Label>
                      <Input id="edit-name" name="name" defaultValue={selectedDvd?.name} placeholder="Movie title" />
                    </div>
                    <div className="space-y-2">
                      <Label>Year</Label>
                      <Select value={editYear} onValueChange={setEditYear}>
                        <SelectTrigger className="w-full">
                          <SelectValue placeholder="Select year" />
                        </SelectTrigger>
                        <SelectContent className="max-h-[200px]">
                          {YEARS.map(year => (
                            <SelectItem key={year} value={year.toString()}>{year}</SelectItem>
                          ))}
                        </SelectContent>
                      </Select>
                    </div>
                    <div className="space-y-2">
                      <Label htmlFor="edit-realisator">Director</Label>
                      <Input id="edit-realisator" name="realisator" defaultValue={selectedDvd?.realisator ?? ""} placeholder="Director name" />
                    </div>
                    <div className="space-y-2">
                      <Label>Actors</Label>
                      <div className="flex gap-2">
                        <Input 
                          value={editActorInput}
                          onChange={(e) => setEditActorInput(e.target.value)}
                          placeholder="Actor name"
                          onKeyDown={(e) => {
                            if (e.key === "Enter") {
                              e.preventDefault()
                              handleAddActor(editActors, setEditActors, editActorInput, setEditActorInput)
                            }
                          }}
                        />
                        <Button 
                          type="button" 
                          variant="outline" 
                          size="icon"
                          onClick={() => handleAddActor(editActors, setEditActors, editActorInput, setEditActorInput)}
                        >
                          <Plus className="h-4 w-4" />
                        </Button>
                      </div>
                      {editActors.length > 0 && (
                        <div className="flex flex-wrap gap-2 mt-2">
                          {editActors.map((actor, i) => (
                            <span 
                              key={i} 
                              className="inline-flex items-center gap-1 px-2 py-1 text-xs rounded-full bg-primary/10 text-primary border border-primary/20"
                            >
                              {actor}
                              <button 
                                type="button" 
                                onClick={() => handleRemoveActor(editActors, setEditActors, actor)}
                                className="hover:text-destructive"
                              >
                                <X className="h-3 w-3" />
                              </button>
                            </span>
                          ))}
                        </div>
                      )}
                    </div>
                    <div className="space-y-2">
                      <Label>Genre</Label>
                      <Select value={editGenre} onValueChange={setEditGenre}>
                        <SelectTrigger className="w-full">
                          <SelectValue placeholder="Select genre" />
                        </SelectTrigger>
                        <SelectContent>
                          {GENRES.map(genre => (
                            <SelectItem key={genre} value={genre}>{genre}</SelectItem>
                          ))}
                        </SelectContent>
                      </Select>
                    </div>
                    {formError && (
                      <div className="rounded-lg bg-destructive/10 border border-destructive/20 p-3">
                        <p className="text-sm text-destructive">{formError}</p>
                      </div>
                    )}
                    <DialogFooter>
                      <Button type="button" variant="outline" onClick={() => setShowEditDialog(false)}>
                        Cancel
                      </Button>
                      <Button type="submit" disabled={formLoading}>
                        {formLoading ? <Loader2 className="h-4 w-4 animate-spin mr-2" /> : null}
                        Save Changes
                      </Button>
                    </DialogFooter>
                  </form>
                </DialogContent>
              </Dialog>

              {/* Delete Confirmation Dialog */}
              <Dialog open={showDeleteDialog} onOpenChange={setShowDeleteDialog}>
                <DialogContent className="sm:max-w-md">
                  <DialogHeader>
                    <DialogTitle>Delete DVD</DialogTitle>
                    <DialogDescription>
                      Are you sure you want to delete &quot;{selectedDvd?.name}&quot;? This action cannot be undone.
                    </DialogDescription>
                  </DialogHeader>
                  {formError && (
                    <div className="rounded-lg bg-destructive/10 border border-destructive/20 p-3">
                      <p className="text-sm text-destructive">{formError}</p>
                    </div>
                  )}
                  <DialogFooter>
                    <Button type="button" variant="outline" onClick={() => setShowDeleteDialog(false)}>
                      Cancel
                    </Button>
                    <Button 
                      type="button" 
                      variant="destructive" 
                      onClick={handleDeleteDvd}
                      disabled={formLoading}
                    >
                      {formLoading ? <Loader2 className="h-4 w-4 animate-spin mr-2" /> : null}
                      Delete
                    </Button>
                  </DialogFooter>
                </DialogContent>
              </Dialog>
            </div>
          </main>
        </div>
      </div>
    </AuthGuard>
  )
}
