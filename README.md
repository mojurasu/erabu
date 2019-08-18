# erabu
Store items in collections and randomly pick an item from them.


## Usage
### Adding items
```
$ erabu add watch Boss Diesel Festina SuperDry
Added items: Boss, Diesel, Festina, SuperDry
```

### Removing items
```
$ erabu del watch Boss
Removed items: Boss
```

### Removing a collection
```
// With confirmation
$ erabu del watch
Remove the collection watch ? [y/N] yes
Removed collection: watch
```

```
// Without confirmation
$ erabu del watch -f
Removed collection: watch
```

### Picking a random item
```
// "Pretty" output
$ erabu pick watch
Picked item: Diesel
```

```
// "Raw" output
$ erabu pick watch --no-format
SuperDry
```

### Listing all Collections
```
$ erabu list
Collections: watch
```

### Listing items inside a collection
```
$ erabu list watch
Collection: watch
     Items: Diesel, Festina, SuperDry
```

### Using a different file location
```
$ erabu add watch Boss Diesel Festina SuperDry --file ~/.erabu
Added items: Boss, Diesel, Festina, SuperDry
```

```
$ erabu pick watch --file ~/.erabu
Picked item: Diesel
```

## Misc
Collections by default are stored as json at `~/.erabu-collections` or the file specified with `--file`
