
### Notes

* `docker build -t="my_cpp" .`
* `docker run --rm -i -t -v $(pwd):/data my_cpp`
    - will put you into Bash inside the container
    - `cd /data`
    - `./go.sh`
