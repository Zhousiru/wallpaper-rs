# wallpaper-rs

A dead simple wallpaper changer for Windows.

Both desktop and lockscreen wallpapers are supported.

## Usage

```
wallpaper-rs <Target> <Image Path>
```

### `<Target>`

Where you want to apply the new wallpaper.

**Possible:** `desktop`, `lockscreen`, `both`

### `<Image Path>`

Path to a new wallpaper, or use the built-in wallpaper provider.

**Possible:**

- `/path/to/image.jpg` for local images.
- `provider:unsplash` for Unsplash editorial images.
- `provider:unsplash-wallpapers`, `provider:unsplash-3d-renders`, `provider:unsplash-<Topic>` for Unsplash topic images.

### HINT

Replace `wallpaper-rs` with `wallpaper-rs-daemon` to hide the console window.
