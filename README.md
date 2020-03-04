# The Simplest Possible Blockchain, in Rust

This is a very simple blockchain implementation that should be understandable without any blockchain or cryptocurrency background. Consisting of four files and less than 200 lines of code, anyone with a little Rust understanding can get through it in the time it takes to read a medium size blog post. [main.rs](src/main.rs) is annotated with everything you need to know.

```shell
cargo run
```

### Background


> If we wish to count lines of code, we should not regard them as "lines produced" but as "lines spent": the current conventional wisdom is so foolish as to book that count on the wrong side of the ledger.
>
> -- Edsger Dijkstra


The genesis of this tutorial is:

<ol>
<li>It's a good deed. There are several similar tutorials in other languages but very little in Rust.</li>

<li>Complexity has a huge role in cryptocurrency. Some projects -- purposefully or not -- incentivize code complexity. The fewer people who understand the currency, the fewer can object to it, or argue effectively for change. And the repercussions can be existential. The famous Ethereum disasters are good examples, but any large codebase -- and blockchain itself -- has a sizable barrier to entry.

Simple codebases that an outsider can easily diagram in their head are very valuable. How minimal can a functional cryptocurrency be? Is it less than 1000 lines? A simple currency could be disruptive, particularly in a post-2017-environment where the currency with the most features doesn't necessarily win. Trust that there are thousands of eyes on an implementation rather than three to ten insiders may be more important than feature count.

Will a simple currency unseat bitcoin? Probably not. But it will be more resistant to disaster than one that needs investor excitement to survive and there are many (many) of those.</li>

<li>The mining reward for this blockchain is the right to write a small bit of text to a block. This removes a lot of (valuable) concepts like addresses, transactions, keys, but it also might be interesting on its own. If the economics of it could be made practical, it might be used in publishing, social media, legal chain of custody, censorship resistance, or public disclosures.</li>
</ol>

_Contributions, questions, corrections welcome._
