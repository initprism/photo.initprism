# Personal Photo Gallery

## Develop
```
# make manifest
cargo run --release

# css optimize
crass src/gallery.css --optimize > dist/assets/css/gallery.css

# compile elm
elm make src/Main.elm --output=dist/assets/js/initprism.js --optimize

# compress js
uglifyjs dist/assets/js/initprism.js --compress pure_funcs='F2,F3,F4,F5,F6,F7,F8,F9,A2,A3,A4,A5,A6,A7,A8,A9',pure_getters,keep_fargs=false,unsafe_comps,unsafe | uglifyjs --mangle > dist/assets/js/initprism.min.js
```

## Serve
```
elm-live src/Main.elm -d dist --pushstate -- --output=dist/assets/js/initprism.js --optimize
```

## Delpoy
```
firebase login
firebase init
firebase deploy
```

## Dependencies
- rust
- elm
- crass
- elm-format