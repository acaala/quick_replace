# qrep

## A fast, lightweight find and replace tool

### Usage

```bash
qrep [OPTIONS] <FROM> <TO> <path/to/file.txt>
```

#### Options

| Flag | Description |
| ---- | ----------- |
| -h | Display help information |
| -r | Replace file without creating a backup (cannot be used with -c) |
| -c | Compress backup file (cannot be used with -r) |
