# Collection App Documentation

## Overview

The Collection App allows users to manage their media collections (DVDs, and future: Books, Games, etc.).

## Architecture

Uses a class-based architecture located in `lib/classes/collection/`:

```
collection/
├── index.ts            # Re-exports everything
├── CollectionApp.ts    # Main orchestrator class
├── items/
│   ├── Dvd.ts          # DVD entity class
│   └── index.ts
└── lists/
    ├── DvdList.ts      # DVD collection class
    └── index.ts
```

## Classes

### Dvd Class

Represents a single DVD item.

```typescript
import { Dvd, type CreateDvdInput, type UpdateDvdInput } from "@/lib/classes/collection"

// Types
interface DvdData {
  id: string
  name: string
  year: number
  realisator: string | null
  actors: string[]
  genre: string | null
  created_at: string
  updated_at: string
}

interface CreateDvdInput {
  name: string
  year: string
  realisator?: string
  actors?: string[]
  genre?: string
}

// Static methods
const dvd = await Dvd.create(input)      // Create new DVD
const dvd = await Dvd.getById("uuid")    // Fetch by ID
const dvd = Dvd.fromAPI(data)            // Create from API response

// Instance methods
const updated = await dvd.save(input)    // Update DVD
await dvd.delete()                       // Delete DVD
const data = dvd.toJSON()                // Serialize
const copy = dvd.clone()                 // Deep copy
```

### DvdList Class

Manages a collection of DVDs with filtering and sorting.

```typescript
import { DvdList, type FilterField, type SortOrder } from "@/lib/classes/collection"

type FilterField = "name" | "year" | "genre"
type SortOrder = "asc" | "desc"

// Static methods
const list = await DvdList.fetchAll()    // Fetch all user DVDs

// Accessors
list.length                              // Number of DVDs
list.all                                 // Get all as array
list.isEmpty                             // Check if empty

// Collection methods (return new DvdList - immutable)
const newList = list.add(dvd)            // Add DVD
const newList = list.remove(id)          // Remove by ID
const newList = list.update(dvd)         // Update existing
const dvd = list.findById(id)            // Find by ID

// Filtering & Sorting (chainable, returns new DvdList)
const filtered = list
  .filter("matrix", "name")
  .sort("year", "desc")

// Iteration
list.map(dvd => dvd.name)
list.forEach(dvd => console.log(dvd))
for (const dvd of list) { ... }
```

### CollectionApp Class

Main orchestrator that manages all collection types.

```typescript
import { CollectionApp } from "@/lib/classes/collection"

const collection = new CollectionApp()

// Load data
await collection.loadDvds()

// Access lists
collection.dvds                          // DvdList instance
collection.loading                       // Loading state
collection.error                         // Error message

// CRUD operations
const dvd = await collection.addDvd(input)
const dvd = await collection.updateDvd(id, input)
await collection.deleteDvd(id)

// Utility
collection.clearError()
```

## React Integration

```typescript
// In a React component
const collectionRef = useRef<CollectionApp>(new CollectionApp())
const collection = collectionRef.current

const [dvds, setDvds] = useState<DvdList>(new DvdList())
const [loading, setLoading] = useState(true)

// Load
useEffect(() => {
  collection.loadDvds().then(list => setDvds(list))
}, [])

// Filter (computed, doesn't mutate state)
const filtered = useMemo(() => 
  dvds.filter(query, field).sort(field, order),
  [dvds, query, field, order]
)

// Add
const handleAdd = async (input) => {
  await collection.addDvd(input)
  setDvds(collection.dvds)
}
```

## Backend API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/collection/dvds` | Get all user DVDs |
| POST | `/api/collection/dvds` | Add new DVD |
| GET | `/api/collection/dvds/:id` | Get single DVD |
| PUT | `/api/collection/dvds/:id` | Update DVD |
| DELETE | `/api/collection/dvds/:id` | Delete DVD |

## Pre-configured Data

### Genres
Action, Adventure, Animation, Biography, Comedy, Crime, Documentary, Drama, Family, Fantasy, Film-Noir, History, Horror, Musical, Mystery, Romance, Sci-Fi, Sport, Thriller, War, Western

### Years
Dynamic range from current year down to 1900.

## Future Extensions

To add a new collection type (e.g., Books):

1. Create `items/Book.ts` with `Book` class and types
2. Create `lists/BookList.ts` with `BookList` class
3. Add exports to `items/index.ts` and `lists/index.ts`
4. Add book operations to `CollectionApp.ts`
5. Export from main `index.ts`
