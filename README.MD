Day 34: Aho-Corasick
<br>
https://cr.yp.to/bib/1975/aho.pdf
<br>
In computer science, the Aho–Corasick algorithm is a string-searching algorithm invented by Alfred V. Aho and Margaret J. Corasick.[1] It is a kind of dictionary-matching algorithm that locates elements of a finite set of strings (the "dictionary") within an input text. It matches all strings simultaneously. The complexity of the algorithm is linear in the length of the strings plus the length of the searched text plus the number of output matches. Note that because all matches are found, there can be a quadratic number of matches if every substring matches (e.g. dictionary = a, aa, aaa, aaaa and input string is aaaa).
<br>
run:
```
fn main() {
    let mut aho = AhoCorasick::new();
    aho.add_keywords(vec!["he", "she", "his", "hers"]);
    let result = aho.search("ahishers");
    for r in result{
        println!("result: {:?} found at: {}", r.1, r.0);
    }
}
```

result:
```
{"his"} found at: 3
{"she", "he"} found at: 5
{"hers"} found at: 7
```