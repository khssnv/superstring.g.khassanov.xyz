# superstring.g.khassanov.xyz

## Usage

```console
git clone https://github.com/khssnv/superstring.g.khassanov.xyz.git
docker build -t superstring .
docker run --rm --name superstring -p 4000:4000 superstring
curl -X POST -H "Content-Type: application/json" \
    -d '{"variant": "naive", "substrings": ["foo", "bar", "art"]}' \
    http://localhost:4000/shortest-common-superstring
docker stop superstring
```
