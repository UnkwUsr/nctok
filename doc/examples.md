# Examples

* [disk usage (file sizes) (aka ncdu alternative)](#disk-usage-file-sizes-aka-ncdu-alternative)
* [git commits count per file](#git-commits-count-per-file)
* [lines of code, comments, blank lines (with tokei)](#lines-of-code-comments-blank-lines-with-tokei)
* [lines count, words count (with coreutils `wc`)](#lines-count-words-count-with-coreutils-wc)
* [matches count per file](#matches-count-per-file)
* [files count in directory](#files-count-in-directory)

### disk usage (file sizes) (aka [ncdu](https://dev.yorhel.nl/ncdu) alternative)

```shell
fd -HI -tf --strip-cwd-prefix --exec-batch du -b --apparent-size \
     | nctok --number-delimiter $'\t'
```

### git commits count per file

```shell
git log --name-only --format="" \
    | grep -v '^$' \
    | perl -lne 'print if -e' \
    | sort | uniq -c | sed 's/^\s*//' | nctok
```

### lines of code, comments, blank lines (with [tokei](https://github.com/XAMPPRocky/tokei))

```bash
_tokei_helper() {
    target="$1"; shift
    tokei -Cfo json $* \
        | jq -r '.[].reports[] | "\(.stats.'$target') \(.name)"' \
        | nctok
}

# by code
alias bycode="_tokei_helper code"
# by comments
alias bycmnts="_tokei_helper comments"
# by blanks
alias byblanks="_tokei_helper blanks"
```

Also, tokei supports filtering by language (read more in tokei's help), and we
can pass it to our aliases:

```shell
bycode -t c++
bycode -t shell
bycode -t c++,rust
```

### lines count, words count (with coreutils `wc`)

```shell
# by lines
fd -HI -tf --strip-cwd-prefix --exec-batch wc -l --total=never \
    | sed 's/^\s*//' | nctok
# by words
fd -HI -tf --strip-cwd-prefix --exec-batch wc -w --total=never \
    | sed 's/^\s*//' | nctok
```

### matches count per file

```shell
rg "your_pattern" --count-matches --null | awk -F '\0' '{print $2, $1}' | nctok
```

### files count in directory

(actually just adding "1" for each file)

```shell
fd -tf | sed 's/^/1 /' | nctok
```
