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

### Picking a random item
```
$ erabu pick watch
Diesel
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

## Misc
Collections are stored as json at `~/.erabu-collections`
